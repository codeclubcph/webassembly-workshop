# Instructor Guide
**WebAssembly in Cloud-Native Environments – 4-Hour Workshop**

> Internal document for workshop facilitators. Not distributed to participants.

---

## Timing Breakdown

```
00:00 – 00:30  Welcome, setup check, intro slides          (30 min)
00:30 – 01:15  Module 1: Runtime Essentials + Labs         (45 min)
01:15 – 02:00  Module 2: First WASM App + Labs             (45 min)
02:00 – 02:10  ☕ Break                                    (10 min)
02:10 – 02:55  Module 3: Performance & Security + Labs     (45 min)
02:55 – 03:45  Module 4: Cloud-Native Patterns + Labs      (50 min)
03:45 – 04:00  Wrap-up, Q&A, resources                     (15 min)
```

---

## Pre-Workshop Checklist (Day Before)

- [ ] Send participants the setup/README.md at least 24h in advance
- [ ] Verify your own setup with `./setup/verify-setup.sh`
- [ ] Run `./build-all.sh` and confirm all labs build cleanly
- [ ] Test each `wasmtime` invocation in Module 1–3 labs
- [ ] Test `spin up` in Module 4 lab 4A
- [ ] Prepare a backup: pre-compiled `.wasm` files for participants who can't build
- [ ] Check Docker is running (needed for Module 3 startup comparison)

---

## Common Participant Issues & Fixes

| Issue | Fix |
|-------|-----|
| `wasmtime: command not found` | `source ~/.bashrc` or restart terminal; check `~/.wasmtime/bin` in PATH |
| `wasm32-wasip1` target missing | `rustup target add wasm32-wasip1` |
| `cargo build` fails on wasm target | Check Rust edition is 2021+; update `rustup update` |
| Module 3 Docker fails | Ensure Docker Desktop is running |
| Spin command not found | `export PATH="$HOME/.fermyon/bin:$PATH"` |
| Lab 2C: `wasmtime` version mismatch | `Cargo.toml` pins `wasmtime = "25"`; update if needed |

---

## Facilitation Notes

### Module 1
- Spend extra time on the **stack machine animation** (slide 2) — most participants are unfamiliar
- The WAT syntax surprises people; emphasize it's a learning tool, not production code
- **Lab 1A**: Most impactful lab of the module. Walk through it live if needed.
- Lab 1C factorial challenge: give ~5 min then show the solution — don't let it block progress

### Module 2
- **Lab 2B** is the security "aha moment" — make sure everyone runs the denied path
- Lab 2C embedding is more complex; have participants read the host code comments carefully
- If time is short, Lab 2D is skippable

### Module 3
- **Lab 3A** (startup benchmark) is the most impactful demo — do it live projected
- The security sandbox demo (Lab 3C) often generates the most discussion
- Lab 3D isolation: if participants are ahead, run this. Otherwise use it as a reference.

### Module 4
- **Lab 4A (Spin)** requires internet for `spin new`. Provide a pre-scaffolded template if offline.
- Lab 4B plugin system: explain the memory-passing ABI before running — it's non-trivial
- **Lab 4E (Architecture Exercise)** can be run as a group discussion if time is short
- The K8s manifests (Lab 4D) are discussion-only — no live cluster needed

---

## Backup: Running Without Cargo/Rust

If participants don't have a working Rust toolchain, pre-built `.wasm` files are sufficient for all CLI labs. Provide them via a shared drive or USB:

```
prebuilt-wasm/
├── hello.wasm            (module 1)
├── memory.wasm           (module 1)
├── calc.wasm             (module 1)
├── lab-2a-hello.wasm     (module 2)
├── lab-2b-fileio.wasm    (module 2)
├── lab-3a-startup.wasm   (module 3)
├── lab-3b-benchmark.wasm (module 3)
└── lab-3c-security.wasm  (module 3)
```

---

## Key Messages to Reinforce

1. **WASM is production-ready today** — not an experiment (Cloudflare, Fastly, Shopify)
2. **Security is structural, not configurational** — this is the key differentiator
3. **WASM and containers are complementary** — right tool for the right job
4. **The ecosystem is moving fast** — WASI 0.2, Component Model, WasmGC are all recent

---