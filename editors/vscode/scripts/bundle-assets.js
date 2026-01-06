#!/usr/bin/env node
/**
 * Bundle assets for VS Code extension packaging
 * 
 * This script copies:
 * 1. syster-lsp binaries (for each platform)
 * 2. sysml.library standard library
 * 
 * For local development: copies the local release build
 * For CI: expects binaries to already be in server/ folder
 */

const fs = require('fs');
const path = require('path');

const EXTENSION_ROOT = path.resolve(__dirname, '..');
const REPO_ROOT = path.resolve(EXTENSION_ROOT, '../..');

// Destination folders in extension
const SERVER_DIR = path.join(EXTENSION_ROOT, 'server');
const STDLIB_DIR = path.join(EXTENSION_ROOT, 'sysml.library');

// Source locations
const STDLIB_SRC = path.join(REPO_ROOT, 'crates/syster-base/sysml.library');

/**
 * Recursively copy a directory
 */
function copyDirSync(src, dest) {
    if (!fs.existsSync(src)) {
        console.error(`Source not found: ${src}`);
        return false;
    }

    fs.mkdirSync(dest, { recursive: true });
    
    const entries = fs.readdirSync(src, { withFileTypes: true });
    
    for (const entry of entries) {
        const srcPath = path.join(src, entry.name);
        const destPath = path.join(dest, entry.name);
        
        if (entry.isDirectory()) {
            copyDirSync(srcPath, destPath);
        } else {
            fs.copyFileSync(srcPath, destPath);
        }
    }
    
    return true;
}

/**
 * Copy a single file
 */
function copyFileIfExists(src, dest) {
    if (!fs.existsSync(src)) {
        return false;
    }
    
    fs.mkdirSync(path.dirname(dest), { recursive: true });
    fs.copyFileSync(src, dest);
    
    // Make executable on Unix
    if (process.platform !== 'win32') {
        fs.chmodSync(dest, 0o755);
    }
    
    console.log(`✓ Copied: ${path.basename(dest)}`);
    return true;
}

function main() {
    console.log('Bundling VS Code extension assets...\n');
    
    // Check if server directory already has binaries (CI mode)
    const existingBinaries = fs.existsSync(SERVER_DIR) && 
        fs.readdirSync(SERVER_DIR).some(f => f.startsWith('syster-lsp'));
    
    if (existingBinaries) {
        console.log('Server binaries already present (CI mode), skipping binary copy.\n');
    } else {
        // Clean and recreate server directory
        if (fs.existsSync(SERVER_DIR)) {
            fs.rmSync(SERVER_DIR, { recursive: true });
        }
        fs.mkdirSync(SERVER_DIR, { recursive: true });
        
        // Copy local release build
        const localBinary = path.join(REPO_ROOT, 'target/release/syster-lsp');
        const localBinaryWin = path.join(REPO_ROOT, 'target/release/syster-lsp.exe');
        
        // Determine current platform
        const platform = process.platform;
        const arch = process.arch;
        
        let copiedBinary = false;
        if (fs.existsSync(localBinary)) {
            const binaryName = `syster-lsp-${platform}-${arch}`;
            copyFileIfExists(localBinary, path.join(SERVER_DIR, binaryName));
            copiedBinary = true;
        } else if (fs.existsSync(localBinaryWin)) {
            copyFileIfExists(localBinaryWin, path.join(SERVER_DIR, 'syster-lsp-win32-x64.exe'));
            copiedBinary = true;
        }
        
        if (!copiedBinary) {
            console.error('\n✗ No LSP binary found! Run `cargo build --release -p syster-lsp` first.');
            process.exit(1);
        }
    }
    
    // Copy stdlib (unless already present)
    if (fs.existsSync(STDLIB_DIR)) {
        console.log('sysml.library already present, skipping.\n');
    } else {
        console.log('Copying SysML standard library...');
        if (copyDirSync(STDLIB_SRC, STDLIB_DIR)) {
            console.log(`✓ Copied sysml.library (${countFiles(STDLIB_DIR)} files)`);
        } else {
            console.error('✗ Failed to copy sysml.library');
            process.exit(1);
        }
    }
    
    console.log('\n✓ Asset bundling complete!');
}

function countFiles(dir) {
    let count = 0;
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    for (const entry of entries) {
        if (entry.isDirectory()) {
            count += countFiles(path.join(dir, entry.name));
        } else {
            count++;
        }
    }
    return count;
}

main();
