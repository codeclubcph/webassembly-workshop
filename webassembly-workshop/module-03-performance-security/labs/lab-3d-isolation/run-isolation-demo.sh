#!/usr/bin/env bash
# run-isolation-demo.sh
# Demonstrates that two WASM instances cannot access each other's memory

set -e

echo "========================================"
echo "  WASM Memory Isolation Demo"
echo "========================================"
echo ""
echo "This demo shows that two WASM module instances running in the"
echo "same process have completely isolated linear memory spaces."
echo ""
echo "The host Rust program:"
echo "  1. Loads the same .wasm module twice → two separate instances"
echo "  2. Writes a secret value into Instance A's memory (address 0)"
echo "  3. Reads address 0 from Instance B's memory"
echo "  4. Shows that Instance B sees 0 (uninitialized), NOT the secret"
echo ""

BINARY="./target/release/lab-3d-isolation"

if [ ! -f "$BINARY" ]; then
  echo "Building host program..."
  cargo build --release 2>&1 | tail -5
fi

WASM="./wasm-guest/target/wasm32-wasip1/release/wasm_guest_isolation.wasm"

if [ ! -f "$WASM" ]; then
  echo "Building WASM guest..."
  cd wasm-guest && cargo build --target wasm32-wasip1 --release 2>&1 | tail -5
  cd ..
fi

echo "Running isolation demo..."
echo ""
$BINARY
