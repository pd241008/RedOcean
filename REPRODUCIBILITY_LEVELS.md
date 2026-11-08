# 🏷️ Reproducibility Levels (P4: Typed Reproducibility)

> Every numerical claim in RedOcean is explicitly tagged on the **R1--R4 scale** to guarantee appropriate reviewer expectations.

---

## 척 The R1--R4 Scale

| Level | Type | Meaning | Pre-registered Acceptance Window | RedOcean Claims |
| :--- | :--- | :--- | :--- | :--- |
| **R1** | **Exact Byte-Identical** | Hash comparison against archived output | Hash equality ($\Delta = 0$) | **Claim C2** (80-byte budget)<br>**Claim C3** (BAN logic proof)<br>**Claim C6** (Lean 4 proof check) |
| **R2** | **Deterministic Numerical** | Exact closed-form equation output; numerical comparison | Relative error $< 0.1\%$ | **Claim C1** (Decibel slopes)<br>**Claim C4** (Energy reduction) |
| **R3** | **Statistical / Tolerance-Based** | Stochastic Monte Carlo output with stated acceptance window | Stated tolerance: $\pm 1.5\%$ relative | **Claim C5** (Optical cliff PER survival) |
| **R4** | **Archival Verification** | Re-derive summary metrics from archived canonical JSON | Identical summary statistics | Manifest comparisons in `results/canonical/` |

