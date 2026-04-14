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

---

## Lab 2A – Hello World in Rust → WASM

**Goal:** Compile your first Rust program to WASM and run it with Wasmtime.

**What you'll learn:** Rust has first-class WASM support — the `wasm32-wasip1` target tells the compiler to produce a WASM binary instead of a native x86/ARM executable. The resulting `.wasm` file is entirely self-contained and will run identically on macOS, Linux, Windows, or any other platform with Wasmtime installed. You don't change a single line of your Rust source code.

> 💡 **What does `wasm32-wasip1` mean?**  
> Breaking it down: `wasm32` = 32-bit WebAssembly architecture, `wasi` = WebAssembly System Interface (the OS abstraction layer), `p1` = Preview 1 (the stable, widely-supported WASI version). There is also `wasm32-wasip2` (using WASI 0.2 and the Component Model) — p1 is what most tooling supports today.

> 💡 **Why `--release`?**  
> Debug builds include extensive debug info and skip optimisations. A debug WASM binary can be 5–10× larger and slower than a release build. Always use `--release` for any benchmarking or deployment.

### Step 1 – Navigate to the project

The project is already set up for you at `labs/lab-2a-hello/`.

```bash
cd module-02-first-wasm-app/labs/lab-2a-hello
```

### Step 2 – Read the program

Open `src/main.rs` and read through it. It's a standard Rust `main()` that prints to stdout — no WASM-specific code at all. That's the point: you write normal Rust, the compiler handles the rest.

### Step 3 – Build for WASM

```bash
cargo build --target wasm32-wasip1 --release
```

### Step 4 – Run with Wasmtime

```bash
wasmtime target/wasm32-wasip1/release/lab-2a-hello.wasm
```

### Step 5 – Inspect the output binary

> 💡 **Why is the binary so large compared to the WAT examples in Module 1?**  
> The Rust standard library (std) is compiled into the binary — it includes memory allocators, panic handlers, formatting code, and WASI glue. For a "hello world" this feels wasteful, but for real programs it means you get the full power of Rust's ecosystem. Tools like `wasm-opt` (from Binaryen) can strip unused code significantly.

```bash
# File size
ls -lh target/wasm32-wasip1/release/lab-2a-hello.wasm

# Section overview — shows every section, its byte range, size, and item count
wasm-tools objdump target/wasm32-wasip1/release/lab-2a-hello.wasm

# What functions are exported?
wasm-tools print target/wasm32-wasip1/release/lab-2a-hello.wasm | grep export
```

Expected output (section overview):
```
  types     |   0xa -   0x87 |  125 bytes | 17 count
  imports   |  0x8a -  0x16a |  224 bytes | 6 count
  functions | 0x16d -  0x248 |  219 bytes | 217 count
  ...
  code      | 0x2ea - 0xcb26 | 51260 bytes | 217 count
  data      | 0xcb29 - 0xf30f | 10214 bytes | 2 count
```

> 💡 **217 functions for a hello world?** That's the Rust standard library — allocator, panic handler, formatting machinery, WASI glue. The `code` section alone is ~50 KB. Compare this to the 9-byte code section in the `add.wasm` WAT example. This is the cost of bringing std — worthwhile for real programs, but visible here.

---

## Lab 2B – File I/O with WASI Capabilities

**Goal:** Read from a file using WASI, and learn how capability grants work.

**What you'll learn:** WASI uses a *capability-based security model*. The module declares what it needs (e.g. `wasi:filesystem`), but the host decides at runtime what it actually gets. If you don't grant a capability, the module cannot use it — not by policy, but because the import simply isn't wired up. This is fundamentally different from Linux process permissions, where a process inherits everything the user has access to by default.

This lab makes that concrete: the same binary either works or fails depending entirely on what you pass to `wasmtime`.

### The code (`labs/lab-2b-fileio/src/main.rs` — already provided)

It reads a text file from disk and prints its content.

### Build

```bash
cd labs/lab-2b-fileio
cargo build --target wasm32-wasip1 --release
```

### Run WITHOUT filesystem access (should fail)

> 💡 **Why does this fail?** The module imports `wasi:filesystem/types` functions, but Wasmtime doesn't link them to any real directory. The first filesystem call returns an error code, and the program exits with an error. The sandbox is working exactly as intended.

