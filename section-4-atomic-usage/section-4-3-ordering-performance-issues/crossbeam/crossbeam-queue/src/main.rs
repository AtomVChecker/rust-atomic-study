// // use std::sync::atomic::{AtomicUsize, Ordering};

// // use crossbeam_queue::SegQueue;
// // use crossbeam_utils::thread::scope;
// // // use rand::{thread_rng, Rng};
// // use std::time::Instant;

// // fn mpmc() {
// //     // #[cfg(miri)]
// //     const COUNT: usize = 1000;
// //     // #[cfg(not(miri))]
// //     // const COUNT: usize = 25_000;
// //     const THREADS: usize = 2000;

// //     let q = SegQueue::<usize>::new();
// //     let v = (0..COUNT).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();

// //     scope(|scope| {
// //         for _ in 0..THREADS {
// //             scope.spawn(|_| {
// //                 for _ in 0..COUNT {
// //                     let n = loop {
// //                         if let Some(x) = q.pop() {
// //                             break x;
// //                         }
// //                     };
// //                     v[n].fetch_add(1, Ordering::SeqCst);
// //                 }
// //             });
// //         }
// //         for _ in 0..THREADS {
// //             scope.spawn(|_| {
// //                 for i in 0..COUNT {
// //                     q.push(i);
// //                 }
// //             });
// //         }
// //     })
// //     .unwrap();

// //     for c in v {
// //         assert_eq!(c.load(Ordering::SeqCst), THREADS);
// //     }
// // }

// // fn main() {
// //     let start_time = Instant::now();
// //     mpmc();
// //     let end_time = Instant::now();
// //     let elapsed = end_time - start_time;
// //     println!("time_:{:?}", elapsed);
// // }

// // use crossbeam_queue::SegQueue;
// // use std::sync::Arc;
// // use std::thread;
// // use std::time::{Duration, Instant};

// // #[derive(Clone)]
// // struct TaskConfig {
// //     repeat_count: u64,
// //     workers: usize,
// //     test_runs: usize,
// // }

// // fn push_to_queue(count: u64, queue: Arc<SegQueue<u64>>) -> thread::JoinHandle<()> {
// //     thread::spawn(move || {
// //         for _ in 0..count {
// //             queue.push(0);
// //         }
// //     })
// // }

// // fn queue_performance_test(config: TaskConfig) -> Duration {
// //     let mut total_duration = Duration::new(0, 0);

// //     for _ in 0..config.test_runs {
// //         let queue = Arc::new(SegQueue::new());
// //         let mut threads = Vec::with_capacity(config.workers);
// //         let start_time = Instant::now();

// //         for _ in 0..config.workers {
// //             threads.push(push_to_queue(config.repeat_count, queue.clone()));
// //         }

// //         for thread in threads {
// //             thread.join().unwrap();
// //         }

// //         total_duration += start_time.elapsed();
// //     }

// //     // 计算平均时间
// //     total_duration / config.test_runs as u32
// // }

// // fn compare_performance(config: TaskConfig) {
// //     let average_duration = queue_performance_test(config.clone());
// //     println!("Test: Repeat Count = {}, Workers = {}, Average Duration = {:?}", config.repeat_count, config.workers, average_duration);
// // }

// // fn main() {
// //     let test_configurations = [
// //         TaskConfig { repeat_count: 100, workers: 10, test_runs: 200 },
// //         TaskConfig { repeat_count: 1000, workers: 10, test_runs: 200 },
// //         TaskConfig { repeat_count: 10000, workers: 10, test_runs: 200 },

// //         TaskConfig { repeat_count: 100, workers: 100, test_runs: 200 },
// //         TaskConfig { repeat_count: 1000, workers: 100, test_runs: 200 },
// //         TaskConfig { repeat_count: 10000, workers: 100, test_runs: 200 },

// //         // TaskConfig { repeat_count: 100, workers: 1000, test_runs: 100 },
// //         // TaskConfig { repeat_count: 1000, workers: 1000, test_runs: 100 },
// //         // TaskConfig { repeat_count: 10000, workers: 1000, test_runs: 100 },

