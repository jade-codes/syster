import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { execSync } from 'child_process';

export interface ServerLocatorOptions {
    workspaceFolder?: vscode.WorkspaceFolder;
    outputChannel?: vscode.OutputChannel;
    extensionPath?: string;
}

/**
 * Get the platform-specific binary name
 */
function getPlatformBinaryName(): string {
    const platform = process.platform;
    const arch = process.arch;
    let name = `syster-lsp-${platform}-${arch}`;
    if (platform === 'win32') {
        name += '.exe';
    }
    return name;
}

/**
 * Find the syster-lsp binary using multiple search strategies
 */
export async function findServerBinary(options: ServerLocatorOptions = {}): Promise<string> {
    const { outputChannel, extensionPath } = options;

    const searchLocations: Array<{ name: string; path: () => string | null }> = [
        // 1. User-configured path (highest priority)
        {
            name: 'Configuration setting (syster.lsp.path)',
            path: () => {
                const configured = vscode.workspace.getConfiguration('syster').get<string>('lsp.path');
                // Only use if it's not the default dev path
                if (configured && !configured.includes('/workspaces/syster/')) {
                    return configured;
                }
                return null;
            }
        },
        // 2. Bundled with extension (for published extension)
        {
            name: 'Bundled server',
            path: () => {
                if (!extensionPath) return null;
                const binaryName = getPlatformBinaryName();
                return path.join(extensionPath, 'server', binaryName);
            }
        },
        // 3. Development fallback - workspace relative paths
        {
            name: 'Development build (release)',
            path: () => {
                // Try to find the workspace root by looking for Cargo.toml
                const workspaceFolders = vscode.workspace.workspaceFolders;
                if (workspaceFolders) {
                    for (const folder of workspaceFolders) {
                        // Check if this is the syster workspace or a parent
                        const candidates = [
                            path.join(folder.uri.fsPath, 'target', 'release', 'syster-lsp'),
                            path.join(folder.uri.fsPath, '..', '..', 'target', 'release', 'syster-lsp'),
                            path.join(folder.uri.fsPath, '..', '..', '..', 'target', 'release', 'syster-lsp'),
                        ];
                        for (const candidate of candidates) {
                            if (fs.existsSync(candidate)) {
                                return candidate;
                            }
                        }
                    }
                }
                // Fallback to common dev paths
                const homedir = process.env.HOME || process.env.USERPROFILE || '';
                return path.join(homedir, 'syster', 'target', 'release', 'syster-lsp');
            }
        },
        {
            name: 'Development build (debug)',
            path: () => {
                const workspaceFolders = vscode.workspace.workspaceFolders;
                if (workspaceFolders) {
                    for (const folder of workspaceFolders) {
                        const candidates = [
                            path.join(folder.uri.fsPath, 'target', 'debug', 'syster-lsp'),
                            path.join(folder.uri.fsPath, '..', '..', 'target', 'debug', 'syster-lsp'),
                            path.join(folder.uri.fsPath, '..', '..', '..', 'target', 'debug', 'syster-lsp'),
                        ];
                        for (const candidate of candidates) {
                            if (fs.existsSync(candidate)) {
                                return candidate;
                            }
                        }
                    }
                }
                const homedir = process.env.HOME || process.env.USERPROFILE || '';
                return path.join(homedir, 'syster', 'target', 'debug', 'syster-lsp');
            }
        }
    ];

    for (const location of searchLocations) {
        const binaryPath = location.path();
        if (binaryPath) {
            outputChannel?.appendLine(`Checking ${location.name}: ${binaryPath}`);
            
            // Check if file exists and is executable
            if (fs.existsSync(binaryPath)) {
                try {
                    fs.accessSync(binaryPath, fs.constants.X_OK);
                    outputChannel?.appendLine(`✓ Found LSP server: ${binaryPath}`);
                    return binaryPath;
                } catch {
                    outputChannel?.appendLine(`✗ File exists but is not executable: ${binaryPath}`);
                }
            } else {
                outputChannel?.appendLine(`✗ File not found: ${binaryPath}`);
            }
        }
    }

    // No server found
    const errorMessage = [
        'Could not find syster-lsp language server binary.',
        '',
        'Please set the path in settings: "syster.lsp.path"',
        '',
        'Or build from source: cargo build --release -p syster-lsp'
    ].join('\n');

    throw new Error(errorMessage);
}
