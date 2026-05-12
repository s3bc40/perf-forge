# AGENTS.md — perf-forge

## Project structure

- **Workspace Cargo.toml** at root (`members = ["rust-benches"]`). Actual package is `rust-benches/`. Don't add deps to the root Cargo.toml.
- **`codspeed-divan-compat`** is aliased as `divan` in `rust-benches/Cargo.toml`. It is CodSpeed's fork of the `divan` benchmarking crate — use its API.
- All 7 bench entries use `harness = false` (divan custom harness, not libtest).
- **Benches 04–07 are stubs** (`fn main() { divan::main(); }`). Only 01–03 have real content. Fill them in as part of the training plan.
- **`python-benches/`** has `pyproject.toml` with `pytest`, `pytest-codspeed`, and `ruff` — `tests/` has 3 benchmark files.
- **`.github/workflows/codspeed.yml` does not exist** — referenced in README but needs to be created.

## Commands

```bash
# Work from rust-benches/ directory
cargo codspeed build              # Build all benches
cargo codspeed run                # Validate locally (no actual measurement)
cargo codspeed run -m simulation  # CPU simulation mode
cargo codspeed run -m walltime    # Walltime mode (needs CodSpeed CI bare-metal runner)
cargo codspeed run -m memory      # Memory mode
cargo check                       # Standard cargo check from root or rust-benches/

# Work from python-benches/ directory
uv sync --dev                     # Install dependencies
uv run pytest tests/ --codspeed   # Run benchmarks
uv run ruff check tests/          # Lint
uv run ruff format tests/         # Format
```

## Divan / CodSpeed conventions

- **Benches that return non-`()` must wrap the value in `black_box()`** inside `bencher.bench(|| ...)` to prevent dead-code elimination. This is the most common gotcha — bare `bencher.bench(|| some_func())` that returns a `HashMap` will fail because `bench` expects `()`.
- `#[divan::bench]` benchmarks returning a value directly (without `bencher`) work fine — the top-level return is fine.
- When using `#[divan::bench(args = [...])]`, use `fn name(bencher: divan::Bencher, arg: Type)` with `bencher.bench(|| ...)`.
- Benchmark function names must not shadow helper function names (duplicate name in same module is an error).

## State of the repo

- The **README.md is the canonical training guide** — contains cycle estimation formula, variance causes, and key CodSpeed concepts. Read it before modifying benchmarks.
- `.zed/settings.json` has CodSpeed MCP server configured.
- **No pre-existing instruction files** (no CLAUDE.md, .cursorrules, etc.). This AGENTS.md is the only one.

## Rust crate quirks

- `rand = "0.8"` with `rand_chacha = "0.3"` are available for any benchmark needing RNG.
- Temp files in `02_walltime.rs` write to `/tmp/codspeed_bench_{PID}.txt` — PID-based names for uniqueness, no cleanup guarantee outside the bench function.
