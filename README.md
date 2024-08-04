# The code, analysis scripts and results for ISSRE 2024 Artifact Evaluation
Version: 1.1 Paper: Understanding Atomics and Memory Ordering Issues in Real-World Rust Software (#60)

This document is to help users reproduce the results we reported in our submission. It contains the following descriptions

## 0. Artifact Expectation
The code and scripts for the tools we build are published in this repository.

## 1. Environment Setup

Pull rust-atomic-study from GitHub.
```sh
$ git clone https://github.com/AtomVChecker/rust-atomic-study.git
```

### 1.1 AtomVChecker
Add the Rust components rust-src, rustc-dev and llvm-tools-preview to compile and install AtomVChecker. It currently supports rustc nightly-2023-03-09.

```sh
$ cd AtomVChecker
$ rustup component add rust-src
$ rustup component add rustc-dev
$ rustup component add llvm-tools-preview
$ cargo install --path .
```

### 1.2 Python
Some experiments use python, the python version is 3.7.13.


## 2. Section 3
TABLE 3 mainly shows the atomic usage in real-world Rust programs(see [atomic-usage-count](section-4-atomic-usage/section-4-1-reason-for-usage/atomic-usage-count)).


## 3. Section 4.1
This experiment mainly shows the comparisons of atomic types with other concurrency primitives(see [performance-test](/section-4-atomic-usage/section-4-1-reason-for-usage/performance-test)).


## 4. Section 4.3
Figure 3 mainly shows Performance gap for different memory orderings. Due to the randomness of compiler optimizations and instruction reordering, there may be some minor differences in the correlation results(see [ordering-performance-issues](section-4-atomic-usage/section-4-3-ordering-performance-issues)).


## 5. Section 6.1
TABLE 8 and TABLE 9 demonstrate the evaluation result of AtomVChecker, including the result of atomic correlations and atomic correlation violations(For the total results of the atomic correlations, see [atomic correlations](https://github.com/AtomVChecker/rust-atomic-study/blob/main/section-5-detection/README.md)).

### 5.1 Example
Test RUSTSEC_2022_0006 and RUSTSEC_2022_0029
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

If you want to check for atomic correlations, you can set the `ATOMVCHECKER_LOG` in detexct.sh to debug (default is info), which will output all atomic operations and the corresponding minimum memory ordering requirements

```sh
# debug mode(check for atomic correlations)
$ export ATOMVCHECKER_LOG=debug
```

The atomic correlation outputs for RUSTSEC-2022-0029 is as follows:
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


`detect.sh` is mainly for development of the detector and brings more flexibility.
You can modify `detect.sh` to use release vesion of AtomVChecker to detect large and complex projects.

For ease of use, you can also run cargo atomvchecker
```sh
$ cd examples/RUSTSEC_2022_0006; cargo clean; cargo atomvchecker -k atomicity_violation
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
