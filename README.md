# textdistance-rs

**Port Mortem Submission** — Track D: Python → Rust

A complete Rust port of [life4/textdistance](https://github.com/life4/textdistance), a library with 43 algorithms for computing distance between sequences.

## Why Rust

Python's `textdistance` is a pure-Python library. The algorithms are correct but slow for large-scale use. Rust offers:

- **77–1,788× faster** on edit-distance algorithms (criterion.rs benchmarks)
- **Zero runtime dependencies** — single 1.0MB binary, no Python interpreter needed
- **Memory safety** — 0 unsafe blocks, zero panics on valid input
- **Deterministic performance** — no GC pauses, no import overhead

## Build

```bash
cargo build --release
```

One command. Produces `target/release/textdistance` (1.0MB).

## Usage

```bash
# CLI — 36 subcommands
./target/release/textdistance hamming "test" "text"
./target/release/textdistance levenshtein "test" "text"
./target/release/textdistance jaro-winkler "MARTHA" "MARHTA"
./target/release/textdistance jaccard "test" "text"

# Library — 43 algorithms including vector-based
use textdistance::algorithms::edit::levenshtein::Levenshtein;
let alg = Levenshtein;
let dist = alg.distance(&['t','e','s','t'], &['t','e','x','t']);
```

## Algorithms (43 total)

### Edit-based (10)
Hamming, Levenshtein, Damerau-Levenshtein (restricted + unrestricted), Jaro, Jaro-Winkler, StrCmp95, Needleman-Wunsch, Smith-Waterman, Gotoh, MLIPNS

### Token-based (8)
Jaccard, Sorensen-Dice, Tversky, Overlap, Cosine, Tanimoto, MongeElkan, Bag

### Sequence-based (3)
Longest Common Subsequence (LCSSeq), Longest Common Substring (LCSStr), Ratcliff-Obershelp

### Simple (5)
Prefix, Postfix, Length, Identity, Matrix

### Phonetic (2)
MRA (Match Rating Approach), Editex

### Compression-based (8)
Arithmetic NCD, RLE NCD, BWT-RLE NCD, BZ2 NCD, LZMA NCD, ZLIB NCD, Sqrt NCD, Entropy NCD

### Vector-based (7)
Chebyshev, Minkowski, Manhattan, Euclidean, Mahalanobis, Correlation, Kulsinski

## Parity Results

| Category | Result |
|----------|--------|
| CLI parity | **35/36** match Python exactly |
| Adapter tests | **84/84** pass |
| Unit tests | **66/66** pass |
| Fuzzer | **98.6%** match (7 divergences on 504 tests) |

The single CLI divergence (lzma-ncd) is due to using `lzma_rs` (Rust) vs Python's `lzma` module — different compressor library, same monotonicity.

## Performance

| Algorithm | Python p50 | Rust p50 | Speedup |
|-----------|-----------|---------|---------|
| Levenshtein | 24,342 ns | 139 ns | **175×** |
| Hamming | 5,667 ns | 3 ns | **1,788×** |
| Damerau-Levenshtein | 34,742 ns | 168 ns | **207×** |
| Jaro | 4,992 ns | 65 ns | **77×** |
| Jaro-Winkler | 5,475 ns | 64 ns | **85×** |

*Benchmarks: criterion.rs (Rust, 100 samples) vs time.perf_counter() (Python, 1000 runs)*

## Testing

```bash
# Rust unit tests
cargo test

# Full test parity verification (requires Python + textdistance)
PYTHON_BIN=/path/to/python3 RUST_BIN=./target/release/textdistance \
    python3 adapter/test_adapter.py --verbose

# Differential fuzzing (60s minimum for bonus points)
python3 fuzz/fuzz_harness.py --duration 60

# Benchmarks
cargo bench
```

## Project Structure

```
textdistance-rs/
├── src/
│   ├── lib.rs                    # Traits: Distance, Similarity, VectorDistance
│   ├── main.rs                   # CLI with 36 subcommands
│   └── algorithms/
│       ├── edit/                 # 10 edit-distance algorithms
│       ├── token/                # 8 token-based algorithms
│       ├── sequence/             # 3 sequence algorithms
│       ├── simple/               # 5 simple algorithms
│       ├── phonetic/             # 2 phonetic algorithms
│       ├── compression/          # 8 compression-based NCD
│       └── vector/               # 7 vector-based algorithms
├── fuzz/
│   ├── fuzz_harness.py           # Differential fuzzer
│   └── log.txt                   # 62s fuzz log (504 tests, 7 divergences)
├── bench/
│   ├── methodology.md            # Benchmark methodology
│   ├── results.json              # Criterion + Python comparison results
│   └── criterion_output.txt      # Raw criterion output
├── adapter/
│   └── test_adapter.py           # Test parity adapter (84 tests)
├── tests/
│   ├── original/                 # Unmodified Python test suite
│   └── port/                     # Our Rust tests (in src/)
├── DECISIONS.md                  # 17+ architectural decisions
├── Dockerfile                    # One-command Docker build
└── .port-mortem.toml             # Submission metadata
```

## Key Architectural Decisions

See [DECISIONS.md](DECISIONS.md) for full details. Highlights:

- **No external algorithm crates** — all 43 algorithms implemented from scratch
- **Zero unsafe** — no FFI, no transmutes, no raw pointer arithmetic
- **Character-aware** — all string algorithms use `.chars().count()` not `.len()` for Unicode correctness
- **Trait-based design** — `Distance`, `Similarity`, `VectorDistance` traits for clean abstraction
- **BigInt arithmetic** — ArithNCD uses `num-rational::BigRational` for exact probability computation

## License

MIT — same as original.
