// Module 3: Memory Instrument
//
// CodSpeed's Memory instrument tracks three key metrics:
//   - Peak memory:  highest RSS during the benchmark
//   - Allocation count:  number of malloc / realloc / calloc calls
//   - Allocation size:   total bytes allocated
//
// Memory measurement works via eBPF-based tracking on CodSpeed CI runners.
// On local runs (cargo codspeed run -m memory) it uses a best-effort approximation.
//
// What to observe in the CodSpeed dashboard:
//   - Compare vec_grow vs vec_grow_with_reserve at the same N:
//     same PEAK memory, but very different ALLOCATION COUNTS.
//   - Compare many_small_allocs vs few_large_allocs:
//     many_small_allocs has higher allocation count AND higher peak (per-object overhead).
//   - fragmentation_pattern shows that interleaved sizes prevent reuse,
//     inflating both peak and allocation count vs a uniform pattern.
//
// Key insight: one large allocation is cheaper (in count, CPU instruction, and cache)
// than many small reallocations, even if the peak memory is identical.
//
// Follow-up tasks:
//   1. Add a bench with Vec::try_reserve_exact vs Vec::with_capacity
//   2. Try Box<[u64]> instead of Vec<u64> — does allocation count change?
//   3. Compare Bump allocator vs system allocator for the same pattern

use divan::black_box;

// --- Vec: grow without reserve -- multiple reallocations as capacity is exhausted
fn vec_grow(n: usize) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

// --- Vec: grow with reserve -- single allocation, zero reallocations
fn vec_grow_with_reserve(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

// --- String: build without reserve
fn string_build(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        s.push_str(&i.to_string());
    }
    s
}

// --- String: build with reserve
fn string_build_with_reserve(n: usize) -> String {
    let est = n * 3; // rough upper-bound: each number up to n takes <= ~log10(n) digits
    let mut s = String::with_capacity(est);
    for i in 0..n {
        s.push_str(&i.to_string());
    }
    s
}

// --- Many small allocations: each element individually boxed
fn many_small_allocs(n: usize) -> Vec<Box<u64>> {
    (0..n).map(|i| Box::new(i as u64)).collect()
}

// --- Few large allocations: one contiguous Vec
fn few_large_allocs(n: usize) -> Vec<u64> {
    (0..n as u64).collect()
}

// --- Fragmentation: interleaved sizes create gaps the allocator cannot easily reuse
fn fragmentation_pattern(count: usize) -> Vec<Vec<u64>> {
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        let size = if i % 2 == 0 { 100 } else { 1000 };
        let mut v = Vec::with_capacity(size);
        for j in 0..size {
            v.push(j as u64);
        }
        result.push(v);
    }
    result
}

// --- Uniform allocation (for comparison with fragmentation)
fn uniform_allocs(count: usize) -> Vec<Vec<u64>> {
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let mut v = Vec::with_capacity(500);
        for j in 0..500 {
            v.push(j as u64);
        }
        result.push(v);
    }
    result
}

fn main() {
    divan::main();
}

#[divan::bench(args = [100, 1000, 10_000])]
fn bench_vec_grow(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(vec_grow(n)));
}

#[divan::bench(args = [100, 1000, 10_000])]
fn bench_vec_grow_with_reserve(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(vec_grow_with_reserve(n)));
}

#[divan::bench(args = [100, 1000])]
fn bench_string_build(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(string_build(n)));
}

#[divan::bench(args = [100, 1000])]
fn bench_string_build_with_reserve(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(string_build_with_reserve(n)));
}

#[divan::bench(args = [100, 1000, 10_000])]
fn bench_many_small_allocs(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(many_small_allocs(n)));
}

#[divan::bench(args = [100, 1000, 10_000])]
fn bench_few_large_allocs(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(few_large_allocs(n)));
}

#[divan::bench(args = [10, 100, 500])]
fn bench_fragmentation(bencher: divan::Bencher, count: usize) {
    bencher.bench(|| black_box(fragmentation_pattern(count)));
}

#[divan::bench(args = [10, 100, 500])]
fn bench_uniform(bencher: divan::Bencher, count: usize) {
    bencher.bench(|| black_box(uniform_allocs(count)));
}
