# ⚡ Fast Verification Guide (VERIFY.md)

> **Principle P2:** Verification is cheaper than full reproduction. A reviewer can verify the system's specifications in under 2 minutes without executing slow Monte Carlo sweeps.

---

## 🚀 One-Command Verification

Run the unified verification target:
```bash
make verify
```

### What `make verify` Executes:

1. **Rust Core Tests (`cargo test`):**
   - Validates Beer-Lambert decibel slopes: $0.6558$ dB/m (Clear Ocean), $1.7285$ dB/m (Coastal Ocean).
   - Validates geometric divergence at $20\text{ m}$ ($36.96$ dB).
   - Validates LO-AKA 80-byte handshake packing ($24\text{ B} + 36\text{ B} + 20\text{ B}$).
   - Validates discrete-event queue ordering.

2. **Embedded Energy Model (`py-redocean/eval_energy.py`):**
   - Verifies $99.8\%$ cryptographic energy reduction on ARM Cortex-M4 and MSP430.
   - Verifies $95.1\%$ framing transmission footprint reduction.

3. **Lean 4 Mechanized Proofs (`lean-correctness/RedOcean.lean`):**
   - Mechanically proves `lo_aka_safety` theorem: nonces are distinct and session key is uncompromised under valid transitions.
   - Mechanically proves `lo_aka_freshness` theorem: timestamp freshness invariant.

4. **Statistical Distribution Check (`r-analytics/analytics.R`):**
   - Validates canonical manifest integrity and fade margins.
