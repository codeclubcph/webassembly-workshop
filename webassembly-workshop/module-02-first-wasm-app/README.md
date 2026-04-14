# Module 2 – Building & Running Your First WASM App with Wasmtime
**⏱ Duration:** 45 minutes | **🧪 Lab time:** 30 minutes

---

## Learning Objectives

By the end of this module you will be able to:
- Compile a Rust program to WASM targeting WASI
- Run, configure, and debug WASM modules with Wasmtime CLI
- Embed and call WASM functions from a Rust host program
- Inspect and understand the compiled WASM output

---

## Background Reading

See [slides/02-first-wasm-app.md](../slides/02-first-wasm-app.md)

---

## Lab 2A – Hello World in Rust → WASM

**Goal:** Compile your first Rust program to WASM and run it with Wasmtime.

### Step 1 – Create project

```bash
cd module-02-first-wasm-app/labs
cargo new lab-2a-hello
cd lab-2a-hello
```

### Step 2 – Write the program

Edit `src/main.rs` (already created for you in `labs/lab-2a-hello/src/main.rs`).

### Step 3 – Build for WASM

```bash
cargo build --target wasm32-wasip1 --release
```

### Step 4 – Run with Wasmtime

```bash
wasmtime target/wasm32-wasip1/release/lab-2a-hello.wasm
```

### Step 5 – Inspect the output binary

```bash
# File size
ls -lh target/wasm32-wasip1/release/lab-2a-hello.wasm

# Section overview
wasm-tools objdump -x target/wasm32-wasip1/release/lab-2a-hello.wasm | head -40

# How many functions are exported?
wasm-tools objdump -x target/wasm32-wasip1/release/lab-2a-hello.wasm | grep Export
```

---

## Lab 2B – File I/O with WASI Capabilities

**Goal:** Read from a file using WASI, and learn how capability grants work.

### The code (`labs/lab-2b-fileio/src/main.rs` — already provided)

It reads a text file from disk and prints its content.

### Build

```bash
cd labs/lab-2b-fileio
cargo build --target wasm32-wasip1 --release
```

### Run WITHOUT filesystem access (should fail)

```bash
wasmtime target/wasm32-wasip1/release/lab-2b-fileio.wasm
# Expect: error – no access to host filesystem
```

### Run WITH filesystem access (grant the capability)

```bash
# Create test data
echo "Secret data from the host!" > /tmp/wasm-test.txt

# Grant access to /tmp only
wasmtime --dir /tmp target/wasm32-wasip1/release/lab-2b-fileio.wasm /tmp/wasm-test.txt
```

**Key insight:** WASM can only access `/tmp` because we explicitly granted it. It cannot reach `/etc/passwd` or any other path.

### Try to access a path NOT granted

```bash
wasmtime --dir /tmp target/wasm32-wasip1/release/lab-2b-fileio.wasm /etc/passwd
# Expect: permission denied – sandbox working correctly!
```

---

## Lab 2C – Embedding WASM in a Rust Host

**Goal:** Call a WASM function from a Rust host program using the Wasmtime API.

### Overview

```
┌─────────────────────┐        ┌────────────────────────┐
│  Rust Host Program  │ calls  │  add.wasm              │
│  (lab-2c-embed)     │───────►│  export: add(i32,i32)  │
└─────────────────────┘        └────────────────────────┘
```

### Step 1 – Build the WASM guest

```bash
cd labs/lab-2c-embed
cd wasm-guest
cargo build --target wasm32-wasip1 --release
```

### Step 2 – Build and run the Rust host

```bash
cd ../host
cargo run
```

Expected output:
```
Loaded WASM module: wasm-guest/target/wasm32-wasip1/release/wasm_guest.wasm
Calling WASM function: add(10, 32)
Result: 42
Calling WASM function: add(100, 200)
Result: 300
```

### Step 3 – Explore

Modify `host/src/main.rs` to:
1. Call `multiply` instead of `add`
2. Pass different arguments
3. Add error handling for out-of-range inputs

---

## Lab 2D – Environment Variables & Arguments (Bonus)

**Goal:** Pass environment variables and CLI args to WASM.

```bash
cd labs/lab-2d-envargs
cargo build --target wasm32-wasip1 --release

# Pass env vars and args
wasmtime \
  --env APP_NAME=my-service \
  --env LOG_LEVEL=debug \
  target/wasm32-wasip1/release/lab-2d-envargs.wasm \
  -- --port 8080 --verbose
```

---

## ✅ Module 2 Checklist

- [ ] Compiled Rust to `.wasm` and ran with `wasmtime`
- [ ] Demonstrated WASI capability grants (filesystem access)
- [ ] Showed sandbox rejection (accessing ungrantd path)
- [ ] Called WASM from a Rust embedding host
- [ ] (Bonus) Passed env vars and args to a WASM module

---

## Key Takeaways

1. Rust + `wasm32-wasip1` = first-class WASM support with zero runtime changes
2. WASI capabilities are **explicitly granted by the host** — deny by default
3. The Wasmtime embedding API lets any application become a WASM host
4. WASM binaries are cross-platform: compile once, run anywhere Wasmtime runs
