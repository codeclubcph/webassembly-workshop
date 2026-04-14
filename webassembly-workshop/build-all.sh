#!/usr/bin/env bash
# build-all.sh – Builds every lab in the workshop
# Run from the root of the webassembly-workshop directory

set -e

ROOT="$(cd "$(dirname "$0")" && pwd)"
WASM_TARGET="wasm32-wasip1"

OK=0
FAIL=0

build_wasm() {
  local label=$1
  local dir=$2
  echo -n "  Building $label ... "
  if (cd "$dir" && cargo build --target $WASM_TARGET --release -q 2>/dev/null); then
    echo "✅"
    ((OK++))
  else
    echo "❌"
    ((FAIL++))
  fi
}

build_native() {
  local label=$1
  local dir=$2
  echo -n "  Building $label (native) ... "
  if (cd "$dir" && cargo build --release -q 2>/dev/null); then
    echo "✅"
    ((OK++))
  else
    echo "❌"
    ((FAIL++))
  fi
}

compile_wat() {
  local label=$1
  local wat=$2
  local wasm="${wat%.wat}.wasm"
  echo -n "  Compiling $label ... "
  if wasm-tools parse "$wat" -o "$wasm" 2>/dev/null; then
    echo "✅"
    ((OK++))
  else
    echo "❌  (is wasm-tools installed?)"
    ((FAIL++))
  fi
}

echo ""
echo "╔══════════════════════════════════════════╗"
echo "║   WASM Workshop – Build All Labs         ║"
echo "╚══════════════════════════════════════════╝"
echo ""

# ── Module 1 ────────────────────────────────────────────────────────────────
echo "▶ Module 1: Runtime Essentials"
compile_wat "lab-1a hello.wat"   "$ROOT/module-01-runtime-essentials/labs/lab-1a/hello.wat"
compile_wat "lab-1b memory.wat"  "$ROOT/module-01-runtime-essentials/labs/lab-1b/memory.wat"
compile_wat "lab-1c calc.wat"    "$ROOT/module-01-runtime-essentials/labs/lab-1c/calc.wat"
compile_wat "lab-1c factorial (solution)" "$ROOT/module-01-runtime-essentials/labs/lab-1c/solution/factorial.wat"
echo ""

# ── Module 2 ────────────────────────────────────────────────────────────────
echo "▶ Module 2: First WASM App"
build_wasm  "lab-2a hello"       "$ROOT/module-02-first-wasm-app/labs/lab-2a-hello"
build_wasm  "lab-2b fileio"      "$ROOT/module-02-first-wasm-app/labs/lab-2b-fileio"
build_wasm  "lab-2c wasm-guest"  "$ROOT/module-02-first-wasm-app/labs/lab-2c-embed/wasm-guest"
build_native "lab-2c host"       "$ROOT/module-02-first-wasm-app/labs/lab-2c-embed/host"
build_wasm  "lab-2d envargs"     "$ROOT/module-02-first-wasm-app/labs/lab-2d-envargs"
echo ""

# ── Module 3 ────────────────────────────────────────────────────────────────
echo "▶ Module 3: Performance & Security"
build_wasm   "lab-3a startup"    "$ROOT/module-03-performance-security/labs/lab-3a-startup"
build_wasm   "lab-3b benchmark (wasm)"   "$ROOT/module-03-performance-security/labs/lab-3b-benchmark"
build_native "lab-3b benchmark (native)" "$ROOT/module-03-performance-security/labs/lab-3b-benchmark"
build_wasm   "lab-3c security"   "$ROOT/module-03-performance-security/labs/lab-3c-security"
build_wasm   "lab-3d wasm-guest" "$ROOT/module-03-performance-security/labs/lab-3d-isolation/wasm-guest"
build_native "lab-3d host"       "$ROOT/module-03-performance-security/labs/lab-3d-isolation"
echo ""

# ── Module 4 ────────────────────────────────────────────────────────────────
echo "▶ Module 4: Cloud-Native"
build_wasm   "lab-4a spin-service"        "$ROOT/module-04-cloud-native/labs/lab-4a-spin-service"
build_wasm   "lab-4b plugin-uppercase"    "$ROOT/module-04-cloud-native/labs/lab-4b-plugin-system/plugins/uppercase"
build_wasm   "lab-4b plugin-reverse"      "$ROOT/module-04-cloud-native/labs/lab-4b-plugin-system/plugins/reverse"
build_wasm   "lab-4b plugin-wordcount"    "$ROOT/module-04-cloud-native/labs/lab-4b-plugin-system/plugins/wordcount"
build_native "lab-4b plugin-host"         "$ROOT/module-04-cloud-native/labs/lab-4b-plugin-system/host"
build_wasm   "lab-4c event-handler"       "$ROOT/module-04-cloud-native/labs/lab-4c-faas-sim/event-handler"
build_native "lab-4c faas-sim"            "$ROOT/module-04-cloud-native/labs/lab-4c-faas-sim"
echo ""

# ── Summary ─────────────────────────────────────────────────────────────────
echo "═══════════════════════════════════════════"
echo "  Results: $OK built successfully, $FAIL failed"
echo "═══════════════════════════════════════════"
if [ "$FAIL" -eq 0 ]; then
  echo "  🎉 All labs built! You are ready for the workshop."
else
  echo "  ⚠️  Some labs failed. Check setup/README.md for dependencies."
fi
echo ""
