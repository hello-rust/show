# Benchmarks

In episode 9,
different implementations were presented,
comparing a Go implementation to one in Rust.
Afterwards, some people submitted pull requests adding some more variants.
And while it was not the aim of the episode
(which was about how Rust catches data races!)
people have asked about benchmarking the solutions.

So here it is.
Benchmarks.
This is also a nice showcase for the [criterion] library.

[criterion]: https://japaric.github.io/criterion.rs/book/

## Setup

1. Install Rust
2. Install Go
3. Install gnuplot if you want plots

## Running the benchmarks

1. In this directory, run `cargo bench` (or `cargo bench -- --noplot` if you don't have gnuplot)
2. Wait for some time (it needs to compile the implementations before executing them)
3. See CLI output
4. Also open `target/criterion/report/index.html` which shows pretty graphs