// //         // TaskConfig { repeat_count: 100, workers: 10000, test_runs: 10 },
// //         // TaskConfig { repeat_count: 1000, workers: 10000, test_runs: 10 },
// //     ];

// //     for config in test_configurations.iter() {
// //         compare_performance(config.clone());
// //     }
// // }



// use crossbeam_queue::SegQueue; // 原始的SegQueue
// use crossbeam_queue::SegQueue1; // 修改后的SegQueue1
// use std::sync::Arc;
// use std::thread;
// use std::time::{Duration, Instant};


// #[derive(Clone)]
// struct TaskConfig {
//     repeat_count: u64,
//     workers: usize,
//     test_runs: usize,
// }

// fn push_to_queue<T>(count: u64, queue: Arc<T>) -> thread::JoinHandle<()> 
// where
//     T: 'static + Send + Sync + for<'a> Fn(u64),
// {
//     thread::spawn(move || {
//         for i in 0..count {
//             (queue)(i);
//         }
//     })
// }

// fn queue_performance_test<T>(config: TaskConfig, queue_constructor: fn() -> T) -> Duration 
// where
//     T: 'static + Send + Sync + for<'a> Fn(u64),
// {
//     let mut total_duration = Duration::new(0, 0);

//     for _ in 0..config.test_runs {
//         let queue = Arc::new(queue_constructor());
//         let mut threads = Vec::with_capacity(config.workers);
//         let start_time = Instant::now();

//         for _ in 0..config.workers {
//             threads.push(push_to_queue(config.repeat_count, queue.clone()));
//         }

//         for thread in threads {
//             thread.join().unwrap();
//         }

//         total_duration += start_time.elapsed();
//     }

//     total_duration / config.test_runs as u32
// }

// fn compare_performance(config: TaskConfig) {
//     let original_queue_test = || {
//         let queue = SegQueue::new();
//         move |x| { queue.push(x); }
//     };

//     let modified_queue_test = || {
//         let queue = SegQueue1::new();
//         move |x| { queue.push(x); }
//     };
//     // println!("Starting 10 second sleep...");
//     // thread::sleep(Duration::from_secs(5));
//     // println!("Sleep completed.");

//     let average_duration_modified = queue_performance_test(config.clone(), modified_queue_test);
//     // println!("Starting 10 second sleep...");
//     // thread::sleep(Duration::from_secs(3));
//     // println!("Sleep completed.");

//     let average_duration_original = queue_performance_test(config.clone(), original_queue_test);
    
    

//     // let ratio = 1.0 - average_duration_original.as_secs_f64() / average_duration_modified.as_secs_f64();

//     let performance_difference = (average_duration_modified.as_secs_f64() / average_duration_original.as_secs_f64() - 1.0) * 100.0;
//     // println!("Test: Repeat Count = {}, Workers = {}, Average Duration Original = {:?}, Average Duration Modified = {:?}, Ratio = {:.2}", 
//     //          config.repeat_count, config.workers, average_duration_original, average_duration_modified, ratio * 100 as f64);
//     println!("Test: Repeat Count = {}, Workers = {}, Average Duration Original = {:?}, Average Duration Modified = {:?}, Performance Difference = {:.2}%", 
//     config.repeat_count, config.workers, average_duration_original, average_duration_modified, performance_difference);

// }

// fn main() {
//     let test_configurations = [
//         // TaskConfig { repeat_count: 100, workers: 10, test_runs: 100 },
//         // TaskConfig { repeat_count: 1000, workers: 10, test_runs: 100 },
//         // TaskConfig { repeat_count: 10000, workers: 10, test_runs: 100 },

//         // TaskConfig { repeat_count: 100, workers: 100, test_runs: 100 },
//         // TaskConfig { repeat_count: 1000, workers: 100, test_runs: 100 },
//         // TaskConfig { repeat_count: 10000, workers: 100, test_runs: 100 },

//         // TaskConfig { repeat_count: 100, workers: 10000, test_runs: 10 },
//         // TaskConfig { repeat_count: 1000, workers: 10000, test_runs: 10 },
//         // TaskConfig { repeat_count: 10000, workers: 1000, test_runs: 50 },
//         // ... 同样的配置
//     ];

