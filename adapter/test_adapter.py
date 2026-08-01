#!/usr/bin/env python3
"""
Test Parity Adapter
Runs original Python tests against both Python and Rust implementations,
compares outputs. Used for differential verification.

Usage:
    python3 test_adapter.py --verbose
    python3 test_adapter.py --algorithm levenshtein
"""

import argparse
import subprocess
import sys
import json
import os
from pathlib import Path

# All test cases extracted from the original test suite
TEST_CASES = {
    "hamming": [
        ("test", "text", 1),
        ("test", "tset", 2),
        ("test", "qwe", 4),
        ("test", "testit", 2),
        ("test", "tesst", 2),
        ("test", "tet", 2),
    ],
    "levenshtein": [
        ("test", "text", 1),
        ("test", "tset", 2),
        ("test", "qwe", 4),
        ("test", "testit", 2),
        ("test", "tesst", 1),
        ("test", "tet", 1),
    ],
    "damerau_levenshtein": [
        ("test", "text", 1),
        ("test", "tset", 1),
        ("test", "qwy", 4),
        ("test", "testit", 2),
        ("test", "tesst", 1),
        ("test", "tet", 1),
        ("cat", "hat", 1),
        ("Niall", "Neil", 3),
        ("aluminum", "Catalan", 7),
        ("ATCG", "TAGC", 2),
        ("ab", "ba", 1),
        ("ab", "cde", 3),
        ("ab", "ac", 1),
        ("ab", "bc", 2),
        ("ab", "bca", 3),
        ("abcd", "bdac", 4),
    ],
    "jaro": [
        ("hello", "haloa", 0.7333333333333334),
        ("fly", "ant", 0.0),
        ("frog", "fog", 0.9166666666666666),
        ("ATCG", "TAGC", 0.8333333333333334),
        ("MARTHA", "MARHTA", 0.944444444),
        ("DWAYNE", "DUANE", 0.822222222),
        ("DIXON", "DICKSONX", 0.7666666666666666),
    ],
    "jaro_winkler": [
        ("elephant", "hippo", 0.44166666666666665),
        ("fly", "ant", 0.0),
        ("frog", "fog", 0.925),
        ("MARTHA", "MARHTA", 0.9611111111111111),
        ("DWAYNE", "DUANE", 0.84),
        ("DIXON", "DICKSONX", 0.8133333333333332),
        ("duck donald", "duck daisy", 0.867272727272),
    ],
    "jaccard": [
        ("test", "text", 0.6),
        ("nelson", "neilsen", 0.625),
        ("decide", "resize", 0.3333333333333333),
    ],
    "sorensen": [
        ("test", "text", 0.75),
    ],
    "cosine": [
        ("test", "text", 0.75),
        ("nelson", "neilsen", 0.7715167498104595),
    ],
    "overlap": [
        ("test", "text", 0.75),
        ("testme", "textthis", 0.6666666666666666),
        ("nelson", "neilsen", 0.8333333333333334),
    ],
    "lcsseq": [
        ("ab", "cd", 0),
        ("abcd", "abcd", 4),
        ("test", "text", 3),
        ("DIXON", "DICKSONX", 4),
    ],
    "lcsstr": [
        ("ab", "abcd", 2),
        ("abcd", "ab", 2),
        ("abcd", "bc", 2),
        ("abcd", "ef", 0),
    ],
    "bag": [
        ("qwe", "qwe", 0),
        ("qwe", "erty", 3),
        ("qwe", "ewq", 0),
        ("qwe", "rtys", 4),
    ],
    "strcmp95": [
        ("MARTHA", "MARHTA", 0.9611111111111111),
        ("DWAYNE", "DUANE", 0.873),
        ("DIXON", "DICKSONX", 0.839333333),
        ("TEST", "TEXT", 0.9066666666666666),
    ],
    "mlipns": [
        ("", "", 1),
        ("a", "", 0),
        ("", "a", 0),
        ("a", "a", 1),
        ("ab", "a", 1),
        ("abc", "abc", 1),
        ("abc", "abcde", 1),
        ("Tomato", "Tamato", 1),
    ],
    "monge_elkan": [
        ("elephant", "hippo", 0.03125),
    ],
    "gotoh": [
        ("ABCD", "ABCD", 4.0),
        ("ABCDEFG", "XBCDFG", 3.0),
    ],
    "needleman_wunsch": [
        ("ABCD", "ABCD", 4.0),
        ("AAAA", "BBBB", 0.0),
    ],
    "smith_waterman": [
        ("ABCD", "ABCD", 4.0),
        ("ABCDEFG", "XBCDFG", 5.0),
    ],
    "mra": [
        ("nelson", "neilsen", 0),
        ("niall", "neal", 0),
        ("cat", "hat", 1),
    ],
    "editex": [
        ("nelson", "neilsen", 2),
        ("niall", "neal", 1),
        ("cat", "hat", 2),
        ("ab", "c", 4),
        ("ALIE", "ALI", 1),
    ],
    # Vector-based algorithms (operate on float vectors, not strings)
    # Note: Vector algorithms are Rust-only, no Python parity test
    # They are tested via Rust unit tests
}