```bash
wasmtime target/wasm32-wasip1/release/lab-2b-fileio.wasm
# Expect: error – no access to host filesystem
```

### Run WITH filesystem access (grant the capability)

> 💡 **What does `--dir /tmp` actually do?**  
> It tells Wasmtime to map the host directory `/tmp` into the module's virtual filesystem namespace. From inside the WASM module, the path still looks like `/tmp` — but it's sandboxed: the module can't use `../` tricks to escape to the real root filesystem because the runtime intercepts every path operation and validates it against the granted list.

```bash
# Create test data
echo 'Secret data from the host!' > /tmp/wasm-test.txt

# Grant access to /tmp only
wasmtime --dir /tmp target/wasm32-wasip1/release/lab-2b-fileio.wasm /tmp/wasm-test.txt
```

**Key insight:** WASM can only access `/tmp` because we explicitly granted it. It cannot reach `/etc/passwd` or any other path.

### Try to access a path NOT granted

> 💡 **Note the error type.** It's not a crash or undefined behaviour — it's a clean `EACCES` (permission denied) returned from the WASI syscall. The module handles the error gracefully. This predictability is a core WASM safety property.

```bash
wasmtime --dir /tmp target/wasm32-wasip1/release/lab-2b-fileio.wasm /etc/passwd
# Expect: permission denied – sandbox working correctly!
```

---

## Lab 2C – Embedding WASM in a Rust Host

**Goal:** Call a WASM function from a Rust host program using the Wasmtime API.

**What you'll learn:** The Wasmtime CLI is convenient, but in production WASM is usually embedded inside a larger application — this is the *embedding* pattern. Your application becomes the host: it loads the `.wasm` file, controls what capabilities the module gets, calls exported functions, and processes the results. This is exactly how Spin, WasmEdge, and Envoy work internally.

The key Wasmtime concepts you'll see in the host code:
- **`Engine`** — the JIT/AOT compiler, shared and reused across many modules
- **`Module`** — a compiled WASM module, reusable across many instances
- **`Store`** — the runtime state for one instance (memory, globals, tables); isolated per request in production
- **`Instance`** — a running copy of a module, bound to one Store

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
🔧 Initializing Wasmtime engine...
📦 Loading WASM module: wasm-guest/target/wasm32-wasip1/release/wasm_guest.wasm
🏗  Instantiating module...

✅ Calling WASM function: multiply(10, 32)
   Result: 320
✅ Calling WASM function: multiply(100, 200)
   Result: 20000

✅ Calling WASM function: multiply(6, 7)
   Result: 42

✅ Calling WASM function: fibonacci(10)
   Result: 55

🎉 All WASM calls completed successfully!
```

### Step 3 – Explore the host code

Open `host/src/main.rs` and read through it. The guest exports three functions — `add`, `multiply`, and `fibonacci` — all defined in `wasm-guest/src/lib.rs`.

> 💡 **Why does `get_typed_func::<(i32, i32), i32>` fail at runtime rather than compile time if the types are wrong?**  
> The Rust generic parameters encode what *you expect* the WASM export's signature to be. Wasmtime checks this against the actual module signature at instantiation time. If they don't match, you get a clear runtime error — not a silent type confusion or crash. This is much safer than raw FFI.

**Try it:** In `host/src/main.rs`, change `"multiply"` to `"add"` (which also exists in the guest) and re-run — it should work identically since both have the same `(i32, i32) -> i32` signature. Then try requesting a function that doesn't exist (e.g. `"subtract"`) and observe the error message you get from Wasmtime.

---

## Lab 2D – Environment Variables & Arguments (Bonus)

**Goal:** Pass environment variables and CLI args to WASM.

**What you'll learn:** Just like filesystem access, environment variables are a capability that must be explicitly granted. A WASM module cannot read the host's `PATH`, `HOME`, `AWS_SECRET_ACCESS_KEY`, or any other variable unless the host deliberately passes it. This is important for secrets hygiene — a compromised dependency inside your WASM module cannot silently exfiltrate credentials from the environment.

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

> **Try this:** Run the module *without* `--env` and observe that it sees an empty environment. Then add only one of the two variables and see that the other is absent. This demonstrates selective environment exposure.

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