//     for config in test_configurations.iter() {
//         compare_performance(config.clone());
//     }
// }

// use crossbeam_queue::{SegQueue, SegQueue1}; // 假设 SegQueue1 是您修改后的队列
// use std::sync::Arc;
// use std::thread;
// use std::time::Instant;

// struct TaskConfig {
//     total_operations: u64,
//     thread_count: usize,
//     test_runs: usize,
// }

// fn producer<T>(queue: Arc<T>, operations: u64)
// where
//     T: Sync + Send + QueueOps,
// {
//     for _ in 0..operations {
//         queue.push(1);
//     }
// }

// fn consumer<T>(queue: Arc<T>, operations: u64)
// where
//     T: Sync + Send + QueueOps,
// {
//     for _ in 0..operations {
//         queue.pop();
//     }
// }

// trait QueueOps {
//     fn push(&self, value: u64);
//     fn pop(&self);
// }

// impl QueueOps for SegQueue<u64> {
//     fn push(&self, value: u64) {
//         self.push(value);
//     }

//     fn pop(&self) {
//         self.pop();
//     }
// }

// impl QueueOps for SegQueue1<u64> {
//     fn push(&self, value: u64) {
//         self.push(value);
//     }

//     fn pop(&self) {
//         self.pop();
//     }
// }

// fn performance_test<T>(config: &TaskConfig, queue_constructor: fn() -> Arc<T>) -> f64
// where
//     T: 'static + Sync + Send + QueueOps,
// {
//     let mut total_duration = 0.0;

//     for _ in 0..config.test_runs {
//         let queue = queue_constructor();
//         let mut handles = vec![];
//         let operations_per_thread = config.total_operations / (2 * config.thread_count) as u64; // 分为 producer 和 consumer

//         // 创建 producer 线程
//         for _ in 0..config.thread_count {
//             let queue_clone = queue.clone();
//             handles.push(thread::spawn(move || {
//                 producer(queue_clone, operations_per_thread);
//             }));
//         }

//         // 创建 consumer 线程
//         for _ in 0..config.thread_count {
//             let queue_clone = queue.clone();
//             handles.push(thread::spawn(move || {
//                 consumer(queue_clone, operations_per_thread);
//             }));
//         }

//         let start_time = Instant::now();
//         for handle in handles {
//             handle.join().unwrap();
//         }

//         total_duration += start_time.elapsed().as_secs_f64();
//     }

//     total_duration / config.test_runs as f64
// }

// fn main() {
//     let test_configurations = [
//         TaskConfig { total_operations: 10000, thread_count: 5, test_runs: 10 }, // 确保线程总数相同
//         TaskConfig { total_operations: 10000, thread_count: 100, test_runs: 10 }, 
//         TaskConfig { total_operations: 10000, thread_count: 1000, test_runs: 10 }, 
//         TaskConfig { total_operations: 10000, thread_count: 10000, test_runs: 10 }, 
//         // 可以添加更多测试配置...
//     ];

//     for config in test_configurations.iter() {
//         let average_duration_segqueue = performance_test(config, || Arc::new(SegQueue::new()));
//         let average_duration_segqueue1 = performance_test(config, || Arc::new(SegQueue1::new()));

//         let performance_difference = (average_duration_segqueue1 - average_duration_segqueue) / average_duration_segqueue * 100.0;
//         println!("Config: Operations = {}, Threads = {}, Test Runs = {}, Average Duration SegQueue: {:.4}s, Average Duration SegQueue1: {:.4}s, Performance Difference: {:.2}%", 
//                  config.total_operations, config.thread_count * 2, config.test_runs, average_duration_segqueue, average_duration_segqueue1, performance_difference);
//     }
// }


// use crossbeam_queue::{SegQueue, SegQueue1};
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

// struct TaskConfig {
//     total_operations: usize,
//     producer_count: usize,
//     consumer_count: usize,
//     test_runs: usize,
// }

// // fn performance_test<T: 'static + Send + Sync + QueueOps>(config: &TaskConfig, queue_constructor: impl Fn() -> T + Sync) -> Duration {
// //     let mut total_duration = Duration::new(0, 0);

