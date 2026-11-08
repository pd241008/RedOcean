# 🗺️ Specification & Guarantee Map (P1: Navigable Guarantees)

> **Traceability Policy:** Every numerical metric, physical channel model, and security guarantee in RedOcean traces directly to a code entity, unit test, or mechanized mathematical proof.

---

## 📑 System Guarantees to Evidence Navigation

| Spec ID | System Specification | Guarantee Level | Manifest / Input | Generating Script | Evidence / Output |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **C1** | **Decibel Path Loss Formulation:** Beer-Lambert extinction, geometric divergence, and pointing jitter combine linearly in decibels ($\text{dB}$) across Jerlov water types. | **R2 (Deterministic Numerical)** | `results/canonical/table2_metrics.json` | `crates/redocean-core/src/channel.rs` | `cargo test test_table_metrics_clear_ocean` |
| **C2** | **LO-AKA 3-Flight Handshake:** Authenticated key exchange completes in exactly 3 flights totaling 80 bytes (Flight 1: 24B, Flight 2: 36B, Flight 3: 20B). | **R1 (Exact Byte-Identical)** | Protocol Spec | `crates/redocean-core/src/protocol.rs` | `cargo test test_handshake_byte_budget` |
| **C3** | **Formal Logic Key Secrecy:** Benthic sensor and gateway achieve mutual belief and session key consensus without plaintext identity leakage. | **R1 (Exact Byte-Identical)** | Logic Annotations | Protocol Logic Model | Formal BAN Logic Verification |
| **C4** | **Embedded Energy Savings:** LO-AKA cuts cryptographic computational energy by $99.8\%$ and transmission footprint by $95.1\%$ relative to DTLS-ECDHE. | **R2 (Deterministic Numerical)** | `results/canonical/energy_benchmark.json` | `py-redocean/eval_energy.py` | `python3 py-redocean/eval_energy.py` |
| **C5** | **Optical Cliff Mitigation:** At $15\text{ m}$ in coastal waters, compact framing sustains reliable delivery under steep extinction. | **R3 (Tolerance: $\pm 1.5\%$)** | `results/canonical/table2_metrics.json` | `crates/redocean-core/src/channel.rs` | `cargo test test_per_optical_cliff` |
| **C6** | **On-the-Fly Mechanized Safety:** Ephemeral session keys and nonces are formally verified to never collide or leak under active optical perturbations. | **R1 (Exact Byte-Identical)** | `lean-correctness/RedOcean.lean` | Lean 4 Theorem Prover | `lean lean-correctness/RedOcean.lean` |

---

## 🔗 Specification Dependency Graph

```
C1 (Physical Decibel Channel Loss)
    └── motivates C2 (LO-AKA 80-byte Handshake)
        ├── proven by C3 (Logic Key Consensus)
        ├── benchmarked by C4 (99.8% Microcontroller Energy Savings)
        ├── validated by C5 (Optical Cliff Mitigation in Coastal Waters)
        └── verified on-the-fly by C6 (Lean 4 Runtime Proofs)
```
