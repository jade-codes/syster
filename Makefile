.PHONY: help build run test clean fmt lint check run-guidelines watch install lint-test-naming run-frontend-guidelines

# Default target
help:
	@echo "Available targets:"
	@echo "  build          - Build the project"
	@echo "  run            - Run the project"
	@echo "  test           - Run tests"
	@echo "  clean          - Clean build artifacts"
	@echo "  fmt            - Format code with rustfmt"
	@echo "  lint           - Run clippy linter"
	@echo "  lint-test-naming - Check test file naming convention"
	@echo "  check          - Run fmt + lint + test"
	@echo "  run-guidelines - Run complete validation (fmt + lint + build + test)"
	@echo "  run-frontend-guidelines - Run frontend validation (typecheck + lint + test + build)"
	@echo "  watch          - Watch and rebuild on changes"
	@echo "  install        - Install the binary"

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
	cargo clippy --all-targets -- -D warnings

# Run all checks (format, lint, test)
check: fmt-check lint test

# Run complete validation pipeline (format, lint, build, test)
# Optimized: clippy already builds, so we skip separate build step
run-guidelines: lint-test-naming
	@echo "=== Running Complete Validation Pipeline ==="
	@echo ""
	@echo "Step 1/3: Formatting code..."
	@cargo fmt
	@echo "✓ Code formatted"
	@echo ""
	@echo "Step 2/3: Running linter (includes build)..."
	@cargo clippy --all-targets -- -D warnings
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

# Lint test file naming convention
# - Test files must be in tests/ directories
# - Test files must have tests_ prefix
lint-test-naming:
	@echo "Checking test file naming conventions..."
	@errors=0; \
	bad_pattern=$$(find crates -name "*_test.rs" -o -name "test_*.rs" 2>/dev/null | grep -v target); \
	if [ -n "$$bad_pattern" ]; then \
		echo "❌ Found test files with old naming pattern (*_test.rs or test_*.rs):"; \
		echo "$$bad_pattern" | sed 's/^/  - /'; \
		errors=1; \
	fi; \
	bad_prefix=$$(find crates -path "*/tests/*.rs" -type f 2>/dev/null | grep -v target | grep -v "mod.rs" | grep -v "/tests_"); \
	if [ -n "$$bad_prefix" ]; then \
		echo "❌ Found test files in tests/ without 'tests_' prefix:"; \
		echo "$$bad_prefix" | sed 's/^/  - /'; \
		errors=1; \
	fi; \
	if [ $$errors -eq 1 ]; then \
		echo ""; \
		echo "Rename to tests_*.rs"; \
		exit 1; \
	fi
	@echo "✓ All test files follow naming convention"

# Run frontend validation pipeline (matches ci-frontend.yml)
run-frontend-guidelines:
	@echo "=== Running Frontend Validation Pipeline ==="
	@echo ""
	@echo "Step 1/4: Type checking packages..."
	@for package in packages/*/; do \
		if [ -f "$${package}tsconfig.json" ]; then \
			echo "  Type checking $$package"; \
			(cd "$$package" && bunx tsc --noEmit) || exit 1; \
		fi; \
	done
	@echo "✓ Type check passed"
	@echo ""
	@echo "Step 2/4: Linting packages..."
	@for package in packages/*/; do \
		if [ -f "$${package}package.json" ]; then \
			if grep -q '"lint"[[:space:]]*:' "$${package}package.json"; then \
				echo "  Linting $$package"; \
				(cd "$$package" && bun run lint) || exit 1; \
			fi; \
		fi; \
	done
	@echo "✓ Linting passed"
	@echo ""
	@echo "Step 3/4: Running tests..."
	@for package in packages/*/; do \
		if [ -f "$${package}package.json" ]; then \
			if grep -q '"test"[[:space:]]*:' "$${package}package.json"; then \
				echo "  Testing $$package"; \
				(cd "$$package" && bun test) || exit 1; \
			fi; \
		fi; \
	done
	@echo "✓ Tests passed"
	@echo ""
	@echo "Step 4/4: Building packages..."
	@for package in packages/*/; do \
		if [ -f "$${package}package.json" ]; then \
			if grep -q '"build"[[:space:]]*:' "$${package}package.json"; then \
				echo "  Building $$package"; \
				(cd "$$package" && bun run build) || exit 1; \
			fi; \
		fi; \
	done
	@echo "✓ Build passed"
	@echo ""
	@echo "=== ✓ All frontend guidelines passed! ==="
