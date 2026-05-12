# Perf Forge — CodSpeed Training Project

A hands-on training repo for mastering CodSpeed's benchmarking platform with Rust and Python. Built to prepare for a pair programming session with CodSpeed for a full-stack engineer role.

## What You'll Learn

| Module | Topic | Key Concepts |
|--------|-------|-------------|
| 1 | CPU Simulation | Valgrind-based simulation, Ir (instructions), cache misses, cycle estimation |
| 2 | Walltime vs Simulation | Real elapsed time, syscall exclusion, I/O impact, Macro Runners |
| 3 | Memory Instrument | Peak memory, allocation count/size, eBPF tracking, allocator support |
| 4 | Cycle Estimation | `cycles ≈ Ir + (L1 Misses × L2/L3 Cost) + (LL Misses × RAM Cost)` |
| 5 | Flamegraph Analysis | Self time, total time, bottleneck coloring (instruction/cache/memory/system) |
| 6 | Benchmark Variance | Toolchain, CI runner, compiler non-determinism, function alignment |
| 7 | Allocator Impact | jemalloc, tcmalloc, mimalloc, fragmentation, thread-cache sizing, decay |

## Project Structure

```
rust-benches/              # Rust benchmarks (divan compat layer)
  benches/
    01_cpu_simulation.rs   # CPU simulation basics: Ir, cache, memory access
    02_walltime.rs         # Walltime: syscalls, I/O, when to use walltime vs sim
    03_memory.rs           # Memory: allocations, peak, fragmentation patterns
    04_cycle_estimation.rs # Deep dive: cycle computation from cache hierarchy
    05_flamegraph.rs       # Flamegraph: self/total time, bottleneck analysis
    06_variance.rs         # Variance: compiler, allocator, environment causes
    07_allocator.rs        # Allocators: jemalloc vs tcmalloc vs libc, tuning
python-benches/            # Python benchmarks (pytest-codspeed)
  tests/
    test_cpu_simulation.py # CPU simulation with Python
    test_memory.py         # Memory tracking in Python
    test_variance.py       # Variance patterns in interpreted code
.github/workflows/
  codspeed.yml             # CI: simulation + walltime + memory modes
```

## Prerequisites

- **Rust**: `cargo`, `cargo-codspeed` (`cargo install cargo-codspeed`)
- **Python**: `uv` or `pip`, Python 3.12+
- **CodSpeed account**: https://codspeed.io
- **Zed editor** (optional): for MCP server integration

## Quick Start

### Rust

```bash
cd rust-benches
cargo fmt --check                  # Check formatting
cargo clippy --all-targets         # Lint checks
cargo codspeed build              # Build all benchmarks
cargo codspeed run                # Run locally (validates setup, no measurement)
cargo codspeed run -m simulation  # CPU simulation mode
cargo codspeed run -m walltime    # Walltime mode (needs CodSpeed CI)
cargo codspeed run -m memory      # Memory mode
```

### Python

```bash
cd python-benches
uv sync --dev                     # Install dependencies
uv run pytest tests/ --codspeed   # Run benchmarks locally
```

### CI

Push to a GitHub repo with CodSpeed connected. The workflow runs on `push` and `pull_request`.

## Zed + CodSpeed MCP Server

Add to your Zed settings (`Ctrl+,` → JSON):

```json
{
  "context_servers": {
    "CodSpeed": {
      "url": "https://mcp.codspeed.io/mcp"
    }
  }
}
```

This gives your AI assistant access to:
- `list_repositories` — CodSpeed-enabled repos
- `list_runs` — Recent performance runs
- `get_run` — Inspect a single run's benchmarks
- `compare_runs` — Compare two runs with markdown report
- `query_flamegraph` — Query flamegraph hot spots and call trees

Example prompts once connected:
- "Explain the regression on branch X"
- "What are the hottest functions in bench_foo?"
- "Compare flamegraphs between main and feat/new-encoder"

## Key CodSpeed Concepts

### CPU Simulation vs Walltime

| | CPU Simulation | Walltime |
|---|---|---|
| **What it measures** | Simulated CPU cycles | Real elapsed time |
| **Syscalls included** | No (excluded for stability) | Yes |
| **Runs** | Once per benchmark | Once on bare-metal Macro Runner |
| **Best for** | Algorithm optimization, cache analysis | I/O-heavy code, integration tests |
| **CI runner** | Any (ubuntu-latest) | `codspeed-macro` (bare-metal) |
| **Profiling** | Flamegraph with instruction/cache/memory breakdown | Flamegraph with hardware events |

### Cycle Estimation Formula

```
cycles ≈ Ir + (L1 Misses × 10-40) + (LL Misses × 100+)
```

- `Ir` = executed instructions (baseline cost)
- L1 miss = fetch from L2/L3 cache (10-40 cycles)
- LL miss = fetch from RAM (100+ cycles)

Execution speed = 1 / (cycles / FREQUENCY)

### Flamegraph Inspector Metrics (CPU Simulation)

- **Self time**: time in function body only (excludes children)
- **Total time**: time including all children
- **Time bar breakdown**:
  - Instructions: CPU instruction execution time
  - Cache: time lost to cache misses (L1, L2, L3)
  - Memory: time waiting for main memory

### Flamegraph Color Modes

- **By Origin**: User / Library / System / Unknown
- **Differential**: Slower / Faster / Added / Removed (vs base)
- **By Bottleneck**: Instruction-bound / Cache-bound / Memory-bound / System-bound
- **By Function**: Same color for same function across benchmarks

### Common Variance Causes

1. **Toolchain updates**: Pin compiler versions, avoid `dtolnay/rust-toolchain@stable`
2. **CI runner variability**: Different CPU models across runs → use Macro Runners
3. **Compiler non-determinism**: Inlining, basic block reordering, loop transforms
4. **Function alignment**: I-cache misses from code layout changes
5. **Allocator behavior**: jemalloc decay, tcmalloc thread-cache sizing, fragmentation

### Reducing Allocator Variance

- jemalloc: `MALLOC_CONF="dirty_decay_ms:-1,muzzy_decay_ms:-1"`
- tcmalloc: disable background release, guarded sampling, heap profiling
- Custom allocator: use `NeverGrowInPlaceAllocator` to eliminate `realloc` non-determinism

### Impact Metrics

- `impact = (speed - baseSpeed) / baseSpeed`
- Positive = faster (green), Negative = slower (red)
- Default regression threshold: 10%
- Commit impact: max regression if threshold exceeded, otherwise geometric mean

## Training Exercises

Each exercise file contains:
1. **Concept explanation** in comments
2. **Benchmark code** to run and observe
3. **What to look for** in the CodSpeed dashboard
4. **Follow-up tasks** to deepen understanding

Start with `01_cpu_simulation.rs` and work through sequentially.

## References

- [CodSpeed Docs](https://codspeed.io/docs)
- [CPU Simulation](https://codspeed.io/docs/instruments/cpu)
- [Walltime](https://codspeed.io/docs/instruments/walltime)
- [Memory](https://codspeed.io/docs/instruments/memory)
- [Benchmark Variance](https://codspeed.io/docs/instruments/cpu/regression-causes)
- [Reducing Variance](https://codspeed.io/docs/instruments/cpu/reducing-variance)
- [Profiling & Flamegraphs](https://codspeed.io/docs/features/profiling)
- [MCP Server](https://codspeed.io/docs/ai/mcp.md)
