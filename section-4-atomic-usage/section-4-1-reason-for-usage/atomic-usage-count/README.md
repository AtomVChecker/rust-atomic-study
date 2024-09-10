# Count atomic usages, including scene usage and ordering usage

## Studied App

1. https://github.com/tikv/tikv
commit:3f7c63646ef5ea842f7ce6552826976feda2f609
2. https://github.com/apache/incubator-teaclave-sgx-sdk
commit:3c903bdac4e503dd27b9b1f761c4abfc55f2464c
3. https://github.com/spacejam/sled
commit:005c023ca94d424d8e630125e4c21320ed160031
4. https://github.com/servo/servo
commit:5d7ed76b79de359ef1de2bdee83b32bd497d7cd8
5. https://github.com/influxdata/influxdb
commit:bb6a5c0bf6968117251617cda99cb39a5274b6dd
6. https://github.com/occlum/occlum
commit:f54eabfa92319490e1b13075b28fef2d3d9129fd
7. https://github.com/AleoHQ/snarkOS
commit:e7d39272d0c008c6d67cf1fdbf0da5de8b5001f7
8. https://github.com/rayon-rs/rayon
commit:d1b18e616eec5ce8520aecb31054b180006527a8
9. https://github.com/Amanieu/parking_lot
commit:0b296160941275d8df757066dd26361d6ae5d455
10. https://github.com/tokio-rs/tokio
commit:7b555185ff9186f618b198126ee853980b187698
11. https://github.com/actix/actix
commit:d0509d350ca9a6b7ec67e00d325518b9128721e3
12. https://github.com/crossbeam-rs/crossbeam
commit:18afbb6ed2f98e55ae5cc10578e54762232a2437
13. https://github.com/matklad/once_cell
commit:8f39b775effd387b175993b0091b082c4d60f921

## Usage:

```cd section-4-atomic-usage/section-4-1-reason-for-usage/atomic-usage-count/project```

```git clone https://github.com/AleoNet/snarkOS.git```

```cd snarkOS```

```git checkout e7d39272d0c008c6d67cf1fdbf0da5de8b5001f7```

```cd atomic-usage-count```

```python count.py ./project/snarkOS```

## Output:

total: 33

field_atomic_operations: 28

global_atomic_operations: 1

other_atomic_operations: 4

Counter({'SeqCst': 20, 'Relaxed': 13})

struct: 0.8484848484848485

global: 0.030303030303030304

other: 0.12121212121212119

{'SeqCst': 0.6060606060606061, 'Relaxed': 0.3939393939393939, 'Acquire/Release': 0.0}

## Script:

You can run `atomic_usage.sh` to automatically download and verify each benchmark.

```cd section-4-atomic-usage/section-4-1-reason-for-usage/atomic-usage-count```

```./atomic_usage.sh```