# Algorithms that need special handling (similarity vs distance)
SIMILARITY_ALGORITHMS = {
    "jaro", "jaro_winkler", "strcmp95",
    "jaccard", "sorensen", "cosine", "overlap",
    "monge_elkan",
    "lcsseq", "lcsstr",
    "gotoh", "needleman_wunsch", "smith_waterman",
}

# Algorithms that should use .distance() instead of __call__()
DISTANCE_ALGORITHMS = {
    "mra",
}

# Algorithms with float results (need tolerance)
FLOAT_ALGORITHMS = {
    "jaro", "jaro_winkler", "strcmp95",
    "jaccard", "sorensen", "cosine", "overlap",
    "monge_elkan",
}


PYTHON_BIN = os.environ.get("PYTHON_BIN", "python3")
RUST_BIN = os.environ.get("RUST_BIN", str(Path(__file__).parent.parent / "target" / "release" / "textdistance"))

# Map Python algorithm names to Rust CLI subcommands
CLI_MAP = {
    "damerau_levenshtein": "damerau-levenshtein",
    "dl_unrestricted": "dl-unrestricted",
    "strcmp95": "str-cmp95",
    "mlipns": "mlipns",
    "needleman_wunsch": "needleman-wunsch",
    "smith_waterman": "smith-waterman",
    "gotoh": "gotoh",
    "monge_elkan": "monge-elkan",
    "jaro_winkler": "jaro-winkler",
    "lcsseq": "lcsseq",
    "lcsstr": "lcsstr",
    "ratcliff_obershelp": "ratcliff-obershelp",
    "sorensen": "sorensen",
    "arith_ncd": "arith-ncd",
    "rle_ncd": "rle-ncd",
    "bwtrle_ncd": "bwtrle-ncd",
    "sqrt_ncd": "sqrt-ncd",
    "entropy_ncd": "entropy-ncd",
    "bz2_ncd": "bz2-ncd",
    "lzma_ncd": "lzma-ncd",
    "zlib_ncd": "zlib-ncd",
}


def call_python(algorithm, s1, s2, is_similarity=False, is_distance=False):
    """Call Python textdistance"""
    if is_similarity:
        code = f"""
import textdistance, sys
s1, s2 = sys.argv[1], sys.argv[2]
alg = textdistance.{algorithm}
if hasattr(alg, 'similarity'):
    print(alg.similarity(s1, s2))
else:
    print(alg(s1, s2))
"""
    elif is_distance:
        code = f"""
import textdistance, sys
s1, s2 = sys.argv[1], sys.argv[2]
alg = textdistance.{algorithm}
if hasattr(alg, 'distance'):
    print(alg.distance(s1, s2))
else:
    print(alg(s1, s2))
"""
    else:
        code = f"""
import textdistance, sys
s1, s2 = sys.argv[1], sys.argv[2]
alg = textdistance.{algorithm}
print(alg(s1, s2))
"""
    
    try:
        result = subprocess.run(
            [PYTHON_BIN, "-c", code, s1, s2],
            capture_output=True, text=True, timeout=10
        )
        if result.returncode != 0:
            return None, result.stderr.strip()
        return result.stdout.strip(), None
    except subprocess.TimeoutExpired:
        return None, "timeout"


