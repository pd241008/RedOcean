# 📜 ADR-003: Closed-Form Optical Channel Loss Formulations in Decibels

> **Status:** `Decided`  
> **Date:** `November 2026`

---

## 🌎 Context
In underwater optical wireless systems, path attenuation follows Beer-Lambert exponential extinction ($e^{-c d}$), while optical geometric divergence scales quadratically with distance ($d^2$). Modeling these cascades in linear units introduces floating-point underflow at ranges beyond 20–30 meters, especially when cascaded with subsea transoceanic cable backhauls (65 km spans).

## 🛤️ Options Considered
1. **Linear Transmittance Scaling ($h = h_{\text{ext}} \cdot h_{\text{geom}} \cdot h_p \cdot I$):** Suffers from severe precision loss and rounding errors across wide dynamic ranges.
2. **Logarithmic Decibel Scale ($PL_{\text{dB}} = A_{\text{ext, dB}} + A_{\text{geom, dB}} + A_{p, \text{dB}} + A_{\text{turb, dB}}$):** Turns multiplicative cascaded loss terms into exact linear additions.

---

## 🎯 Decision
> [!IMPORTANT]
> **All optical attenuation, pointing jitter, turbulence fading margins, and received powers are strictly formulated and computed in decibels (dB) and decibel-milliwatts (dBm).**

## 🧠 Reasoning
- Beer-Lambert becomes linear: $A_{\text{ext, dB}} = 4.34294 \cdot c \cdot d$.
- Subsea optical cable losses (0.154 dB/km, connector insertion loss) sum directly into link budgets.
- Electrical SNR maps directly to OOK Bit Error Rates without intermediate subnormal floating-point operations.

## ⚖️ Consequences
- **Good:** 🟢 Eliminates floating-point underflow; aligns with IEEE and ITU-T telecommunication standards.
- **Bad:** 🔴 Requires log-normal and Q-function evaluations when mapping SNR back to Bit Error Rates ($P_b$).

## 🔄 Revisit When
Never. Decibel scaling is standard across optical and marine communications engineering.

