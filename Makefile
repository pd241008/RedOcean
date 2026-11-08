# ═══════════════════════════════════════════════════════════════
# RedOcean Unified Build & Reproducibility Makefile
# Follows Design Dungeons Engineering Standards
# ═══════════════════════════════════════════════════════════════

.PHONY: all proto build test verify smoke clean paper

all: proto build test verify

# Compile Protocol Buffers IPC schemas
proto:
	@echo "📦 Compiling Protocol Buffers boundary schemas..."
	protoc --proto_path=proto --python_out=py-redocean proto/redocean_events.proto
	@echo "✅ Protobuf schemas compiled into py-redocean/"

# Build Rust simulation core
build: proto
	@echo "🦀 Building Rust core simulation engine..."
	cd crates/redocean-core && cargo build --release

# Run all unit tests and formal proof validations
test:
	@echo "🧪 Running Rust unit tests..."
	cd crates/redocean-core && cargo test
	@echo "🧪 Running Lean 4 mechanized proofs..."
	lean lean-correctness/RedOcean.lean
	@echo "🧪 Running Python energy profiling tests..."
	python3 py-redocean/eval_energy.py
	@echo "✅ All tests passed."

# Fast verification path (System Audit: < 2 minutes)
verify: test
	@echo "⚡ Running R statistical validation..."
	Rscript r-analytics/analytics.R results/canonical/table2_metrics.json
	@echo "✅ All specifications C1-C6 successfully verified!"

# 15-Minute Sanity Smoke Test
smoke:
	@echo "💨 Executing RedOcean smoke test..."
	cd crates/redocean-core && cargo run -- 42
	lean lean-correctness/RedOcean.lean
	python3 py-redocean/eval_energy.py
	@echo "✅ Smoke test completed successfully."

# Local manuscript compilation (Optional, local only)
paper:
	@if [ -d "artifact" ]; then \
		echo "📄 Compiling local LaTeX manuscript..."; \
		cd artifact && pdflatex -interaction=nonstopmode IEEE_Paper_Underwater_Optical_Authentication.tex > /dev/null && \
		pdflatex -interaction=nonstopmode IEEE_Paper_Underwater_Optical_Authentication.tex > /dev/null && \
		echo "✅ Generated artifact/IEEE_Paper_Underwater_Optical_Authentication.pdf"; \
	else \
		echo "ℹ️ artifact/ directory is local-only; skipping paper compilation."; \
	fi

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cd crates/redocean-core && cargo clean
	rm -rf py-redocean/__pycache__ py-redocean/*_pb2.py
	rm -f results/*.fresh.* results/fresh_run.log
	@echo "✅ Cleanup complete."
