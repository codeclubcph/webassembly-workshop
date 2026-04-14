# Slides – Module 2: Building & Running Your First WASM App with Wasmtime
**Duration:** 45 minutes (15 min theory + 30 min lab)

---

## Slide 1 – Compiling Rust to WASM

Rust has first-class WASM support via the `wasm32-wasip1` target.

```bash
# Create a new project
cargo new hello-wasm
cd hello-wasm

# Build for WASM
cargo build --target wasm32-wasip1 --release

# Output
ls target/wasm32-wasip1/release/hello-wasm.wasm
```

The compiled binary is self-contained and portable.

---

## Slide 2 – Running with Wasmtime CLI

```bash
# Run directly
wasmtime target/wasm32-wasip1/release/hello-wasm.wasm

# Pass arguments
wasmtime target/wasm32-wasip1/release/hello-wasm.wasm -- arg1 arg2

# Mount a host directory (explicit capability grant)
wasmtime --dir /tmp target/wasm32-wasip1/release/hello-wasm.wasm

# Set environment variables
wasmtime --env MY_VAR=hello target/wasm32-wasip1/release/hello-wasm.wasm

# Limit memory
wasmtime --max-wasm-stack 512000 target/wasm32-wasip1/release/hello-wasm.wasm
```

---

## Slide 3 – Inspecting a WASM Binary

```bash
# View module structure
wasm-tools dump target/wasm32-wasip1/release/hello-wasm.wasm

# Print WAT (human-readable)
wasm-tools print target/wasm32-wasip1/release/hello-wasm.wasm | head -100

# Check imports and exports
wasm-tools objdump -x target/wasm32-wasip1/release/hello-wasm.wasm

# Validate the binary
wasm-tools validate target/wasm32-wasip1/release/hello-wasm.wasm
```

---

## Slide 4 – Using Wasmtime from Code (Embedding API)

Wasmtime has embedding APIs for Rust, Python, Go, C, and more.

**Rust Embedding Example:**

```rust
use wasmtime::*;

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "add.wasm")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let result = add.call(&mut store, (5, 3))?;
    println!("5 + 3 = {}", result); // 8
    Ok(())
}
```

---

## Slide 5 – Languages That Compile to WASM

| Language | Toolchain | WASI Support | Maturity |
|----------|-----------|-------------|----------|
| **Rust** | `rustc --target wasm32-wasip1` | ✅ Excellent | 🟢 Production |
| **C/C++** | `clang` + WASI SDK | ✅ Excellent | 🟢 Production |
| **Go** | `GOOS=wasip1 GOARCH=wasm go build` | ✅ Good | 🟢 Production |
| **Python** | `py2wasm` / Pyodide | ⚠️ Partial | 🟡 Maturing |
| **JavaScript** | Javy (Shopify) | ⚠️ Partial | 🟡 Maturing |
| **C#/.NET** | `dotnet publish -r wasi-wasm` | ✅ Good | 🟡 Maturing |
| **Swift** | SwiftWasm | ⚠️ Partial | 🟡 Maturing |

---

## Slide 6 – The WASM Binary Format

```
Magic bytes:  00 61 73 6D  → "\0asm"
Version:      01 00 00 00  → version 1

Sections (in order):
  Type      – function signatures
  Import    – host functions needed
  Function  – function index → type mapping
  Table     – function references
  Memory    – linear memory declarations
  Global    – global variables
  Export    – names exposed to host
  Element   – table initialization
  Code      – function bodies (bytecode)
  Data      – memory initialization
  Custom    – debug info, names, etc.
```

---

## Lab 2 instructions → see [module-02-first-wasm-app/README.md](../module-02-first-wasm-app/README.md)