def call_rust(algorithm, s1, s2):
    """Call Rust port via CLI"""
    cli_cmd = CLI_MAP.get(algorithm, algorithm)
    try:
        result = subprocess.run(
            [RUST_BIN, cli_cmd, s1, s2],
            capture_output=True, text=True, timeout=10
        )
        if result.returncode != 0:
            return None, result.stderr.strip()
        return result.stdout.strip(), None
    except FileNotFoundError:
        return None, "binary not found"
    except subprocess.TimeoutExpired:
        return None, "timeout"


def values_match(py_val, rs_val, is_float=False, tolerance=1e-6):
    """Check if two values match"""
    if py_val is None and rs_val is None:
        return True
    if py_val is None or rs_val is None:
        return False
    
    if is_float:
        try:
            py_f = float(py_val)
            rs_f = float(rs_val)
            return abs(py_f - rs_f) < tolerance
        except ValueError:
            return py_val == rs_val
    
    return py_val == rs_val


def run_tests(algorithms=None, verbose=False):
    """Run all test cases and compare outputs"""
    if algorithms is None:
        algorithms = list(TEST_CASES.keys())
    
    total = 0
    passed = 0
    failed = 0
    errors = []
    
    for alg in algorithms:
        if alg not in TEST_CASES:
            if verbose:
                print(f"[SKIP] {alg}: no test cases defined")
            continue
        
        test_cases = TEST_CASES[alg]
        is_similarity = alg in SIMILARITY_ALGORITHMS
        is_distance = alg in DISTANCE_ALGORITHMS
        is_float = alg in FLOAT_ALGORITHMS
        
        for s1, s2, expected in test_cases:
            total += 1
            
            # Call Python
            py_out, py_err = call_python(alg, s1, s2, is_similarity, is_distance)
            
            # Call Rust
            rs_out, rs_err = call_rust(alg, s1, s2)
            
            # Compare
            if py_err:
                print(f"ERROR: {alg}({repr(s1)}, {repr(s2)}) Python error: {py_err}")
                errors.append({"algorithm": alg, "input": [s1, s2], "error": py_err})
                continue
            
            if rs_err:
                print(f"FAIL:  {alg}({repr(s1)}, {repr(s2)}) Rust error: {rs_err}")
                print(f"  Python: {py_out}")
                failed += 1
                errors.append({"algorithm": alg, "input": [s1, s2], "rust_error": rs_err})
                continue
            
            if values_match(py_out, rs_out, is_float):
                passed += 1
                if verbose:
                    print(f"PASS:  {alg}({repr(s1)}, {repr(s2)}) = {rs_out}")
            else:
                failed += 1
                print(f"FAIL:  {alg}({repr(s1)}, {repr(s2)})")
                print(f"  Python: {py_out}")
                print(f"  Rust:   {rs_out}")
                errors.append({
                    "algorithm": alg,
                    "input": [s1, s2],
                    "python": py_out,
                    "rust": rs_out,
                })
    
    return total, passed, failed, errors


def main():
    parser = argparse.ArgumentParser(description="Test parity adapter")
    parser.add_argument("--verbose", "-v", action="store_true")
    parser.add_argument("--algorithm", "-a", nargs="+", help="Specific algorithms to test")
    parser.add_argument("--output", "-o", default="test_results.json")
    args = parser.parse_args()
    
    print("=" * 60)
    print("  Port Mortem — Test Parity Adapter")
    print("=" * 60)
    print()
    
    total, passed, failed, errors = run_tests(args.algorithm, args.verbose)
    
    print()
    print("=" * 60)
    print(f"  RESULTS: {passed}/{total} passed, {failed} failed")
    print("=" * 60)
    
    # Write results
    results = {
        "total": total,
        "passed": passed,
        "failed": failed,
        "pass_rate": f"{(passed/total*100):.1f}%" if total > 0 else "N/A",
        "errors": errors,
    }
    
    with open(args.output, "w") as f:
        json.dump(results, f, indent=2)
    
    print(f"  Results saved to: {args.output}")
    
    if failed == 0:
        print("\n[+] ALL TESTS PASSED — Test parity achieved!")
    else:
        print(f"\n[-] {failed} tests failed — investigate above")
    
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
