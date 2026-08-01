#!/bin/bash
# Full benchmark runner
# Produces: bench/results.json, bench/criterion/ (HTML reports)

set -e

echo "============================================"
echo "  Port Mortem — Benchmark Suite"
echo "============================================"
echo ""

# Step 1: Build release
echo "[1/4] Building release binary..."
cargo build --release 2>&1 | tail -3
echo "  [OK] Release binary ready"

# Step 2: Run criterion benchmarks
echo ""
echo "[2/4] Running criterion benchmarks..."
cargo bench 2>&1 | tee bench/criterion_output.txt
echo "  [OK] Criterion benchmarks complete"

# Step 3: Measure startup time
echo ""
echo "[3/4] Measuring startup time..."

# Rust startup
echo "  Measuring Rust startup..."
RUST_START_TOTAL=0
for i in $(seq 1 100); do
    START=$(python3 -c "import time; print(time.time_ns())")
    ./target/release/textdistance hamming "test" "text" > /dev/null 2>&1
    END=$(python3 -c "import time; print(time.time_ns())")
    ELAPSED=$((END - START))
    RUST_START_TOTAL=$((RUST_START_TOTAL + ELAPSED))
done
RUST_START_US=$((RUST_START_TOTAL / 100 / 1000))
echo "  Rust median startup: ${RUST_START_US}μs"

# Python startup
echo "  Measuring Python startup..."
PYTHON_START_TOTAL=0
for i in $(seq 1 100); do
    START=$(python3 -c "import time; print(time.time_ns())")
    python3 -c "import textdistance; textdistance.hamming('test', 'text')" > /dev/null 2>&1
    END=$(python3 -c "import time; print(time.time_ns())")
    ELAPSED=$((END - START))
    PYTHON_START_TOTAL=$((PYTHON_START_TOTAL + ELAPSED))
done
PYTHON_START_US=$((PYTHON_START_TOTAL / 100 / 1000))
echo "  Python median startup: ${PYTHON_START_US}μs"

# Step 4: Measure RSS
echo ""
echo "[4/4] Measuring memory usage (RSS)..."

# Rust RSS
RUST_RSS=$(/usr/bin/time -l ./target/release/textdistance levenshtein "algorithm" "altruistic" 2>&1 | grep "peak" | awk '{print $1}')
echo "  Rust peak RSS: ${RUST_RSS} bytes"

# Python RSS
PYTHON_RSS=$(/usr/bin/time -l python3 -c "
import textdistance
textdistance.levenshtein('algorithm', 'altruistic')
" 2>&1 | grep "peak" | awk '{print $1}')
echo "  Python peak RSS: ${PYTHON_RSS} bytes"

# Generate results JSON
echo ""
echo "Generating results.json..."
cat > bench/results.json << EOF
{
    "environment": {
        "os": "$(uname -s) $(uname -r)",
        "arch": "$(uname -m)",
        "date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    },
    "startup": {
        "rust_median_us": $RUST_START_US,
        "python_median_us": $PYTHON_START_US,
        "speedup": $(echo "scale=2; $PYTHON_START_US / $RUST_START_US" | bc)
    },
    "memory": {
        "rust_peak_bytes": ${RUST_RSS:-0},
        "python_peak_bytes": ${PYTHON_RSS:-0}
    },
    "criterion_report": "bench/criterion/"
}
EOF

echo "  [OK] bench/results.json written"
echo ""
echo "============================================"
echo "  BENCHMARK COMPLETE"
echo "============================================"
echo "  Criterion HTML: bench/criterion/"
echo "  Results JSON:   bench/results.json"
echo "  Raw output:     bench/criterion_output.txt"
echo "============================================"
