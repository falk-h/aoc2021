# Advent of Code 2021

Run tests:

```sh
cargo test
```

Run some specific solution:

```sh
cargo run --release -- dayX::partY
```

Run parts of some specific day:

```sh
cargo run --release -- dayX
```

Run all days:

```sh
cargo run --release -- --all
```

## Benchmarks

Benchmarks run with the full input, and not just the test input.

Run all benchmarks:

```sh
cargo install cargo-criterion # Only needs to be run once
cargo criterion
```

Run the benchmark for some specific solution:

```sh
cargo criterion -- dayX::partY
```

Run the benchmark for some specific day:

```sh
cargo criterion -- dayX
```
