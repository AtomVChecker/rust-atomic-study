use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use crossbeam_channel::{unbounded,bounded};
use crossbeam_utils::thread::scope;

fn mpmcunbounded() {
    const COUNT: usize = 800;
    const THREADS: usize = 10000;
    let (s, r) = unbounded::<usize>();
    let v = AtomicUsize::new(0);
    scope(|scope| {
        for i in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..COUNT {
                    s.send(i).expect("Failed to send data");
                }
            });
        }
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..COUNT {
                    let received_data = r.recv().unwrap();
                    v.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    }).unwrap();
    assert_eq!(v.load(Ordering::SeqCst), 8000000);
}
fn mpmcbounded() {
    const COUNT: usize = 1000;
    const THREADS: usize = 2000;
    let (s, r) = bounded::<usize>(10);
    let v = AtomicUsize::new(0);
    scope(|scope| {
        for i in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..COUNT {
                    s.send(i).expect("Failed to send data");
                }
            });
        }
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..COUNT {
                    let received_data = r.recv().unwrap();
                    v.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    }).unwrap();
    assert_eq!(v.load(Ordering::SeqCst), 2000000);
}

fn main() {
    let start_time = Instant::now();
    mpmcunbounded();
    let end_time = Instant::now();
    let elapsed = end_time - start_time;
    println!("time_:{:?}", elapsed);
}