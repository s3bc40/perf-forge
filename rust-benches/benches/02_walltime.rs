// Module 2: Walltime vs CPU Simulation
//
// KEY DIFFERENCE:
// - CPU Simulation EXCLUDES syscalls from the measurement for stability.
//   Syscall time is recorded separately in the trace view but not in execution speed.
// - Walltime INCLUDES everything: user code + syscalls + I/O + network + disk.
//
// When to use Walltime:
// - Code that relies heavily on syscalls (file I/O, network, process spawning)
// - Integration tests on API endpoints
// - Multi-threaded code where parallelism effects matter
//
// When to use CPU Simulation:
// - Pure algorithm optimization
// - Cache behavior analysis
// - Consistent, hardware-agnostic measurements
//
// Walltime requires CodSpeed Macro Runners (runs-on: codspeed-macro)
// because it needs bare-metal machines for low-noise measurements.
//
// What to observe:
// - In CPU simulation, the file_read benchmark will show very low Ir
//   because the actual I/O is excluded — only the wrapper code is measured.
// - In walltime, the same benchmark captures the real disk I/O time.
// - The gap between simulation and walltime tells you how much time is in syscalls.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use std::thread;

use divan::black_box;

fn create_temp_file(content: &str) -> PathBuf {
    let path = PathBuf::from(format!("/tmp/codspeed_bench_{}.txt", std::process::id()));
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    path
}

fn read_file_content(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap()
}

// Simulates I/O-bound work — walltime captures the real cost,
// CPU simulation only measures the thread::sleep wrapper (nearly zero Ir).
fn simulated_io_work() {
    // In real code this would be a network call or disk read.
    // We use sleep to simulate the walltime gap.
    thread::sleep(Duration::from_micros(100));
}

// Pure CPU work — both simulation and walltime should agree closely.
fn pure_cpu_work() -> u64 {
    let mut sum = 0u64;
    for i in 0u64..10_000 {
        sum = sum.wrapping_add(i.wrapping_mul(7).wrapping_add(3));
    }
    sum
}

fn main() {
    divan::main();
}

#[divan::bench]
fn file_write_and_read(bencher: divan::Bencher) {
    let content = "hello benchmark world ".repeat(100);
    let path = create_temp_file(&content);
    bencher.bench(|| {
        black_box(read_file_content(&path));
    });
    let _ = fs::remove_file(&path);
}

#[divan::bench]
fn simulated_io() {
    simulated_io_work();
}

#[divan::bench]
fn pure_cpu() -> u64 {
    pure_cpu_work()
}

// Demonstrates: syscall-heavy code where simulation underestimates real cost.
// The open/read/close syscalls dominate walltime but are invisible to simulation.
#[divan::bench]
fn open_read_close_dev_null(bencher: divan::Bencher) {
    bencher.bench(|| {
        let _ = fs::read_to_string("/dev/null");
    });
}
