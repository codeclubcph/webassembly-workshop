# Module 4 – WASM in Microservices, Serverless & Edge
**⏱ Duration:** 50 minutes | **🧪 Lab time:** 30 minutes

---

## Learning Objectives

By the end of this module you will be able to:
- Build an HTTP handler as a WASM component using Fermyon Spin
- Understand how WASM plugs into Kubernetes via runwasi
- Recognize common WASM-native platforms (Cloudflare Workers, Fastly)
- Apply a decision framework for choosing WASM vs containers in your architecture

---

## Lab 4A – HTTP Microservice with Fermyon Spin

**Goal:** Build a WASM-based HTTP microservice that responds to requests.

**What you'll learn:** Fermyon Spin is a framework built specifically for WASM-first microservices. Instead of writing a `main()` loop that listens on a socket, you write a *handler function* — Spin owns the HTTP server, the TCP socket, the TLS termination, and the routing table. Your WASM module just implements the handler. This is the same model as AWS Lambda or Azure Functions, but with WASM-speed cold starts (sub-millisecond) instead of container-speed cold starts (seconds).

Spin's `spin.toml` manifest declares capabilities at the component level — which routes it handles, which outbound hosts it may call, which key-value stores it may access. This is WASI's capability model applied to cloud services: the module cannot reach any network destination that isn't listed in the manifest.

### Step 1 – Install Spin

```bash
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
export PATH="$HOME/.fermyon/bin:$PATH"
spin --version
```

### Step 2 – Create a new Spin app

```bash
cd module-04-cloud-native/labs
spin new -t http-rust lab-4a-spin-service
cd lab-4a-spin-service
```

### Step 3 – Explore the generated code

Look at `src/lib.rs`. Spin generates a handler skeleton.

Replace it with the code from `labs/lab-4a-spin-service/src/lib.rs` (already provided).

### Step 4 – Build and run

```bash
spin build
spin up
```

> 💡 **What `spin build` does:** It calls `cargo build --target wasm32-wasip1 --release` under the hood (using the `[component.build]` section of `spin.toml`), then packages the resulting `.wasm` file. `spin up` loads the WASM module into its embedded Wasmtime engine, instantiates one instance per route, and starts the HTTP listener — all before it prints "Serving http://127.0.0.1:3000".

### Step 5 – Test the endpoints

```bash
# Health check
curl http://127.0.0.1:3000/health

# Echo endpoint
curl -X POST http://127.0.0.1:3000/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from WASM!"}'

# Info endpoint
curl http://127.0.0.1:3000/info
```

### Step 6 – Observe the startup

```bash
# Time how fast Spin serves the first request
time curl -s http://127.0.0.1:3000/health
```

> 💡 **Why no startup delay?** In a container-based FaaS (e.g., AWS Lambda backed by Firecracker), the first request to a cold function must wait for the microVM to boot. With Spin, the WASM module is already instantiated before `spin up` finishes printing to the terminal. The "first request" is warm by the time you send it, because instantiation is so fast it happens at server startup rather than at request time.

---

## Lab 4B – WASM as a Plugin System

**Goal:** Build a host application that loads user-supplied WASM "plugin" modules.

**What you'll learn:** The plugin pattern is one of WASM's killer use cases in production today. Shopify uses it for storefront extensions, Envoy Proxy uses it for custom filter chains, Figma uses it for design plugins, and Zed editor uses it for language extensions. The core insight is: you want user-submitted or third-party code to run inside your process (for performance — no IPC overhead) but you cannot trust it (for security — it must not read your data or crash your server). WASM gives you both. The host app exposes *only* the functions it chooses to expose as imports; the plugin can call nothing else.

String passing is the most instructive part of this lab. WASM functions can only pass numeric types (`i32`, `i64`, `f32`, `f64`) across the module boundary — there's no "pass a string" instruction. The convention for strings is: allocate memory in the module's linear memory, write the bytes there, and pass a (pointer, length) pair as two `i32` arguments. The host reads or writes those bytes directly via the module's exported `memory`. You will see this pattern in the plugin's exported `alloc`, `transform`, and `dealloc` functions.

This demonstrates the extension/plugin pattern used by Envoy, Shopify, etc.

### Architecture

```
Host App (Rust)
  │
  ├── loads plugin-a.wasm  (transform: uppercase)
  ├── loads plugin-b.wasm  (transform: reverse)
  └── loads plugin-c.wasm  (transform: word-count)

Each plugin:
  - Receives a string via shared memory
  - Returns a transformed string
  - Cannot access anything else
```

