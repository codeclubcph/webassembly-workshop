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

## Lab 3A – Startup Time Benchmark

**Goal:** Measure the cold-start time difference between WASM and Docker containers.

**What you'll learn:** Cold-start latency is one of the most impactful practical differences between WASM and containers. A Docker container must go through kernel namespace creation, cgroup setup, overlay filesystem mounting, and process spawning before your first line of code runs. WASM skips all of that — the Wasmtime runtime validates and JIT-compiles the module, allocates linear memory, and starts executing in milliseconds. With AOT pre-compilation it's even faster: the binary is already native code, so startup is just a memory map and a function call.

You will see these numbers yourself in this lab — they are not theoretical.

### Prerequisites

- Docker installed and running
- Wasmtime installed
- `time` command available (built into zsh/bash)

### Step 1 – Measure WASM startup time

> 💡 **What is being timed here?**  
> Each `wasmtime` invocation includes: loading the `.wasm` file from disk, validating it, JIT-compiling it to native code, instantiating (allocating memory and linking imports), and executing the `_start` function. The total wall-clock time at the end of each line captures all of this end-to-end.

> ⚠️ **zsh vs bash:** zsh's `time` prints a single summary line per command (e.g. `0.01s user 0.00s system 99% cpu 0.007 total`) — the last number is the wall-clock total. The `grep real` pattern used in bash won't match; just read the `total` value directly.

```bash
cd module-03-performance-security/labs/lab-3a-startup
cargo build --target wasm32-wasip1 --release 2>/dev/null

echo "=== WASM Startup Times ==="
for i in {1..10}; do
  { time wasmtime target/wasm32-wasip1/release/lab-3a-startup.wasm > /dev/null; } 2>&1
done
```

Expected: each line ends with something like `0.007 total` — roughly **5–15ms** per run.

### Step 2 – Build a comparable Docker container

> 💡 **Why Alpine?**  
> The Dockerfile uses `alpine:3.19` — the smallest viable Linux container base image (~7 MB). This gives Docker the best possible chance. Even so, the kernel overhead of setting up namespaces and cgroups dominates the startup time regardless of image size.

```bash
# Build the Docker image
docker build -t wasm-vs-container-demo .

# Warm up (pull layers, prime caches)
docker run --rm wasm-vs-container-demo

# Measure cold starts (force new container each time)
echo "=== Docker Startup Times ==="
for i in {1..10}; do
  { time docker run --rm wasm-vs-container-demo > /dev/null; } 2>&1
done
```

Expected: each line ends with something like `0.200 total` — roughly **200–500ms** per run.

### Step 3 – AOT Compilation (even faster)

> 💡 **What is AOT compilation?**  
> `wasmtime compile` pre-compiles the `.wasm` binary to a `.cwasm` file containing native machine code for your CPU. When you run it with `--allow-precompiled`, Wasmtime skips the JIT step entirely — it's already native code. This is how Fastly Compute@Edge achieves sub-millisecond WASM startup in production.

```bash
# Pre-compile WASM to native code (AOT)
wasmtime compile target/wasm32-wasip1/release/lab-3a-startup.wasm \
  -o lab-3a-startup.cwasm

echo "=== WASM AOT Startup Times ==="
for i in {1..10}; do
  { time wasmtime run --allow-precompiled lab-3a-startup.cwasm > /dev/null; } 2>&1
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

**What you'll learn:** WASM is not a bytecode interpreter — Wasmtime compiles it to native machine code before running it. The main source of overhead compared to a raw native binary is *bounds-checked memory accesses*: every `load` and `store` instruction checks that the address is within the module's linear memory. Modern CPUs and runtimes use guard pages to elide most of these checks, so the overhead is typically 10–20% for compute-heavy code. For I/O-bound workloads (most web services), the overhead is negligible because the bottleneck is the network, not the CPU.

**Why Fibonacci(40)?** It's entirely CPU-bound (no I/O, no memory pressure), recursive, and takes long enough (~1 second) that timing noise is small relative to the measurement.

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

> 💡 **Reading the results:** The `real` time includes process startup. To isolate pure compute, look at `user` time (CPU time actually spent), which is more comparable between native and WASM.

```bash
echo "=== Native (x86_64) ==="
{ time ./target/release/lab-3b-benchmark > /dev/null; } 2>&1

