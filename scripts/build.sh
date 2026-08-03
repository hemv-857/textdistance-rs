#!/bin/bash
# Build script — one command to a runnable artifact
# This is what judges run

set -e

echo "============================================"
echo "  textdistance-rs — Build"
echo "============================================"
echo ""

echo "[1/3] Building release binary..."
cargo build --release
echo "  [OK] Binary: target/release/textdistance"

echo ""
echo "[2/3] Running tests..."
cargo test
echo "  [OK] All tests passed"

echo ""
echo "[3/3] Verifying binary works..."
./target/release/textdistance hamming "test" "text"
echo "  [OK] Binary functional"

echo ""
echo "============================================"
echo "  BUILD COMPLETE"
echo "============================================"
echo "  Binary:     target/release/textdistance"
echo "  Test suite: cargo test"
echo "  Benchmarks: cargo bench"
echo "============================================"
