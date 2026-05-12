// Module 5: Flamegraph Analysis
//
// CodSpeed's flamegraph inspector provides:
//   - Self time: time spent in the function body (excludes children)
//   - Total time: self + all descendents
//   - Time bar breakdown: Instructions / Cache / Memory / System
//   - Bottleneck coloring: instruction-bound vs cache-bound vs memory-bound
//
// This module creates benchmarks with different call-graph shapes
// so you can observe how the flamegraph renders them:
//   - deep_call: 5-level chain — flamegraph shows a narrow tall stack
//   - wide_tree: one root with 1000 children — flamegraph shows a fan shape
//   - instruction_hot: single function with heavy ALU work — bottleneck colored
//   - cache_cold: random memory access — cache/memory bottleneck
//
// What to observe in CodSpeed dashboard:
//   - deep_call: self time is nearly zero for all but the leaf;
//     total time accumulates up the stack
//   - wide_tree: the root has high total time but low self time;
//     children have high self time
//   - instruction_hot: the time bar should show mostly "Instructions"
//   - cache_cold: the time bar should show significant "Cache" or "Memory"
//   - Toggle bottleneck coloring: instruction_hot → green,
//     cache_cold → red/orange
//
// Follow-up:
// 1. Add a benchmark with deep recursion (stack overflow risk — try ~1000)
// 2. Add a syscall-heavy benchmark and check "System" time in the bar
// 3. Add a benchmark where one hot path dominates and compare with
//    a version where work is evenly distributed

use divan::black_box;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

// --- Deep call chain: 5 levels. Flamegraph shows a narrow tall stack.
fn level1(x: u64) -> u64 {
    level2(x)
}
fn level2(x: u64) -> u64 {
    level3(x)
}
fn level3(x: u64) -> u64 {
    level4(x)
}
fn level4(x: u64) -> u64 {
    level5(x)
}
fn level5(x: u64) -> u64 {
    x.wrapping_mul(7).wrapping_add(3)
}

// --- Wide call tree: 1 root fans out to many children.
fn wide_root(count: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..count {
        sum = sum.wrapping_add(wide_leaf(i));
    }
    sum
}
fn wide_leaf(x: u64) -> u64 {
    x.wrapping_mul(x).wrapping_add(1)
}

// --- Instruction-hot: heavy ALU work in a single tight loop.
fn instruction_hot(n: u64) -> u64 {
    let mut acc = 0u64;
    for i in 0..n {
        acc = acc.wrapping_add(i);
        acc ^= acc >> 7;
        acc ^= acc << 13;
        acc ^= acc >> 17;
    }
    acc
}

// --- Cache-cold: random memory access pattern.
fn cache_cold(data: &[u64], indices: &[usize]) -> u64 {
    let mut sum = 0u64;
    for &i in indices {
        sum = sum.wrapping_add(black_box(data[i]));
    }
    sum
}

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_deep_call() -> u64 {
    level1(black_box(42))
}

#[divan::bench(args = [10, 100, 1000])]
fn bench_wide_tree(bencher: divan::Bencher, count: u64) {
    bencher.bench(|| black_box(wide_root(count)));
}

#[divan::bench(args = [1_000, 10_000])]
fn bench_instruction_hot(bencher: divan::Bencher, n: u64) {
    bencher.bench(|| black_box(instruction_hot(n)));
}

#[divan::bench(args = [1024, 16384, 131_072])]
fn bench_cache_cold(bencher: divan::Bencher, n: usize) {
    let data: Vec<u64> = (0..n as u64).collect();
    let mut rng = StdRng::seed_from_u64(42);
    let mut indices: Vec<usize> = (0..n).collect();
    indices.shuffle(&mut rng);
    bencher.bench(|| cache_cold(&data, &indices));
}
