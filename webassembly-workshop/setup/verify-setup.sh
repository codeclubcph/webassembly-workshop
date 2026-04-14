#!/usr/bin/env bash
# verify-setup.sh – Checks all required tools for the WASM workshop

PASS=0
FAIL=0

check() {
  local tool=$1
  local cmd=$2
  if eval "$cmd" &>/dev/null; then
    echo "  ✅  $tool"
    ((PASS++))
  else
    echo "  ❌  $tool — NOT FOUND (see setup/README.md)"
    ((FAIL++))
  fi
}

echo ""
echo "========================================="
echo "  WASM Workshop – Environment Check"
echo "========================================="
echo ""

check "Wasmtime"      "wasmtime --version"
check "Rust (rustc)"  "rustc --version"
check "Cargo"         "cargo --version"
check "wasm32 target" "rustup target list --installed | grep wasm32-wasip1"
check "wasm-tools"    "wasm-tools --version"
check "Node.js"       "node --version"
check "npm"           "npm --version"
check "Docker"        "docker --version"

echo ""
echo "========================================="
echo "  Results: $PASS passed, $FAIL failed"
echo "========================================="

if [ "$FAIL" -eq 0 ]; then
  echo "  🎉 All good! You are ready for the workshop."
else
  echo "  ⚠️  Please fix the issues above before the workshop."
fi
echo ""
