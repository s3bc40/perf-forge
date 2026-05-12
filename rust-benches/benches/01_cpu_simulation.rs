// Module 1: CPU Simulation Basics
//
// CodSpeed's CPU simulation instruments your benchmarks via Valgrind.
// The benchmark runs ONCE and the CPU behavior is simulated.
//
// Key metric: Ir (Instructions Retired) — the baseline cost of your code.
// The simulation also tracks L1 cache misses and LL (last-level) cache misses.
//
// What to observe in CodSpeed dashboard:
// - Compare Ir between recursive and iterative fibonacci (massive difference)
// - Look at the flamegraph: self time vs total time for each function
// - Check the bottleneck coloring: instruction-bound vs cache-bound spans
//
// Follow-up: Change N values and watch how Ir scales (O(2^n) vs O(n))

use divan::black_box;

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let (mut a, mut b) = (1u64, 1u64);
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

// HashMap vs BTreeMap: different cache behavior patterns
// HashMap: O(1) average but poor cache locality (random access)
// BTreeMap: O(log n) but better cache locality (sequential nodes)
use std::collections::{BTreeMap, HashMap};

fn build_hashmap(size: usize) -> HashMap<u64, u64> {
    (0..size as u64).map(|i| (i, i * 2)).collect()
}

fn build_btreemap(size: usize) -> BTreeMap<u64, u64> {
    (0..size as u64).map(|i| (i, i * 2)).collect()
}

fn lookup_hashmap(map: &HashMap<u64, u64>, keys: &[u64]) {
    for &k in keys {
        black_box(map.get(&k));
    }
}

fn lookup_btreemap(map: &BTreeMap<u64, u64>, keys: &[u64]) {
    for &k in keys {
        black_box(map.get(&k));
    }
}

// String concatenation patterns — allocation + instruction patterns
fn concat_push(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        s.push_str(&format!("{}", i));
    }
    s
}

fn concat_with_reserve(n: usize) -> String {
    let mut s = String::with_capacity(n * 3);
    for i in 0..n {
        s.push_str(&format!("{}", i));
    }
    s
}

fn main() {
    divan::main();
}

#[divan::bench]
fn fib_recursive_20() -> u64 {
    fibonacci_recursive(black_box(20))
}

#[divan::bench]
fn fib_iterative_20() -> u64 {
    fibonacci_iterative(black_box(20))
}

#[divan::bench]
fn fib_recursive_25() -> u64 {
    fibonacci_recursive(black_box(25))
}

#[divan::bench]
fn fib_iterative_25() -> u64 {
    fibonacci_iterative(black_box(25))
}

#[divan::bench(args = [100, 1000, 10000])]
fn bench_build_hashmap(bencher: divan::Bencher, size: usize) {
    bencher.bench(|| black_box(build_hashmap(size)));
}

#[divan::bench(args = [100, 1000, 10000])]
fn bench_build_btreemap(bencher: divan::Bencher, size: usize) {
    bencher.bench(|| black_box(build_btreemap(size)));
}

#[divan::bench(args = [100, 1000, 10000])]
fn bench_lookup_hashmap(bencher: divan::Bencher, size: usize) {
    let map = build_hashmap(size);
    let keys: Vec<u64> = (0..size as u64).collect();
    bencher.bench(|| lookup_hashmap(&map, &keys));
}

#[divan::bench(args = [100, 1000, 10000])]
fn bench_lookup_btreemap(bencher: divan::Bencher, size: usize) {
    let map = build_btreemap(size);
    let keys: Vec<u64> = (0..size as u64).collect();
    bencher.bench(|| lookup_btreemap(&map, &keys));
}

#[divan::bench(args = [100, 1000])]
fn string_concat_push(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(concat_push(n)));
}

#[divan::bench(args = [100, 1000])]
fn string_concat_reserve(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(concat_with_reserve(n)));
}
