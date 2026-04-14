# Slides – Session 00: Welcome & Introduction
**Duration:** 30 minutes

---

## Slide 1 – Welcome

- Who we are: The Better Software Initiative
- Today's goal: understand and use WASM in cloud-native contexts
- Agenda walkthrough
- Introduce yourself (1 sentence each): role, tech stack, what brings you here

---

## Slide 2 – What is WebAssembly?

> "WebAssembly (WASM) is a binary instruction format for a stack-based virtual machine."  
> — W3C Specification

- Originally designed for the **browser** (2017, W3C standard)
- Now **escaping the browser**: server-side, edge, embedded, cloud-native
- Runs on a portable, sandboxed virtual machine
- Near-native performance from **any source language**

**Key insight:** WASM ≠ JavaScript replacement. WASM = universal, safe bytecode.

---

## Slide 3 – Why Now?

Timeline of key milestones:

| Year | Event |
|------|-------|
| 2017 | WASM becomes W3C standard |
| 2019 | WASI (WebAssembly System Interface) announced |
| 2020 | Docker co-founder: "If WASI had existed in 2008, we wouldn't have needed Docker" |
| 2022 | WASM Component Model draft |
| 2023 | Wasmtime 1.0; CNCF WASM working group |
| 2024 | WasmCloud, Fermyon, Fastly production deployments at scale |
| 2025+ | Major cloud providers add native WASM runtimes |

---

## Slide 4 – WASM vs Containers (teaser)

| Dimension | Containers | WebAssembly |
|-----------|-----------|-------------|
| Startup time | ~100ms–1s | **<1ms** |
| Image size | 10s–100s MB | **100s KB–few MB** |
| Security boundary | Linux namespaces + cgroups | **Capability-based sandbox** |
| Portability | Per-OS image layer | **Single binary, runs anywhere** |
| Memory isolation | Process-level | **Linear memory, strict** |

*(We will explore this in depth in Module 3)*

---

## Slide 5 – WASM's Place in the Cloud-Native Stack

```
┌──────────────────────────────────────────────────────┐
│  Developer Code (Rust / C / Go / Python / JS / ...)  │
└─────────────────────────┬────────────────────────────┘
                          │ compile
                          ▼
               ┌─────────────────────┐
               │   .wasm binary      │  ← portable, sandboxed
               └──────────┬──────────┘
                          │ run via runtime
          ┌───────────────┼────────────────────┐
          ▼               ▼                    ▼
    Wasmtime         WasmEdge            Browser
    (server)         (edge/k8s)          (JS host)
          │               │
          ▼               ▼
   Microservices    Cloudflare Workers
   Serverless       Fastly Compute
   Kubernetes       AWS Lambda (WASM)
```

---

## Slide 6 – Workshop Logistics

- 4 modules + hands-on labs
- Ask questions any time (raise hand or chat)
- Lab solutions available in `*/labs/solution/`
- Take breaks as needed
- Feedback form at the end

**Let's go! 🚀**
