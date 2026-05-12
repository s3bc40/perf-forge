// Module 4: Cycle Estimation
//
// Formula: cycles ≈ Ir + (L1 Misses × 10-40) + (LL Misses × 100+)
//
// KEY INSIGHT: Two benchmarks with the same instruction count can have
// very different cycle counts depending on cache behavior.
//
// This module demonstrates how memory access patterns affect cache misses
// and therefore the total cycle cost:
//   - Sequential access at small N (fits in L1): lowest cycle cost per element
//   - Random access at large N (spills to RAM): highest cycle cost per element
//   - Strided access: shows how cache line utilization affects throughput
//
// What to observe in CodSpeed dashboard:
//   - Compare sequential vs random at n=1_048_576:
//     Similar Ir, but random has 10-100× more cache misses
//   - The stride benchmark: stride=1 uses full cache lines;
//     stride=8 wastes 7/8 of each cache line, doubling miss rate
//   - For n <= 16384 (fits in L1 ~32KB), misses should be minimal
//   - For n >= 131072 (L2 ~256KB), L2 misses appear
//   - For n >= 1_048_576 (L3 ~8MB+), LL misses dominate
//
// Follow-up:
// 1. Multiply the benchmark outputs by the formula above to get
//    estimated cycles, then compare with actual walltime
// 2. Try non-power-of-2 sizes to avoid cache associativity conflicts
// 3. Add a pointer-chasing benchmark via linked list traversal

use divan::black_box;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

fn sequential_sum(data: &[u64]) -> u64 {
    let mut sum = 0u64;
    for &x in data {
        sum = sum.wrapping_add(black_box(x));
    }
    sum
}

fn random_sum(data: &[u64], indices: &[usize]) -> u64 {
    let mut sum = 0u64;
    for &i in indices {
        sum = sum.wrapping_add(black_box(data[i]));
    }
    sum
}

fn strided_sum(data: &[u64], stride: usize) -> u64 {
    let mut sum = 0u64;
    let len = data.len();
    let mut i = 0;
    while i < len {
        sum = sum.wrapping_add(black_box(data[i]));
        i = i.wrapping_add(stride);
    }
    sum
}

fn pointer_chase(pointers: &[usize], start: usize) -> usize {
    let mut idx = start;
    for _ in 0..pointers.len() {
        idx = black_box(pointers[idx]);
    }
    idx
}

fn main() {
    divan::main();
}

#[divan::bench(args = [1024, 16384, 131_072, 1_048_576])]
fn bench_sequential_read(bencher: divan::Bencher, n: usize) {
    let data: Vec<u64> = (0..n as u64).collect();
    bencher.bench(|| sequential_sum(&data));
}

#[divan::bench(args = [1024, 16384, 131_072, 1_048_576])]
fn bench_random_read(bencher: divan::Bencher, n: usize) {
    let data: Vec<u64> = (0..n as u64).collect();
    let mut rng = StdRng::seed_from_u64(42);
    let mut indices: Vec<usize> = (0..n).collect();
    indices.shuffle(&mut rng);
    bencher.bench(|| random_sum(&data, &indices));
}

#[divan::bench(args = [1, 2, 4, 8, 16, 32, 64])]
fn bench_strided_read(bencher: divan::Bencher, stride: usize) {
    let n = 1_048_576usize;
    let data: Vec<u64> = (0..n as u64).collect();
    bencher.bench(|| strided_sum(&data, stride));
}

#[divan::bench(args = [1024, 16384, 131_072, 1_048_576])]
fn bench_pointer_chase(bencher: divan::Bencher, n: usize) {
    let mut rng = StdRng::seed_from_u64(42);
    let mut pointers: Vec<usize> = (0..n).collect();
    pointers.shuffle(&mut rng);
    // Create a cycle so pointer chasing never hits an invalid index:
    // link last element back to first
    let last = *pointers.last().unwrap();
    let first_pos = pointers.iter().position(|&x| x == 0).unwrap();
    pointers[last] = first_pos;
    bencher.bench(|| black_box(pointer_chase(&pointers, 0)));
}
