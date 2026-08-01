#!/usr/bin/env python3
"""
Differential Fuzzer for Port Mortem
Runs original Python textdistance and Rust port on same inputs,
diffs outputs. Zero divergence = bonus points.

Usage:
    python3 fuzz_harness.py [--duration 60] [--algorithms all]
"""

import argparse
import random
import string
import subprocess
import sys
import time
import os
from pathlib import Path

# Map CLI subcommand -> Python class name
ALGORITHMS = {
    "hamming": "Hamming",
    "levenshtein": "Levenshtein",
    "damerau-levenshtein": "DamerauLevenshtein",
    "jaro": "Jaro",
    "jaro-winkler": "JaroWinkler",
    "str-cmp95": "StrCmp95",
    "mlipns": "MLIPNS",
    "needleman-wunsch": "NeedlemanWunsch",
    "smith-waterman": "SmithWaterman",
    "gotoh": "Gotoh",
    "bag": "Bag",
    "jaccard": "Jaccard",
    "sorensen": "SorensenDice",
    "tversky": "Tversky",
    "overlap": "Overlap",
    "cosine": "Cosine",
    "tanimoto": "Tanimoto",
    "monge-elkan": "MongeElkan",
    "lcsseq": "LCSSeq",
    "lcsstr": "LCSStr",
    "ratcliff-obershelp": "RatcliffObershelp",
    "prefix": "Prefix",
    "postfix": "Postfix",
    "length": "Length",
    "identity": "Identity",
    "matrix": "Matrix",
    "mra": "MRA",
    "editex": "Editex",
}

# Input generators
def gen_random_string(min_len=0, max_len=50):
    length = random.randint(min_len, max_len)
    return ''.join(random.choices(string.ascii_letters + string.digits, k=length))

def gen_unicode_string(min_len=0, max_len=30):
    chars = string.ascii_letters + string.digits + "äöüñçßαβγδ"
    length = random.randint(min_len, max_len)
    return ''.join(random.choices(chars, k=length))

def gen_similar_strings():
    base = gen_random_string(3, 20)
    if len(base) == 0:
        return base, base
    pos = random.randint(0, len(base) - 1)
    mutations = [
        base[:pos] + base[pos+1:],
        base[:pos] + random.choice(string.ascii_letters) + base[pos+1:],
        base[:pos] + random.choice(string.ascii_letters) + base[pos:],
    ]
    return base, random.choice(mutations)

def gen_empty_strings():
    return "", ""

def gen_identical_strings():
    s = gen_random_string(1, 30)
    return s, s

def gen_single_char():
    c = random.choice(string.ascii_letters)
    return c, c

def gen_long_strings():
    s1 = gen_random_string(100, 500)
    s2 = gen_random_string(100, 500)
    return s1, s2

INPUT_GENERATORS = [
    lambda: (gen_random_string(), gen_random_string()),
    lambda: (gen_unicode_string(), gen_unicode_string()),
    gen_similar_strings,
    gen_empty_strings,
    gen_identical_strings,
    gen_single_char,
    gen_long_strings,
]

# Algorithms that use .similarity() in the CLI
SIMILARITY_ALGS = {
    "jaro", "jaro-winkler", "str-cmp95", "mlipns",
    "needleman-wunsch", "smith-waterman", "gotoh",
    "jaccard", "sorensen", "tversky", "overlap", "cosine", "tanimoto",
    "monge-elkan", "lcsseq", "lcsstr", "ratcliff-obershelp",
    "prefix", "postfix", "identity", "matrix",
}

# Algorithms that return strings in Python __call__ but numbers via distance/similarity
STRING_RETURN_ALGS = {"lcsseq", "lcsstr", "prefix", "postfix"}


PYTHON_BIN = os.environ.get("PYTHON_BIN", "python3")


def call_python(algorithm, py_class, s1, s2, use_similarity):
    """Call Python textdistance"""
    try:
        if use_similarity:
            method = "similarity"
        else:
            method = "distance"
        code = f"""
import textdistance, sys
s1, s2 = sys.argv[1], sys.argv[2]
print(textdistance.{py_class}(external=False).{method}(s1, s2))
"""
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
    try:
        result = subprocess.run(
            [str(Path(__file__).parent.parent / "target" / "release" / "textdistance"), algorithm, s1, s2],
            capture_output=True, text=True, timeout=10
        )
        if result.returncode != 0:
            return None, result.stderr.strip()
        return result.stdout.strip(), None
    except FileNotFoundError:
        return None, "binary not found"
    except subprocess.TimeoutExpired:
        return None, "timeout"


