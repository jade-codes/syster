#!/bin/bash
# Syster Release Workflow
# 
# This script automates the release process for all Syster components.
# Components must be released in dependency order:
#   1. base (syster-base) - Core library, no internal deps
#   2. cli (syster-cli) - Depends on base
#   3. language-server (syster-lsp) - Depends on base
#   4. language-client (sysml-language-support) - Depends on language-server binary
#   5. viewer (syster-viewer) - Standalone VS Code extension
#
# Usage:
#   ./scripts/release.sh <component> <version>
#   ./scripts/release.sh all <version>
#
# Options:
#   --dry-run    Preview changes without applying them
#   --no-push    Make changes locally but don't push to remote
#   --resume     Resume from last checkpoint after failure
#   --force      Skip confirmation prompts
#
# Examples:
#   ./scripts/release.sh base 0.3.2-alpha
#   ./scripts/release.sh cli 0.3.2-alpha --dry-run
#   ./scripts/release.sh all 0.3.2-alpha

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
BACKUP_DIR="$ROOT_DIR/.release-backups"
CHECKPOINT_FILE="$ROOT_DIR/.release-checkpoint"

# Options (can be set via flags)
DRY_RUN=false
NO_PUSH=false
RESUME=false
FORCE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_dry() { echo -e "${CYAN}[DRY-RUN]${NC} Would: $1"; }

# Parse command line options
parse_options() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --dry-run) DRY_RUN=true; shift ;;
            --no-push) NO_PUSH=true; shift ;;
            --resume) RESUME=true; shift ;;
            --force) FORCE=true; shift ;;
            *) break ;;
        esac
    done
    # Return remaining args
    echo "$@"
}

# Confirmation prompt (skipped with --force)
confirm() {
    local msg="$1"
    if $FORCE; then
        return 0
    fi
    echo -e "${YELLOW}$msg${NC}"
    read -p "Continue? [y/N] " -n 1 -r
    echo
    [[ $REPLY =~ ^[Yy]$ ]]
}

# Create backup of a file
backup_file() {
    local file="$1"
    local component="$2"
    
    if $DRY_RUN; then
        log_dry "Backup $file"
        return
    fi
    
    mkdir -p "$BACKUP_DIR/$component"
    local backup_name="$(basename "$file").$(date +%Y%m%d_%H%M%S).bak"
    cp "$file" "$BACKUP_DIR/$component/$backup_name"
    log_info "Backed up $file"
}

