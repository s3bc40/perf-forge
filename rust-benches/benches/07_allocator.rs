// Module 7: Allocator Impact
//
// The choice of allocator and allocation pattern significantly
// affects both performance and variance:
//   - jemalloc: good multi-threaded scaling, configurable decay
//   - tcmalloc: thread-cache sizing, aggressive caching
//   - mimalloc: compact, fast, free-list sharding
//   - glibc malloc: simple, prone to fragmentation
//
// KEY CONCEPTS:
//   - Allocation count: number of malloc/realloc/calloc calls
//   - Fragmentation: wasted space from interleaved sizes
//   - Realloc cost: growing Vec without capacity hint causes
//     repeated reallocation and copying
//   - Thread-cache: per-thread free-list reduces contention
//
// This module benchmarks different allocation patterns to show how
// they affect allocator behavior. On CodSpeed CI with Memory mode or
// CPU simulation, you'll see allocation counts and peak memory.
//
// What to observe in CodSpeed dashboard:
//   - many_tiny_allocs vs few_large_allocs:
//     Same total bytes (approx), but very different allocation counts
//   - mixed_sizes: shows fragmentation cost vs uniform_allocs
//   - vec_grow_no_reserve vs vec_grow_with_reserve:
//     Same peak memory, but no_reserve has many more reallocs
//   - arena_style_allocs: single allocation, minimum allocator overhead
//
// Follow-up:
// 1. Add jemalloc as a global allocator and re-run
//    (add to Cargo.toml: jemallocator = "0.5" and use #[global_allocator])
// 2. Try with mimalloc (mimalloc = "0.1")
// 3. Compare Box<[u64]> vs Vec<u64> — allocation count difference?
// 4. Tune jemalloc with MALLOC_CONF env vars

use divan::black_box;

fn many_tiny_allocs(n: usize) -> Vec<Box<u8>> {
    (0..n).map(|_| Box::new(0u8)).collect()
}

fn few_large_allocs(n: usize) -> Vec<Vec<u64>> {
    (0..n).map(|_| vec![0u64; 1024]).collect()
}

fn mixed_sizes(n: usize) -> Vec<Vec<u64>> {
    (0..n).map(|i| vec![0u64; (i % 10 + 1) * 100]).collect()
}

fn uniform_allocs(n: usize) -> Vec<Vec<u64>> {
    (0..n).map(|_| vec![0u64; 500]).collect()
}

fn vec_grow_no_reserve(n: usize) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

fn vec_grow_with_reserve(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

fn arena_style_allocs(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

fn main() {
    divan::main();
}

#[divan::bench(args = [100, 1_000, 10_000])]
fn bench_many_tiny_allocs(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(many_tiny_allocs(n)));
}

#[divan::bench(args = [10, 100, 1_000])]
fn bench_few_large_allocs(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(few_large_allocs(n)));
}

#[divan::bench(args = [10, 100, 500])]
fn bench_mixed_sizes(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(mixed_sizes(n)));
}

#[divan::bench(args = [10, 100, 500])]
fn bench_uniform_allocs(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(uniform_allocs(n)));
}

#[divan::bench(args = [100, 1_000, 10_000])]
fn bench_vec_grow_no_reserve(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(vec_grow_no_reserve(n)));
}

#[divan::bench(args = [100, 1_000, 10_000])]
fn bench_vec_grow_with_reserve(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(vec_grow_with_reserve(n)));
}

#[divan::bench(args = [100, 1_000, 10_000])]
fn bench_arena_style(bencher: divan::Bencher, n: usize) {
    bencher.bench(|| black_box(arena_style_allocs(n)));
}