// //     for _ in 0..config.test_runs {
// //         let queue = Arc::new(queue_constructor());
// //         let mut handles = vec![];

// //         let start = std::time::Instant::now();
// //         for _ in 0..config.producer_count {
// //             let queue_clone = queue.clone();
// //             let operations = config.total_operations / config.producer_count;
// //             handles.push(thread::spawn(move || {
// //                 for _ in 0..operations {
// //                     queue_clone.push(1);
// //                 }
// //             }));
// //         }

// //         for _ in 0..config.consumer_count {
// //             let queue_clone = queue.clone();
// //             let operations = config.total_operations / config.consumer_count;
// //             handles.push(thread::spawn(move || {
// //                 for _ in 0..operations {
// //                     while queue_clone.pop().is_none() {
// //                         thread::yield_now();
// //                     }
// //                 }
// //             }));
// //         }

        
// //         for handle in handles {
// //             handle.join().unwrap();
// //         }
// //         total_duration += start.elapsed();
// //     }

// //     total_duration / config.test_runs as u32
// // }

// fn performance_test<T: 'static + Send + Sync + QueueOps>(config: &TaskConfig, queue_constructor: impl Fn() -> T + Sync) -> Duration {
//     let mut total_duration = Duration::new(0, 0);

//     for _ in 0..config.test_runs {
//         let queue = Arc::new(queue_constructor());
//         let mut handles = vec![];

//         // 启动计时器
//         let start = std::time::Instant::now();

//         for _ in 0..config.producer_count {
//             let queue_clone = queue.clone();
//             let operations = config.total_operations / config.producer_count;
//             handles.push(thread::spawn(move || {
//                 for _ in 0..operations {
//                     queue_clone.push(1);
//                 }
//             }));
//         }

//         for _ in 0..config.consumer_count {
//             let queue_clone = queue.clone();
//             let operations = config.total_operations / config.consumer_count;
//             handles.push(thread::spawn(move || {
//                 for _ in 0..operations {
//                     while queue_clone.pop().is_none() {
//                         thread::yield_now();
//                     }
//                 }
//             }));
//         }

//         for handle in handles {
//             handle.join().unwrap();
//         }

//         total_duration += start.elapsed();
//     }

//     total_duration / config.test_runs as u32
// }

// trait QueueOps {
//     fn push(&self, value: usize);
//     fn pop(&self) -> Option<usize>;
// }

// impl QueueOps for SegQueue<usize> {
//     fn push(&self, value: usize) {
//         self.push(value);
//     }

//     fn pop(&self) -> Option<usize> {
//         self.pop()
//     }
// }

// impl QueueOps for SegQueue1<usize> {
//     fn push(&self, value: usize) {
//         self.push(value);
//     }

//     fn pop(&self) -> Option<usize> {
//         self.pop()
//     }
// }

// fn main() {
//     let test_configurations = [
//         TaskConfig { total_operations: 1000000, producer_count: 10, consumer_count: 10, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 10, consumer_count: 100, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 100, consumer_count: 10, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 100, consumer_count: 100, test_runs: 50 },// 添加更多测试配置...
//         TaskConfig { total_operations: 1000000, producer_count: 1000, consumer_count: 100, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 100, consumer_count: 1000, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 1000, consumer_count: 1000, test_runs: 50 },
//     ];

//     for config in test_configurations.iter() {
//         let duration_segqueue = performance_test(config, || SegQueue::new());
//         let duration_segqueue1 = performance_test(config, || SegQueue1::new());

//         let performance_difference = (duration_segqueue1.as_secs_f64() - duration_segqueue.as_secs_f64()) / duration_segqueue.as_secs_f64() * 100.0;
//         println!("Config: Operations = {}, Producers = {}, Consumers = {}, Test Runs = {}, Duration SegQueue: {:?}, Duration SegQueue1: {:?}, Performance Difference: {:.2}%", 
//                  config.total_operations, config.producer_count, config.consumer_count, config.test_runs, duration_segqueue, duration_segqueue1, performance_difference);
//     }
// }





