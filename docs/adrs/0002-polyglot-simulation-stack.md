# 📜 ADR-002: Polyglot Simulation and Verification Architecture

> **Status:** `Decided`  
> **Date:** `November 2026`

---

## 🌎 Context
Simulating underwater optical networks with coupled physical-layer channel dynamics, embedded cryptographic execution, and runtime formal verification requires disparate performance characteristics:
1. Microsecond discrete-event priority scheduling and SIMD vector math.
2. Fast multi-language zero-copy data interchange.
3. Automated parametric sweeps over massive configuration spaces.
4. Rigorous statistical computing (ECDF, Kaplan-Meier survival curves).
5. Interactive theorem proving for runtime trace correctness.

## 🛤️ Options Considered
1. **Monolithic C++ (NS-3 / OMNeT++):** Heavy, slow compilation, difficult to embed formal verification or modern cryptographic primitives safely.
2. **Pure Python Simulator:** Simple, but garbage-collected jitter and single-threaded GIL render microsecond optical scheduling and large-scale Monte Carlo sweeps prohibitively slow.
3. **Polyglot Monorepo (Rust + Protobuf + Python + R + Lean 4):** Strict domain decoupling with language-appropriate strengths.

---

## 🎯 Decision
> [!IMPORTANT]
> **We adopt a 5-tier polyglot architecture: Rust (`crates/redocean-core`) for event orchestration, Protocol Buffers (`proto/`) for boundary contracts, Python (`py-redocean`) for parametric sweeps, R (`r-analytics`) for statistical analysis, and Lean 4 (`lean-correctness`) for on-the-fly mechanized proofs.**

## 🧠 Reasoning
- Rust delivers C++ performance with guaranteed memory safety and zero runtime GC pauses.
- Protocol Buffers eliminate struct padding and endianness discrepancies across language boundaries.
- Python offers rapid batch scenario execution.
- R provides standard implementations of non-parametric survival estimators.
- Lean 4 verifies runtime state safety invariants on-the-fly.

## ⚖️ Consequences
- **Good:** 🟢 Each subsystem executes in its optimal runtime; formal proofs gate simulation commits.
- **Bad:** 🔴 Developers require multi-toolchain setups (cargo, python, R, lean, protoc).

## 🔄 Revisit When
If a single runtime evolves that natively supports both high-throughput SIMD scheduling and mechanized dependent type theory.