### Build plugins

```bash
cd labs/lab-4b-plugin-system

# Build all three plugins
cd plugins/uppercase  && cargo build --target wasm32-wasip1 --release && cd ../..
cd plugins/reverse    && cargo build --target wasm32-wasip1 --release && cd ../..
cd plugins/wordcount  && cargo build --target wasm32-wasip1 --release && cd ../..
```

### Build and run the host

```bash
cd host
cargo run -- "Hello from WebAssembly Cloud!"
```

Expected output:
```
🔌 Plugin Pipeline Demo
Input: "Hello from WebAssembly Cloud!"

Applying plugin: uppercase
  → "HELLO FROM WEBASSEMBLY CLOUD!"

Applying plugin: reverse
  → "!duolC ylbmessAbeW morf olleH"

Applying plugin: wordcount
  → Word count: 4
```

> 💡 **Why can't the plugin escape?** Each plugin is instantiated with an empty set of WASI imports — the host wires up no filesystem, no network, no environment variables. The plugin's `transform` function receives bytes, transforms them, and writes the result. It has no mechanism to do anything else, because there are no other imports to call. This is why Shopify can safely run merchant-authored code in their checkout pipeline: even a deliberately malicious merchant plugin cannot read another merchant's cart data.

---

## Lab 4C – Simulating Serverless: Cold Start vs Warm Request

**Goal:** Simulate the FaaS (Function-as-a-Service) execution model with WASM.

**What you'll learn:** In a FaaS platform, "cold start" means the platform must load, instantiate, and initialise your function's runtime environment before it can handle the first request. For a Node.js Lambda, that can mean 300–1000ms. For a JVM function, sometimes multiple seconds. Platforms like Fastly Compute and wasmCloud solve this by using WASM: instantiation takes under 1ms, so it's economically viable to *instantiate fresh per request* rather than keeping warm instances around.

This has architectural implications. Warm instances introduce subtle bugs: global state from one request leaks into the next. Fresh-per-request instantiation eliminates an entire class of bugs (accidental state sharing across tenants or requests) while costing less than 1ms. This lab makes that tradeoff concrete and measurable.

This mimics how platforms like Fastly Compute or wasmCloud handle requests.

```bash
cd labs/lab-4c-faas-sim
cargo run
```

The simulator:
1. Accepts "invocations" on stdin (type a JSON payload and press Enter)
2. Loads and instantiates the WASM module fresh for each invocation (cold)
3. Reports timing for each: instantiation + execution time
4. Compares to a "warm" path (pre-instantiated instance pool)

Try it:
```
{"event": "order.placed", "orderId": "abc123"}
{"event": "user.signup", "userId": "user456"}
{"event": "payment.failed", "orderId": "abc123"}
```

> 💡 **Reading the output:** You will see two timing columns — `instantiate` and `execute`. Notice that `instantiate` is consistently in the sub-millisecond range even for "cold" invocations. For the "warm" path the instantiation cost is amortised across many requests (the instance is reused from a pool). The key observation is that even the cold path is fast enough that per-request instantiation is practical — which eliminates all concerns about shared state between invocations by construction.

---

## Lab 4D – Kubernetes WASM Manifest (Discussion Lab)

**Goal:** Understand how WASM workloads deploy on Kubernetes.

**What you'll learn:** Kubernetes has always been container-centric, but since Kubernetes 1.20 it supports custom runtime classes via the `RuntimeClass` API and the containerd shim model. `runwasi` is a containerd shim that speaks the WASM execution model instead of the OCI container model. When the Kubernetes scheduler places a Pod with `runtimeClassName: wasmtime`, containerd hands the OCI image to the `runwasi` shim, which extracts the `.wasm` file and runs it with Wasmtime instead of starting a container. From Kubernetes' perspective it's just a Pod. From containerd's perspective it's a WASM module. No code changes in your orchestration layer.

This matters because it means WASM workloads can use the entire existing Kubernetes ecosystem: Services, Ingress, HPA, PodDisruptionBudgets, GitOps tooling, Helm charts — everything works unchanged.

> 📝 This lab is a review-and-discuss exercise — no live K8s cluster required.

### Review the manifests

```bash
ls labs/lab-4d-kubernetes/
```

Files:
- `runtime-class.yaml` – Registers the Wasmtime runtime class with containerd
- `deployment.yaml` – Deploys a WASM module as a K8s Deployment
- `service.yaml` – Exposes it via a ClusterIP Service

