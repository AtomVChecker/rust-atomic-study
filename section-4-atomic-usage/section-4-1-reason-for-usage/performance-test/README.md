# An example to test the performance of concurrency primitives: atomic, mutex, and channel.

## Usages

```cd performance-test/src```

```./run.sh```

## Output

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

## Configuration

Chipset Model: Apple M1

Type: GPU

Bus: Built-In

Total Number of Cores: 7

Vendor: Apple (0x106b)

Metal Family: Supported, Metal GPUFamily Apple 7
