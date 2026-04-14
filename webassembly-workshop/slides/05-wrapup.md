# Workshop Wrap-Up & Next Steps
**⏱ Duration:** 15 minutes

---

## 🏁 What We Covered Today

| Module | Topic | Key Takeaway |
|--------|-------|-------------|
| **Intro** | WASM fundamentals | WASM = portable, sandboxed bytecode escaping the browser |
| **Module 1** | Runtime & execution model | Stack-based VM, linear memory, WASI capability model |
| **Module 2** | Building & running WASM | Rust → WASM → Wasmtime; WASI deny-by-default security |
| **Module 3** | Performance & security | 10–100× faster cold starts; structural sandbox vs policy-based |
| **Module 4** | Cloud-native patterns | Spin microservices, plugin systems, K8s, FaaS simulation |

---

## 🧠 The Mental Model to Take Away

```
           ┌─────────────────────────────────────────┐
           │         Decision Framework               │
           │                                         │
           │  Cold start < 10ms needed?  → WASM ✅   │
           │  Untrusted code isolation?  → WASM ✅   │
           │  Multi-arch portability?    → WASM ✅   │
           │  Rich OS / GPU / long run?  → Container │
           │  Mature ecosystem priority? → Container │
           │                                         │
           │  → WASM and containers are COMPLEMENTARY │
           └─────────────────────────────────────────┘
```

---

## 🔭 What's Next: WASM Ecosystem to Watch

| Project / Standard | Why it matters |
|--------------------|----------------|
| **WASM Component Model** | Language-agnostic composition of modules |
| **wasmCloud** (CNCF) | Distributed WASM actor framework |
| **Fermyon Spin** | Production-ready WASM microservices |
| **WasmEdge** | Docker + K8s native WASM runtime |
| **WASI 0.2** | Stable, capability-safe system interface |
| **wasi:http** | First-class HTTP in WASM (no FFI needed) |
| **WASM GC** | Garbage-collected languages (Java, Kotlin, Dart) in WASM |
| **WASM threads** | Shared memory multithreading in WASM |

---

## 📚 Recommended Resources

### Official Specs & Docs
- [webassembly.org](https://webassembly.org) – Official spec and roadmap
- [WASI.dev](https://wasi.dev) – WASI standards
- [Component Model spec](https://component-model.bytecodealliance.org)

### Runtimes
- [Wasmtime docs](https://docs.wasmtime.dev)
- [WasmEdge docs](https://wasmedge.org/docs)
- [Fermyon Spin docs](https://developer.fermyon.com/spin)

### Books & Articles
- *WebAssembly: The Definitive Guide* – O'Reilly
- Bytecode Alliance blog: [bytecodealliance.org/articles](https://bytecodealliance.org/articles)
- Lin Clark's illustrated WASM series on hacks.mozilla.org

### Hands-on Practice
- [wasmbyexample.dev](https://wasmbyexample.dev) – WASM examples in many languages
- [webassembly.studio](https://webassembly.studio) – In-browser WASM IDE
- CNCF WasmDay conference talks (YouTube)

---

## 🎯 Action Items for This Week

1. **Try it at work:** Identify one function in your codebase that could be a WASM candidate (short-lived, CPU-bound, needs isolation)
2. **Experiment:** Run `wasmtime` on a real Rust program from your team's repos
3. **Read:** Bytecode Alliance's Component Model explainer
4. **Share:** Present the WASM vs container comparison slide to your team

---

## 💬 Q&A

Open floor for questions. Common topics:

- *"How do we debug WASM in production?"* — DWARF debug info + wasmtime's `--wasm-backtrace-details=on`
- *"Can WASM replace our entire microservice stack?"* — Gradually; start with compute-heavy, stateless functions
- *"What about observability?"* — OpenTelemetry WASM SDK is maturing; Spin has built-in traces
- *"What languages should we learn for WASM?"* — Rust has the best support today; Go is catching up fast

---

## 📋 Feedback

Please fill in the workshop feedback form — it takes 2 minutes and directly shapes future sessions.

**Thank you for attending! 🚀**

*© The Better Software Initiative*
