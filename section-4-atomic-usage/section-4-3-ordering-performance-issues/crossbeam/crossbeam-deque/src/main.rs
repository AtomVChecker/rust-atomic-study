use crossbeam_deque::Injector;
use std::thread;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, Mutex};
use crossbeam_utils::thread::scope;

use crossbeam_deque::Steal::{Empty, Success};
use std::time::Instant;
use crossbeam_deque::Worker;

fn mpsc() {
    const COUNT: usize = 500;
    // const COUNT: usize = 25_000;
    const THREADS: usize = 7000;

    let q = Injector::new();
    // let v = (0..COUNT).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();
    let v = AtomicUsize::new(0);
    scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..COUNT {
                    q.push(i);
                }
            });
        }

        for _ in 0..THREADS {
            scope.spawn(|_| {
                for _ in 0..COUNT {
                    loop {
                        if let Success(n) = q.steal() {
                            // v[1].fetch_add(1, SeqCst);
                            v.fetch_add(1, SeqCst);
                            break;
                        }
                        #[cfg(miri)]
                        std::hint::spin_loop();
                    }
                }
            });
        }
    })
    .unwrap();

    // for c in v {
    //     assert_eq!(c.load(SeqCst), THREADS);
    // }
    assert_eq!(v.load(SeqCst), THREADS * COUNT);
}

fn stampede() {
    const THREADS: usize = 10000;
    // #[cfg(miri)]
    // const COUNT: usize = 500;
    // #[cfg(not(miri))]
    const COUNT: usize = 5000;

    let q = Injector::new();

    for i in 0..COUNT {
        q.push(Box::new(i + 1));
    }
    let remaining = Arc::new(AtomicUsize::new(COUNT));

    scope(|scope| {
        for _ in 0..THREADS {
            let remaining = remaining.clone();
            let q = &q;

            scope.spawn(move |_| {
                let mut last = 0;
                while remaining.load(SeqCst) > 0 {
                    if let Success(x) = q.steal() {
                        assert!(last < *x);
                        last = *x;
                        remaining.fetch_sub(1, SeqCst);
                    }
                }
            });
        }

        let mut last = 0;
        while remaining.load(SeqCst) > 0 {
            if let Success(x) = q.steal() {
                assert!(last < *x);
                last = *x;
                remaining.fetch_sub(1, SeqCst);
            }
        }
    })
    .unwrap();
}

fn stampede_lifo() {
    const THREADS: usize = 10000;
    // #[cfg(miri)]
    // const COUNT: usize = 500;
    // #[cfg(not(miri))]
    const COUNT: usize = 50000;

    let w = Worker::new_lifo();

    for i in 0..COUNT {
        w.push(Box::new(i + 1));
    }
    let remaining = Arc::new(AtomicUsize::new(COUNT));

    scope(|scope| {
        for _ in 0..THREADS {
            let s = w.stealer();
            let remaining = remaining.clone();

            scope.spawn(move |_| {
                let mut last = 0;
                while remaining.load(SeqCst) > 0 {
                    if let Success(x) = s.steal() {
                        assert!(last < *x);
                        last = *x;
                        remaining.fetch_sub(1, SeqCst);
                    }
                }
            });
        }

        let mut last = COUNT + 1;
        while remaining.load(SeqCst) > 0 {
            if let Some(x) = w.pop() {
                assert!(last > *x);
                last = *x;
                remaining.fetch_sub(1, SeqCst);
            }
        }
    })
    .unwrap();
}

fn main() {
    
    let start_time = Instant::now();
    mpsc();
    let end_time = Instant::now();
    let elapsed = end_time - start_time;
    println!("time_:{:?}", elapsed);
    println!("time_:");
}
