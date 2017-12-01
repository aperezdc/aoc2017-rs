# AOC2017 in Rust

These are programs written in [Rust](https://rust-lang.org) which implement
solutions for the [Advent of Code 2017](http://adventofcode.com/2017) (AoC).

## Code Structure

This is a single `aoc2017` crate, which contains:

- Some shared utility code in `src/lib.rs`.
- Programs for each day's assignmens in `src/bin/*.rs`.

## Building

The code needs a version of Rust that allows functions to return `impl Trait`.
At the time of writing, this means using a nightly version of the compiler.
Use Cargo normally for building:

```sh
cargo build --release
```
