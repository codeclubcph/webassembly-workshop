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

## Background Reading

See [slides/04-cloud-native.md](../slides/04-cloud-native.md)

---

## Lab 4A – HTTP Microservice with Fermyon Spin

**Goal:** Build a WASM-based HTTP microservice that responds to requests.

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

> Notice: there is no meaningful startup delay even for the very first request.

---

## Lab 4B – WASM as a Plugin System

**Goal:** Build a host application that loads user-supplied WASM "plugin" modules.

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

---

## Lab 4C – Simulating Serverless: Cold Start vs Warm Request

**Goal:** Simulate the FaaS (Function-as-a-Service) execution model with WASM.

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

---

## Lab 4D – Kubernetes WASM Manifest (Discussion Lab)

**Goal:** Understand how WASM workloads deploy on Kubernetes.

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
- `spec.template.spec.runtimeClassName: wasmtime` → uses WASM shim
- Image is a standard OCI image but contains `.wasm` instead of ELF binaries
- Very low resource requests (`memory: "16Mi"`, `cpu: "50m"`)
- Annotation `module.wasm.image/variant: compat-smart`

### Discussion Questions

1. What changes would your CI/CD pipeline need to ship WASM to Kubernetes?
2. How would you handle a WASM module that needs database access?
3. What observability challenges arise with WASM vs traditional containers?
4. When would you run WASM *inside* a container vs as a replacement?

---

## Lab 4E – Architecture Decision Exercise (Group Activity)

**Goal:** Apply the decision framework to real scenarios.

Work in pairs or small groups. For each scenario, decide:
- **Container**, **WASM**, or **Hybrid**? 
- Justify your choice using the criteria from Module 3.

### Scenarios

**Scenario A:** An e-commerce platform runs checkout discount logic for 50,000 stores. Each store has custom rules. The rules change frequently. Average function duration: 2ms.

**Scenario B:** A social media platform needs to run user-submitted video transcoding jobs. Jobs can run for minutes. They need GPU access.

**Scenario C:** A CDN wants to run A/B testing logic at 300 PoPs globally. Each request needs 1–5ms of logic. Cold starts happen on every PoP for every new deployment.

**Scenario D:** A FinTech company processes payment events from a Kafka topic. Processing takes 50–500ms per event. It requires a PostgreSQL connection.

**Scenario E:** A SaaS platform allows customers to write custom data transformation functions in Python or JavaScript. Functions process uploaded files. Strong isolation is required.

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
