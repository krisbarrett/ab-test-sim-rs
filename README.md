# ab-test-sim-rs

Port of [ab-test-simulation](https://github.com/krisbarrett/ab-test-simulation) from C++ to Rust.

## Build

```shell
$ cargo build --release
```

## Run

```shell
$ time ./target/release/ab-test-sim-rs
false positive rate: 5%
true positive rate: 80.4%
./target/release/ab-test-sim-rs  12.77s user 0.04s system 708% cpu 1.809 total
```
