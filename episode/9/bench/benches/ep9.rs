#[macro_use]
extern crate criterion;

use criterion::{Criterion, Fun};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn ep9(c: &mut Criterion) {
    // A list of functions we want to benchmark. Ugly block syntax because I'm
    // lazy and this is the easiest way to make it compile stuff right where we
    // need it.
    let fns = vec![
        Fun::new("rust fixed", |b, _| {
            let path = PathBuf::from("../rust/fixed");
            compile_rust(&path);
            let bin = path.join("target/release/fixed");
            b.iter(|| run(&bin))
        }),
        Fun::new("rust pascal", |b, _| {
            let path = PathBuf::from("../rust/pascal");
            compile_rust(&path);
            let bin = path.join("target/release/fixed");
             b.iter(|| run(&bin))
        }),
        Fun::new("rust union", |b, _| {
            let path = PathBuf::from("../rust/union");
            compile_rust(&path);
            let bin = path.join("target/release/fixed");
             b.iter(|| run(&bin))
        }),
        Fun::new("go fixed", |b, _| {
            let path = PathBuf::from("../go");
            compile_go(&path, "fixed.go");
            let bin = path.join("fixed");
            b.iter(|| run(&bin))
        }),
        Fun::new("go worker", |b, _| {
            let path = PathBuf::from("../go");
            compile_go(&path, "worker.go");
            let bin = path.join("worker");
            b.iter(|| run(&bin))
        }),
    ];

    // Using this, we get a graph comparing all the functions
    c.bench_functions("Episode 9", fns, &());
}

// Run the functions we just defined as a group
criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = ep9
);
criterion_main!(benches);

fn compile_rust(path: &Path) {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--quiet")
        .current_dir(path)
        .status()
        .expect("failed to execute process");
}

fn compile_go(path: &Path, name: &str) {
    Command::new("go")
        .arg("build")
        .arg(name)
        .current_dir(path)
        .status()
        .expect("failed to execute process");
}

fn run(path: &Path) {
    Command::new(path)
        .arg("../text/hamlet_gut.txt")
        .arg("../text/henry_v_gut.txt")
        .arg("../text/macbeth_gut_f.txt")
        .arg("../text/romeo_and_juliet_gut.txt")
        .stdout(Stdio::null())
        .status()
        .expect("failed to execute process");
}
