use std::sync::mpsc::channel;
use crossbeam_skiplist::{SkipList, SkipListAcqRel, SkipListSeqCst};
use crossbeam_epoch as epoch;
use std::time::{Duration, Instant};
use std::thread;
use rand::random;

use std::sync::{Arc, Mutex};
fn run_test_seqcst(thread_count: usize, count: usize) -> Duration {
    let s = Arc::new(SkipListSeqCst::new(epoch::default_collector().clone()));
    let start_time = Instant::now();

    let producer_handles: Vec<_> = (0..thread_count).map(|_| {
        let s = s.clone();
        thread::spawn(move || {
            let guard = &epoch::pin();
            for i in 0..count {
                s.insert(i, random::<i32>(), guard);
            }
        })
    }).collect();

    let consumer_handles: Vec<_> = (0..thread_count).map(|_| {
        let s = s.clone();
        thread::spawn(move || {
        let guard = &epoch::pin();
            for i in 0..count {
                if let Some(e) = s.get(&i, guard) {
                    if let Some(next) = e.next() {
                        let x = *next.value();
                    }
                    if let Some(prev) = e.prev() {
                        let x = *prev.value();
                    }
                }
            }
        })
    }).collect();

    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    start_time.elapsed()
}

fn run_test_acqrel(thread_count: usize, count: usize) -> Duration {
    let s = Arc::new(SkipListAcqRel::new(epoch::default_collector().clone()));
    let start_time = Instant::now();

    let producer_handles: Vec<_> = (0..thread_count).map(|_| {
        let s = s.clone();
        thread::spawn(move || {
            let guard = &epoch::pin();
            for i in 0..count {
                s.insert(i, random::<i32>(), guard);
            }
        })
    }).collect();

    let consumer_handles: Vec<_> = (0..thread_count).map(|_| {
        let s = s.clone();
        thread::spawn(move || {
            let guard = &epoch::pin();

            for i in 0..count {
                if let Some(e) = s.get(&i, guard) {
                    if let Some(next) = e.next() {
                        let x = *next.value();
                    }
                    if let Some(prev) = e.prev() {
                        let x = *prev.value();
                    }
                }
            }
        })
    }).collect();


    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    start_time.elapsed()

}

fn run_test_normal(thread_count: usize, count: usize) -> Duration {

    let s = Arc::new(SkipList::new(epoch::default_collector().clone()));
    let start_time = Instant::now();

    let producer_handles: Vec<_> = (0..thread_count).map(|_| {
        let s = s.clone();
        thread::spawn(move || {
            let guard = &epoch::pin();
            for i in 0..count {
                s.insert(i, random::<i32>(), guard);
            }
        })
    }).collect();

    let consumer_handles: Vec<_> = (0..thread_count).map(|_| {
        let s = s.clone();
        thread::spawn(move || {
            let guard = &epoch::pin();
            for i in 0..count {
                if let Some(e) = s.get(&i, guard) {
                    if let Some(next) = e.next() {
                        let x = *next.value();
                    }
                    if let Some(prev) = e.prev() {
                        let x = *prev.value();
                    }
                }
            }
            
        })
    }).collect();


    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    start_time.elapsed()
}

fn calculate_median_and_average(durations: Vec<Duration>) -> (Duration, Duration) {
    let mut durations_sorted = durations.clone();
    durations_sorted.sort();
    let median = durations_sorted[durations_sorted.len() / 2];
    let average = durations.iter().sum::<Duration>() / durations.len() as u32;
    (median, average)
}

fn main() {
    // Number of tests
    let test_runs = 10;
    // total number of threads
    let thread_count = 10;
    // Number of operations per thread
    let count = 10;


    let mut durations_normal: Vec<Duration> = Vec::new();
    let mut durations_acqrel: Vec<Duration> = Vec::new();
    let mut durations_seqcst: Vec<Duration> = Vec::new();

    for _ in 0..test_runs {
        durations_normal.push(run_test_normal(thread_count, count));
        thread::sleep(Duration::from_secs(3));
        durations_acqrel.push(run_test_acqrel(thread_count, count));
        thread::sleep(Duration::from_secs(3));
        durations_seqcst.push(run_test_seqcst(thread_count, count));
    }

    let (median_normal, average_normal) = calculate_median_and_average(durations_normal);

    let (median_acqrel, average_acqrel) = calculate_median_and_average(durations_acqrel);
    
    let (median_seqcst, average_seqcst) = calculate_median_and_average(durations_seqcst);

    
    println!("Median Normal: {:?}", median_normal);
    println!("Median AcqRel: {:?}", median_acqrel);
    println!("Median SeqCst: {:?}", median_seqcst);

    println!("Average Normal: {:?}", average_normal);
    println!("Average AcqRel: {:?}", average_acqrel);    
    println!("Average SeqCst: {:?}", average_seqcst);

    // Calculate and print performance gaps
    let diff_median_percent = ((median_seqcst.as_millis() as f64 - median_normal.as_millis() as f64 ) / median_normal.as_millis() as f64) * 100.0;
    let diff_average_percent = ((average_seqcst.as_millis() as f64 - average_normal.as_millis() as f64 ) / average_normal.as_millis() as f64) * 100.0;

    // Calculating the performance gap between AcqRel and Normal
    let diff_median_acqrel_normal_percent = ((median_acqrel.as_millis() as f64 - median_normal.as_millis() as f64) / median_normal.as_millis() as f64) * 100.0;
    let diff_average_acqrel_normal_percent = ((average_acqrel.as_millis() as f64 - average_normal.as_millis() as f64) / average_normal.as_millis() as f64) * 100.0;

    // Calculating the performance gap between AcqRel and SeqCst
    let diff_median_acqrel_seqcst_percent = ((median_seqcst.as_millis() as f64 - median_acqrel.as_millis() as f64) / median_acqrel.as_millis() as f64) * 100.0;
    let diff_average_acqrel_seqcst_percent = ((average_seqcst.as_millis() as f64 - average_acqrel.as_millis() as f64) / average_acqrel.as_millis() as f64) * 100.0;

    println!("Median performance difference (Normal vs AcqRel): {:.2}%", diff_median_acqrel_normal_percent);
    println!("Median performance difference (Normal vs SeqCst): {:.2}%", diff_median_percent);
    println!("Median performance difference (AcqRel vs SeqCst): {:.2}%", diff_median_acqrel_seqcst_percent);
    
    
    println!("Average performance difference (Normal vs AcqRel): {:.2}%", diff_average_acqrel_normal_percent);
    println!("Average performance difference (Normal vs SeqCst): {:.2}%", diff_average_percent);
    println!("Average performance difference (AcqRel vs SeqCst): {:.2}%", diff_average_acqrel_seqcst_percent);

    
}