// use crossbeam_queue::{SegQueue, SegQueue1};
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

// struct TaskConfig {
//     total_operations: usize,
//     producer_count: usize,
//     consumer_count: usize,
//     test_runs: usize,
// }

// fn performance_test<T: 'static + Send + Sync + QueueOps>(config: &TaskConfig, queue_constructor: impl Fn() -> T + Sync) -> Vec<Duration> {
//     let mut durations = Vec::new();

//     for _ in 0..config.test_runs {
//         let queue = Arc::new(queue_constructor());
//         let mut handles = vec![];

//         // Start timer
//         let start = std::time::Instant::now();

//         // Producer threads
//         for _ in 0..config.producer_count {
//             let queue_clone = queue.clone();
//             let operations = config.total_operations / config.producer_count;
//             handles.push(thread::spawn(move || {
//                 for _ in 0..operations {
//                     queue_clone.push(1);
//                 }
//             }));
//         }

//         // Consumer threads
//         for _ in 0..config.consumer_count {
//             let queue_clone = queue.clone();
//             let operations = config.total_operations / config.consumer_count;
//             handles.push(thread::spawn(move || {
//                 loop {
//                     if let Some(_) = queue_clone.pop() {
//                         break;
//                     }
//                     thread::yield_now();
//                 }
//             }));
//         }

//         for handle in handles {
//             handle.join().unwrap();
//         }

//         durations.push(start.elapsed());
//     }

//     durations
// }

// fn calculate_median(durations: &mut [Duration]) -> Duration {
    
    
//     durations.sort();
//     let total_duration: Duration = durations.iter().sum();
//     total_duration / durations.len() as u32;
//     println!(":{:?}", total_duration);

//     let mid = durations.len() / 2;
//     durations[mid]
// }

// // fn calculate_average(durations: &[Duration]) -> Duration {
// //     let total_duration: Duration = durations.iter().sum();
// //     total_duration / durations.len() as u32
// // }

// trait QueueOps {
//     fn push(&self, value: usize);
//     fn pop(&self) -> Option<usize>;
// }

// impl QueueOps for SegQueue<usize> {
//     fn push(&self, value: usize) {
//         self.push(value);
//     }

//     fn pop(&self) -> Option<usize> {
//         self.pop()
//     }
// }

// impl QueueOps for SegQueue1<usize> {
//     fn push(&self, value: usize) {
//         self.push(value);
//     }

//     fn pop(&self) -> Option<usize> {
//         self.pop()
//     }
// }

// fn main() {
//     let test_configurations = [
//         // 同样的测试配置...
//         TaskConfig { total_operations: 1000000, producer_count: 10, consumer_count: 10, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 10, consumer_count: 100, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 100, consumer_count: 10, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 100, consumer_count: 100, test_runs: 50 },// 添加更多测试配置...
//         TaskConfig { total_operations: 1000000, producer_count: 1000, consumer_count: 100, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 100, consumer_count: 1000, test_runs: 50 },
//         TaskConfig { total_operations: 1000000, producer_count: 1000, consumer_count: 1000, test_runs: 50 },
//     ];

//     for config in test_configurations.iter() {
//         let mut durations_segqueue = performance_test(config, || SegQueue::new());
//         let mut durations_segqueue1 = performance_test(config, || SegQueue1::new());

//         let median_duration_segqueue = calculate_median(&mut durations_segqueue);
//         let median_duration_segqueue1 = calculate_median(&mut durations_segqueue1);

//         let performance_difference = (median_duration_segqueue1.as_secs_f64() - median_duration_segqueue.as_secs_f64()) / median_duration_segqueue.as_secs_f64() * 100.0;
//         println!("Config: Operations = {}, Producers = {}, Consumers = {}, Test Runs = {}, Median Duration SegQueue: {:?}, Median Duration SegQueue1: {:?}, Performance Difference: {:.2}%", 
//                  config.total_operations, config.producer_count, config.consumer_count, config.test_runs, median_duration_segqueue, median_duration_segqueue1, performance_difference);
//     }
// }


