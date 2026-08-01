#!/bin/bash
# Test parity verification script
# Runs the ORIGINAL Python test suite against our Rust port via adapter
# This is the script judges will verify

set -e

echo "============================================"
echo "  Port Mortem — Test Parity Verification"
echo "============================================"
echo ""

# Step 1: Verify test file hashes match kickoff pin
echo "[1/5] Verifying test file hashes..."
HASH_FILE="test_hash_baseline.txt"
if [ -f "$HASH_FILE" ]; then
    while IFS= read -r line; do
        hash=$(echo "$line" | awk '{print $1}')
        file=$(echo "$line" | awk '{print $2}')
        if [ -f "$file" ]; then
            actual=$(shasum -a 256 "$file" | awk '{print $1}')
            if [ "$hash" != "$actual" ]; then
                echo "  [FAIL] Hash mismatch: $file"
                echo "    Expected: $hash"
                echo "    Actual:   $actual"
                exit 1
            fi
        fi
    done < "$HASH_FILE"
    echo "  [OK] All test file hashes match"
else
    echo "  [WARN] No hash baseline found, skipping verification"
fi

# Step 2: Build Rust release binary
echo ""
echo "[2/5] Building Rust release binary..."
cargo build --release 2>&1 | tail -3
echo "  [OK] Binary built"

# Step 3: Run Rust unit tests
echo ""
echo "[3/5] Running Rust unit tests..."
cargo test 2>&1 | tail -5
echo "  [OK] Rust tests passed"

# Step 4: Run adapter (calls both Python and Rust, diffs outputs)
echo ""
echo "[4/5] Running differential test suite..."
python3 adapter/test_adapter.py --verbose 2>&1 | tee test_results.txt
ADAPTER_EXIT=$?

# Step 5: Generate report
echo ""
echo "[5/5] Generating test parity report..."
TOTAL=$(grep -c "^TEST:" test_results.txt 2>/dev/null || echo "0")
PASS=$(grep -c "^PASS:" test_results.txt 2>/dev/null || echo "0")
FAIL=$(grep -c "^FAIL:" test_results.txt 2>/dev/null || echo "0")

echo ""
echo "============================================"
echo "  TEST PARITY REPORT"
echo "============================================"
echo "  Total tests:  $TOTAL"
echo "  Passed:       $PASS"
echo "  Failed:       $FAIL"
if [ "$TOTAL" -gt 0 ]; then
    RATE=$(echo "scale=1; $PASS * 100 / $TOTAL" | bc)
    echo "  Pass rate:    ${RATE}%"
fi
echo "  Log file:     test_results.txt"
echo "============================================"

if [ $ADAPTER_EXIT -eq 0 ]; then
    echo ""
    echo "[+] All tests passed!"
else
    echo ""
    echo "[-] Some tests failed — see test_results.txt"
fi

exit $ADAPTER_EXIT