echo ""
echo "=== WASM (Wasmtime JIT) ==="
{ time wasmtime target/wasm32-wasip1/release/lab-3b-benchmark.wasm > /dev/null; } 2>&1

echo ""
echo "=== WASM (AOT compiled) ==="
wasmtime compile target/wasm32-wasip1/release/lab-3b-benchmark.wasm -o bench.cwasm
{ time wasmtime run --allow-precompiled bench.cwasm > /dev/null; } 2>&1
```

> 💡 **Reading the results (zsh):** Each `time` line shows `Xs user Ys system Z% cpu W total`. The `total` is wall-clock time; `user` is CPU time spent in your process. For comparing compute performance, `user` is the most meaningful — it excludes process startup and OS scheduling noise.

---

## Lab 3C – Security: Sandbox Escape Attempt

**Goal:** Demonstrate that WASM cannot access resources beyond its grants.

**What you'll learn:** WASM security is *structural*, not policy-based. A Linux container's security relies on seccomp filters and AppArmor profiles that must be correctly configured. If a syscall is missing from the filter, a malicious process can use it. WASM has no syscall instruction at all — the instruction set simply doesn't include one. Every interaction with the outside world must go through a typed WASI import that the host explicitly wires up. If the host doesn't wire it, the import doesn't exist, and the module cannot call it.

In this lab the `lab-3c-security` binary plays the role of a "malicious" module — it tries several escape techniques, and you observe that all of them are blocked by the sandbox, not by any configuration on your part.

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
# Only /tmp write should succeed — /etc/passwd is still unreachable

# Run with specific env var only
echo "=== Test 3: Specific env var ==="
wasmtime --env ALLOWED_VAR=hello target/wasm32-wasip1/release/lab-3c-security.wasm
# Only ALLOWED_VAR should be visible — all other env vars are absent
```

> 💡 **Key observation:** The failures are clean errors returned from WASI functions — not crashes, not undefined behaviour, not segfaults. The module handles each failure gracefully and reports what it could and couldn't access. This predictability is a core property of the sandbox: even adversarial code cannot cause unexpected host-side effects.

### Contrast: Container escape surface

Discuss what a malicious container can access with default Docker settings:
- All environment variables of the container (including any secrets you accidentally injected)
- Full view of container filesystem
- Any syscall not blocked by the seccomp profile (default Docker allows ~300+ syscalls)
- Potential host escape via kernel exploits (CVE-2019-5736 is a real example)

---

## Lab 3D – Memory Isolation Demo

**Goal:** Show that two WASM modules cannot access each other's memory.

**What you'll learn:** Two WASM modules running in the same OS *process* still cannot read each other's linear memory. In contrast, two native threads in the same process share the entire address space — one buggy or compromised thread can overwrite another's data. WASM's linear memory is fully isolated at the VM level: each module instance gets its own bounded region, and the engine enforces that every memory access stays within that region.

This matters for multi-tenant systems (e.g., serverless platforms running untrusted user code) and plugin architectures (e.g., a plugin that shouldn't be able to exfiltrate data from another plugin). The isolation is free — you don't write any sandboxing code. It's a property of the WASM execution model itself.

```bash
cd labs/lab-3d-isolation
# Build the demo first
cargo build --release
cd wasm-guest && cargo build --target wasm32-wasip1 --release && cd ..
# Run the isolation demo
./run-isolation-demo.sh
```

**What to look for in the output:** The script prints memory contents from both modules before and after module A writes to its memory. You should see that module B's memory is completely unchanged — even though both modules are running inside the same OS process. Look for lines like:

```
[Module A] Writing 0xDEADBEEF to its own memory...
[Module A] Memory[0] = 0xDEADBEEF
[Module B] Memory[0] = 0x00000000  ← completely unaffected
```

> 💡 **How it works:** The demo uses the Wasmtime embedding API to spin up two `Store` instances within the same process. Each `Store` owns its own linear memory allocation — a separate `Vec<u8>` on the host heap. When module A writes to its memory, module B's `Store` sees none of it. The host (the Rust embedding code) would have to explicitly copy bytes between stores to enable communication, and that copying is the only legitimate cross-module channel. There is no shared memory segment, no shared pointer, and no way for either module to discover the other's address.

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