### Key differences from container deployments

Review `deployment.yaml` and note:
- `spec.template.spec.runtimeClassName: wasmtime` → uses WASM shim instead of runc
- Image is a standard OCI image but contains `.wasm` instead of ELF binaries
- Very low resource requests (`memory: "16Mi"`, `cpu: "50m"`)
- Annotation `module.wasm.image/variant: compat-smart`

> 💡 **Why so little memory?** A typical containerised Go or Java microservice requests 128–512Mi just to start the runtime. WASM modules have no garbage collector, no JIT warm-up heap, and no OS inside the image. The `memory: "16Mi"` request is realistic for many WASM workloads, not a typo. This means you can run far more WASM Pods per node than container Pods — which directly reduces infrastructure cost.

> 💡 **What `compat-smart` means:** OCI images containing WASM modules use an annotation to tell the containerd shim whether the module targets WASI preview 1 (`compat`) or the newer WASI Component Model (`compat-smart` lets the shim auto-detect). This is just a hint for the shim — the WASM binary itself is unchanged.

### Discussion Questions

1. What changes would your CI/CD pipeline need to ship WASM to Kubernetes?
2. How would you handle a WASM module that needs database access?
3. What observability challenges arise with WASM vs traditional containers?
4. When would you run WASM *inside* a container vs as a replacement?

---

## Lab 4E – Architecture Decision Exercise (Group Activity)

**Goal:** Apply the decision framework to real scenarios.

**What you'll learn:** There is no universal answer to "should I use WASM or containers?" The right choice depends on the shape of the workload, the trust model, the cold-start budget, and the operational environment. This exercise forces you to articulate *why* one approach fits better — that reasoning process is what you'll take back to your team.

Use these decision criteria as a guide:
- **Cold-start sensitivity?** WASM wins if requests must be served in <10ms from a cold state
- **Untrusted code?** WASM wins if you cannot trust the code (plugin, user-submitted function)
- **Long-running jobs (>30s)?** Containers win — WASM has no benefit over long durations
- **GPU / native hardware?** Containers win — WASM has no GPU access path today
- **Existing ecosystem (DBs, messaging)?** Containers win — WASM database client libraries are immature
- **High tenant count?** WASM wins — lower per-instance overhead means more tenants per host
- **Multi-language requirement?** WASM is language-agnostic; both options support polyglot

Work in pairs or small groups. For each scenario, decide:
- **Container**, **WASM**, or **Hybrid**? 
- Justify your choice using the criteria from Module 3.

### Scenarios

**Scenario A:** An e-commerce platform runs checkout discount logic for 50,000 stores. Each store has custom rules. The rules change frequently. Average function duration: 2ms.

> *Hint: Think about trust (merchant-authored code), tenant count, and cold-start frequency.*

**Scenario B:** A social media platform needs to run user-submitted video transcoding jobs. Jobs can run for minutes. They need GPU access.

> *Hint: Check the GPU criterion above. WASM has no GPU access path today.*

**Scenario C:** A CDN wants to run A/B testing logic at 300 PoPs globally. Each request needs 1–5ms of logic. Cold starts happen on every PoP for every new deployment.

> *Hint: 300 PoPs means 300 cold starts per deployment. What's the cold-start cost in each model?*

**Scenario D:** A FinTech company processes payment events from a Kafka topic. Processing takes 50–500ms per event. It requires a PostgreSQL connection.

> *Hint: PostgreSQL client libraries and Kafka consumer groups are mature in containers. What's the WASM ecosystem story here?*

**Scenario E:** A SaaS platform allows customers to write custom data transformation functions in Python or JavaScript. Functions process uploaded files. Strong isolation is required.

> *Hint: Python and JavaScript can compile to WASM (via Wizer/Javy). The "strong isolation" requirement is the key discriminator here.*

---

## ✅ Module 4 Checklist

- [ ] Built and ran a WASM HTTP service with Spin
- [ ] Understood the plugin/extension WASM pattern
- [ ] Simulated the FaaS cold-start model
- [ ] Reviewed K8s WASM deployment manifests
- [ ] Applied the decision framework to real scenarios

---

## Key Takeaways

1. Fermyon Spin makes building WASM microservices as easy as writing a function
2. The plugin pattern enables **safe, multi-tenant extensibility** in any language
3. WASM FaaS eliminates cold-start penalties — ideal for event-driven serverless
4. K8s + runwasi makes WASM workloads a first-class scheduling primitive
5. WASM and containers are **complementary**, not mutually exclusive
