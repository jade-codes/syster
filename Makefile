.PHONY: help build run test clean fmt lint check run-guidelines watch install install-dev lint-test-naming run-ui-guidelines

# VS Code extension installation paths
VSCODE_EXT_DIR_LINUX = $(HOME)/.vscode-server/extensions
VSCODE_EXT_DIR_MAC = $(HOME)/.vscode/extensions
VSCODE_EXT_DIR_WIN = $(APPDATA)/Code/User/extensions
EXT_NAME = jade-codes.sysml-language-support

# Detect OS and set extension directory
ifeq ($(OS),Windows_NT)
    VSCODE_EXT_DIR = $(VSCODE_EXT_DIR_WIN)
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Darwin)
        VSCODE_EXT_DIR = $(VSCODE_EXT_DIR_MAC)
    else
        # Check if running in devcontainer/remote
        ifneq ($(wildcard $(VSCODE_EXT_DIR_LINUX)),)
            VSCODE_EXT_DIR = $(VSCODE_EXT_DIR_LINUX)
        else
            VSCODE_EXT_DIR = $(HOME)/.vscode/extensions
        endif
    endif
endif

# Default target
help:
	@echo "Available targets:"
	@echo "  build             - Build the project"
	@echo "  run               - Run the project"
	@echo "  test              - Run tests"
	@echo "  clean             - Clean build artifacts"
	@echo "  fmt               - Format code with rustfmt"
	@echo "  lint              - Run clippy linter"
	@echo "  lint-test-naming  - Check test file naming convention"
	@echo "  check             - Run fmt + lint + test"
	@echo "  run-guidelines    - Run complete validation (fmt + lint + build + test)"
	@echo "  run-ui-guidelines - Run ui validation (typecheck + lint + test + build)"
	@echo "  watch             - Watch and rebuild on changes"
	@echo "  install           - Install the binary"
	@echo "  install-dev       - Install VS Code extension for development"

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

# Install the binary and VS Code extension
EDITORS_VSCODE_DIR?=editors/vscode
install:
	@echo "=== Installing Syster ==="
	@echo ""
	@echo "Step 1/3: Building syster-lsp in release mode..."
	@cargo build --release -p syster-lsp
	@echo "✓ syster-lsp built"
	@echo ""
	@echo "Step 2/3: Building and packaging VS Code extension..."
	@(cd ${EDITORS_VSCODE_DIR} && npm install && npm run package)
	@echo "✓ VS Code extension packaged"
	@echo ""
	@echo "Step 3/3: Installing VS Code extension..."
	@vsix_file=$$(ls -t ${EDITORS_VSCODE_DIR}/syster-*.vsix 2>/dev/null | head -1); \
	if [ -z "$$vsix_file" ]; then \
		echo "❌ Error: No .vsix file found"; \
		exit 1; \
	fi; \
	if command -v code >/dev/null 2>&1; then \
		code --install-extension "$$vsix_file" --force && \
		echo "✓ VS Code extension installed"; \
	else \
		echo "⚠ VS Code 'code' command not found. Extension packaged but not installed."; \
		echo "  To install manually, run:"; \
		echo "  code --install-extension $$vsix_file --force"; \
	fi
	@echo ""
	@echo "=== ✓ Installation complete! ==="

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
run-ui-guidelines:
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

# Install VS Code extension for development
# Builds release binaries and copies extension to VS Code extensions folder
install-dev: release
	@echo "=== Installing VS Code Extension for Development ==="
	@echo ""
	@echo "Step 1/3: Building VS Code extension..."
	@cd editors/vscode && npm install && npm run compile
	@echo "✓ Extension built"
	@echo ""
	@echo "Step 2/3: Copying extension to $(VSCODE_EXT_DIR)/$(EXT_NAME)..."
	@mkdir -p "$(VSCODE_EXT_DIR)/$(EXT_NAME)"
	@rm -rf "$(VSCODE_EXT_DIR)/$(EXT_NAME)/*"
	@cp -r editors/vscode/* "$(VSCODE_EXT_DIR)/$(EXT_NAME)/"
	@echo "✓ Extension copied"
	@echo ""
	@echo "Step 3/3: Copying LSP server binary..."
	@mkdir -p "$(VSCODE_EXT_DIR)/$(EXT_NAME)/server"
	@PLATFORM=$$(uname -s | tr '[:upper:]' '[:lower:]'); \
	ARCH=$$(uname -m); \
	if [ "$$ARCH" = "x86_64" ]; then ARCH="x64"; fi; \
	if [ "$$ARCH" = "aarch64" ]; then ARCH="arm64"; fi; \
	BINARY_NAME="syster-lsp-$$PLATFORM-$$ARCH"; \
	cp target/release/syster-lsp "$(VSCODE_EXT_DIR)/$(EXT_NAME)/server/$$BINARY_NAME"; \
	echo "  Copied as $$BINARY_NAME"
	@echo "✓ LSP server copied"
	@echo ""
	@echo "=== ✓ Extension installed! ==="
	@echo "Reload VS Code to activate the extension."
