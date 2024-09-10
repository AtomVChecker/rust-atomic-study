# Test the performance gap between different memory ordering in a real-world program.

To increase the accuracy of our experimental results, we set up an environment with high data contention and use repeated measurements with averaging to mitigate random errors. We decide to run ten experiments with thread counts of 10, 100, 1,000, 10,000, and 100,000, each of which performs 100 operations. Despite these measures, the results still show some variation due to the randomness of compiler optimizations and instruction reordering.

## Usages

```cd section-4-atomic-usage/section-4-3-ordering-performance-issues/crossbeam/crossbeam-skiplist```

```cargo run <test_runs> <thread_count> <count>```

* `test_runs` is the number of test iterations to perform

* `thread_count` is the total number of threads to spawn for each test


* `count` is the number of operations each thread will perform


## Result

arm_AcqRel_vs_Relaxed = [0.68, 1.51, 5.03, 6.24, 7.18]
arm_86_SeqCst_vs_Relaxed = [1.62, 2.59, 7.94, 9.48, 12.54]
arm_SeqCst_vs_AcqRel = [0.93, 1.06, 2.77, 2.86, 5.00]


## Configuration

Chipset Model: Apple M1

Type: GPU

Bus: Built-In

Total Number of Cores: 7

Vendor: Apple (0x106b)

Metal Family: Supported, Metal GPUFamily Apple 7
