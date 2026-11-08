# 📜 ADR-001: Record Architecture Decisions

> **Status:** `Decided`  
> **Date:** `November 2026`

---

## 🌎 Context
In building the RedOcean framework for Underwater Optical Wireless Communication (UOWC) and lightweight authentication, we face multidimensional design tradeoffs across optical physics, embedded cryptography, and multi-language verification. Future maintainers, reviewers, and collaborators must understand the foundational rationale behind architectural choices without having to reverse-engineer git commits or paper prose.

## 🛤️ Options Considered
1. **Implicit Decisions in Code Comments:** Fast initially, but scatters design rationale across repositories and files.
2. **Monolithic Design Document:** Hard to track revisions as discrete milestones evolve.
3. **Architecture Decision Records (ADRs):** Lightweight, version-controlled markdown records adopting the Design Dungeons standard.

---

## 🎯 Decision
> [!IMPORTANT]
> **We will record all foundational architectural decisions as ADRs in `docs/adrs/` using the Design Dungeons template.**

## 🧠 Reasoning
ADRs provide clear traceability between scientific claims in our IEEE/ACM Transactions on Networking manuscript and the physical code in `crates/`, `proto/`, `py-redocean/`, `r-analytics/`, and `lean-correctness/`.

## ⚖️ Consequences
- **Good:** 🟢 Every design choice has an immutable problem statement, options matrix, and tradeoffs.
- **Bad:** 🔴 Requires discipline to author an ADR before committing architectural refactors.

## 🔄 Revisit When
When transitioning from theoretical simulation to physical sea-trial deployments.

