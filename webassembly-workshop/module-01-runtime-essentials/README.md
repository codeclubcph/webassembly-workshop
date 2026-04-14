# Module 1 – WASM Runtime Essentials & Execution Model
**⏱ Duration:** 45 minutes | **🧪 Lab time:** 20 minutes

---

## Learning Objectives

By the end of this module you will be able to:
- Explain the WASM stack-based execution model
- Read and write basic WebAssembly Text Format (WAT)
- Use `wasm-tools` to inspect, validate, and convert WASM binaries
- Understand linear memory and the role of WASI

---

## Lab 1A – Hello, WAT!

**Goal:** Write a WASM module in WAT, compile it to binary, and run it.

**What you'll learn:** WAT (WebAssembly Text Format) is the human-readable form of a `.wasm` binary — a 1:1 text representation of every byte. You'll rarely write WAT by hand in production, but reading it is essential for understanding what your compiler actually generates and for debugging. In this lab you write the simplest possible real program: printing a string to stdout entirely through WASI, without any standard library.

### Step 1 – Read the module

Open `labs/lab-1a/hello.wat` and read through it before running anything — the inline comments explain every line. The full content is reproduced below for reference:

> 💡 **Why do we import `fd_write`?**  
> A bare WASM module has zero OS access — it cannot write to a terminal, open a file, or even get the time. Everything must be imported from the host. `fd_write` is the WASI function that writes bytes to a file descriptor. File descriptor 1 is stdout — the same convention as POSIX. The host (Wasmtime) provides the real implementation; the module just declares it needs it.

> 💡 **Why is the string stored at byte offset 8?**  
> The iovec struct (which describes the buffer to write) lives at bytes 0–7 of linear memory. Placing the string at offset 8 keeps it immediately after the struct, avoiding overlap. This is the kind of low-level memory layout decision that Rust or C would handle automatically — doing it manually in WAT shows you exactly what's happening under the hood.

```bash
cd module-01-runtime-essentials/labs/lab-1a
```

File contents (`hello.wat`):

```wat
(module
  ;; Import the "fd_write" function from WASI to write to stdout
  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write
      (param i32 i32 i32 i32)
      (result i32)))

  ;; Declare 1 page (64KB) of linear memory and export it
  (memory (export "memory") 1)

  ;; Store the string "Hello, WASM!\n" at byte offset 8
  (data (i32.const 8) "Hello, WASM!\n")

  ;; Entry point function
  (func $main (export "_start")
    ;; Build an iovec struct at offset 0:
    ;;   iov_base = 8  (pointer to string)
    ;;   iov_len  = 13 (length of string)
    (i32.store (i32.const 0) (i32.const 8))
    (i32.store (i32.const 4) (i32.const 13))

    ;; Call fd_write(fd=1, iovs=0, iovs_len=1, nwritten=28)
    (call $fd_write
      (i32.const 1)   ;; stdout file descriptor
      (i32.const 0)   ;; pointer to iovec array
      (i32.const 1)   ;; number of iovecs
      (i32.const 28)) ;; where to store bytes written
    drop              ;; discard return value
  )
)
```

### Step 2 – Compile to binary

> 💡 **What does `wasm-tools parse` do?**  
> It reads the text format (WAT) and encodes it as the binary WASM format. The output file is identical in meaning — just a more compact representation that runtimes can load faster. The binary always starts with the 4-byte magic number `\0asm` (hex: `00 61 73 6D`).

```bash
wasm-tools parse hello.wat -o hello.wasm
```

### Step 3 – Run with Wasmtime

```bash
wasmtime hello.wasm
```

Expected output:
```
Hello, WASM!
```

### Step 4 – Inspect the binary

> 💡 **What are you looking at?**  
> `xxd` shows the raw bytes in hex. The first 4 bytes (`00 61 73 6d`) are the WASM magic number. `wasm-tools print` decompiles the binary back to WAT — notice it's exactly what you wrote. `wasm-tools validate` type-checks the module: if this passes it means the runtime has verified all type signatures, memory accesses, and control flow are safe *before* running a single instruction.

```bash
# Check file magic bytes
xxd hello.wasm | head -2

# Print WAT back from binary
wasm-tools print hello.wasm

# Validate the binary
wasm-tools validate hello.wasm
```

---

## Lab 1B – Linear Memory Explorer

**Goal:** Understand how WASM linear memory works.

**What you'll learn:** WASM modules don't have heap or stack memory in the traditional sense — they have a single flat byte array called *linear memory*. It starts at a declared size (in 64 KB pages) and can grow at runtime. All data — strings, structs, arrays — lives in this one array. The module can read and write any byte in it freely, but it *cannot* reach outside it: any access beyond the declared size causes an immediate trap (a clean, safe error), never undefined behaviour or a security hole.

> 💡 **Why 64 KB pages?**  
> This mirrors the OS virtual memory page size convention. One WASM page = 65,536 bytes. The maximum size is 65,536 pages = 4 GB (the full range of a 32-bit address space). Most real modules use far less.

Open the file already provided at `labs/lab-1b/memory.wat`:

```bash
cd module-01-runtime-essentials/labs/lab-1b
```

