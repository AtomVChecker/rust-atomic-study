use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Instant;

#[derive(Clone)]
struct TaskConfig {
    repeat_count: u64,
    workers: usize,
}

fn increment_with_mutex(count: u64, shared_counter: Arc<Mutex<u64>>) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..count {
            let mut num = shared_counter.lock().unwrap();
            *num += 1;
        }
    })
}

fn increment_with_atomic(count: u64, shared_counter: Arc<AtomicU64>) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..count {
            shared_counter.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn increment_with_channel(count: u64, sender: mpsc::Sender<u64>) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..count {
            sender.send(1).unwrap();
        }
    })
}

fn mutex_performance_test(config: TaskConfig) -> std::time::Duration {
    let shared_counter = Arc::new(Mutex::new(0));
    let mut threads = Vec::with_capacity(config.workers);
    let start_time = Instant::now();

    for _ in 0..config.workers {
        threads.push(increment_with_mutex(config.repeat_count, shared_counter.clone()));
    }

    for thread in threads {
        thread.join().unwrap();
    }
    start_time.elapsed()
}

fn atomic_performance_test(config: TaskConfig) -> std::time::Duration {
    let shared_counter = Arc::new(AtomicU64::new(0));
    let mut threads = Vec::with_capacity(config.workers);
    let start_time = Instant::now();

    for _ in 0..config.workers {
        threads.push(increment_with_atomic(config.repeat_count, shared_counter.clone()));
    }

    for thread in threads {
        thread.join().unwrap();
    }
    start_time.elapsed()
}

fn channel_performance_test(config: TaskConfig) -> std::time::Duration {
    let (sender, receiver) = mpsc::channel();
    let mut threads = Vec::with_capacity(config.workers);
    let start_time = Instant::now();

    for _ in 0..config.workers {
        let sender_clone = sender.clone();
        threads.push(increment_with_channel(config.repeat_count, sender_clone));
    }

    let counter_handle = thread::spawn(move || {
        receiver.iter().take((config.repeat_count * config.workers as u64) as usize).sum::<u64>()
    });

    for thread in threads {
        thread.join().unwrap();
    }
    drop(sender); 

    counter_handle.join().unwrap();
    start_time.elapsed()
}

fn compare_performance(config: TaskConfig) {
    let mut ratio_mutex_atomic = 0f64;
    let mut ratio_channel_atomic = 0f64;

    for _ in 0..10 {
        let mutex_duration = mutex_performance_test(config.clone());
        let atomic_duration = atomic_performance_test(config.clone());
        let channel_duration = channel_performance_test(config.clone());

        ratio_mutex_atomic += (mutex_duration.as_micros() as f64) / (atomic_duration.as_micros() as f64);
        ratio_channel_atomic += (channel_duration.as_micros() as f64) / (atomic_duration.as_micros() as f64);
    }

    ratio_mutex_atomic /= 10.0;
    ratio_channel_atomic /= 10.0;

    println!("Test: Repeat Count = {}, Workers = {}", config.repeat_count, config.workers);
    println!("Mutex/Atomic duration ratio: {:.2}", ratio_mutex_atomic);
    println!("Channel/Atomic duration ratio: {:.2}", ratio_channel_atomic);
}

fn main() {
    let test_configurations = [
        TaskConfig { repeat_count: 100, workers: 10 },
        TaskConfig { repeat_count: 100, workers: 1000 },
        TaskConfig { repeat_count: 500, workers: 10 },
        TaskConfig { repeat_count: 500, workers: 1000 },
        TaskConfig { repeat_count: 800, workers: 10 },
        TaskConfig { repeat_count: 800, workers: 1000 },
        TaskConfig { repeat_count: 1000, workers: 10 },
        TaskConfig { repeat_count: 1000, workers: 1000 },
        TaskConfig { repeat_count: 1500, workers: 10 },
        TaskConfig { repeat_count: 1500, workers: 1000 },
    ];

    for config in test_configurations.iter() {
        compare_performance(config.clone());
    }
}
