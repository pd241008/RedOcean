# 📜 ADR-004: Adopting NIST ASCON-128a Sponge Primitive for LO-AKA

> **Status:** `Decided`  
> **Date:** `November 2026`

---

## 🌎 Context
Benthic optical sensor nodes run on energy-constrained 16-bit MSP430 or 32-bit ARM Cortex-M microcontrollers. Standard terrestrial TLS 1.3 / DTLS 1.3 handshakes rely on ECDHE-P256 public-key operations (consuming >2,000 uJ per handshake) and require multi-kilobyte frames (>1.2 KB) that suffer catastrophic packet error rates ($PER > 0.99$) under steep underwater decibel losses.

## 🛤️ Options Considered
1. **DTLS 1.3 over UDP (ECDHE-P256 + AES-GCM):** Industry standard, but bloated packet sizes cause total handshake dropouts in turbid water.
2. **AES-128-CCM / HMAC-SHA256:** Moderate performance, but requires multiple distinct cryptographic primitives with larger code footprints.
3. **NIST ASCON-128a Lightweight Sponge:** Single 320-bit state permutation providing AEAD, hashing, and key derivation in a unified primitive.

---

## 🎯 Decision
> [!IMPORTANT]
> **We build the LO-AKA protocol around NIST ASCON-128a, achieving mutual authentication and session key derivation in exactly three flights totaling 80 bytes.**

## 🧠 Reasoning
- ASCON-128a uses lightweight bitwise shifts and Boolean operations that execute efficiently on both 16-bit and 32-bit architectures.
- Consumes only 4.21 uJ on ARM Cortex-M4 (a 99.8% energy savings over DTLS-ECDHE).
- The 80-byte footprint compresses the optical transmission window, mitigating the "optical cliff" effect and boosting handshake completion probability from 1.5% to 91.0% in 15m coastal waters.

## ⚖️ Consequences
- **Good:** 🟢 Drastic energy and transmission savings; identity unobservability via pseudo-identities.
- **Bad:** 🔴 Requires symmetric pre-shared master keys initialized at node deployment time.

## 🔄 Revisit When
When post-quantum public-key primitives achieve sub-100-byte handshakes and sub-10 uJ energy envelopes on microcontroller silicon.