File contents (`memory.wat`):

```wat
(module
  (memory (export "memory") 1)  ;; 1 page = 64 KiB

  ;; Store a value in memory at address 100
  (func (export "store") (param $addr i32) (param $val i32)
    (i32.store (local.get $addr) (local.get $val)))

  ;; Load a value from memory at address 100
  (func (export "load") (param $addr i32) (result i32)
    (i32.load (local.get $addr)))

  ;; Return current memory size in pages
  (func (export "mem_size") (result i32)
    memory.size)

  ;; Grow memory by N pages, return previous size (-1 on failure)
  (func (export "mem_grow") (param $pages i32) (result i32)
    (memory.grow (local.get $pages)))
)
```

```bash
wasm-tools parse memory.wat -o memory.wasm
wasm-tools validate memory.wasm
wasm-tools print memory.wasm
```

Run interactively with Wasmtime's `--invoke` flag:

> 💡 **What is `--invoke` doing?**  
> Instead of calling the `_start` entry point (what `main()` maps to), `--invoke` calls any exported function by name and passes arguments directly. This lets you poke at individual functions as if they were a library — great for exploration and testing.

```bash
# Store value 42 at address 100
wasmtime --invoke store memory.wasm 100 42

# Load value from address 100 — should return 42
wasmtime --invoke load memory.wasm 100

# Get memory size (should be 1 page = 64 KB)
wasmtime --invoke mem_size memory.wasm

# Grow by 1 page (returns the OLD size — 1)
wasmtime --invoke mem_grow memory.wasm 1

# Check new size — should now be 2
wasmtime --invoke mem_size memory.wasm
```

> **Try this:** What happens if you load from address 0, or from address 70000 (beyond one page)?
>
> ```bash
> # Address 0 — always zero (WASM zeroes all memory on init)
> wasmtime --invoke load memory.wasm 0
> # Output: 0
>
> # Address 70000 — beyond the 64KB page boundary, should trap
> wasmtime --invoke load memory.wasm 70000
> # Output: memory fault at wasm address 0x11170 in linear memory of size 0x10000
> #         wasm trap: out of bounds memory access
> ```
>
> The trap is clean and descriptive — Wasmtime tells you exactly which address was accessed and the size of the memory region. No crash, no undefined behaviour, no security hole.
>
> You may also see a warning about `--invoke` being experimental for functions with arguments/return values — this is a Wasmtime CLI warning, not an error. The behaviour is correct.

---

## Lab 1C – Stack Machine Arithmetic (Exploration)

**Goal:** Understand the stack-based execution model.

**What you'll learn:** WASM is a *stack machine* — every instruction either pushes a value onto an implicit value stack or pops one (or more) off it and pushes the result. There are no named registers like in x86 or ARM. This makes WASM easy to validate (the type-checker can simulate the stack statically) and easy to compile to native code (the JIT maps stack slots to registers).

Tracing the stack manually for a few instructions is the fastest way to build an intuition for the execution model. The comments in `calc.wat` show the stack state after each instruction.

Open `labs/lab-1c/calc.wat` and trace the stack manually:

```bash
cd module-01-runtime-essentials/labs/lab-1c
```

File contents (`calc.wat`):

```wat
(module
  ;; Compute: (10 + 5) * 3 - 2
  (func (export "compute") (result i32)
    i32.const 10   ;; stack: [10]
    i32.const 5    ;; stack: [10, 5]
    i32.add        ;; stack: [15]
    i32.const 3    ;; stack: [15, 3]
    i32.mul        ;; stack: [45]
    i32.const 2    ;; stack: [45, 2]
    i32.sub        ;; stack: [43]
  )               ;; returns 43
)
```

```bash
wasm-tools parse calc.wat -o calc.wasm
wasmtime --invoke compute calc.wasm
# Expected: 43
```

> 💡 **Why does a function with `(result i32)` not need an explicit `return`?**  
> In WASM, a function's return value is whatever is left on the stack when it ends. The type-checker enforces that exactly one `i32` remains — if you accidentally leave 0 or 2 values, validation fails before the module ever runs.

**Challenge:** Modify `calc.wat` to compute the factorial of 5 using a `loop` and `br_if` instruction. You'll need:
- A `local` variable to hold the accumulator
- `loop` to create a backward jump target
- `br_if` to conditionally break out of the loop
- `local.get` / `local.set` to read and write the local

Solution is in `labs/lab-1c/solution/` — try it yourself first!

---

## ✅ Module 1 Checklist

- [ ] Ran `wasmtime hello.wasm` and saw "Hello, WASM!"
- [ ] Inspected a `.wasm` binary with `wasm-tools`
- [ ] Understood linear memory pages (64KB each)
- [ ] Traced stack operations in `calc.wat`
- [ ] (Bonus) Implemented factorial in WAT

---

## Key Takeaways

1. WASM is a **stack-based VM** with strict typing
2. **Linear memory** is the only mutable state; it's bounded and isolated
3. **WASI** provides OS capabilities via imports — nothing is granted by default
4. WAT is the human-readable form; `wasm-tools` bridges WAT ↔ WASM
