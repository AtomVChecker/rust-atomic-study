# Results

## Atomic correlations

Atomic correlations reflect AtomVChecker's ability to detect potential memory ordering misuses through static analysis(* means experimental results on atomic correlations from other softwares in Table 3). All the experiments are done on an 20.04-Ubuntu system with a 3.20 GHz Intel processor.

| Library      | # Atomic Correlations | # Involved Structures | % False Positive  |
|--------------|-----------------------|-----------------------|-------------------|
| Once_Cell    | 11                    | 4                     | 9.09%             |
| Parking-lot  | 34                    | 6                     | 17.64%            |
| Rayon        | 21                    | 8                     | 14.29%            |
| Crossbeam    | 162                   | 26                    | 22.84%            |
| TiKV*        | 291                   | 42                    | 19.24%            |
| Teaclave*    | 163                   | 18                    | 13.12%            |
| Sled*        | 104                   | 20                    | 20.68%            |
| Servo*       | 42                    | 9                     | 11.31%            |
| InfluxDB*    | 67                    | 12                    | 18.44%            |
| Occlum*      | 51                    | 15                    | 14.12%            |
| SnarkOS*     | 22                    | 7                     | 12.95%            |
| Actix*       | 58                    | 14                    | 18.64%            |
| Tokio*       | 185                   | 37                    | 14.37%            |
| **Total**    | **1211**              | **218**               | **15.90%**        |

## Memory Ordering Misuses

A showcase of memory ordering issues found by AtomVChecker. 

Crate | Version | Information
----- | ------- | -----------
[RUSTSEC-2022-0006](https://github.com/Amanieu/thread_local-rs) | v1.1.0 | [ARC](https://github.com/Amanieu/thread_local-rs/pull/34)
[RUSTSEC-2022-0029](https://github.com/crossbeam-rs/crossbeam) | v0.3.0 | [CIU](https://github.com/crossbeam-rs/crossbeam/pull/98)
[Bottlerocket](https://github.com/bottlerocket-os/bottlerocket) | v1.17.0 | [SMO](https://github.com/bottlerocket-os/bottlerocket/pull/3701)
[Ripgrep](https://github.com/BurntSushi/ripgrep) | 14.1.0 | [SMO](https://github.com/BurntSushi/ripgrep/pull/2706)
[Termcolor](https://github.com/BurntSushi/termcolor) | 1.4.0 | [SMO](https://github.com/BurntSushi/termcolor/pull/82)
[Coldsnap](https://github.com/awslabs/coldsnap) | v0.6.0 | [SMO](https://github.com/awslabs/coldsnap/pull/309)
[Fragile](https://github.com/mitsuhiko/fragile) | 2.0.0 | [SMO](https://github.com/mitsuhiko/fragile/pull/36)
[Rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple) | v24.0.0 | [SMO](https://github.com/blackbeam/rust-mysql-simple/pull/367)
[Rayon](https://github.com/rayon-rs/rayon) | v1.9.0 | [SMO](https://github.com/rayon-rs/rayon/pull/1140)
[Snarkos](https://github.com/AleoNet/snarkOS) | v2.0.2 | [SMO](https://github.com/AleoNet/snarkOS/pull/2317)
[Occlum](https://github.com/occlum/occlum) | 0.29.7 | [SMO](https://github.com/occlum/occlum/pull/1389)

Description of information:

* `ARC`: AtomicPtr related Concurrency bug
* `CIU`: Critical-state inconsistent update bug
* `SMO`: Performance loss caused by incorrect use of strong memory ordering
