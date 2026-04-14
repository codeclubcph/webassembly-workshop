# Module 3 – Performance & Security vs. Containers
**⏱ Duration:** 45 minutes | **🧪 Lab time:** 20 minutes

---

## Learning Objectives

By the end of this module you will be able to:
- Measure and compare WASM vs container startup times
- Explain the WASM structural security model vs Linux container model
- Run a benchmark comparing WASM and native execution
- Identify attack surfaces and how WASM mitigates them

---

## Background Reading

See [slides/03-performance-security.md](../slides/03-performance-security.md)

---

## Lab 3A – Startup Time Benchmark

**Goal:** Measure the cold-start time difference between WASM and Docker containers.

### Prerequisites

- Docker installed and running
- Wasmtime installed
- `time` command available (built into zsh/bash)

### Step 1 – Measure WASM startup time

```bash
cd module-03-performance-security/labs/lab-3a-startup

# Build the WASM module
cargo build --target wasm32-wasip1 --release 2>/dev/null

# Run 10 times and measure
echo "=== WASM Startup Times ==="
for i in {1..10}; do
  { time wasmtime target/wasm32-wasip1/release/lab-3a-startup.wasm; } 2>&1 | grep real
done
```

### Step 2 – Build a comparable Docker container

```bash
# Build the Docker image
docker build -t wasm-vs-container-demo .

# Warm up (pull layers)
docker run --rm wasm-vs-container-demo

# Measure cold starts (force new container each time)
echo "=== Docker Startup Times ==="
for i in {1..10}; do
  { time docker run --rm wasm-vs-container-demo; } 2>&1 | grep real
done
```

### Step 3 – AOT Compilation (even faster)

```bash
# Pre-compile WASM to native code (AOT)
wasmtime compile target/wasm32-wasip1/release/lab-3a-startup.wasm \
  -o lab-3a-startup.cwasm

echo "=== WASM AOT Startup Times ==="
for i in {1..10}; do
  { time wasmtime run --allow-precompiled lab-3a-startup.cwasm; } 2>&1 | grep real
done
```

### Expected results (approximate)

| Method | Typical startup |
|--------|----------------|
| Docker container | 300ms – 800ms |
| WASM (JIT) | 2ms – 10ms |
| WASM (AOT) | < 1ms |

---

## Lab 3B – CPU Performance Benchmark

**Goal:** Compare WASM vs native execution on a CPU-bound task (Fibonacci).

### The benchmark (`labs/lab-3b-benchmark/`)

Computes `fibonacci(40)` natively (as a Rust binary) and as a WASM module.

### Step 1 – Build native binary

```bash
cd labs/lab-3b-benchmark
cargo build --release
```

### Step 2 – Build WASM binary

```bash
cargo build --target wasm32-wasip1 --release
```

### Step 3 – Run and compare

```bash
echo "=== Native (x86_64) ==="
time ./target/release/lab-3b-benchmark

echo ""
echo "=== WASM (Wasmtime JIT) ==="
time wasmtime target/wasm32-wasip1/release/lab-3b-benchmark.wasm

echo ""
echo "=== WASM (AOT compiled) ==="
wasmtime compile target/wasm32-wasip1/release/lab-3b-benchmark.wasm -o bench.cwasm
time wasmtime run --allow-precompiled bench.cwasm
```

> 💡 WASM typically runs within 10–20% of native speed for CPU-bound tasks.

---

## Lab 3C – Security: Sandbox Escape Attempt

**Goal:** Demonstrate that WASM cannot access resources beyond its grants.

### Scenario: Malicious module

The `labs/lab-3c-security/src/main.rs` file simulates a "malicious" WASM module that tries to:
1. Read `/etc/passwd`
2. Read environment variables not explicitly shared
3. Write to `/tmp` without permission

```bash
cd labs/lab-3c-security
cargo build --target wasm32-wasip1 --release

# Run WITHOUT any capability grants
echo "=== Test 1: No capabilities ==="
wasmtime target/wasm32-wasip1/release/lab-3c-security.wasm
# All attempts should fail gracefully

# Run WITH only /tmp access
echo "=== Test 2: Only /tmp granted ==="
wasmtime --dir /tmp target/wasm32-wasip1/release/lab-3c-security.wasm
# Only /tmp write should succeed

# Run with specific env var only
echo "=== Test 3: Specific env var ==="
wasmtime --env ALLOWED_VAR=hello target/wasm32-wasip1/release/lab-3c-security.wasm
# Only ALLOWED_VAR should be visible
```

### Contrast: Container escape surface

Discuss what a malicious container can access with default Docker settings:
- All environment variables of the container
- Full view of container filesystem
- Potential host escape via kernel exploits

---

## Lab 3D – Memory Isolation Demo

**Goal:** Show that two WASM modules cannot access each other's memory.

```bash
cd labs/lab-3d-isolation
# See the demo script and explanation in this directory
./run-isolation-demo.sh
```

The demo runs two WASM instances in the same process via the embedding API and shows:
- Each has its own linear memory space
- Writing to one does NOT affect the other
- The host controls all inter-module communication

---

## ✅ Module 3 Checklist

- [ ] Measured WASM cold-start vs Docker cold-start
- [ ] Ran AOT compilation and measured the speedup
- [ ] Benchmarked WASM vs native for Fibonacci(40)
- [ ] Demonstrated sandbox rejection of unauthorized file access
- [ ] Understood the memory isolation model

---

## Key Takeaways

1. WASM cold starts are **10–100× faster** than container cold starts
2. WASM CPU performance is **within 10–20% of native** for compute-bound tasks
3. WASM security is **structural**: denial by default, explicit grants only
4. Memory isolation is **guaranteed by the VM**: modules cannot share memory without the host's explicit intermediation
5. For I/O-bound workloads, WASM and containers are comparable — the bottleneck is I/O, not compute
