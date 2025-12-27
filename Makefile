.PHONY: help build run test clean fmt lint check run-guidelines watch install

# Default target
help:
	@echo "Available targets:"
	@echo "  build         - Build the project"
	@echo "  run           - Run the project"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  fmt           - Format code with rustfmt"
	@echo "  lint          - Run clippy linter"
	@echo "  check         - Run fmt + lint + test"
	@echo "  run-guidelines - Run complete validation (fmt + lint + build + test)"
	@echo "  watch         - Watch and rebuild on changes"
	@echo "  install       - Install the binary"

# Build the project
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Check formatting without applying
fmt-check:
	cargo fmt -- --check

# Run clippy linter
lint:
	cargo clippy -- -D warnings

# Run all checks (format, lint, test)
check: fmt-check lint test

# Run complete validation pipeline (format, lint, build, test)
# Optimized: clippy already builds, so we skip separate build step
run-guidelines:
	@echo "=== Running Complete Validation Pipeline ==="
	@echo ""
	@echo "Step 1/3: Formatting code..."
	@cargo fmt
	@echo "✓ Code formatted"
	@echo ""
	@echo "Step 2/3: Running linter (includes build)..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✓ Linting passed"
	@echo ""
	@echo "Step 3/3: Running tests..."
	@cargo test --lib -- --test-threads=4
	@cargo test --test '*' -- --test-threads=4
	@cargo test --doc
	@echo ""
	@echo "=== ✓ All guidelines passed! ==="

# Fast check - just clippy + lib tests (skip integration/doc tests)
run-guidelines-fast:
	@echo "=== Fast Validation ==="
	@cargo fmt
	@cargo clippy --all-targets -- -D warnings
	@cargo test --lib -- --test-threads=4
	@echo "=== ✓ Fast check passed! ==="

# Super fast - clippy + unit tests only (skip slow stdlib tests)
run-guidelines-quick:
	@echo "=== Quick Validation (skipping stdlib tests) ==="
	@cargo fmt
	@cargo clippy --all-targets -- -D warnings
	@cargo test --lib -- --test-threads=4 --skip stdlib
	@echo "=== ✓ Quick check passed! ==="

# Full clean validation (use when you need a fresh build)
run-guidelines-clean:
	@echo "=== Running Complete Validation Pipeline (Clean) ==="
	@cargo clean
	@$(MAKE) run-guidelines

# Watch for changes and rebuild
watch:
	cargo watch -x build

# Install the binary
install:
	cargo install --path .