def values_match(py_out, rs_out, tolerance=1e-6):
    """Compare Python and Rust outputs, allowing numeric tolerance"""
    if py_out == rs_out:
        return True
    try:
        pv = float(py_out)
        rv = float(rs_out)
        if pv == rv:
            return True
        if abs(pv) < tolerance and abs(rv) < tolerance:
            return True
        if abs(pv - rv) / max(abs(pv), abs(rv), 1e-10) < tolerance:
            return True
        return False
    except (ValueError, TypeError):
        return False


def run_fuzz(duration=60, algorithms=None):
    """Run differential fuzzing for specified duration"""
    if algorithms is None:
        algorithms = list(ALGORITHMS.keys())

    log = []
    divergences = 0
    total = 0
    start = time.time()
    errors = 0
    per_alg_stats = {alg: {"total": 0, "divs": 0} for alg in algorithms}

    print(f"[*] Starting differential fuzzer for {duration}s")
    print(f"[*] Algorithms: {', '.join(algorithms)}")
    print(f"[*] Press Ctrl+C to stop early")
    print()

    try:
        while time.time() - start < duration:
            for alg in algorithms:
                py_class = ALGORITHMS[alg]
                use_similarity = alg in SIMILARITY_ALGS

                generator = random.choice(INPUT_GENERATORS)
                s1, s2 = generator()

                py_out, py_err = call_python(alg, py_class, s1, s2, use_similarity)
                rs_out, rs_err = call_rust(alg, s1, s2)

                total += 1
                per_alg_stats[alg]["total"] += 1

                if py_err and rs_err:
                    pass
                elif py_err:
                    errors += 1
                elif rs_err:
                    errors += 1
                elif not values_match(py_out, rs_out):
                    divergences += 1
                    per_alg_stats[alg]["divs"] += 1
                    entry = {
                        "algorithm": alg,
                        "input": [s1, s2],
                        "python": py_out,
                        "rust": rs_out,
                    }
                    log.append(entry)
                    print(f"[DIVERGENCE] {alg}({repr(s1)}, {repr(s2)})")
                    print(f"  Python: {py_out}")
                    print(f"  Rust:   {rs_out}")

                if total % 500 == 0:
                    elapsed = time.time() - start
                    print(f"  [{elapsed:.1f}s] {total} tests, {divergences} divergences, {errors} errors")

    except KeyboardInterrupt:
        print("\n[*] Interrupted by user")

    elapsed = time.time() - start

    log_path = Path("fuzz/log.txt")
    log_path.parent.mkdir(parents=True, exist_ok=True)
    with open(log_path, "w") as f:
        f.write(f"Differential Fuzz Log\n")
        f.write(f"Duration: {elapsed:.1f}s\n")
        f.write(f"Total tests: {total}\n")
        f.write(f"Divergences: {divergences}\n")
        f.write(f"Errors: {errors}\n\n")
        for alg in algorithms:
            s = per_alg_stats[alg]
            status = "OK" if s["divs"] == 0 else f"FAIL ({s['divs']} divs)"
            f.write(f"  {alg}: {s['total']} tests - {status}\n")
        f.write(f"\n")
        for entry in log:
            f.write(f"DIVERGENCE: {entry['algorithm']}\n")
            f.write(f"  Input: {repr(entry['input'])}\n")
            f.write(f"  Python: {entry['python']}\n")
            f.write(f"  Rust:   {entry['rust']}\n\n")

    print(f"\n{'='*60}")
    print(f"FUZZ RESULTS")
    print(f"{'='*60}")
    print(f"Duration:     {elapsed:.1f}s")
    print(f"Total tests:  {total}")
    print(f"Divergences:  {divergences}")
    print(f"Errors:       {errors}")
    print(f"Log file:     fuzz/log.txt")

    print(f"\nPer-algorithm breakdown:")
    for alg in algorithms:
        s = per_alg_stats[alg]
        status = "OK" if s["divs"] == 0 else f"FAIL ({s['divs']} divs)"
        print(f"  {alg:25s} {s['total']:5d} tests - {status}")

    if divergences == 0 and total > 0:
        print(f"\n[+] ZERO DIVERGENCES - Bonus points earned!")
    else:
        print(f"\n[-] {divergences} divergences found")

    return divergences


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Differential fuzzer")
    parser.add_argument("--duration", type=int, default=60, help="Fuzz duration in seconds")
    parser.add_argument("--algorithms", nargs="+", default=None, help="Algorithms to fuzz")
    args = parser.parse_args()
    run_fuzz(args.duration, args.algorithms)
