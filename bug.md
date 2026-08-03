# PORT MORTEM — Complete Bug Report & Resolution Guide

**Date:** 2026-08-02
**Scope:** Full scan of `python/textdistance/` — 36 bugs identified across all modules.
**Rust Cross-Check:** Each algorithm was also checked in `textdistance-rs/` for the same bugs.

---

## Table of Contents

1. [Critical Bugs (4)](#critical-bugs)
2. [High Bugs (10)](#high-bugs)
3. [Medium Bugs (14)](#medium-bugs)
4. [Low Bugs (8)](#low-bugs)
5. [Rust Cross-Reference Summary](#rust-cross-reference)
6. [Resolution Checklist](#resolution-checklist)

---

## Critical Bugs

### BUG-01: Arbitrary Code Execution via `import_module()`

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/libraries.py:109` |
| **Severity** | CRITICAL |
| **Rust** | N/A (no equivalent — Rust loads compiled binaries only) |

**Description:**
`import_module(self.module_name)` is called with a module name string loaded from a JSON file (`LIBRARIES_PATH`). If the JSON file is tampered with, an attacker can load any Python module, enabling arbitrary code execution.

```python
# libraries.py:109
module = import_module(self.module_name)  # DANGEROUS
```

**Resolution Steps:**
1. Create an allowlist of permitted modules at the top of `libraries.py`:
   ```python
   ALLOWED_MODULES = {
       'textdistance',
       'jellyfish',
       'fuzzy',
       'Levenshtein',
       'rapidfuzz',
       'jellyfish._jellyfish',
   }
   ```
2. Add validation before the import:
   ```python
   if self.module_name not in ALLOWED_MODULES:
       raise ValueError(f"Module '{self.module_name}' is not in the allowlist")
   module = import_module(self.module_name)
   ```
3. Optionally, validate `func_name` against a known function list as well.

---

### BUG-02: DamerauLevenshtein Off-by-One Return Value

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/edit_based.py:203` |
| **Severity** | CRITICAL |
| **Rust** | **NOT present** — Rust correctly returns `d[len1*(len2+1)+len2]` at `damerau_levenshtein.rs:75` |

**Description:**
The matrix is `(len(s1)+1) x len(s2)+1)`, but the return uses `d[len(s1)-1][len(s2)-1]` instead of `d[len(s1)][len(s2)]`. This returns the wrong distance value.

```python
# edit_based.py:203 — BUGGY
return d[len(s1) - 1][len(s2) - 1]
```

**Resolution Steps:**
1. Open `python/textdistance/algorithms/edit_based.py`
2. Change line 203 from:
   ```python
   return d[len(s1) - 1][len(s2) - 1]
   ```
   to:
   ```python
   return d[len(s1)][len(s2)]
   ```
3. Run existing tests to verify:
   ```bash
   python -m pytest python/tests/test_edit/test_damerau_levenshtein.py -v
   ```

---

### BUG-03: Tversky Bias Formula Mathematically Incorrect

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/token_based.py:136-142` |
| **Severity** | CRITICAL |
| **Rust** | **NOT present** — Rust correctly implements `tversky.rs:39` |

**Description:**
The bias branch computes an incorrect formula. The correct Tversky similarity is:
`T = |A∩B| / (|A∩B| + α|A-B| + β|B-A|)`

But the code computes:
```python
result = alpha * beta * (a_val - b_val) + b_val * beta
return c_val / (result + c_val)
```

**Resolution Steps:**
1. Replace lines 136-142 in `token_based.py` with:
   ```python
   s1, s2 = sequences
   alpha, beta = ks
   a_minus_b = s1 - intersection
   b_minus_a = s2 - intersection
   denominator = intersection + self.bias + alpha * a_minus_b + beta * b_minus_a
   if denominator == 0:
       return 0
   return (intersection + self.bias) / denominator
   ```
2. Run tests:
   ```bash
   python -m pytest python/tests/test_token/ -v
   ```

---

### BUG-04: MongeElkan Double Normalization

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/token_based.py:283` |
| **Severity** | CRITICAL |
| **Rust** | **PRESENT** — `monge_elkan.rs:43` has the same double division |

**Description:**
`sum(maxes) / len(seq) / len(maxes)` divides twice. The correct Monge-Elkan normalization is `sum(maxes) / len(seq)`.

```python
# token_based.py:283 — BUGGY
return sum(maxes) / len(seq) / len(maxes)
```

**Resolution Steps (Python):**
1. Change line 283 from:
   ```python
   return sum(maxes) / len(seq) / len(maxes)
   ```
   to:
   ```python
   return sum(maxes) / len(maxes)
   ```
   (Note: `len(maxes) == len(seq)` so dividing once is correct.)

**Resolution Steps (Rust):**
1. Open `textdistance-rs/src/algorithms/token/monge_elkan.rs:43`
2. Change:
   ```rust
   total / count as f64 / count as f64
   ```
   to:
   ```rust
   total / count as f64
   ```
3. Update the test to verify the fix.

---

## High Bugs

### BUG-05: Arbitrary Module Import — No Error Handling

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/libraries.py:115` |
| **Severity** | HIGH |
| **Rust** | N/A |

**Description:**
`getattr(module, self.func_name)` raises `AttributeError` if the function doesn't exist.

**Resolution Steps:**
1. Wrap in try/except:
   ```python
   try:
       obj = getattr(module, self.func_name)
   except AttributeError:
       raise ValueError(f"Function '{self.func_name}' not found in module '{self.module_name}'")
   ```

---

### BUG-06: Non-Callable Presets Passed Without Guard

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/libraries.py:118` |
| **Severity** | HIGH |
| **Rust** | N/A |

**Description:**
`obj(**self.presets)` will raise `TypeError` if `obj` is not callable or doesn't accept the preset kwargs.

**Resolution Steps:**
1. Add callable check:
   ```python
   if not callable(obj):
       raise TypeError(f"'{self.func_name}' is not callable")
   try:
       obj = obj(**self.presets)
   except TypeError as e:
       raise ValueError(f"Invalid presets for '{self.func_name}': {e}")
   ```

---

### BUG-07: Missing Attribute Access Without Error Handling

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/libraries.py:121` |
| **Severity** | HIGH |
| **Rust** | N/A |

**Description:**
`getattr(obj, self.attr)` silently fails if `attr` doesn't exist on the object.

**Resolution Steps:**
1. Add try/except:
   ```python
   try:
       result = getattr(obj, self.attr)
   except AttributeError:
       raise ValueError(f"Attribute '{self.attr}' not found on '{self.func_name}'")
   ```

---

### BUG-08: `libs_names.index()` ValueError

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/libraries.py:40` |
| **Severity** | HIGH |
| **Rust** | N/A |

**Description:**
`libs_names.index([lib.module_name, lib.func_name])` raises `ValueError` if the lib isn't in the JSON data.

**Resolution Steps:**
1. Use a safer lookup:
   ```python
   key = [lib.module_name, lib.func_name]
   try:
       idx = libs_names.index(key)
   except ValueError:
       continue  # or append new entry
   ```

---

### BUG-09: `limits[x.algorithm]` KeyError

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/benchmark.py:100` |
| **Severity** | HIGH |
| **Rust** | N/A |

**Description:**
`limits[x.algorithm]` raises `KeyError` if the external lib's algorithm name doesn't have a benchmark entry.

**Resolution Steps:**
1. Use `.get()` with a default:
   ```python
   limit = limits.get(x.algorithm, float('inf'))
   ```

---

### BUG-10: BaseSimilarity.quick_answer `max()` on Empty Sequences

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/base.py:182` |
| **Severity** | HIGH |
| **Rust** | N/A (no equivalent class) |

**Description:**
`self.maximum(*sequences)` calls `max()` with no arguments when sequences is empty, raising `ValueError`.

**Resolution Steps:**
1. Add empty check:
   ```python
   if not sequences:
       return 0
   return self.maximum(*sequences)
   ```

---

### BUG-11: Prefix Missing `super().__init__()`

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/simple.py:22` |
| **Severity** | HIGH |
| **Rust** | N/A (no class inheritance) |

**Description:**
`Prefix.__init__` never calls `super().__init__()`, so `self.external` is never set. Accessing `self.external` via inherited methods raises `AttributeError`.

**Resolution Steps:**
1. Add super call:
   ```python
   def __init__(self, qval: int = 1, sim_test=None) -> None:
       super().__init__(qval=qval)
       self.qval = qval
       self.sim_test = sim_test or self._ident
   ```

---

### BUG-12: MongeElkan `maximum()` Passes Tuple Instead of Unpacking

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/token_based.py:267` |
| **Severity** | HIGH |
| **Rust** | N/A (Rust uses a different architecture) |

**Description:**
`self.algorithm.maximum(sequences)` passes the tuple as a single argument instead of unpacking.

**Resolution Steps:**
1. Change line 267:
   ```python
   result = self.algorithm.maximum(*sequences)
   ```

---

### BUG-13: EntropyNCD Division by Zero on Empty Input

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/compression_based.py:230` |
| **Severity** | HIGH |
| **Rust** | **NOT present** — Rust has a guard at `entropy_ncd.rs:48` |

**Description:**
`element_count / total_count` causes `ZeroDivisionError` when `data` is empty.

**Resolution Steps:**
1. Add guard at the start of `_compress`:
   ```python
   def _compress(self, data: Sequence) -> float:
       total_count = len(data)
       if total_count == 0:
           return 0.0
       # ... rest of method
   ```

---

### BUG-14: MLIPNS Loop Logic Incorrect

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/edit_based.py:824-831` |
| **Severity** | HIGH |
| **Rust** | **PRESENT** — `mlipns.rs:50-57` has the same flawed logic |

**Description:**
The loop decrements `ham` by 1 each iteration but never recomputes the actual Hamming distance after removing mismatched characters. This produces incorrect results.

**Resolution Steps (Python):**
1. Rewrite the loop to actually remove the most-mismatched character and recompute:
   ```python
   def __call__(self, *sequences):
       sequences = self._get_sequences(*sequences)
       result = self.quick_answer(*sequences)
       if result is not None:
           return result
       for _ in range(self.maxmismatches + 1):
           ham = Hamming()(*sequences)
           maxlen = max(map(len, sequences))
           if maxlen == 0:
               return 1
           if 1 - (maxlen - ham) / maxlen <= self.threshold:
               return 1
           # Trim the most-mismatched character
           sequences = [s[:-1] for s in sequences]
       return 0
   ```

**Resolution Steps (Rust):**
1. Rewrite `mlipns.rs:50-57` with the same algorithmic fix.

---

## Medium Bugs

### BUG-15: `.split()` on Non-String Inputs

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/utils.py:14` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
Calling `.split()` on a non-string (e.g., `None`) raises `AttributeError`.

**Resolution Steps:**
1. Add type validation:
   ```python
   if not isinstance(text, str):
       raise TypeError(f"Expected str, got {type(text).__name__}")
   ```

---

### BUG-16: Empty Text Causes ValueError

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/utils.py:20` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
`min(subtexts, key=len)` fails on an empty subtexts list.

**Resolution Steps:**
1. Guard against empty input:
   ```python
   if not any(texts):
       return float('inf')
   ```

---

### BUG-17: Missing `.equality` Attribute Check

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/utils.py:19` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
If the function `f` lacks an `.equality` attribute, raises `AttributeError`.

**Resolution Steps:**
1. Use `getattr` with default:
   ```python
   if getattr(f, 'equality', False):
       # ...
   ```

---

### BUG-18: Fragile List Comparison in `libs_names`

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/libraries.py:38` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
Comparing `[module, func] in libs_names` is fragile against JSON structure changes.

**Resolution Steps:**
1. Convert to tuples for comparison:
   ```python
   libs_names_set = {(l[0], l[1]) for l in libs_names}
   if (lib.module_name, lib.func_name) not in libs_names_set:
       continue
   ```

---

### BUG-19: Only First Sequence Type-Checked

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/libraries.py:147` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
`isinstance(sequences[0], (tuple, list))` only checks the first sequence.

**Resolution Steps:**
1. Check all sequences or each individually.

---

### BUG-20: Empty `sequences` Causes ValueError in min/max

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/libraries.py:157` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
`min(map(len, sequences))` and `max(...)` raise `ValueError` on empty input.

**Resolution Steps:**
1. Add guard:
   ```python
   if not sequences:
       return False
   ```

---

### BUG-21: Benchmark File Handle Resource Leak

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/benchmark.py:117` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
`LIBRARIES_PATH.open('w', ...)` is not properly closed if `json.dump` raises.

**Resolution Steps:**
1. Use context manager:
   ```python
   with LIBRARIES_PATH.open('w', encoding='utf-8') as f:
       json.dump(data, f)
   ```

---

### BUG-22: setup.py Resource Leak

| Field | Value |
|-------|-------|
| **File** | `python/setup.py:93` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
`open('README.md', ...).read()` opens without closing.

**Resolution Steps:**
1. Use `with` statement:
   ```python
   with open('README.md', encoding='utf-8') as f:
       long_description = f.read()
   ```

---

### BUG-23: Hamming Assertion Fails on Float Result

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/edit_based.py:59` |
| **Severity** | MEDIUM |
| **Rust** | N/A (static typing) |

**Description:**
`assert isinstance(result, int)` fails when external libs return float.

**Resolution Steps:**
1. Remove assertion or coerce:
   ```python
   return int(result)
   ```

---

### BUG-24: Levenshtein Same Assertion Bug

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/edit_based.py:136` |
| **Severity** | MEDIUM |
| **Rust** | N/A (static typing) |

**Description:**
Same as BUG-23 — `assert isinstance(result, int)` fails on float.

**Resolution Steps:**
1. Same fix as BUG-23.

---

### BUG-25: StrCmp95 `_in_range` Accepts Control Characters

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/edit_based.py:665` |
| **Severity** | MEDIUM |
| **Rust** | **PRESENT** — `strcmp95.rs:71-73` has the identical bug |

**Description:**
`0 < ord(char) < 91` accepts ASCII control characters (1-31) and symbols (32-47, 58-64).

```python
# edit_based.py:665 — BUGGY
return 0 < ord(char) < 91
```

**Resolution Steps (Python):**
1. Change to:
   ```python
   return ('A' <= char <= 'Z') or ('0' <= char <= '9')
   ```

**Resolution Steps (Rust):**
1. In `strcmp95.rs:71-73`, change:
   ```rust
   fn in_range(ch: char) -> bool {
       let o = ch as u32;
       o > 0 && o < 91
   }
   ```
   to:
   ```rust
   fn in_range(ch: char) -> bool {
       ('A'..='Z').contains(&ch) || ('0'..='9').contains(&ch)
   }
   ```

---

### BUG-26: Postfix `__call__` No Empty-Sequence Guard

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/simple.py:51` |
| **Severity** | MEDIUM |
| **Rust** | N/A |

**Description:**
`s = sequences[0]` raises `IndexError` when `sequences` is empty.

**Resolution Steps:**
1. Add guard:
   ```python
   if not sequences:
       return ''
   ```

---

### BUG-27: RatcliffObershelp `ecount` Mismatch

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/sequence_based.py:179` |
| **Severity** | MEDIUM |
| **Rust** | **PARTIALLY present** — `ratcliff_obershelp.rs:43-50` has `unwrap_or(0)` fallback risk |

**Description:**
`ecount` is computed from raw sequences, but `_find` operates on processed sequences after `_get_sequences`. When `qval != 1`, lengths differ.

**Resolution Steps:**
1. Compute `ecount` after `_get_sequences`:
   ```python
   sequences = self._get_sequences(*sequences)
   ecount = sum(map(len, sequences))
   return scount * self._find(*sequences) / ecount
   ```

---

### BUG-28: `find()` May Return -1

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/sequence_based.py:170` |
| **Severity** | MEDIUM |
| **Rust** | **PARTIALLY present** — uses `.unwrap_or(0)` which silently defaults to 0 |

**Description:**
If `LCSStr` returns a substring not found in some sequence, `find` returns -1, causing incorrect slicing.

**Resolution Steps:**
1. Add assertion or handle explicitly:
   ```python
   pos = s.find(subseq)
   assert pos != -1, f"substring '{subseq}' not found in '{s}'"
   before = s[:pos]
   after = s[pos + length:]
   ```

---

## Low Bugs

### BUG-29: Silent `Inf` Return on Empty Inputs

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/utils.py:12` |
| **Severity** | LOW |
| **Rust** | N/A |

**Resolution Steps:**
1. Document behavior or raise `ValueError`.

---

### BUG-30: Incorrect Default Sort Order

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/benchmark.py:124` |
| **Severity** | LOW |
| **Rust** | N/A |

**Resolution Steps:**
1. Use explicit key:
   ```python
   installed.sort(key=lambda x: (x.algorithm, x.time))
   ```

---

### BUG-31: `python_requires='>=3.5'` Too Low

| Field | Value |
|-------|-------|
| **File** | `python/setup.py:115` |
| **Severity** | LOW |
| **Rust** | N/A |

**Resolution Steps:**
1. Change to `python_requires='>=3.10'` (for `X | Y` type union syntax).

---

### BUG-32: Dead `TypeError` Catch (Python 2 Leftover)

| Field | Value |
|-------|-------|
| **File** | `python/setup.py:93` |
| **Severity** | LOW |
| **Rust** | N/A |

**Resolution Steps:**
1. Remove the `TypeError` catch; simplify to a single `with` block.

---

### BUG-33: Wildcard Imports Pollute Namespace

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/__init__.py:19` |
| **Severity** | LOW |
| **Rust** | N/A |

**Resolution Steps:**
1. Replace `from .algorithms import *` with explicit named imports.

---

### BUG-34: `Matrix.__init__` Ignores `external` Param

| Field | Value |
|-------|-------|
| **File** | `python/textdistance/algorithms/simple.py:86` |
| **Severity** | LOW |
| **Rust** | N/A |

**Resolution Steps:**
1. Add `self.external = external` to `__init__`.

---

### BUG-35: `exit(1)` on Non-Error Path

| Field | Value |
|-------|-------|
| **File** | `python/licenses_example/compare.py:18` |
| **Severity** | LOW |
| **Rust** | N/A |

**Resolution Steps:**
1. Change to `exit(0)`.

---

### BUG-36: Relative Path in `compare.py`

| Field | Value |
|-------|-------|
| **File** | `python/licenses_example/compare.py:12` |
| **Severity** | LOW |
| **Rust** | N/A |

**Resolution Steps:**
1. Compute path relative to script:
   ```python
   Path(__file__).parent / 'choosealicense.com' / '_licenses'
   ```

---

## Rust Cross-Reference

| Bug | Algorithm | Present in Rust? | Rust File | Status |
|-----|-----------|-----------------|-----------|--------|
| BUG-02 | DamerauLevenshtein off-by-one | **No** | `damerau_levenshtein.rs:75` | Already correct |
| BUG-03 | Tversky bias formula | **No** | `tversky.rs:39` | Already correct |
| BUG-04 | MongeElkan double normalization | **Yes** | `monge_elkan.rs:43` | **NEEDS FIX** |
| BUG-10 | BaseSimilarity quick_answer | N/A | — | No equivalent |
| BUG-13 | EntropyNCD div-by-zero | **No** | `entropy_ncd.rs:48` | Guarded |
| BUG-14 | MLIPNS loop logic | **Yes** | `mlipns.rs:50` | **NEEDS FIX** |
| BUG-25 | StrCmp95 `_in_range` | **Yes** | `strcmp95.rs:71` | **NEEDS FIX** |
| BUG-27 | RatcliffObershelp ecount | Partially | `ratcliff_obershelp.rs:43` | `unwrap_or(0)` risk |

**Rust bugs confirmed present: 4** (MongeElkan, MLIPNS, StrCmp95, RatcliffObershelp partial)

---

## Resolution Checklist

| # | Bug | Language | Priority | Status |
|---|-----|----------|----------|--------|
| BUG-01 | Arbitrary code execution | Python | CRITICAL | [ ] |
| BUG-02 | DamerauLevenshtein off-by-one | Python | CRITICAL | [ ] |
| BUG-03 | Tversky bias formula | Python | CRITICAL | [ ] |
| BUG-04 | MongeElkan double normalization | Python + Rust | CRITICAL | [x] FIXED |
| BUG-05 | Missing error handling (libraries) | Python | HIGH | [ ] |
| BUG-06 | Non-callable presets | Python | HIGH | [ ] |
| BUG-07 | Missing attribute access | Python | HIGH | [ ] |
| BUG-08 | libs_names index ValueError | Python | HIGH | [ ] |
| BUG-09 | limits KeyError (benchmark) | Python | HIGH | [ ] |
| BUG-10 | BaseSimilarity empty sequences | Python | HIGH | [ ] |
| BUG-11 | Prefix missing super() | Python | HIGH | [ ] |
| BUG-12 | MongeElkan maximum tuple | Python | HIGH | [ ] |
| BUG-13 | EntropyNCD div-by-zero | Python | HIGH | [ ] |
| BUG-14 | MLIPNS loop logic | Python + Rust | HIGH | [x] FIXED (Rust) |
| BUG-15 | .split() on non-string | Python | MEDIUM | [ ] |
| BUG-16 | Empty text ValueError | Python | MEDIUM | [ ] |
| BUG-17 | Missing .equality check | Python | MEDIUM | [ ] |
| BUG-18 | Fragile list comparison | Python | MEDIUM | [ ] |
| BUG-19 | First sequence only type check | Python | MEDIUM | [ ] |
| BUG-20 | Empty sequences min/max | Python | MEDIUM | [ ] |
| BUG-21 | Benchmark resource leak | Python | MEDIUM | [ ] |
| BUG-22 | setup.py resource leak | Python | MEDIUM | [ ] |
| BUG-23 | Hamming assertion on float | Python | MEDIUM | [ ] |
| BUG-24 | Levenshtein assertion on float | Python | MEDIUM | [ ] |
| BUG-25 | StrCmp95 _in_range control chars | Python + Rust | MEDIUM | [x] FIXED (Rust) |
| BUG-26 | Postfix empty guard | Python | MEDIUM | [ ] |
| BUG-27 | RatcliffObershelp ecount | Python + Rust | MEDIUM | [ ] |
| BUG-28 | find() returns -1 | Python + Rust | MEDIUM | [ ] |
| BUG-29 | Silent Inf return | Python | LOW | [ ] |
| BUG-30 | Sort order | Python | LOW | [ ] |
| BUG-31 | python_requires too low | Python | LOW | [ ] |
| BUG-32 | Dead TypeError catch | Python | LOW | [ ] |
| BUG-33 | Wildcard imports | Python | LOW | [ ] |
| BUG-34 | Matrix ignores external | Python | LOW | [ ] |
| BUG-35 | exit(1) non-error | Python | LOW | [ ] |
| BUG-36 | Relative path | Python | LOW | [ ] |

---


