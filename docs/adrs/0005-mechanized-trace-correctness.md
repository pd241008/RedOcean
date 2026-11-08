# 📜 ADR-005: On-The-Fly Mechanized Trace Correctness with Lean 4

> **Status:** `Decided`  
> **Date:** `November 2026`

---

## 🌎 Context
In safety-critical ocean engineering, discrete-event simulation runs can exhibit hidden state desynchronization or replay susceptibility under severe optical packet loss. Conventional post-hoc unit testing only covers sampled execution paths, leaving catastrophic edge cases unverified.

## 🛤️ Options Considered
1. **Ad-Hoc Assertions in Rust:** Fast, but provides no formal mathematical guarantee of correctness.
2. **Offline Prover (ProVerif / Tamarin):** Standard for protocol specification, but decoupled from dynamic runtime simulation traces.
3. **Lean 4 Mechanized Correctness Gate:** Interactive theorem prover executing mechanized safety proofs directly on incoming simulation event traces.

---

## 🎯 Decision
> [!IMPORTANT]
> **We incorporate Lean 4 (`lean-correctness`) as a formal verification gate, proving freshness, distinct nonces, and session key safety invariants on-the-fly.**

## 🧠 Reasoning
Lean 4 compiles to native binary executables, allowing direct integration into the verification pipeline. Mechanized proofs guarantee that nonce collisions and key leakage are mathematically impossible under the operational invariants.

## ⚖️ Consequences
- **Good:** 🟢 Closes the loop between discrete-event simulation output and mathematical certainty.
- **Bad:** 🔴 Requires formal proof maintenance alongside protocol specification changes.

## 🔄 Revisit When
If proof extraction overhead hinders real-time hardware-in-the-loop sea-trial operations.

