# Slides – Module 4: WASM in Microservices, Serverless & Edge
**Duration:** 50 minutes (20 min theory + 30 min lab)

---

## Slide 1 – WASM in Microservices

**Traditional microservice architecture:**
```
Client → API Gateway → Service A (container) → Service B (container)
```

**WASM-enhanced microservice architecture:**
```
Client → API Gateway → WASM Host
                           ├── Module A.wasm  (business logic)
                           ├── Module B.wasm  (auth)
                           └── Module C.wasm  (transform)
```

**Benefits:**
- Modules share a process but are strongly isolated
- Hot-swappable (load new .wasm without restart)
- No inter-process overhead for in-process composition
- Multi-language: each module in a different source language

---

## Slide 2 – WASM as a Plugin System

WASM is an excellent **extensibility mechanism**:

Real-world examples:
- **Envoy Proxy**: HTTP filters as WASM modules (user-written C++ / Rust)
- **Shopify Functions**: Checkout customizations as WASM
- **Figma**: Plugin sandbox using WASM
- **Cloudflare Workers**: JavaScript → WASM for isolate-per-request
- **OpenTelemetry**: WASM-based data pipelines (Moose)

```
┌──────────────┐         ┌──────────────────────┐
│  Host App    │ imports │  WASM Plugin Module  │
│  (Rust/Go)   │◄───────►│  (any language)      │
└──────────────┘         └──────────────────────┘
    Grants only                 Cannot escape
    requested caps              sandbox
```

---

## Slide 3 – WASM in Serverless

**Serverless challenges solved by WASM:**

| Problem | Container FaaS | WASM FaaS |
|---------|---------------|-----------|
| Cold start | 100ms–5s | <5ms |
| Idle cost | Warm containers $$$ | Near-zero |
| Security per function | Shared kernel risk | Structural isolation |
| Scale to zero | Slow | Instant |

**Production platforms using WASM for serverless:**
- **Fastly Compute** (formerly Compute@Edge)
- **Cloudflare Workers** (V8 isolates + WASM)
- **Fermyon Spin** (open source, Kubernetes-native)
- **wasmCloud** (CNCF sandbox project)

---

## Slide 4 – Fermyon Spin (Hands-on Preview)

Spin is an open-source framework for building WASM microservices:

```bash
# Install Spin
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash

# Create a new HTTP handler in Rust
spin new -t http-rust hello-spin
cd hello-spin

# Build and run locally
spin build
spin up

# Test
curl http://127.0.0.1:3000
```

Spin component manifest (`spin.toml`):
```toml
spin_manifest_version = 2

[application]
name = "hello-spin"
version = "0.1.0"

[[trigger.http]]
route = "/..."
component = "hello-spin"

[component.hello-spin]
source = "target/wasm32-wasip1/release/hello_spin.wasm"
allowed_outbound_hosts = []
[component.hello-spin.build]
command = "cargo build --target wasm32-wasip1 --release"
```

---

## Slide 5 – WASM at the Edge

**Edge computing** = run code as close to users as possible (CDN PoPs, IoT gateways)

**Why WASM excels at edge:**
- Single binary that runs on x86, ARM, RISC-V, MIPS
- No OS image shipping overhead
- Extremely fast startup (essential for per-request isolation)
- Strict sandboxing (untrusted code at the edge is a real concern)

**Edge platforms using WASM:**
| Platform | Description |
|----------|-------------|
| Cloudflare Workers | 300+ PoPs, V8 isolates |
| Fastly Compute | Rust/Go WASM at CDN edge |
| Akamai EdgeWorkers | JavaScript WASM |
| Netlify Edge Functions | Deno + WASM |
| Vercel Edge Runtime | V8 + WASM |

---

## Slide 6 – WASM in Kubernetes (WasmEdge + containerd)

WASM modules can run as Kubernetes pods via **runwasi** + **containerd**:

```yaml
# Deploy a WASM workload on K8s
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wasm-demo
spec:
  replicas: 3
  selector:
    matchLabels:
      app: wasm-demo
  template:
    metadata:
      labels:
        app: wasm-demo
      annotations:
        module.wasm.image/variant: compat-smart
    spec:
      runtimeClassName: wasmtime   # ← WASM runtime class
      containers:
        - name: hello-wasm
          image: ghcr.io/example/hello-wasm:latest
          resources:
            limits:
              memory: "32Mi"
              cpu: "100m"
```

**Required:** containerd + runwasi shim + Wasmtime/WasmEdge runtime class

---

## Slide 7 – wasmCloud: Distributed WASM

**wasmCloud** (CNCF Sandbox) provides:
- Actor model for WASM components
- Capability providers (database, HTTP, messaging)
- Multi-cloud, multi-edge deployment
- Hot reloading of components

```
┌─────────────────────────────────────────────┐
│            wasmCloud Lattice                 │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐ │
│  │ Actor    │   │ Actor    │   │ Actor    │ │
│  │ (WASM)   │   │ (WASM)   │   │ (WASM)   │ │
│  └────┬─────┘   └────┬─────┘   └────┬─────┘ │
│       │              │              │        │
│  ┌────▼──────────────▼──────────────▼─────┐  │
│  │         Capability Providers           │  │
│  │   (HTTP / DB / Messaging / KV Store)   │  │
│  └────────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
```

---

## Slide 8 – Real-World Decision Framework

```
Is startup latency critical? (<10ms)
    YES → WASM ✅
    NO  → Continue...

Is multi-tenant untrusted code execution required?
    YES → WASM ✅
    NO  → Continue...

Is binary portability across CPU architectures required?
    YES → WASM ✅
    NO  → Continue...

Are rich OS dependencies needed? (filesystem, GPU, network stacks)
    YES → Container 🐳
    NO  → Either works; prefer WASM for density + security
```

---

## Lab 4 instructions → see [module-04-cloud-native/README.md](../module-04-cloud-native/README.md)