# Restore backups for a component
restore_backups() {
    local component="$1"
    local backup_subdir="$BACKUP_DIR/$component"
    
    if [ ! -d "$backup_subdir" ]; then
        log_warn "No backups found for $component"
        return 1
    fi
    
    log_info "Restoring backups for $component..."
    
    # Find and restore the most recent backup for each file
    for backup in "$backup_subdir"/*.bak; do
        if [ -f "$backup" ]; then
            local original_name=$(basename "$backup" | sed 's/\.[0-9_]*\.bak$//')
            local dest=$(find "$ROOT_DIR/$component" -name "$original_name" -type f | head -1)
            if [ -n "$dest" ]; then
                cp "$backup" "$dest"
                log_info "Restored $dest from backup"
            fi
        fi
    done
    
    log_success "Backups restored for $component"
}

# Save checkpoint for resume capability
save_checkpoint() {
    local component="$1"
    local version="$2"
    local step="$3"
    
    if $DRY_RUN; then
        return
    fi
    
    echo "$component:$version:$step:$(date +%s)" >> "$CHECKPOINT_FILE"
    log_info "Checkpoint saved: $component step $step"
}

# Check if we should skip a step (for resume)
should_skip_step() {
    local component="$1"
    local step="$2"
    
    if ! $RESUME || [ ! -f "$CHECKPOINT_FILE" ]; then
        return 1  # Don't skip
    fi
    
    if grep -q "^$component:.*:$step:" "$CHECKPOINT_FILE"; then
        log_info "Skipping $component step $step (already completed)"
        return 0  # Skip
    fi
    
    return 1  # Don't skip
}

# Clear checkpoints after successful release
clear_checkpoints() {
    if [ -f "$CHECKPOINT_FILE" ]; then
        rm "$CHECKPOINT_FILE"
        log_info "Cleared checkpoint file"
    fi
}

# Verify git status is clean
verify_git_clean() {
    local dir="$1"
    cd "$dir"
    
    if ! git diff --quiet || ! git diff --cached --quiet; then
        log_error "Git working directory is not clean in $dir"
        log_error "Please commit or stash changes before releasing"
        git status --short
        return 1
    fi
    
    return 0
}

# Verify we're on main branch
verify_main_branch() {
    local dir="$1"
    cd "$dir"
    
    local branch=$(git branch --show-current 2>/dev/null || git rev-parse --abbrev-ref HEAD)
    if [ "$branch" != "main" ]; then
        log_warn "Not on main branch (current: $branch)"
        if ! confirm "Release from $branch branch?"; then
            return 1
        fi
    fi
    
    return 0
}

# Pre-flight checks before release
preflight_checks() {
    local component="$1"
    local dir="$ROOT_DIR/$component"
    
    log_info "Running pre-flight checks for $component..."
    
    # Check directory exists
    if [ ! -d "$dir" ]; then
        log_error "Component directory not found: $dir"
        return 1
    fi
    
    # Verify git status
    if ! verify_git_clean "$dir"; then
        return 1
    fi
    
    # Verify branch
    if ! verify_main_branch "$dir"; then
        return 1
    fi
    
    # Check for required tools
    if ! command -v cargo &> /dev/null && [[ "$component" =~ ^(base|cli|language-server)$ ]]; then
        log_error "cargo not found - required for Rust components"
        return 1
    fi
    
    if ! command -v npm &> /dev/null && [[ "$component" =~ ^(language-client|viewer)$ ]]; then
        log_error "npm not found - required for Node.js components"
        return 1
    fi
    
    log_success "Pre-flight checks passed for $component"
    return 0
}

# Update version in Cargo.toml
update_cargo_version() {
    local file="$1"
    local version="$2"
    
    if $DRY_RUN; then
        log_dry "Update $file version to $version"
        return
    fi
    
    backup_file "$file" "$(basename "$(dirname "$file")")"
    log_info "Updating $file to version $version"
    sed -i "s/^version = \".*\"/version = \"$version\"/" "$file"
}

# Update version in package.json
update_package_version() {
    local file="$1"
    local version="$2"
    
    if $DRY_RUN; then
        log_dry "Update $file version to $version"
        return
    fi
    
    backup_file "$file" "$(basename "$(dirname "$file")")"
    log_info "Updating $file to version $version"
    # Use jq if available, otherwise sed
    if command -v jq &> /dev/null; then
        tmp=$(mktemp)
        jq ".version = \"$version\"" "$file" > "$tmp" && mv "$tmp" "$file"
    else
        sed -i "s/\"version\": \".*\"/\"version\": \"$version\"/" "$file"
    fi
}

# Update syster-base dependency version in Cargo.toml
update_base_dep() {
    local file="$1"
    local version="$2"
    
    if $DRY_RUN; then
        log_dry "Update syster-base dependency in $file to $version"
        return
    fi
    
    log_info "Updating syster-base dependency in $file to $version"
    sed -i "s/syster-base = \".*\"/syster-base = \"$version\"/" "$file"
}

# Update lspVersion in package.json (for language-client)
update_lsp_version() {
    local file="$1"
    local version="$2"
    
    if $DRY_RUN; then
        log_dry "Update lspVersion in $file to $version"
        return
    fi
    
    log_info "Updating lspVersion in $file to $version"
    if command -v jq &> /dev/null; then
        tmp=$(mktemp)
        jq ".lspVersion = \"$version\"" "$file" > "$tmp" && mv "$tmp" "$file"
    else
        sed -i "s/\"lspVersion\": \".*\"/\"lspVersion\": \"$version\"/" "$file"
    fi
}

# Run validation (make run-guidelines)
run_validation() {
    local component="$1"
    local dir="$ROOT_DIR/$component"
    
    if $DRY_RUN; then
        log_dry "Run validation for $component (make run-guidelines)"
        return 0
    fi
    
    log_info "Running validation for $component..."
    cd "$dir"
    if [ -f "Makefile" ]; then
        if ! make run-guidelines; then
            log_error "Validation failed for $component"
            log_error "Run 'make run-guidelines' manually to see errors"
            return 1
        fi
    else
        log_warn "No Makefile found, skipping validation"
    fi
    
    return 0
}

# Update changelog with new version
update_changelog() {
    local file="$1"
    local version="$2"
    local date=$(date +%Y-%m-%d)
    
    if [ ! -f "$file" ]; then
        log_warn "No CHANGELOG.md found at $file"
        return 0
    fi
    
    log_info "Checking CHANGELOG.md for version $version entry..."
    if grep -q "## \[$version\]" "$file"; then
        log_success "CHANGELOG already has entry for $version"
    else
        log_warn "No CHANGELOG entry for version $version"
        if ! $DRY_RUN && ! $FORCE; then
            if ! confirm "Continue without CHANGELOG entry?"; then
                log_error "Please add CHANGELOG entry for version $version before releasing"
                return 1
            fi
        fi
    fi
    
    return 0
}

# Git operations: commit, tag, push
git_release() {
    local component="$1"
    local version="$2"
    local tag_prefix="$3"
    local dir="$ROOT_DIR/$component"
    
    cd "$dir"
    local tag="$tag_prefix$version"
    
    if $DRY_RUN; then
        log_dry "Git add all changes"
        log_dry "Git commit: 'chore: release $tag'"
        log_dry "Git tag: $tag"
        log_dry "Git push origin main"
        log_dry "Git push tag $tag"
        return 0
    fi
    
    # Check for changes
    if git diff --quiet && git diff --cached --quiet; then
        log_info "No changes to commit in $component"
    else
        log_info "Committing changes in $component..."
        git add -A
        git commit -m "chore: release $tag"
    fi
    
    # Check if tag already exists
    if git rev-parse "$tag" >/dev/null 2>&1; then
        log_error "Tag $tag already exists!"
        if ! confirm "Delete existing tag and recreate?"; then
            return 1
        fi
        git tag -d "$tag"
        git push origin --delete "$tag" 2>/dev/null || true
    fi
    
    # Create tag
    log_info "Creating tag $tag..."
    git tag -a "$tag" -m "$component $version release"
    
    # Push (unless --no-push)
    if $NO_PUSH; then
        log_warn "Skipping push (--no-push specified)"
        log_info "To push manually: git push origin main && git push origin $tag"
    else
        log_info "Pushing to origin..."
        git push origin main
        git push origin "$tag"
    fi
    
    log_success "Released $component $version with tag $tag"
    return 0
}

# Release base (syster-base)
release_base() {
    local version="$1"
    log_info "=== Releasing syster-base $version ==="
    
    local dir="$ROOT_DIR/base"
    
    # Pre-flight checks
    if ! should_skip_step "base" "preflight"; then
        if ! preflight_checks "base"; then
            return 1
        fi
        save_checkpoint "base" "$version" "preflight"
    fi
    
    cd "$dir"
    
    # Step 1: Update version
    if ! should_skip_step "base" "version"; then
        update_cargo_version "$dir/Cargo.toml" "$version"
        save_checkpoint "base" "$version" "version"
    fi
    
    # Step 2: Update changelog
    if ! should_skip_step "base" "changelog"; then
        if ! update_changelog "$dir/CHANGELOG.md" "$version"; then
            return 1
        fi
        save_checkpoint "base" "$version" "changelog"
    fi
    
    # Step 3: Run validation
    if ! should_skip_step "base" "validation"; then
        if ! run_validation "base"; then
            log_error "Validation failed. Fix errors and run with --resume to continue."
            return 1
        fi
        save_checkpoint "base" "$version" "validation"
    fi
    
    # Step 4: Git release
    if ! should_skip_step "base" "git"; then
        if ! git_release "base" "$version" "v"; then
            return 1
        fi
        save_checkpoint "base" "$version" "git"
    fi
    
    log_success "=== syster-base $version released ==="
    return 0
}

# Release cli (syster-cli)
release_cli() {
    local version="$1"
    local base_version="${2:-$version}"  # Use same version as base if not specified
    log_info "=== Releasing syster-cli $version ==="
    
    local dir="$ROOT_DIR/cli"
    
    # Pre-flight checks
    if ! should_skip_step "cli" "preflight"; then
        if ! preflight_checks "cli"; then
            return 1
        fi
        save_checkpoint "cli" "$version" "preflight"
    fi
    
    cd "$dir"
    
    # Step 1: Update version and dependency
    if ! should_skip_step "cli" "version"; then
        update_cargo_version "$dir/Cargo.toml" "$version"
        update_base_dep "$dir/Cargo.toml" "$base_version"
        save_checkpoint "cli" "$version" "version"
    fi
    
    # Step 2: Update changelog
    if ! should_skip_step "cli" "changelog"; then
        if ! update_changelog "$dir/CHANGELOG.md" "$version"; then
            return 1
        fi
        save_checkpoint "cli" "$version" "changelog"
    fi
    
    # Step 3: Run validation
    if ! should_skip_step "cli" "validation"; then
        if ! run_validation "cli"; then
            log_error "Validation failed. Fix errors and run with --resume to continue."
            return 1
        fi
        save_checkpoint "cli" "$version" "validation"
    fi
    
    # Step 4: Git release
    if ! should_skip_step "cli" "git"; then
        if ! git_release "cli" "$version" "cli-v"; then
            return 1
        fi
        save_checkpoint "cli" "$version" "git"
    fi
    
    log_success "=== syster-cli $version released ==="
    return 0
}

# Release language-server (syster-lsp)
release_lsp() {
    local version="$1"
    local base_version="${2:-$version}"
    log_info "=== Releasing syster-lsp $version ==="
    
    local dir="$ROOT_DIR/language-server"
    
    # Pre-flight checks
    if ! should_skip_step "lsp" "preflight"; then
        if ! preflight_checks "language-server"; then
            return 1
        fi
        save_checkpoint "lsp" "$version" "preflight"
    fi
    
    cd "$dir"
    
    # Step 1: Update version and dependency
    if ! should_skip_step "lsp" "version"; then
        update_cargo_version "$dir/crates/syster-lsp/Cargo.toml" "$version"
        update_base_dep "$dir/crates/syster-lsp/Cargo.toml" "$base_version"
        save_checkpoint "lsp" "$version" "version"
    fi
    
    # Step 2: Update changelog
    if ! should_skip_step "lsp" "changelog"; then
        if ! update_changelog "$dir/CHANGELOG.md" "$version"; then
            return 1
        fi
        save_checkpoint "lsp" "$version" "changelog"
    fi
    
    # Step 3: Run validation
    if ! should_skip_step "lsp" "validation"; then
        if ! run_validation "language-server"; then
            log_error "Validation failed. Fix errors and run with --resume to continue."
            return 1
        fi
        save_checkpoint "lsp" "$version" "validation"
    fi
    
    # Step 4: Git release
    if ! should_skip_step "lsp" "git"; then
        if ! git_release "language-server" "$version" "lsp-v"; then
            return 1
        fi
        save_checkpoint "lsp" "$version" "git"
    fi
    
    log_success "=== syster-lsp $version released ==="
    return 0
}

# Release language-client (sysml-language-support VS Code extension)
release_client() {
    local version="$1"
    local lsp_version="${2:-$version}"
    log_info "=== Releasing sysml-language-support $version ==="
    
    local dir="$ROOT_DIR/language-client"
    
    # Pre-flight checks
    if ! should_skip_step "client" "preflight"; then
        if ! preflight_checks "language-client"; then
            return 1
        fi
        save_checkpoint "client" "$version" "preflight"
    fi
    
    cd "$dir"
    
    # Step 1: Update version and lspVersion
    if ! should_skip_step "client" "version"; then
        update_package_version "$dir/package.json" "$version"
        update_lsp_version "$dir/package.json" "$lsp_version"
        save_checkpoint "client" "$version" "version"
    fi
    
    # Step 2: Update changelog
    if ! should_skip_step "client" "changelog"; then
        if ! update_changelog "$dir/CHANGELOG.md" "$version"; then
            return 1
        fi
        save_checkpoint "client" "$version" "changelog"
    fi
    
    # Step 3: Run validation
    if ! should_skip_step "client" "validation"; then
        if ! run_validation "language-client"; then
            log_error "Validation failed. Fix errors and run with --resume to continue."
            return 1
        fi
        save_checkpoint "client" "$version" "validation"
    fi
    
    # Step 4: Git release
    if ! should_skip_step "client" "git"; then
        if ! git_release "language-client" "$version" "client-v"; then
            return 1
        fi
        save_checkpoint "client" "$version" "git"
    fi
    
    log_success "=== sysml-language-support $version released ==="
    return 0
}

# Release viewer (syster-viewer VS Code extension)
release_viewer() {
    local version="$1"
    log_info "=== Releasing syster-viewer $version ==="
    
    local dir="$ROOT_DIR/viewer"
    
    # Pre-flight checks
    if ! should_skip_step "viewer" "preflight"; then
        if ! preflight_checks "viewer"; then
            return 1
        fi
        save_checkpoint "viewer" "$version" "preflight"
    fi
    
    cd "$dir"
    
    # Step 1: Update version
    if ! should_skip_step "viewer" "version"; then
        update_package_version "$dir/package.json" "$version"
        save_checkpoint "viewer" "$version" "version"
    fi
    
    # Step 2: Update changelog
    if ! should_skip_step "viewer" "changelog"; then
        if ! update_changelog "$dir/CHANGELOG.md" "$version"; then
            return 1
        fi
        save_checkpoint "viewer" "$version" "changelog"
    fi
    
    # Step 3: Run validation
    if ! should_skip_step "viewer" "validation"; then
        if ! run_validation "viewer"; then
            log_error "Validation failed. Fix errors and run with --resume to continue."
            return 1
        fi
        save_checkpoint "viewer" "$version" "validation"
    fi
    
    # Step 4: Git release
    if ! should_skip_step "viewer" "git"; then
        if ! git_release "viewer" "$version" "viewer-v"; then
            return 1
        fi
        save_checkpoint "viewer" "$version" "git"
    fi
    
    log_success "=== syster-viewer $version released ==="
    return 0
}

# Release all components in order
release_all() {
    local version="$1"
    log_info "=== Starting full release for version $version ==="
    
    if ! $DRY_RUN && ! $FORCE; then
        echo ""
        echo "This will release ALL components with version $version:"
        echo "  1. base (syster-base)"
        echo "  2. cli (syster-cli)"
        echo "  3. language-server (syster-lsp)"
        echo "  4. language-client (sysml-language-support)"
        echo "  5. viewer (syster-viewer)"
        echo ""
        if ! confirm "Proceed with full release?"; then
            log_info "Release cancelled"
            return 1
        fi
    fi
    
    local failed=false
    
    if ! release_base "$version"; then
        log_error "Base release failed"
        failed=true
    fi
    
    if ! $failed && ! release_cli "$version" "$version"; then
        log_error "CLI release failed"
        failed=true
    fi
    
    if ! $failed && ! release_lsp "$version" "$version"; then
        log_error "LSP release failed"
        failed=true
    fi
    
    if ! $failed && ! release_client "$version" "$version"; then
        log_error "Client release failed"
        failed=true
    fi
    
    if ! $failed && ! release_viewer "$version"; then
        log_error "Viewer release failed"
        failed=true
    fi
    
    if $failed; then
        log_error "Full release failed. Use --resume to continue from where it stopped."
        return 1
    fi
    
    # Clear checkpoints on success
    clear_checkpoints
    
    log_success "=== All components released at version $version ==="
    return 0
}

# Show current versions
show_versions() {
    log_info "Current component versions:"
    echo ""
    
    # Base
    local base_ver=$(grep '^version' "$ROOT_DIR/base/Cargo.toml" | head -1 | cut -d'"' -f2)
    echo "  base (syster-base):           $base_ver"
    
    # CLI
    local cli_ver=$(grep '^version' "$ROOT_DIR/cli/Cargo.toml" | head -1 | cut -d'"' -f2)
    local cli_base=$(grep 'syster-base' "$ROOT_DIR/cli/Cargo.toml" | grep -v '#' | head -1 | cut -d'"' -f2)
    echo "  cli (syster-cli):             $cli_ver (base: $cli_base)"
    
    # LSP
    local lsp_ver=$(grep '^version' "$ROOT_DIR/language-server/crates/syster-lsp/Cargo.toml" | head -1 | cut -d'"' -f2)
    local lsp_base=$(grep 'syster-base' "$ROOT_DIR/language-server/crates/syster-lsp/Cargo.toml" | grep -v '#' | head -1 | cut -d'"' -f2)
    echo "  language-server (syster-lsp): $lsp_ver (base: $lsp_base)"
    
    # Client
    local client_ver=$(grep '"version"' "$ROOT_DIR/language-client/package.json" | head -1 | cut -d'"' -f4)
    local client_lsp=$(grep '"lspVersion"' "$ROOT_DIR/language-client/package.json" | head -1 | cut -d'"' -f4)
    echo "  language-client:              $client_ver (lsp: $client_lsp)"
    
    # Viewer
    local viewer_ver=$(grep '"version"' "$ROOT_DIR/viewer/package.json" | head -1 | cut -d'"' -f4)
    echo "  viewer (syster-viewer):       $viewer_ver"
    
    echo ""
}

# Print usage
usage() {
    echo "Syster Release Workflow"
    echo ""
    echo "Usage:"
    echo "  $0 <command> [args] [options]"
    echo ""
    echo "Commands:"
    echo "  versions                    Show current versions of all components"
    echo "  base <version>              Release syster-base"
    echo "  cli <version> [base_ver]    Release syster-cli"
    echo "  lsp <version> [base_ver]    Release syster-lsp"
    echo "  client <version> [lsp_ver]  Release language-client"
    echo "  viewer <version>            Release syster-viewer"
    echo "  all <version>               Release all components with same version"
    echo "  restore <component>         Restore backups for a component"
    echo "  clean                       Clear checkpoints and backups"
    echo ""
    echo "Options:"
    echo "  --dry-run    Preview changes without applying them"
    echo "  --no-push    Make changes locally but don't push to remote"
    echo "  --resume     Resume from last checkpoint after failure"
    echo "  --force      Skip confirmation prompts"
    echo ""
    echo "Examples:"
    echo "  $0 versions"
    echo "  $0 base 0.3.2-alpha --dry-run    # Preview base release"
    echo "  $0 cli 0.3.2-alpha               # Release CLI"
    echo "  $0 all 0.4.0-alpha --no-push     # Release all, don't push"
    echo "  $0 all 0.4.0-alpha --resume      # Resume failed release"
    echo "  $0 restore cli                   # Restore CLI from backup"
    echo ""
    echo "Release Order (dependencies):"
    echo "  1. base       - Core library (no internal deps)"
    echo "  2. cli        - Depends on base"
    echo "  3. lsp        - Depends on base"
    echo "  4. client     - Depends on lsp binary"
    echo "  5. viewer     - Standalone"
    echo ""
    echo "Redundancy Features:"
    echo "  - Backups: Files are backed up before modification"
    echo "  - Checkpoints: Progress is saved for --resume capability"
    echo "  - Dry-run: Preview all changes before applying"
    echo "  - Validation: make run-guidelines runs before each commit"
    echo "  - Confirmation: Prompts before destructive operations"
}

# Clean up checkpoints and backups
clean() {
    log_info "Cleaning up..."
    
    if [ -f "$CHECKPOINT_FILE" ]; then
        rm "$CHECKPOINT_FILE"
        log_info "Removed checkpoint file"
    fi
    
    if [ -d "$BACKUP_DIR" ]; then
        rm -rf "$BACKUP_DIR"
        log_info "Removed backup directory"
    fi
    
    log_success "Cleanup complete"
}

# Main - parse options first
ARGS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run) DRY_RUN=true; shift ;;
        --no-push) NO_PUSH=true; shift ;;
        --resume) RESUME=true; shift ;;
        --force) FORCE=true; shift ;;
        *) ARGS+=("$1"); shift ;;
    esac
done
set -- "${ARGS[@]}"

# Show mode if special flags are set
if $DRY_RUN; then
    log_info "DRY-RUN MODE - No changes will be made"
fi
if $NO_PUSH; then
    log_info "NO-PUSH MODE - Changes won't be pushed to remote"
fi
if $RESUME; then
    log_info "RESUME MODE - Skipping completed steps"
fi

case "${1:-}" in
    versions|version|v)
        show_versions
        ;;
    base)
        [ -z "${2:-}" ] && { log_error "Version required"; usage; exit 1; }
        release_base "$2"
        ;;
    cli)
        [ -z "${2:-}" ] && { log_error "Version required"; usage; exit 1; }
        release_cli "$2" "${3:-$2}"
        ;;
    lsp|language-server)
        [ -z "${2:-}" ] && { log_error "Version required"; usage; exit 1; }
        release_lsp "$2" "${3:-$2}"
        ;;
    client|language-client)
        [ -z "${2:-}" ] && { log_error "Version required"; usage; exit 1; }
        release_client "$2" "${3:-$2}"
        ;;
    viewer)
        [ -z "${2:-}" ] && { log_error "Version required"; usage; exit 1; }
        release_viewer "$2"
        ;;
    all)
        [ -z "${2:-}" ] && { log_error "Version required"; usage; exit 1; }
        release_all "$2"
        ;;
    restore)
        [ -z "${2:-}" ] && { log_error "Component required"; usage; exit 1; }
        restore_backups "$2"
        ;;
    clean)
        clean
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        usage
        exit 1
        ;;
esac
