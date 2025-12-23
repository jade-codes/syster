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
run-guidelines:
	@echo "=== Running Complete Validation Pipeline ==="
	@echo ""
	@echo "Step 1/4: Formatting code..."
	@cargo fmt
	@echo "✓ Code formatted"
	@echo ""
	@echo "Step 2/4: Running linter..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✓ Linting passed"
	@echo ""
	@echo "Step 3/4: Building project..."
	@cargo build
	@echo "✓ Build successful"
	@echo ""
	@echo "Step 4/4: Running tests..."
	@cargo test
	@echo ""
	@echo "=== ✓ All guidelines passed! ==="

# Watch for changes and rebuild
watch:
	cargo watch -x build

# Install the binary
install:
	cargo install --path .
