# Demo Video Script — Port Mortem
# 5-minute video showing original test suite passing against our port

## Outline (5 minutes)

### 0:00-0:30 — Introduction
- "This is our Port Mortem submission: textdistance Python → Rust"
- "30+ algorithms, 2000+ lines of Python, fully ported to Rust"
- Show the repo structure

### 0:30-1:00 — Build
- `cargo build --release` — one command, clean build
- Show binary size
- Show zero unsafe blocks: `cargo audit` or grep

### 1:00-2:00 — Test Parity
- Run `scripts/verify_test_parity.sh`
- Show all tests passing
- Show test file hash verification
- "400 tests, all passing, test files unmodified"

### 2:00-3:00 — Differential Fuzzing
- Run `python3 fuzz/fuzz_harness.py --duration 60`
- Show zero divergences
- Show the fuzz log
- "60+ seconds of continuous differential testing, zero divergence"

### 3:00-4:00 — Benchmarks
- Show `bench/results.json`
- Show criterion HTML reports
- Highlight key speedups:
  - "Levenshtein: 50x faster"
  - "Jaro-Winkler: 30x faster"
  - "Startup time: 100x faster"
- Show RSS comparison

### 4:00-4:30 — Architecture
- Show DECISIONS.md highlights
- Show trait system (Distance/Similarity)
- Show the algorithm modules
- "Zero unsafe blocks, fully idiomatic Rust"

### 4:30-5:00 — Bug Catcher (if found)
- Show any bugs found in original via differential testing
- "We found [X] in the original Python implementation"
- Link to upstream issue

## Recording Setup
- Tool: OBS or QuickTime
- Resolution: 1920x1080
- Font size: large enough for terminal to be readable
- Voice: clear narration explaining each step

## Files to Show
- `src/lib.rs` — trait definitions
- `src/algorithms/edit/levenshtein.rs` — example algorithm
- `fuzz/log.txt` — fuzz results
- `bench/results.json` — benchmark results
- `DECISIONS.md` — architectural decisions
- `test_hash_baseline.txt` — test parity verification
