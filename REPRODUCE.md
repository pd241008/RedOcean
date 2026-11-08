# 🔁 Full Reproduction Guide (REPRODUCE.md)

> This guide documents the end-to-end pipeline to regenerate all intermediate telemetry, protocol traces, and statistical outputs from scratch.

---

## 🛠️ Prerequisites

Ensure the following toolchains are installed:
- **Rust:** `cargo 1.80+` / `rustc 1.80+`
- **Protocol Buffers:** `protoc 3.20+`
- **Python:** `python 3.10+` with `pip`
- **R:** `R 4.2+` with `Rscript`
- **Lean 4:** `lean 4.35+` / `lake`
- **LaTeX:** `pdflatex` (TeX Live 2024+)

---

## 🏃 Step-by-Step Reproduction Pipeline

### Step 1: Recompile Protocol Buffers Boundary Contracts
```bash
make proto
```
Generates Python bindings into `py-redocean/redocean_events_pb2.py`.

### Step 2: Build and Run Core Rust Simulation Engine
```bash
cd crates/redocean-core
cargo build --release
cargo test
cargo run --release -- 42 > ../../results/fresh_run.log
cd ../..
```

### Step 3: Run Embedded Energy Verification
```bash
python3 py-redocean/eval_energy.py
```

### Step 4: Run R Statistical Engine
```bash
Rscript r-analytics/analytics.R results/canonical/table2_metrics.json
```

### Step 5: Verify Lean 4 Safety Proofs
```bash
lean lean-correctness/RedOcean.lean
```

### Step 6: Compile Manuscript
```bash
cd artifact
pdflatex -interaction=nonstopmode IEEE_Paper_Underwater_Optical_Authentication.tex
pdflatex -interaction=nonstopmode IEEE_Paper_Underwater_Optical_Authentication.tex
```
Output: `artifact/IEEE_Paper_Underwater_Optical_Authentication.pdf` (12 pages).

