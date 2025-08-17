#!/bin/bash
#
# This script runs a comprehensive set of tests for all supported targets
# and relevant feature combinations. It is designed to ensure that both native
# and Wasm builds are functioning correctly.
#
# The script will exit immediately if any command fails.
set -e

# --- Native Target Tests ---
# For the native target (the default), we can test with --all-features.
# This command enables all optional features defined in Cargo.toml,
# including 'scalar' and 'diesel', providing the most comprehensive test coverage.
echo "--- Running Native Tests (all features) ---"
cargo test --all-features
echo "--- Native Tests Passed ---"
echo

# --- Wasm Target Compilation Checks ---
# For the wasm32-unknown-unknown target, we can't run tests directly in a standard
# environment without a specific Wasm test runner (like wasm-pack).
# Instead, we run `cargo check`, which compiles the code and runs the type
# checker. This is a crucial and effective way to verify that the library
# is compatible with the Wasm target.

# We must use --no-default-features because the 'diesel' feature, which is part
# of the default feature set, is native-only and will not compile for Wasm.

echo "--- Running Wasm Compilation Check (base library, no default features) ---"
cargo check --target wasm32-unknown-unknown --no-default-features
echo "--- Wasm Base Check Passed ---"
echo

# Here, we test the 'scalar' feature specifically, as it is designed to be
# Wasm-compatible.
echo "--- Running Wasm Compilation Check (with 'scalar' feature) ---"
cargo check --target wasm32-unknown-unknown --no-default-features --features scalar
echo "--- Wasm 'scalar' Check Passed ---"
echo

echo "=========================================="
echo " All checks and tests passed successfully! "
echo "=========================================="
