# tests/port/ — Rust Port Tests

This directory contains tests specific to the Rust port of textdistance.

## Test Organization

- **Unit tests**: Embedded in `src/**/*.rs` files via `#[cfg(test)]` modules (66 tests)
- **Adapter tests**: `adapter/test_adapter.py` runs original Python tests against the Rust CLI (84 tests)
- **Differential fuzzing**: `fuzz/fuzz_harness.py` compares Python vs Rust outputs (500+ tests per 60s run)

## Running

```bash
# Rust unit tests (66 tests)
cargo test

# Adapter tests (84 tests, requires Python + textdistance)
python3 adapter/test_adapter.py --verbose

# Differential fuzzing (60s minimum for bonus points)
python3 fuzz/fuzz_harness.py --duration 60
```
