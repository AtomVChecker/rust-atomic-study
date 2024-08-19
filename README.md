# The code, analysis scripts and results for ISSRE 2024 Artifact Evaluation
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.13334676.svg)](https://doi.org/10.5281/zenodo.13334676)

Version: 1.1 Paper: Understanding Atomics and Memory Ordering Issues in Real-World Rust Software (#60)

This document is to help users reproduce the results we reported in our submission. It contains the following descriptions

## 1. Artifact Description
The code and scripts for the tools we build are published in this repository. The content is divided into two main sections:
- section-4-atomic-usage: atomic usage in real-world Rust programs(see [atomic-usage-count](section-4-atomic-usage/section-4-1-reason-for-usage/atomic-usage-count)), comparisons of atomic types with other concurrency primitives(see [performance-test](/section-4-atomic-usage/section-4-1-reason-for-usage/performance-test)), and performance gaps across different memory orderings(see [atomic correlations](https://github.com/AtomVChecker/rust-atomic-study/blob/main/section-5-detection/README.md)) 

- section-5-detection: a detailed overview of AtomVChecker, including its code and experimental examples that demonstrate its effectiveness in detecting memory ordering misuses in Rust(see [AtomVChecker](section-5-detection/AtomVChecker))

## 2. Environment Setup

Pull rust-atomic-study from GitHub.
```sh
$ git clone https://github.com/AtomVChecker/rust-atomic-study.git
```

### 2.1 AtomVChecker
Add the Rust components rust-src, rustc-dev and llvm-tools-preview to compile and install AtomVChecker. It currently supports rustc nightly-2023-03-09.

```sh
$ cd AtomVChecker
$ rustup component add rust-src
$ rustup component add rustc-dev
$ rustup component add llvm-tools-preview
$ cargo install --path .
```

### 2.2 Python && Rust
These experiments use Python and Rust, the python version is 3.7.13 and the Rust Compiler (rustc) version is nightly-2023-03-09.


## 3. Getting Started

### AtomVChecker
AtomVChecker is a static analyzer to detect memory ordering misuses in Rust programs. 

#### Example

Test ordering_misuse
```sh
$ ./detect.sh examples/ordering_misuse
```
It will print one performance loss caused by incorrect use of strong memory ordering(SMO) in json format, like the following one:

```
    [
      {
        "AtomicCorrelationViolation": {
          "bug_kind": "AtimicCorrelationViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "atomic": "src/main.rs:21:22: 21:52"
          },
          "explanation": "Using an atomic operation with a stronger memory ordering than necessary can lead to unnecessary performance overhead. Using Relaxed is sufficient to ensure the correctness of the program"
        }
      }
    ]
```

If you want to check for atomic correlations, you can set the `ATOMVCHECKER_LOG` in detexct.sh to debug (default is info), which will output all atomic operations and the corresponding minimum memory ordering requirements

```sh
# debug mode(check for atomic correlations)
$ export ATOMVCHECKER_LOG=debug
```

Each atomic correlation is represented by an AtomicInfo<atomic_place, atomic_value, atomic_operate, caller_instance, ordering, source_info>. For detailed explanations, please see Section V-C of [paper](/paper.pdf). The atomic correlation outputs for ordering_misuse is as follows:
```
    {
      AtomicInfo { 
        atomic_place: Some(_27), 
        atomic_value: [_26], 
        atomic_operate: Some(Load), 
        caller_instance: NodeIndex(53), 
        ordering: [Relaxed], 
        source_info: "src/main.rs:41:50: 41:73"
        }: {Relaxed}, 
      AtomicInfo { 
        atomic_place: Some(_2), 
        atomic_value: [_0], 
        atomic_operate: Some(ReadModifyWrite), 
        caller_instance: NodeIndex(55), 
        ordering: [SeqCst], 
        source_info: "src/main.rs:21:22: 21:52" 
        }: {Relaxed}
    }: 2
```


`detect.sh` is mainly for development of the detector and brings more flexibility.
You can modify `detect.sh` to use release vesion of AtomVChecker to detect large and complex projects.

For ease of use, you can also run cargo atomvchecker
```sh
$ cd examples/ordering_misuse; cargo clean; cargo atomvchecker -k atomicity_violation
```
Note that you need to run
```sh
cargo clean
```
before re-running atomvchecker.

You can also specify blacklist or whitelist of crate names.

The `-b` implies the list is a blacklist.

The `-l` is followed by a list of crate names seperated by commas.
```sh
$ cd YourProject; cargo clean; cargo atomvchecker -k atomicity_violation -b -l cc,tokio_util,indicatif
```

## 4. Reproducibility Instructions

### 4.1 TABLE 3 in Section 3
TABLE 3 mainly shows the atomic usage in real-world Rust programs(For more details, please see [atomic-usage-count](section-4-atomic-usage/section-4-1-reason-for-usage/atomic-usage-count)).

Usage:

```sh
$ cd atomic-usage-count/project
$ git clone https://github.com/AleoNet/snarkOS.git
$ git checkout e7d39272d0c008c6d67cf1fdbf0da5de8b5001f7
$ cd atomic-usage-count
$ python count.py ./project/snarkOS
```


Output:

```
total: 33
field_atomic_operations: 28
global_atomic_operations: 1
other_atomic_operations: 4
Counter({'SeqCst': 20, 'Relaxed': 13})
struct: 0.8484848484848485
global: 0.030303030303030304
other: 0.12121212121212119
{'SeqCst': 0.6060606060606061, 'Relaxed': 0.3939393939393939, 'Acquire/Release': 0.0}
```

### 4.2 Experiments in Section 4.1
These experiments mainly shows the comparisons of atomic types with other concurrency primitives(For more details, please see [performance-test](/section-4-atomic-usage/section-4-1-reason-for-usage/performance-test)).

Usages

```
cd performance-test/src
./run.sh
```

Output
```
Test: Repeat Count = 100, Workers = 10
Mutex/Atomic duration ratio: 3.65
Channel/Atomic duration ratio: 2.52

Test: Repeat Count = 100, Workers = 1000
Mutex/Atomic duration ratio: 2.43
Channel/Atomic duration ratio: 1.96

Test: Repeat Count = 500, Workers = 10
Mutex/Atomic duration ratio: 7.47
Channel/Atomic duration ratio: 5.18

Test: Repeat Count = 500, Workers = 1000
Mutex/Atomic duration ratio: 8.42
Channel/Atomic duration ratio: 6.35

Test: Repeat Count = 800, Workers = 10
Mutex/Atomic duration ratio: 9.32
Channel/Atomic duration ratio: 5.77

Test: Repeat Count = 800, Workers = 1000
Mutex/Atomic duration ratio: 7.46
Channel/Atomic duration ratio: 6.26

Test: Repeat Count = 1000, Workers = 10
Mutex/Atomic duration ratio: 9.55
Channel/Atomic duration ratio: 6.00

Test: Repeat Count = 1000, Workers = 1000
Mutex/Atomic duration ratio: 8.95
Channel/Atomic duration ratio: 6.40

Test: Repeat Count = 1500, Workers = 10
Mutex/Atomic duration ratio: 11.40
Channel/Atomic duration ratio: 7.09

Test: Repeat Count = 1500, Workers = 1000
Mutex/Atomic duration ratio: 8.43
Channel/Atomic duration ratio: 6.88
```

### 4.3 Figure 3 in Section 4.3
Figure 3 mainly shows Performance gap for different memory orderings. Due to the randomness of compiler optimizations and instruction reordering, there may be some minor differences in the correlation results(For more details, please see [ordering-performance-issues](section-4-atomic-usage/section-4-3-ordering-performance-issues)).

Usages

```
cd crossbeam-skiplist
cargo run <test_runs> <thread_count> <count>
```

* <test_runs> is the number of test iterations to perform

* <thread_count> is the total number of threads to spawn for each test

* \<count> is the number of operations each thread will perform


Result
```
arm_AcqRel_vs_Relaxed = [0.68, 1.51, 5.03, 6.24, 7.18]
arm_86_SeqCst_vs_Relaxed = [1.62, 2.59, 7.94, 9.48, 12.54]
arm_SeqCst_vs_AcqRel = [0.93, 1.06, 2.77, 2.86, 5.00]
```

### 3.4 AtomVChecker
AtomVChecker currently detects three types of memory ordering misuse. For the full detail, please check our ISSRE 2024 paper.

TABLE 8 and TABLE 9 demonstrate the evaluation result of AtomVChecker, including the result of atomic correlations and atomic correlation violations(For the total results, see [atomic correlations](section-5-detection/README.md) and Table 5 in Section 6).

#### critical-state inconsistent update bug(CIU)
Atomic operations can correlate with the critical state of non-atomic memory addresses. In weakly ordered architectures, such as ARM64, atomic correlation violations occur when atomic operations with weak memory orderings in concurrent code cause other threads to fail to synchronize the changes to these critical states, which can lead to critical-state inconsistent update bug

Example: RUSTSEC_2022_0029

```
$ ./detect.sh toys/RUSTSEC_2022_0029
```
It will print one atomic concurrency bug(CIU bug):

```
      {
        "AtomicCorrelationViolation": {
          "bug_kind": "AtimicCorrelationViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "atomic": "src/main.rs:298:41: 298:54"
          },
          "explanation": "Using an atomic operation with a weaker memory ordering than necessary can lead to an inconsistent memory state. Using Acquire is sufficient to ensure the program's correctness."
        }
      },
      {
        "AtomicCorrelationViolation": {
          "bug_kind": "AtimicCorrelationViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "atomic": "src/main.rs:177:45: 177:65"
          },
          "explanation": "Using an atomic operation with a weaker memory ordering than necessary can lead to an inconsistent memory state. Using Release is sufficient to ensure the program's correctness."
        }
      }
```


#### AtomicPtr related Concurrency bug(ARC)

Such bugs manifest only in AtomicPtr and are architecture-specific, such as DEC Alpha that lacks data dependency. AtomicPtr operations target the addresses, which mean that the corresponding content operations are not atomic. Therefore, AtomicPtr inherently establishes an atomic correlation between the atomic pointer and the content. Atomic correlation violations occur when atomic operations with weak memory orderings in concurrent code cause other threads to fail to synchronize the content pointed by the atomic pointer, which can result in atomicPtr related Concurrency bug.

Example: RUSTSEC_2022_0006

```sh
$ ./detect.sh examples/RUSTSEC_2022_0006
```
It will print one atomic concurrency bug(ARC bug) in json format, like the following one:

```
    [
      {
        "AtomicCorrelationViolation": {
          "bug_kind": "AtimicCorrelationViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "atomic": "src/main.rs:381:33: 381:56"
          },
          "explanation": "Using an atomic operation with a weaker memory ordering than necessary can lead to an inconsistent memory state. Using Acquire is sufficient to ensure the program's correctness."
        }
      }
    ]
```

#### Performance loss caused by incorrect use of strong memory ordering(SMO)
This occurs when incorrectly use of strong memory ordering

Example: fragile

```sh
$ ./detect.sh examples/fragile
```
It will print two performance losses caused by incorrect use of strong memory ordering(SMO) in json format, like the following one:

```
    [
      {
        "AtomicCorrelationViolation": {
          "bug_kind": "AtimicCorrelationViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "atomic": "src/thread_id.rs:6:31: 6:61"
          },
          "explanation": "Using an atomic operation with a stronger memory ordering than necessary can lead to unnecessary performance overhead. Using Relaxed is sufficient to ensure the correctness of the program"
        }
      },
      {
        "AtomicCorrelationViolation": {
          "bug_kind": "AtimicCorrelationViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "atomic": "src/registry.rs:60:35: 60:65"
          },
          "explanation": "Using an atomic operation with a stronger memory ordering than necessary can lead to unnecessary performance overhead. Using Relaxed is sufficient to ensure the correctness of the program"
        }
      }
    ]

```


#### Atomic Correlations

If you want to check for atomic correlations, you can set the `ATOMVCHECKER_LOG` in detexct.sh to debug (default is info), which will output all atomic operations and the corresponding minimum memory ordering requirements

```sh
# debug mode(check for atomic correlations)
$ export ATOMVCHECKER_LOG=debug
```

As an example, the atomic correlation outputs for RUSTSEC-2022-0029 is as follows:
```
    {
      AtomicInfo { 
        atomic_place: Some(_56), 
        atomic_value: [_55], 
        atomic_operate: Some(Load), 
        caller_instance: NodeIndex(55), 
        ordering: [Relaxed], 
        source_info: "src/main.rs:298:41: 298:54" 
        }: {Acquire}, 
      AtomicInfo { 
        atomic_place: Some(_66), 
        atomic_value: [], 
        atomic_operate: Some(Store), 
        caller_instance: NodeIndex(65), 
        ordering: [Relaxed], 
        source_info: "src/main.rs:177:45: 177:65" 
        }: {Release}
    }: 2
```