// 修改mpmc
// use crossbeam_queue::{SegQueue, SegQueue1};
// use crossbeam_utils::thread::scope;
// use std::sync::{Arc, Mutex};
// use std::time::{Duration, Instant};

// trait QueueOps {
//     fn new() -> Self where Self: Sized;
//     fn push(&self, value: usize);
//     fn pop(&self) -> Option<usize>;
// }

// impl QueueOps for SegQueue<usize> {
//     fn new() -> Self {
//         SegQueue::new()
//     }

//     fn push(&self, value: usize) {
//         self.push(value);
//     }

//     fn pop(&self) -> Option<usize> {
//         self.pop()
//     }
// }

// impl QueueOps for SegQueue1<usize> {
//     fn new() -> Self {
//         SegQueue1::new()
//     }

//     fn push(&self, value: usize) {
//         self.push(value);
//     }

//     fn pop(&self) -> Option<usize> {
//         self.pop()
//     }
// }

// // fn run_test<T: 'static + QueueOps + Sync + Send>(thread_count: usize, count: usize, test_runs: usize) -> Vec<Duration> {
// //     let mut durations = Vec::new();

// //     for _ in 0..test_runs {
// //         let queue = Arc::new(T::new());
// //         let start = Instant::now();

// //         scope(|s| {
// //             for _ in 0..thread_count {
// //                 let q_clone = queue.clone();
// //                 s.spawn(move |_| {
// //                     for _ in 0..count {
// //                         q_clone.push(1);
// //                     }
// //                 });
// //             }

// //             for _ in 0..thread_count {
// //                 let q_clone = queue.clone();
// //                 s.spawn(move |_| {
// //                     for _ in 0..count {
// //                         while q_clone.pop().is_none() {}
// //                     }
// //                 });
// //             }
// //         }).unwrap();

// //         durations.push(start.elapsed());
// //     }

// //     durations
// // }

// fn run_test<T: 'static + QueueOps + Sync + Send>(thread_count: usize, count: usize, test_runs: usize) -> Vec<Duration> {
//     let mut durations = Vec::new();

//     for _ in 0..test_runs {
//         let queue = Arc::new(T::new());
//         let accumulated = Arc::new(Mutex::new(0));
//         let start = Instant::now();

//         scope(|s| {
//             for _ in 0..thread_count {
//                 let q_clone = queue.clone();
//                 s.spawn(move |_| {
//                     for _ in 0..count {
//                         q_clone.push(1);
//                     }
//                 });
//             }

//             for _ in 0..thread_count {
//                 let q_clone = queue.clone();
//                 let acc_clone = accumulated.clone();
//                 s.spawn(move |_| {
//                     for _ in 0..count {
//                         if let Some(value) = q_clone.pop() {
//                             let mut acc = acc_clone.lock().unwrap();
//                             *acc += value;
//                         }
//                     }
//                 });
//             }
//         }).unwrap();

//         durations.push(start.elapsed());
//     }

//     durations
// }


// fn calculate_median(durations: &mut [Duration]) -> Duration {
//     durations.sort();
//     durations[durations.len() / 2]
// }

// fn calculate_average(durations: &[Duration]) -> Duration {
//     durations.iter().sum::<Duration>() / durations.len() as u32
// }

// fn main() {
//     let test_configurations = [
//         // (10, 1000),
//         // (100, 1000),
//         // (1000, 1000),
//         (1, 1000),
//         (10, 1000),
//         (100, 1000),
//         (1000, 1000),
//         (10000, 1000),
//         // 更多配置...
//     ];

//     for (thread_count, count) in test_configurations.iter() {
//         let test_runs = 10; // 测试运行次数

//         let mut durations_segqueue1 = run_test::<SegQueue1<usize>>(*thread_count, *count, test_runs);
//         let mut durations_segqueue = run_test::<SegQueue<usize>>(*thread_count, *count, test_runs);
            
//         let median_segqueue = calculate_median(&mut durations_segqueue);
//         let average_segqueue = calculate_average(&durations_segqueue);
    
//         let median_segqueue1 = calculate_median(&mut durations_segqueue1);
//         let average_segqueue1 = calculate_average(&durations_segqueue1);
    
