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

## Background Reading

See [slides/01-runtime-essentials.md](../slides/01-runtime-essentials.md)

---

## Lab 1A – Hello, WAT!

**Goal:** Write a WASM module in WAT, compile it to binary, and run it.

### Step 1 – Write the module

```bash
mkdir -p labs/lab-1a && cd labs/lab-1a
```

Create `hello.wat`:

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

Create `memory.wat`:

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

```bash
# Store value 42 at address 100
wasmtime --invoke store memory.wasm 100 42

# Load value from address 100
wasmtime --invoke load memory.wasm 100

# Get memory size (should be 1 page)
wasmtime --invoke mem_size memory.wasm

# Grow by 1 page (should return 1, the old size)
wasmtime --invoke mem_grow memory.wasm 1

# Check new size (should be 2)
wasmtime --invoke mem_size memory.wasm
```

---

## Lab 1C – Stack Machine Arithmetic (Exploration)

**Goal:** Understand the stack-based execution model.

Create `calc.wat` and trace the stack manually:

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

**Challenge:** Modify `calc.wat` to compute the factorial of 5 using a `loop` and `br_if` instruction. Solution in `labs/lab-1c/solution/`.

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
