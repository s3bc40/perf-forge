// Module 6: Benchmark Variance
//
// Common variance causes (from README):
// 1. Toolchain updates — pin compiler versions
// 2. CI runner variability — different CPU models
// 3. Compiler non-determinism — inlining, basic block reordering
// 4. Function alignment — I-cache misses from code layout changes
// 5. Allocator behavior — jemalloc decay, tcmalloc thread-cache sizing
//
// This module creates benchmarks that are deliberately sensitive to
// these factors, so you can observe variance across runs.
//
// What to observe in CodSpeed dashboard:
//   - Run the same file multiple times — branch variants may show
//     different variance magnitudes
//   - predictable_branches vs unpredictable_branches:
//     unpredictable should have higher variance (branch predictor
//     behavior depends on alignment and history)
//   - with_reserve vs without_reserve: allocator state at the start
//     of each bench iteration can vary
//   - cold_function vs hot_function: inline decisions change the
//     call-graph shape and I-cache footprint, shifting cycle counts
//
// Follow-up:
// 1. Compare results across different Rust toolchain versions
// 2. Add #[inline(never)] to understand inlining impact on variance
// 3. Try running with different allocators (jemalloc, mimalloc)

use divan::black_box;

// --- Predictable branches: easy for the branch predictor.
fn predictable_branches(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        if i % 2 == 0 {
            sum = sum.wrapping_add(i);
        } else {
            sum = sum.wrapping_sub(i);
        }
    }
    sum
}

// --- Unpredictable branches: hash-like condition thwarts prediction.
fn unpredictable_branches(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        let cond = i.wrapping_mul(7).wrapping_add(3) ^ (i >> 3);
        if cond & 1 == 0 {
            sum = sum.wrapping_add(i);
        } else {
            sum = sum.wrapping_sub(i);
        }
    }
    sum
}

// --- Allocator variance: repeated reallocation.
fn vec_without_reserve(n: usize) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

fn vec_with_reserve(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

// --- Inline-sensitive: code that the compiler may inline differently.
#[inline(never)]
fn never_inline_helper(x: u64) -> u64 {
    x.wrapping_mul(17).wrapping_add(31).wrapping_sub(x >> 3)
}

#[inline(always)]
fn always_inline_helper(x: u64) -> u64 {
    x.wrapping_mul(17).wrapping_add(31).wrapping_sub(x >> 3)
}

fn cold_function_loop(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(never_inline_helper(i));
    }
    sum
}

fn hot_function_loop(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(always_inline_helper(i));
    }
    sum
}

fn main() {
    divan::main();
}

#[divan::bench(args = [10_000, 100_000])]
fn bench_predictable_branches(bencher: divan::Bencher, n: u64) {
    bencher.bench(|| black_box(predictable_branches(n)));
}

#[divan::bench(args = [10_000, 100_000])]
fn bench_unpredictable_branches(bencher: divan::Bencher, n: u64) {
    bencher.bench(|| black_box(unpredictable_branches(n)));
}

#[divan::bench(args = [100, 1_000, 10_000])]
fn bench_vec_without_reserve(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(vec_without_reserve(n)));
}

#[divan::bench(args = [100, 1_000, 10_000])]
fn bench_vec_with_reserve(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(vec_with_reserve(n)));
}

#[divan::bench(args = [1_000, 10_000])]
fn bench_cold_function(bencher: divan::Bencher, n: u64) {
    bencher.bench(|| black_box(cold_function_loop(n)));
}

#[divan::bench(args = [1_000, 10_000])]
fn bench_hot_function(bencher: divan::Bencher, n: u64) {
    bencher.bench(|| black_box(hot_function_loop(n)));
}
