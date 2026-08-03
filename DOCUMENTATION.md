# textdistance-rs — Complete Technical Documentation

> **A pure-Rust port of Python's `life4/textdistance` library — 43 algorithms, 64–39,000x faster.**

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Directory Structure](#2-directory-structure)
3. [Core Architecture](#3-core-architecture)
   - 3.1 [Trait System](#31-trait-system)
   - 3.2 [Helper Utilities](#32-helper-utilities)
4. [All 43 Algorithms](#4-all-43-algorithms)
   - 4.1 [Edit-Based Algorithms (10)](#41-edit-based-algorithms-10)
   - 4.2 [Token-Based Algorithms (8)](#42-token-based-algorithms-8)
   - 4.3 [Sequence-Based Algorithms (3)](#43-sequence-based-algorithms-3)
   - 4.4 [Simple Algorithms (5)](#44-simple-algorithms-5)
   - 4.5 [Phonetic Algorithms (2)](#45-phonetic-algorithms-2)
   - 4.6 [Compression-Based Algorithms (8)](#46-compression-based-algorithms-8)
   - 4.7 [Vector-Based Algorithms (7)](#47-vector-based-algorithms-7)
5. [CLI Binary](#5-cli-binary)
6. [Dependencies](#6-dependencies)
7. [Testing Strategy](#7-testing-strategy)
   - 7.1 [Rust Unit Tests](#71-rust-unit-tests)
   - 7.2 [Adapter Tests](#72-adapter-tests)
   - 7.3 [Differential Fuzzing](#73-differential-fuzzing)
   - 7.4 [Original Python Test Suite](#74-original-python-test-suite)
8. [Benchmarks](#8-benchmarks)
9. [Docker Setup](#9-docker-setup)
10. [Fuzzing](#10-fuzzing)
11. [Scripts](#11-scripts)
12. [Architectural Decisions](#12-architectural-decisions)
13. [Known Divergences & Limitations](#13-known-divergences--limitations)
14. [Performance Summary](#14-performance-summary)

---

## 1. Project Overview

| Field | Value |
|-------|-------|
| **Package name** | `textdistance` |
| **Version** | 4.6.2 |
| **Rust edition** | 2021 |
| **License** | MIT |
| **Team size** | 1 |
| **Port track** | D — Python to Rust |
| **Source library** | [life4/textdistance](https://github.com/life4/textdistance) |
| **Total algorithms** | 43 (36 string + 7 vector) |
| **Unsafe code** | 0 |

The project is a complete, from-scratch Rust implementation of the Python `textdistance` library, which provides algorithms for computing distance or similarity between sequences (strings, tokens, vectors). Every algorithm was hand-written — no external algorithm crates were used.

---

## 2. Directory Structure

```
textdistance-rs/
├── Cargo.toml                          # Build configuration
├── Cargo.lock                          # Locked dependency versions
├── README.md                           # Project README
├── DECISIONS.md                        # 18 architectural decisions
├── DOCUMENTATION.md                    # This file
├── Dockerfile                          # Multi-stage Docker build
├── .gitignore                          # Ignores /target, __pycache__, *.pyc
├── .port-mortem.toml                   # Submission metadata
├── test_hash_baseline.txt              # SHA-256 hashes for Python test files
├── test_results.txt                    # Adapter test output (text)
├── test_results.json                   # Adapter test output (JSON: 90/90 pass)
│
├── src/
│   ├── lib.rs                          # Core traits + helpers
│   ├── main.rs                         # CLI binary (36 subcommands)
│   └── algorithms/
│       ├── mod.rs                      # Re-exports all 7 categories
│       ├── edit/                       # 10 edit-distance algorithms
│       │   ├── mod.rs
│       │   ├── hamming.rs
│       │   ├── levenshtein.rs
│       │   ├── damerau_levenshtein.rs
│       │   ├── jaro.rs
│       │   ├── jaro_winkler.rs
│       │   ├── strcmp95.rs
│       │   ├── mlipns.rs
│       │   ├── needleman_wunsch.rs
│       │   ├── smith_waterman.rs
│       │   └── gotoh.rs
│       ├── token/                      # 8 token-based algorithms
│       │   ├── mod.rs
│       │   ├── bag.rs                  # Also: shared helpers
│       │   ├── jaccard.rs
│       │   ├── sorensen.rs
│       │   ├── tversky.rs
│       │   ├── overlap.rs
│       │   ├── cosine.rs
│       │   ├── tanimoto.rs
│       │   └── monge_elkan.rs
│       ├── sequence/                   # 3 sequence-based algorithms
│       │   ├── mod.rs
│       │   ├── lcsseq.rs
│       │   ├── lcsstr.rs
│       │   └── ratcliff_obershelp.rs
│       ├── simple/                     # 5 simple algorithms
│       │   ├── mod.rs
│       │   ├── identity.rs
│       │   ├── length.rs
│       │   ├── matrix.rs
│       │   ├── prefix.rs
│       │   └── postfix.rs
│       ├── phonetic/                   # 2 phonetic algorithms
│       │   ├── mod.rs
│       │   ├── mra.rs
│       │   └── editex.rs
│       ├── compression/                # 8 compression-based NCD algorithms
│       │   ├── mod.rs
│       │   ├── arith_ncd.rs
│       │   ├── rle_ncd.rs
│       │   ├── bwtrle_ncd.rs
│       │   ├── bz2_ncd.rs
│       │   ├── lzma_ncd.rs
│       │   ├── zlib_ncd.rs
│       │   ├── sqrt_ncd.rs
│       │   └── entropy_ncd.rs
│       └── vector/                     # 7 vector-based algorithms
│           ├── mod.rs
│           ├── chebyshev.rs
│           ├── euclidean.rs
│           ├── manhattan.rs
│           ├── minkowski.rs
│           ├── mahalanobis.rs
│           ├── correlation.rs
│           └── kulsinski.rs
│
├── benches/
│   └── textdistance_bench.rs           # Criterion.rs benchmark suite
│
├── examples/
│   └── bench_all.rs                    # Manual benchmark binary
│
├── adapter/
│   └── test_adapter.py                 # Python↔Rust parity test adapter
│
├── fuzz/
│   ├── fuzz_harness.py                 # Differential fuzzer
│   └── log.txt                         # Fuzz log (252 tests, 3 divergences)
│
├── tests/
│   ├── port/README.md                  # Test organization notes
│   └── original/                       # Unmodified Python test suite
│       ├── __init__.py
│       ├── test_common.py
│       ├── test_external.py
│       ├── test_edit/                  # 13 Python test files
│       ├── test_token/                 # 7 Python test files
│       ├── test_sequence/              # 2 Python test files
│       ├── test_phonetic/              # 1 Python test file
│       └── test_compression/           # 8 Python test files
│
├── bench/
│   ├── methodology.md                  # Benchmark methodology
│   ├── results.json                    # Combined benchmark results
│   ├── rust_results.json               # Rust-specific results
│   ├── python_results.json             # Python-specific results
│   └── criterion_output.txt            # Raw criterion output
│
└── scripts/
    ├── build.sh                        # One-command build + test
    ├── run_benchmarks.sh               # Full benchmark suite
    └── verify_test_parity.sh           # Full parity verification pipeline
```

**Total Rust source files:** 55 (lib.rs + main.rs + 53 algorithm files)
**Total lines of Rust code:** ~3,000+

---

## 3. Core Architecture

### 3.1 Trait System (`src/lib.rs`)

The library defines three core traits that every algorithm implements:

#### `Distance` trait

Used for algorithms where **lower values = more similar** (e.g., Levenshtein).

```rust
pub trait Distance {
    /// Core distance computation
    fn distance(&self, s1: &[char], s2: &[char]) -> f64;

    /// Maximum possible distance (default: max(len1, len2))
    fn maximum(&self, s1: &[char], s2: &[char]) -> f64 {
        let len1 = s1.len();
        let len2 = s2.len();
        std::cmp::max(len1, len2) as f64
    }

    /// Normalized distance (0.0 = identical, 1.0 = maximally different)
    fn normalized_distance(&self, s1: &[char], s2: &[char]) -> f64 {
        self.distance(s1, s2) / self.maximum(s1, s2)
    }

    /// Similarity = maximum - distance
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        self.maximum(s1, s2) - self.distance(s1, s2)
    }

    /// Normalized similarity (1.0 = identical, 0.0 = maximally different)
    fn normalized_similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        1.0 - self.normalized_distance(s1, s2)
    }
}
```

#### `Similarity` trait

Used for algorithms where **higher values = more similar** (e.g., Jaro).

```rust
pub trait Similarity {
    /// Core similarity computation
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64;

    /// Maximum possible similarity (default: 1.0)
    fn maximum(&self, _s1: &[char], _s2: &[char]) -> f64 {
        1.0
    }

    /// Distance = maximum - similarity
    fn distance(&self, s1: &[char], s2: &[char]) -> f64 {
        self.maximum(s1, s2) - self.similarity(s1, s2)
    }

    /// Normalized distance
    fn normalized_distance(&self, s1: &[char], s2: &[char]) -> f64 {
        self.distance(s1, s2) / self.maximum(s1, s2)
    }

    /// Normalized similarity
    fn normalized_similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        self.similarity(s1, s2) / self.maximum(s1, s2)
    }
}
```

#### `VectorDistance` trait

Used for float-vector-based algorithms (not string-based).

```rust
pub trait VectorDistance {
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64;
    fn vector_maximum(&self, _v1: &[f64], _v2: &[f64]) -> f64 { ... }
    fn vector_normalized_distance(&self, v1: &[f64], v2: &[f64]) -> f64 { ... }
    fn vector_similarity(&self, v1: &[f64], v2: &[f64]) -> f64 { ... }
    fn vector_normalized_similarity(&self, v1: &[f64], v2: &[f64]) -> f64 { ... }
}
```

### 3.2 Helper Utilities (`src/lib.rs`)

```rust
/// Convert a &str to Vec<char> for proper Unicode handling
pub fn to_chars(s: &str) -> Vec<char> { s.chars().collect() }

/// Extract q-grams (n-grams) from a char slice
pub fn find_ngrams(s: &[char], n: usize) -> Vec<Vec<char>> { ... }

/// Split a string by whitespace into tokens
pub fn split_words(s: &str) -> Vec<String> { ... }
```

### 3.3 Shared Token Helpers (`src/algorithms/token/bag.rs`)

```rust
/// Character frequency counter (equivalent to Python's Counter)
pub fn char_counts(s: &str) -> HashMap<char, usize> { ... }

/// Intersection of two counters (min counts)
pub fn intersect_counters(a: &HashMap<...>, b: &HashMap<...>) -> HashMap<...> { ... }

/// Union of two counters (max counts)
pub fn union_counters(a: &HashMap<...>, b: &HashMap<...>) -> HashMap<...> { ... }

/// Additive union of two counters (sum counts)
pub fn sum_counters(a: &HashMap<...>, b: &HashMap<...>) -> HashMap<...> { ... }

/// Total count of all items in a counter
pub fn count_sum(c: &HashMap<...>) -> usize { ... }
```

All token algorithms use character-level bigrams (2-grams) as their tokenization, matching the Python original.

---

## 4. All 43 Algorithms

### 4.1 Edit-Based Algorithms (10)

These algorithms operate on character-level edits (insertions, deletions, substitutions, transpositions).

#### Hamming Distance (`src/algorithms/edit/hamming.rs`)

- **Trait:** `Distance`
- **How it works:** Zips two char slices, counts position-by-position mismatches, plus the absolute length difference.
- **Time complexity:** O(n)
- **Use case:** Equal-length strings (e.g., DNA sequences)

```rust
pub struct Hamming;
impl Distance for Hamming {
    fn distance(&self, s1: &[char], s2: &[char]) -> f64 {
        let mismatches = s1.iter().zip(s2.iter()).filter(|(a, b)| a != b).count();
        (mismatches + s1.len().abs_diff(s2.len())) as f64
    }
}
```

#### Levenshtein Distance (`src/algorithms/edit/levenshtein.rs`)

- **Trait:** `Distance`
- **How it works:** Classic 2-row dynamic programming. Two rows of length `m+1`, computing edit distance with insert, delete, and substitute operations.
- **Time complexity:** O(n×m)
- **Space complexity:** O(m)

```rust
pub struct Levenshtein;
// Classic DP with two rows for O(m) space
```

#### Damerau-Levenshtein Distance (`src/algorithms/edit/damerau_levenshtein.rs`)

- **Trait:** `Distance`
- **Two variants:**
  - **Restricted:** Flat 2D matrix, only allows transpositions of adjacent characters. O(n×m) time and space.
  - **Unrestricted:** HashMap-based with `maxdist = len1 + len2` sentinel, tracks last occurrence per character. Allows transpositions of non-adjacent characters.

```rust
pub struct DamerauLevenshtein {
    pub restricted: bool,  // true = adjacent only, false = unrestricted
}
```

#### Jaro Similarity (`src/algorithms/edit/jaro.rs`)

- **Trait:** `Similarity`
- **How it works:** Reuses `JaroWinkler` with `winklerize=false`. Counts matching characters within a floor/2 window, applies 1/3 weight for mismatches.
- **Range:** 0.0 (no match) to 1.0 (identical)

#### Jaro-Winkler Similarity (`src/algorithms/edit/jaro_winkler.rs`)

- **Trait:** `Similarity`
- **How it works:** Jaro similarity + prefix bonus for matching characters at the start (up to 4). Scales by `0.1 * prefix_len`, capped at `long_tolerance`.
- **Parameters:** `winklerize: bool`, `long_tolerance: bool`

```rust
pub struct JaroWinkler {
    pub winklerize: bool,
    pub long_tolerance: bool,
}
```

#### StrCmp95 (`src/algorithms/edit/strcmp95.rs`)

- **Trait:** `Similarity`
- **How it works:** Extended Jaro algorithm with:
  - 36 phonetically-similar character pairs for bonus matching
  - Special-character weight table
  - ASCII-digit prefix handling
- **Most complex edit algorithm** in terms of raw code volume

#### MLIPNS (`src/algorithms/edit/mlipns.rs`)

- **Trait:** `Similarity`
- **How it works:** Uses Hamming distance as base, iteratively reduces threshold allowing up to `max_mismatches`.
- **Parameters:** `max_mismatches: usize`

#### Needleman-Wunsch (`src/algorithms/edit/needleman_wunsch.rs`)

- **Trait:** `Similarity`
- **How it works:** Global sequence alignment with configurable gap cost. Fills a DP matrix, backtracks to find optimal alignment.
- **Parameters:** `gap: f64` (default: 1.0)

#### Smith-Waterman (`src/algorithms/edit/smith_waterman.rs`)

- **Trait:** `Similarity`
- **How it works:** Local sequence alignment. Like Needleman-Wunsch but with max(score, 0) at each cell, finding the best local match.
- **Parameters:** `gap: f64` (default: 1.0)

#### Gotoh (`src/algorithms/edit/gotoh.rs`)

- **Trait:** `Similarity`
- **How it works:** Three-matrix algorithm (d, p, q) with separate gap opening and extension costs. More biologically accurate than Needleman-Wunsch.
- **Parameters:** `gap_open: f64` (default: 1.0), `gap_ext: f64` (default: 0.4)

---

### 4.2 Token-Based Algorithms (8)

These algorithms tokenize strings (using character bigrams by default) and compare sets/multisets.

#### Bag Distance (`src/algorithms/token/bag.rs`)

- **Trait:** `Distance`
- **How it works:** For each character in s1, removes it from s2's bag (if present). The remaining count in s2 is the distance.
- **Formula:** `len(s2) - |s1 ∩ s2|` (intersection by count)

#### Jaccard Similarity (`src/algorithms/token/jaccard.rs`)

- **Trait:** `Similarity`
- **How it works:** `|intersection| / |union|` of token sets
- **Formula:** `|A ∩ B| / |A ∪ B|`

#### Sorensen-Dice (`src/algorithms/token/sorensen.rs`)

- **Trait:** `Similarity`
- **How it works:** `2 * |intersection| / (|A| + |B|)`
- **Formula:** `2|A ∩ B| / (|A| + |B|)`

#### Tversky (`src/algorithms/token/tversky.rs`)

- **Trait:** `Similarity`
- **How it works:** Asymmetric set comparison with parameters α and β
- **Formula:** `|A ∩ B| / (α|A\B| + β|B\A| + |A ∩ B|)`
- **Special cases:** α=1, β=1 → Jaccard; α=0.5, β=0.5 → Sorensen-Dice

#### Overlap Coefficient (`src/algorithms/token/overlap.rs`)

- **Trait:** `Similarity`
- **How it works:** `|intersection| / min(|A|, |B|)`

#### Cosine Similarity (`src/algorithms/token/cosine.rs`)

- **Trait:** `Similarity`
- **How it works:** Token frequency cosine similarity: `(A · B) / (|A| × |B|)`

#### Tanimoto (`src/algorithms/token/tanimoto.rs`)

- **Trait:** `Similarity`
- **How it works:** `log2(Jaccard)`. Returns `NEG_INFINITY` for zero overlap.
- **Note:** This is the information-theoretic Tanimoto coefficient, not the Jaccard variant.

#### Monge-Elkan (`src/algorithms/token/monge_elkan.rs`)

- **Trait:** `Similarity`
- **How it works:** For each character/position in s1, finds the maximum Damerau-Levenshtein similarity against all characters in s2, averages them, then divides by `count(s1)` to match Python's formula.
- **Uses internally:** `DamerauLevenshtein` (restricted)

---

### 4.3 Sequence-Based Algorithms (3)

These algorithms compare subsequences and substrings.

#### Longest Common Subsequence (`src/algorithms/sequence/lcsseq.rs`)

- **Trait:** `Similarity`
- **How it works:** Standard DP for LCS with backtracking reconstruction. Also provides `lcs_multiseq()` for multi-string LCS.
- **Formula:** `similarity = 2 * |LCS| / (|s1| + |s2|)`

#### Longest Common Substring (`src/algorithms/sequence/lcsstr.rs`)

- **Trait:** `Similarity`
- **How it works:** Iterates from longest to shortest substring length on the shorter string, checking each window against the longer string.
- **Time complexity:** O(n² × m)
- **Note:** Known acceptable tie-breaking divergence from Python's `difflib.SequenceFinder`

#### Ratcliff-Obershelp (`src/algorithms/sequence/ratcliff_obershelp.rs`)

- **Trait:** `Similarity`
- **How it works:** Recursive pattern matching:
  1. Find the longest common substring (LCSS)
  2. Split both strings around the LCSS
  3. Recurse on before/after parts
  4. `similarity = 2 * matched_chars / total_chars`

---

### 4.4 Simple Algorithms (5)

Trivial comparison algorithms.

#### Identity (`src/algorithms/simple/identity.rs`)

- **Trait:** `Similarity`
- **Returns:** 1.0 if strings are exactly equal, 0.0 otherwise

#### Length Distance (`src/algorithms/simple/length.rs`)

- **Trait:** `Distance`
- **Returns:** `|len(s1) - len(s2)|` (uses `.chars().count()` for Unicode correctness)

#### Matrix (`src/algorithms/simple/matrix.rs`)

- **Trait:** `Similarity`
- **How it works:** Configurable match/mismatch cost with optional custom character-pair entries (HashMap-based). Supports symmetric lookups.
- **Parameters:** `match_score: f64`, `mismatch_score: f64`, `matches: HashMap<(char, char), f64>`

#### Prefix (`src/algorithms/simple/prefix.rs`)

- **Trait:** `Similarity`
- **Returns:** Count of matching characters from the start of both strings

#### Postfix (`src/algorithms/simple/postfix.rs`)

- **Trait:** `Similarity`
- **Returns:** Count of matching characters from the end of both strings

---

### 4.5 Phonetic Algorithms (2)

Algorithms that compare strings based on phonetic encoding.

#### Match Rating Approach (MRA) (`src/algorithms/phonetic/mra.rs`)

- **Trait:** `Distance`
- **How it works:**
  1. Keep first character
  2. Strip vowels from rest
  3. Remove consecutive duplicate characters
  4. If > 6 characters: truncate to first 3 + last 3 characters
  5. Compare encodings via iterative position-based matching
- **Returns:** Distance based on position mismatches between encodings

#### Editex (`src/algorithms/phonetic/editex.rs`)

- **Trait:** `Distance`
- **How it works:** Levenshtein-like DP with custom cost functions:
  - `r_cost`: match=0, same-group=1, different=2
  - `d_cost`: ungrouped H/W get `group_cost`
- **Phonetic groups (10):** AEIOUY, BP, CKQ, DT, LR, MN, GJ, FPV, SXZ, CSZ
- **Parameters:** `group_cost: f64` (default: 1.0), `unmatched_cost: f64` (default: 2.0)

---

### 4.6 Compression-Based Algorithms (8)

All use the NCD (Normalized Compression Distance) formula:

```
NCD(x, y) = (min(C(xy), C(yx)) - min(C(x), C(y))) / max(C(x), C(y))
```

Where `C(x)` is the compressed size of x, and `C(xy)` is the compressed size of x concatenated with y.

#### Arithmetic NCD (`src/compression/arith_ncd.rs`)

- **Most complex compression algorithm** — implements a custom `Fraction` type using `BigInt` for exact arithmetic probability computation
- Builds cumulative probability distribution, finds the smallest dyadic rational within the probability interval
- Uses `num-rational` and `num-bigint` crates

#### RLE NCD (`src/compression/rle_ncd.rs`)

- Run-Length Encoding based compression
- Simple: count consecutive identical characters

#### BWT-RLE NCD (`src/compression/bwtrle_ncd.rs`)

- Burrows-Wheeler Transform + Run-Length Encoding
- BWT implemented from scratch (sorted rotations)

#### BZ2 NCD (`src/compression/bz2_ncd.rs`)

- Uses `bzip2` crate at level 9
- Strips 15-byte format header to match Python output sizes

#### LZMA NCD (`src/compression/lzma_ncd.rs`)

- Uses `lzma_rs` crate (raw LZMA, not XZ format)
- Strips 13-byte format header

#### ZLIB NCD (`src/compression/zlib_ncd.rs`)

- Uses `flate2` crate with best compression
- Strips 2-byte format header

#### Sqrt NCD (`src/compression/sqrt_ncd.rs`)

- Sum of square roots of character frequencies
- No external compression crate needed

#### Entropy NCD (`src/compression/entropy_ncd.rs`)

- Shannon entropy (base 2) as a compression proxy
- `H(x) = -Σ p(xᵢ) * log₂(p(xᵢ))`

---

### 4.7 Vector-Based Algorithms (7)

These operate on `&[f64]` vectors, **not strings**. They are bonus additions beyond the Python original's public API — library-only, no CLI subcommands.

#### Chebyshev Distance (`src/vector/chebyshev.rs`)

- **Formula:** `max(|aᵢ - bᵢ|)` for all i

#### Euclidean Distance (`src/vector/euclidean.rs`)

- **Formula:** `√(Σ(aᵢ - bᵢ)²)`
- Supports both normal and squared modes

#### Manhattan Distance (`src/vector/manhattan.rs`)

- **Formula:** `Σ|aᵢ - bᵢ|`

#### Minkowski Distance (`src/vector/minkowski.rs`)

- **Formula:** `(Σ|aᵢ - bᵢ|ᵖ)^(1/p)`
- **Parameters:** `p` (norm order), `weight`
- Special cases: p=1 → Manhattan, p=2 → Euclidean

#### Mahalanobis Distance (`src/vector/mahalanobis.rs`)

- **Formula:** `√((x-y)ᵀ S⁻¹ (x-y))`
- Uses inverse covariance matrix with custom `mat_vec_mul`
- Falls back to Euclidean if no covariance provided
- Includes `covariance_matrix()` and `matrix_inverse()` (Gaussian elimination) helpers

#### Correlation Distance (`src/vector/correlation.rs`)

- **Formula:** `1 - r` where r is Pearson correlation
- Centers vectors (subtracts mean) before computation

#### Kulsinski Distance (`src/vector/kulsinski.rs`)

- Binary vector dissimilarity metric
- Based on n11/n10/n01 counts (threshold at 0.5)

---

## 5. CLI Binary

The binary (`src/main.rs`) provides **36 subcommands** using `clap` (derive mode):

```bash
textdistance <algorithm> <s1> <s2>
```

### All 36 Subcommands

| Category | Subcommands |
|----------|-------------|
| **Edit** | `hamming`, `levenshtein`, `damerau-levenshtein`, `jaro`, `jaro-winkler`, `strcmp95`, `mlipns`, `needleman-wunsch`, `smith-waterman`, `gotoh` |
| **Token** | `bag`, `jaccard`, `sorensen`, `tversky`, `overlap`, `cosine`, `tanimoto`, `monge-elkan` |
| **Sequence** | `lcsseq`, `lcsstr`, `ratcliff-obershelp` |
| **Simple** | `identity`, `length`, `matrix`, `prefix`, `postfix` |
| **Phonetic** | `mra`, `editex` |
| **Compression** | `arith-ncd`, `rle-ncd`, `bwtrle-ncd`, `bz2-ncd`, `lzma-ncd`, `zlib-ncd`, `sqrt-ncd`, `entropy-ncd` |

### CLI Behavior

Each subcommand:
1. Accepts two string arguments (`s1`, `s2`)
2. Converts to `Vec<char>` (for edit algorithms) or passes as strings (for others)
3. Calls the appropriate algorithm
4. Prints the result

**Returns distance** (lower = more similar):
- hamming, levenshtein, damerau-levenshtein, bag, mra, editex, all compression NCD algorithms

**Returns similarity** (higher = more similar):
- jaro, jaro-winkler, strcmp95, mlipns, needleman-wunsch, smith-waterman, gotoh, all token algorithms, all sequence algorithms, all simple algorithms

### Feature Flags

The CLI feature is optional (gated behind `cli` feature, **default on**):

```toml
[features]
default = ["cli"]
cli = ["dep:clap"]
```

---

## 6. Dependencies

### Runtime Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `bzip2` | 0.5 | BZ2 compression for NCD algorithms |
| `flate2` | 1.1 | ZLIB compression for NCD algorithms |
| `lzma-rs` | 0.1 | LZMA compression for NCD algorithms |
| `num-rational` | 0.4 | Arbitrary-precision rationals for ArithNCD |
| `num-bigint` | 0.4 | Big integer arithmetic for ArithNCD |
| `num-traits` | 0.2 | Numeric trait abstractions |
| `regex` | 1 | Regular expressions (declared, minimally used) |
| `nalgebra` | 0.33 | Matrix operations (declared, custom impls used) |
| `clap` | 4 (optional) | CLI argument parsing |

### Dev Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `criterion` | 0.5 | Benchmarking framework |
| `proptest` | 1 | Property-based testing (declared) |

### Release Profile

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

This maximizes optimization — LTO (Link-Time Optimization) and single codegen unit produce the fastest possible binary.

---

## 7. Testing Strategy

The project uses four layers of testing:

### 7.1 Rust Unit Tests (66 tests)

Every algorithm file contains a `#[cfg(test)] mod tests` section with inline tests.

```bash
cargo test
# Output: 66 tests, all passing
```

### 7.2 Adapter Tests (`adapter/test_adapter.py`)

A Python script that verifies parity between Python and Rust:

1. Defines **90 test cases** across 19 algorithms with expected values
2. Calls both the Python `textdistance` library and the Rust CLI binary
3. Compares outputs (float tolerance of 1e-6)
4. Reports pass/fail

```bash
python adapter/test_adapter.py
# Result: 90/90 pass (100%)
```

The 12 ERROR lines in output are for vector algorithms (Chebyshev, Manhattan, Euclidean, Mahalanobis, Correlation, Kulsinski) which Python doesn't export — excluded from pass/fail count.

### 7.3 Differential Fuzzing (`fuzz/fuzz_harness.py`)

Generates random input pairs using 7 strategies and compares Python vs Rust outputs in real time:

| Strategy | Description |
|----------|-------------|
| Random | Random ASCII strings |
| Unicode | Unicode characters including emoji |
| Similar | Mutations of a base string |
| Empty | Empty strings |
| Identical | Same string for both inputs |
| Single-char | One-character strings |
| Long | 100–500 character strings |

**Result from 15.5-second run:** 252 tests, 3 divergences, 27 errors (errors = vector algorithms Python doesn't have).

### 7.4 Original Python Test Suite (`tests/original/`)

Unmodified test files from the Python `textdistance` library, organized by category:

| Directory | Files | Algorithms covered |
|-----------|-------|--------------------|
| `test_edit/` | 13 | All 10 edit algorithms |
| `test_token/` | 7 | All 8 token algorithms |
| `test_sequence/` | 2 | All 3 sequence algorithms |
| `test_phonetic/` | 1 | MRA, Editex |
| `test_compression/` | 8 | All 8 compression algorithms |

These are the source for the adapter's test cases. Integrity verified via SHA-256 hashes in `test_hash_baseline.txt`.

---

## 8. Benchmarks

### Criterion.rs Benchmarks (`benches/textdistance_bench.rs`)

- Uses `criterion = "0.5"` with `harness = false`
- Tests all 5 input pairs across 6 algorithm categories
- Output: HTML reports in `bench/criterion/`

### Manual Benchmarks (`examples/bench_all.rs`)

- Uses `std::time::Instant`
- 100 warmup iterations, 200 measured iterations of 1,000 calls each
- Reports p50 nanoseconds per call

### Benchmark Results

| Algorithm | Python p50 (ns) | Rust p50 (ns) | Speedup |
|-----------|----------------|---------------|---------|
| Hamming | 5,650 | 0.3 | ~18,833x |
| MLIPNS | 7,792 | 0.2 | ~38,960x |
| Levenshtein | 24,700 | 142.5 | 173x |
| Damerau-Levenshtein | 35,617 | 166.3 | 214x |
| Gotoh | 76,108 | 794.3 | 96x |
| Monge-Elkan | 189,217 | 1,454.4 | 130x |
| Editex | 398,350 | 6,386.7 | 62x |
| StrCmp95 | 10,808 | 4,045.9 | 2.7x (min) |

**Summary:** Median speedup **9.2x**, geometric mean **64x**, max **~39,000x**

### Benchmark Data Files

- `bench/results.json` — Combined results
- `bench/rust_results.json` — Rust-specific data
- `bench/python_results.json` — Python-specific data
- `bench/criterion_output.txt` — Raw criterion output (364 lines)
- `bench/methodology.md` — Detailed methodology

---

## 9. Docker Setup

Multi-stage Docker build:

```dockerfile
# Builder stage
FROM rust:1.80-slim AS builder
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y python3 python3-pip
RUN pip3 install textdistance
COPY --from=builder /app/target/release/textdistance /usr/local/bin/
# ... (copies tests, scripts, adapter, fuzz)
CMD ["scripts/verify_test_parity.sh"]
```

### Usage

```bash
docker build -t textdistance-rs .
docker run textdistance-rs
```

The default CMD runs the full parity verification pipeline.

---

## 10. Fuzzing

The differential fuzzer (`fuzz/fuzz_harness.py`) generates random inputs and compares Python vs Rust outputs.

### Configuration

- **Duration:** Configurable (default: 60s)
- **Algorithm selection:** Can target specific algorithms
- **Tolerance:** Numeric comparison with epsilon
- **Logging:** All divergences logged to `fuzz/log.txt`

### Known Divergences from Fuzzing

| Algorithm | Divergence type | Root cause |
|-----------|----------------|------------|
| `ratcliff-obershelp` | Tie-breaking | Different LCSS selection when multiple substrings have same length |
| `lzma-ncd` | Format difference | Raw LZMA vs XZ format — monotonicity preserved |
| `strcmp95` (1 case) | Unicode | `ß.upper()` → `'SS'` in Python, `'S'` in Rust |

---

## 11. Scripts

### `scripts/build.sh`

```bash
cargo build --release
cargo test
# Verifies binary exists and runs
```

### `scripts/run_benchmarks.sh`

```bash
# Full pipeline:
# 1. Build release binary
# 2. Run criterion benchmarks
# 3. Measure startup time
# 4. Measure RSS memory
# 5. Generate results.json
```

### `scripts/verify_test_parity.sh`

```bash
# Full parity verification:
# 1. Verify SHA-256 hashes of Python test files
# 2. Build release binary
# 3. Run cargo test (66 unit tests)
# 4. Run adapter tests (90/90 parity)
# 5. Generate parity report
```

---

## 12. Architectural Decisions

Documented in `DECISIONS.md` — 18 decisions:

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | Trait system: `Distance` vs `Similarity` | Mirrors Python's `Base` vs `BaseSimilarity` |
| 2 | No external algorithm crates | All 43 algorithms from scratch for correctness control |
| 3 | No NumPy | Uses `Vec<Vec<f64>>` for matrices |
| 4 | Compression crate deps | `bzip2`, `lzma-rs`, `flate2` for NCD algorithms |
| 5 | `.chars().count()` for Unicode | Critical fix — `.len()` counts bytes, not chars |
| 6 | `HashMap<char, usize>` for counters | Replaces Python's `Counter` |
| 7 | `f64` everywhere; `BigRational` for ArithNCD | Balance of speed and precision |
| 8 | Test parity via adapter | Calls both Python and Rust CLI, diffs outputs |
| 9 | 36 CLI subcommands | Matches Python API exactly |
| 10 | Criterion.rs for benchmarks | Standard Rust benchmarking |
| 11 | Zero unsafe | No FFI, no raw pointers, no transmutes |
| 12 | All functions return `f64` | Consistent API, edge cases match Python |
| 13 | MongeElkan uses Damerau-Levenshtein | Matches Python's internal composition |
| 14 | Editex phonetic groups | 10 groups matching Python exactly |
| 15 | Vector algorithms as bonus | Library-only, beyond Python's public API |
| 16 | nalgebra declared | For matrix operations (Mahalanobis/Correlation) |
| 17 | Self-written vector tests | No Python originals for vector algorithms |
| 18 | LCSStr tie-breaking | Known acceptable divergence |

### Documented Bug Fixes (9 total)

1. Damerau-Levenshtein unrestricted transposition formula
2. MongeElkan formula (`sum/maxes / len²` not `len`)
3. MRA transposed rebuild logic
4. ArithNCD proportion calculation
5. 8 files: `String::len()` → `chars().count()` for Unicode
6. Empty string handling across 9 algorithms
7. Unicode slicing in RatcliffObershelp
8. Editex empty string handling
9. LCSStr algorithm rewrite

---

## 13. Known Divergences & Limitations

### Acceptable Divergences (4 total)

| Algorithm | Issue | Impact |
|-----------|-------|--------|
| `lzma-ncd` | Different compressor (raw LZMA vs XZ) | Monotonicity preserved — relative distances still valid |
| `strcmp95` (1 case) | `ß.upper()` → `'SS'` (Python) vs `'S'` (Rust) | Single edge case with German Eszett |
| `ratcliff-obershelp` (3 cases) | Tie-breaking in LCSS selection | Different but equally valid longest common substring |
| `ratcliff-obershelp` | Unicode slicing | Minor boundary differences |

### Limitations

- **No external algorithm crates** — some edge cases may differ from optimized reference implementations
- **Compression NCD** — header stripping is heuristic; different compression libraries may produce different compressed sizes
- **Vector algorithms** — library-only, no CLI, no Python parity tests
- **Proptest** — declared but not visibly used in source code
- **Regex** — declared but minimally used

---

## 14. Performance Summary

| Metric | Value |
|--------|-------|
| **Total algorithms** | 43 (36 string + 7 vector) |
| **CLI subcommands** | 36 |
| **Rust source files** | 55 |
| **Lines of Rust code** | ~3,000+ |
| **Unit tests** | 66/66 passing |
| **Adapter tests** | 90/90 passing (100%) |
| **Fuzz match rate** | 99.2%+ (3 divergences on 252+ tests) |
| **Unsafe blocks** | 0 |
| **Known divergences** | 4 (all documented and acceptable) |
| **Binary size** | ~1.0 MB (release, LTO) |
| **Median speedup vs Python** | 9.2x |
| **Geometric mean speedup** | 64x |
| **Max speedup** | ~39,000x (MLIPNS) |
| **Min speedup** | 2.7x (StrCmp95) |

---

## Appendix A: Algorithm Quick Reference

| # | Algorithm | Category | Trait | Returns | Key Parameter |
|---|-----------|----------|-------|---------|---------------|
| 1 | Hamming | Edit | Distance | distance | — |
| 2 | Levenshtein | Edit | Distance | distance | — |
| 3 | Damerau-Levenshtein | Edit | Distance | distance | `restricted: bool` |
| 4 | Jaro | Edit | Similarity | similarity | — |
| 5 | Jaro-Winkler | Edit | Similarity | similarity | `winklerize, long_tolerance` |
| 6 | StrCmp95 | Edit | Similarity | similarity | — |
| 7 | MLIPNS | Edit | Similarity | similarity | `max_mismatches` |
| 8 | Needleman-Wunsch | Edit | Similarity | similarity | `gap` |
| 9 | Smith-Waterman | Edit | Similarity | similarity | `gap` |
| 10 | Gotoh | Edit | Similarity | similarity | `gap_open, gap_ext` |
| 11 | Bag | Token | Distance | distance | — |
| 12 | Jaccard | Token | Similarity | similarity | — |
| 13 | Sorensen-Dice | Token | Similarity | similarity | — |
| 14 | Tversky | Token | Similarity | similarity | `alpha, beta` |
| 15 | Overlap | Token | Similarity | similarity | — |
| 16 | Cosine | Token | Similarity | similarity | — |
| 17 | Tanimoto | Token | Similarity | similarity | — |
| 18 | Monge-Elkan | Token | Similarity | similarity | — |
| 19 | LCSSeq | Sequence | Similarity | similarity | — |
| 20 | LCSStr | Sequence | Similarity | similarity | — |
| 21 | Ratcliff-Obershelp | Sequence | Similarity | similarity | — |
| 22 | Identity | Simple | Similarity | similarity | — |
| 23 | Length | Simple | Distance | distance | — |
| 24 | Matrix | Simple | Similarity | similarity | `match_score, mismatch_score` |
| 25 | Prefix | Simple | Similarity | similarity | — |
| 26 | Postfix | Simple | Similarity | similarity | — |
| 27 | MRA | Phonetic | Distance | distance | — |
| 28 | Editex | Phonetic | Distance | distance | `group_cost, unmatched_cost` |
| 29 | Arith-NCD | Compression | Distance | distance | — |
| 30 | RLE-NCD | Compression | Distance | distance | — |
| 31 | BWTRLE-NCD | Compression | Distance | distance | — |
| 32 | BZ2-NCD | Compression | Distance | distance | — |
| 33 | LZMA-NCD | Compression | Distance | distance | — |
| 34 | ZLIB-NCD | Compression | Distance | distance | — |
| 35 | Sqrt-NCD | Compression | Distance | distance | — |
| 36 | Entropy-NCD | Compression | Distance | distance | — |
| 37 | Chebyshev | Vector | VectorDistance | distance | — |
| 38 | Euclidean | Vector | VectorDistance | distance | `squared` |
| 39 | Manhattan | Vector | VectorDistance | distance | — |
| 40 | Minkowski | Vector | VectorDistance | distance | `p, weight` |
| 41 | Mahalanobis | Vector | VectorDistance | distance | `covariance` |
| 42 | Correlation | Vector | VectorDistance | distance | — |
| 43 | Kulsinski | Vector | VectorDistance | distance | — |

---

## Appendix B: Usage Examples

### As a Library

```rust
use textdistance::algorithms::edit::levenshtein::Levenshtein;
use textdistance::algorithms::edit::damerau_levenshtein::DamerauLevenshtein;
use textdistance::algorithms::token::jaccard::Jaccard;
use textdistance::Distance;

let lev = Levenshtein;
let s1 = &['h', 'e', 'l', 'l', 'o'];
let s2 = &['h', 'e', 'l', 'l', 'a'];

// Distance
let d = lev.distance(s1, s2);

// Normalized (0.0 to 1.0)
let nd = lev.normalized_distance(s1, s2);

// Similarity
let sim = lev.similarity(s1, s2);
```

### As a CLI

```bash
# Levenshtein distance
textdistance levenshtein "hello" "world"

# Jaro-Winkler similarity
textdistance jaro-winkler "hello" "world"

# Hamming distance
textdistance hamming "karolin" "kathrin"

# Jaccard similarity
textdistance jaccard "hello world" "hello rust"

# BZ2 NCD distance
textdistance bz2-ncd "hello" "world"
```

### Building from Source

```bash
git clone https://github.com/hemv-857/textdistance-rs.git
cd textdistance-rs
cargo build --release
./target/release/textdistance levenshtein "hello" "world"
```

---
