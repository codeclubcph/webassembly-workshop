# Slides – Module 1: WASM Runtime Essentials & Execution Model
**Duration:** 45 minutes (25 min theory + 20 min lab)

---

## Slide 1 – The WASM Execution Model

WASM runs on a **stack-based virtual machine**:

- Instructions operate on a **value stack**
- Strongly typed: `i32`, `i64`, `f32`, `f64`, `v128`, `funcref`, `externref`
- Structured control flow (no arbitrary jumps)
- **Linear memory**: one flat, contiguous byte array
- **No direct OS access** — everything goes through host imports

```
┌─────────────────────────────────────┐
│         WASM Module                 │
│  ┌──────────┐   ┌────────────────┐  │
│  │ Functions│   │ Linear Memory  │  │
│  │ (bytecode│   │  (sandboxed)   │  │
│  │  stack   │   │ 0 … max 4 GB   │  │
│  │  VM)     │   └────────────────┘  │
│  └──────────┘   ┌────────────────┐  │
│  ┌──────────┐   │  Tables        │  │
│  │ Globals  │   │ (funcref/ptr)  │  │
│  └──────────┘   └────────────────┘  │
└──────────────┬──────────────────────┘
               │ imports / exports
        ┌──────┴──────┐
        │  Host / OS  │  (Wasmtime, browser, etc.)
        └─────────────┘
```

---

## Slide 2 – WebAssembly Text Format (WAT)

WASM has a human-readable text format: **WAT**

```wat
(module
  ;; Declare a function that adds two i32 values
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add)

  ;; Export it so the host can call it
  (export "add" (func $add))
)
```

Compile WAT → WASM binary:
```bash
wasm-tools parse add.wat -o add.wasm
```

---

## Slide 3 – WASI: WebAssembly System Interface

WASM on its own has **no OS access**. WASI provides a standard interface:

| WASI Capability | What it provides |
|-----------------|-----------------|
| `wasi:filesystem` | File read/write |
| `wasi:sockets` | Network I/O |
| `wasi:clocks` | Timestamps |
| `wasi:random` | Secure random bytes |
| `wasi:cli` | stdin / stdout / stderr |
| `wasi:http` | HTTP client/server |

> 🔑 WASI uses a **capability-based security model**: modules only get the capabilities you explicitly grant.

---

## Slide 4 – The Component Model

**WASM Core** = low-level bytes  
**WASM Component Model** = high-level composition

- Defines rich types: records, variants, options, results, strings
- Enables **language-agnostic interfaces** (WIT files)
- Compose components into larger systems without shared memory

```
┌──────────────┐     WIT interface     ┌──────────────┐
│  Component A │ ◄──────────────────► │  Component B │
│  (Rust)      │                       │  (Go)        │
└──────────────┘                       └──────────────┘
```

---

## Slide 5 – Runtimes Landscape

| Runtime | Use Case | Notable Users |
|---------|----------|---------------|
| **Wasmtime** | Server-side, Bytecode Alliance | Fastly, Azure |
| **WasmEdge** | Cloud-native, Docker, K8s | Second State |
| **Wasmer** | General purpose | |
| **WAVM** | High-performance JIT | |
| **V8 / SpiderMonkey** | Browser + Deno/Node | Google, Mozilla |
| **wazero** | Go-native, zero deps | Tetrate |

**This workshop uses Wasmtime** (most standards-compliant, great CLI).

---

## Slide 6 – How a WASM Module Loads and Runs

```
Source code (.rs / .c / .py)
        │
        ▼  (compiler: rustc, clang, etc.)
   .wasm file  (binary: magic bytes 0x00 0x61 0x73 0x6D)
        │
        ▼  (runtime: Wasmtime)
   1. Parse & validate (type-check)
   2. Compile to native machine code (JIT or AOT)
   3. Instantiate (allocate memory, link imports)
   4. Execute (call exported function)
```

Validation guarantees:
- No undefined behavior
- No stack overflows (checked)
- No out-of-bounds memory access (trapped, not undefined)

---

## Lab 1 instructions → see [module-01-runtime-essentials/README.md](../module-01-runtime-essentials/README.md)
