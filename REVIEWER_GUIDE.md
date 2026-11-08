# 🧭 RedOcean Verification & Architecture Guide

> **Framework:** RedOcean UOWC Simulation and Formal Verification Engine  
> **Discipline:** Design Dungeons Canonical Systems Monorepo

---

## ⚡ Quick Navigation

If you have a limited time budget, choose the review path that fits:

| Budget | Path | Target Action | Verification Script |
| :--- | :--- | :--- | :--- |
| **15 Minutes** | **Sanity Path** | Verify environment, check toolchains, compile Lean 4 proofs, smoke-test core Rust channel physics | `make smoke` |
| **30 Minutes** | **System Audit Path** | Verify decibel path loss metrics, embedded energy models, and formal safety guarantees | `make verify` |
| **2 Hours** | **Full Reproduction** | Full Monte Carlo parametric sweeps across Jerlov water regimes and R survival curve generation | `make reproduce` |
| **Ad-Hoc** | **Provenance Path** | Inspect ADRs (`docs/adrs/`), Protobuf contracts (`proto/`), and mathematical derivations | Read `docs/` |

---

## 🗺️ Review Paths

### 1️⃣ 15-Minute Sanity Path
Designed to quickly verify that the environment is intact and that formal proofs hold:
```bash
# 1. Compile and run Rust smoke test
cd crates/redocean-core && cargo run

# 2. Mechanized Lean 4 verification
lean lean-correctness/RedOcean.lean
```

### 2️⃣ 30-Minute System Audit Path
Validates all headline specifications against canonical manifests:
```bash
make verify
```
This target verifies:
- **Specification C1:** Beer-Lambert decibel slopes across Jerlov regimes ($0.6558$ dB/m clear, $1.7285$ dB/m coastal).
- **Specification C2:** LO-AKA 3-flight, 80-byte handshake budget ($24\text{ B} + 36\text{ B} + 20\text{ B}$).
- **Specification C4:** Microcontroller energy reduction ($99.8\%$ computational, $95.1\%$ framing).
- **Specification C6:** Lean 4 safety theorem proving distinct nonces and session key freshness.

### 3️⃣ 2-Hour Full Reproduction Path
Reruns the stochastic channel sweeps and statistical survival modeling:
```bash
make reproduce
```

---

## 📂 Repository Layout at a Glance

```text
RedOcean/
├── crates/redocean-core/   # Rust discrete-event engine & SIMD decibel channel physics
├── proto/                  # Protocol Buffers boundary schemas (redocean_events.proto)
├── py-redocean/            # Python ingestion bridge and parametric sweep harness
├── r-analytics/            # R statistical computing (ECDF, Kaplan-Meier models, ggplot2)
├── lean-correctness/       # Lean 4 mechanized runtime trace formal verification
├── results/                # Canonical manifests, seed logs, and verification fixtures
├── docs/adrs/              # Architecture Decision Records (ADR-001 through ADR-005)
├── Makefile                # Unified build, test, and verification automation
├── README.md               # Project front door and architectural overview
└── CLAIM_MAP.md            # Traceable mapping of system specifications to code evidence
```