//         // 计算中位数和平均时间的性能差异百分比
//         let median_diff_percentage = (median_segqueue1.as_secs_f64() - median_segqueue.as_secs_f64()) / median_segqueue.as_secs_f64() * 100.0;
//         let average_diff_percentage = (average_segqueue1.as_secs_f64() - average_segqueue.as_secs_f64()) / average_segqueue.as_secs_f64() * 100.0;
    
//         println!("Config: Threads = {}, Count = {}", thread_count, count);
//         println!("SegQueue - Median: {:?}, Average: {:?}", median_segqueue, average_segqueue);
//         println!("SegQueue1 - Median: {:?}, Average: {:?}", median_segqueue1, average_segqueue1);
//         println!("Performance Difference - Median: {:.2}%, Average: {:.2}%", median_diff_percentage, average_diff_percentage);
//     }
    
// }

use crossbeam_queue::{SegQueue, SegQueue1};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

// 定义队列操作的 trait
trait QueueOps {
    fn new() -> Self where Self: Sized;
    fn push(&self, value: usize);
    fn pop(&self) -> Option<usize>;
}

// 为 SegQueue 实现队列操作
impl QueueOps for SegQueue<usize> {
    fn new() -> Self {
        SegQueue::new()
    }

    fn push(&self, value: usize) {
        self.push(value);
    }

    fn pop(&self) -> Option<usize> {
        self.pop()
    }
}

// 为 SegQueue1 实现队列操作
impl QueueOps for SegQueue1<usize> {
    fn new() -> Self {
        SegQueue1::new()
    }

    fn push(&self, value: usize) {
        self.push(value);
    }

    fn pop(&self) -> Option<usize> {
        self.pop()
    }
}

// 测试函数，返回每次测试的持续时间
fn run_test<T: 'static + QueueOps + Sync + Send>(thread_count: usize, count: usize, test_runs: usize) -> Vec<Duration> {
    let mut durations = Vec::with_capacity(test_runs);

    for _ in 0..test_runs {
        let queue = Arc::new(T::new());
        let start = Instant::now();

        // 创建生产者线程
        let mut handles = Vec::with_capacity(thread_count * 2);
        for _ in 0..thread_count {
            let queue_clone = queue.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..count {
                    queue_clone.push(1);
                }
            }));
        }

        // 创建消费者线程
        for _ in 0..thread_count {
            let queue_clone = queue.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..count {
                    while queue_clone.pop().is_none() {}
                }
            }));
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        durations.push(Instant::now() - start);
    }

    durations
}

fn calculate_median(durations: &mut [Duration]) -> Duration {
    durations.sort_unstable();
    durations[durations.len() / 2]
}

fn calculate_average(durations: &[Duration]) -> Duration {
    durations.iter().sum::<Duration>() / durations.len() as u32
}

fn main() {
    let thread_count = 100000;  // 总线程数
    let count = 1000;      // 每个线程的操作数
    let test_runs = 5;     // 测试次数

    let mut durations_segqueue = run_test::<SegQueue<usize>>(thread_count, count, test_runs);
    let mut durations_segqueue1 = run_test::<SegQueue1<usize>>(thread_count, count, test_runs);

    let median_segqueue = calculate_median(&mut durations_segqueue);
    let average_segqueue = calculate_average(&durations_segqueue);

    let median_segqueue1 = calculate_median(&mut durations_segqueue1);
    let average_segqueue1 = calculate_average(&durations_segqueue1);

    // 计算性能差异
    let median_diff = (median_segqueue1.as_secs_f64() - median_segqueue.as_secs_f64()) / median_segqueue.as_secs_f64() * 100.0;
    let average_diff = (average_segqueue1.as_secs_f64() - average_segqueue.as_secs_f64()) / average_segqueue.as_secs_f64() * 100.0;

    println!("SegQueue - Median Duration: {:?}, Average Duration: {:?}", median_segqueue, average_segqueue);
    println!("SegQueue1 - Median Duration: {:?}, Average Duration: {:?}", median_segqueue1, average_segqueue1);
    println!("Performance Difference - Median: {:.2}%, Average: {:.2}%", median_diff, average_diff);
}
