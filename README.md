# advent-of-code

## 2021

Implemented in Icarus in directory `advent2021/`.

Most days' solutions should work with the current head version of
https://github.com/asoffer/Icarus with the following known exceptions:

 - day02.ic has a non-deterministic type verification failure. If you try running it
   enough times, it should eventually work.
 - day03.ic part 2 isn't implemented yet.

## 2020

Implemented with Rust in directory `advent2020/`.
To run a given day's solution:

```
cargo run --bin day01
```

To run tests for all days together:

```
cargo test
```

## 2018

Implemented with Rust in directory `advent2018/`.
All days are runnable from the main binary by specifying their number on the command line:

```
cargo run 1 2 3
```
