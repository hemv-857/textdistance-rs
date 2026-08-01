# Demo Video Script (5 minutes)

## Setup (0:00 - 0:15)
- Show terminal with repo cloned
- `cd textdistance-rs`

## Section 1: Build (0:15 - 0:45)
```bash
cargo build --release
# Show binary size
ls -lh target/release/textdistance
```
- Talk: "Pure Rust, zero unsafe blocks, 1MB binary"

## Section 2: Unit Tests (0:45 - 1:15)
```bash
cargo test
```
- Show: 66/66 tests pass, zero warnings
- Talk: "All algorithms have unit tests"

## Section 3: Adapter Tests (1:15 - 2:00)
```bash
PYTHON_BIN=/Users/hemang/textdistance-original/.venv/bin/python3 \
  RUST_BIN=/Users/hemang/Desktop/textdistance-rs/target/release/textdistance \
  python3 adapter/test_adapter.py
```
- Show: 84/84 pass
- Talk: "Original Python test suite running against our port, unmodified"

## Section 4: Differential Fuzz (2:00 - 2:45)
```bash
PYTHON_BIN=/Users/hemang/textdistance-original/.venv/bin/python3 \
  RUST_BIN=/Users/hemang/Desktop/textdistance-rs/target/release/textdistance \
  python3 fuzz/fuzz_harness.py --duration 30
```
- Show: 4 divergences on 528+ tests (99.2% match)
- Show divergences are acceptable (ratcliff-obershelp tie-breaking, str-cmp95 Unicode)

## Section 5: CLI Demo (2:45 - 3:30)
```bash
# Show several algorithms
./target/release/textdistance levenshtein "hello" "world"
./target/release/textdistance hamming "test" "text"
./target/release/textdistance jaro "algorithm" "altruistic"
./target/release/textdistance jaccard "hello" "world"
./target/release/textdistance monge-elkan "hello" "world"
./target/release/textdistance lcsseq "hello" "world"
./target/release/textdistance ratcliff-obershelp "hello" "world"
./target/release/textdistance editex "hello" "world"
```

## Section 6: Benchmarks (3:30 - 4:15)
```bash
cargo run --release --example bench_all 2>&1 | head -40
```
- Show: Speedups from 2.7× to 38,000×
- Talk: "Geometric mean 64× faster than Python"

## Section 7: Summary (4:15 - 5:00)
- Show README.md
- Show DECISIONS.md
- Show GitHub repo
- Talk: "43 algorithms ported, 0 unsafe, 84/84 tests, 99.2% fuzz match"
- Show `.port-mortem.toml` with kickoff hash

## Key Points to Mention
- 72-hour port of life4/textdistance (3,500 stars)
- 43 algorithms (36 CLI + 7 vector)
- Zero unsafe blocks (+5 bonus)
- Differential fuzz survivor (+5 bonus)
- Bug catchers found: MRA transposed rebuild, ArithNCD proportions, 7 Unicode bugs
- 18 architectural decisions documented
