# ⚠️ Known Limitations & Boundaries (P5: Upfront Disclosure)

> **Honesty-First Policy (Design Dungeons):** Clearly articulating where our model holds and where physical boundaries lie prevents overselling and false equivalencies.

---

## 🌊 Physical & Mathematical Boundaries

1. **Empirical Execution Deferred:**
   - In accordance with safety-critical marine engineering practices, stochastic empirical executions are formally deferred to subsequent hardware-in-the-loop sea trials. Current metrics reflect closed-form mathematical derivations and discrete-event simulation models.
   - No empirical sea-trial graphs have been fabricated.

2. **Optical Channel Assumptions:**
   - **Turbulence Validity Range:** The log-normal fading distribution model is valid under weak-to-moderate scintillation ($\sigma_I^2 \le 0.40$) and link distances $d \le 30\text{ m}$. For strong oceanic turbulence regimes ($d > 50\text{ m}$), Gamma-Gamma or $K$-distribution formulations must be substituted.
   - **Line-of-Sight Alignment:** The mathematical pointing jitter model assumes a small-angle radial Rayleigh displacement. Catastrophic pointing loss occurs if AUV drift exceeds transceiver field-of-view ($\theta_e > \theta_{div} / 2$).

3. **Modulation Scheme:**
   - Closed-form Bit Error Rate expressions are derived assuming intensity modulation with direct detection (IM/DD) On-Off Keying (OOK) and adaptive thresholding. Coherent modulation (e.g., QPSK/QAM) would alter the SNR conversion curve.

4. **Cryptographic Assumptions:**
   - LO-AKA assumes initial pre-shared master keys ($K_i$) are securely provisioned onto benthic sensor nodes prior to physical deployment.
   - Perfect Forward Secrecy (PFS) is guaranteed only for sessions completed prior to physical node capture and non-volatile memory extraction.

