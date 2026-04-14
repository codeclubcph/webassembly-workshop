# Slides – Module 3: Performance & Security vs. Containers
**Duration:** 45 minutes (25 min theory + 20 min lab)

---

## Slide 1 – Startup Time: The Critical Difference

| Runtime | Cold-start time |
|---------|----------------|
| JVM microservice | 500ms – 5s |
| Docker container | 100ms – 1s |
| Native binary | 5ms – 50ms |
| **WASM (Wasmtime JIT)** | **1ms – 5ms** |
| **WASM (Wasmtime AOT)** | **< 1ms** |

**Why does WASM start so fast?**
- No OS-level container setup (no namespaces, no cgroup allocation)
- No kernel boot process
- Module validation + compilation is fast and parallelizable
- AOT: already native code, just load and run

> 🔑 This is transformative for **serverless** and **edge** where cold starts dominate cost.

---

## Slide 2 – Memory Efficiency

**Container overhead:**
- Full OS filesystem layer
- Kernel structures per container
- Typically 10MB–500MB just for the runtime

**WASM overhead:**
- A single `.wasm` file (often 100KB–5MB)
- Linear memory: only what the module uses
- No OS image, no daemon, no overlay filesystem

**Practical impact:** You can run **10–100x more WASM instances** on the same host vs containers.

---

## Slide 3 – Throughput & Latency

WASM performance characteristics:
- **CPU-bound tasks**: 10–30% slower than native C (due to bounds checks)
  - Bounds checks on every memory access (can be elided by optimizer)
  - No SIMD penalty (WASM SIMD is standardized)
- **I/O-bound tasks**: comparable to native (bottleneck is I/O, not compute)
- **JIT vs AOT**: AOT compiled WASM approaches native speed

**When WASM beats containers:**
- Functions called infrequently (cold start dominates)
- High-density multi-tenant workloads
- Functions needing strict isolation without overhead

---

## Slide 4 – Security Model: Containers

**Container security relies on:**
- Linux namespaces (PID, network, mount, UTS, IPC, user)
- cgroups (resource limits)
- seccomp (syscall filtering)
- AppArmor / SELinux (MAC policies)

**Weaknesses:**
- Shares the host kernel → kernel exploits affect all containers
- Misconfigured seccomp can expose dangerous syscalls
- Container escapes have been found (CVE-2019-5736, runc, etc.)
- Default Docker: no user namespace isolation

---

## Slide 5 – Security Model: WebAssembly

**WASM security is structural, not policy-based:**

```
┌─────────────────────────────────────┐
│         Sandbox Guarantees          │
│                                     │
│  • Memory isolation (linear mem)    │
│  • No arbitrary syscalls            │
│  • No raw pointers / undefined UB   │
│  • No shared mutable state          │
│  • Capability-based I/O (WASI)      │
│  • Typed imports / exports only     │
└─────────────────────────────────────┘
```

**Threat model:** Even if a WASM module is malicious, it cannot:
- Access memory of other modules
- Make syscalls directly
- Escape its sandbox without an explicit host vulnerability

> WASM security is **by construction**, not by configuration.

---

## Slide 6 – Side-by-Side Comparison

| Feature | Docker Container | WebAssembly |
|---------|-----------------|-------------|
| Startup time | 100ms–1s | <1ms–5ms |
| Image size | 10–500MB | 100KB–5MB |
| Memory per instance | 50–200MB | 1–20MB |
| Kernel dependency | Shared host kernel | None (fully sandboxed) |
| Security boundary | Linux isolation | Structural sandbox |
| Syscall access | Via seccomp filter | Via capability grants only |
| Language support | Any (pre-compiled binary) | 20+ languages → WASM |
| Networking | Full TCP/IP stack | WASI sockets (growing) |
| Multi-tenancy | Moderate (namespace overhead) | Excellent (lightweight) |
| Ecosystem maturity | 🟢 Very mature | 🟡 Rapidly maturing |
| Best for | Long-running services | Short functions, edge, plugins |

---

## Slide 7 – When to Choose What

**Choose Containers when:**
- Long-running stateful services
- Rich OS dependencies (databases, complex networking)
- Mature ecosystem & tooling is a priority
- Team familiarity with Docker/K8s

**Choose WASM when:**
- Ultra-low latency cold starts required
- High-density multi-tenant execution
- Plugin/extension systems (untrusted code)
- Edge / CDN workloads
- Strict security isolation without ops overhead

---

## Lab 3 instructions → see [module-03-performance-security/README.md](../module-03-performance-security/README.md)
