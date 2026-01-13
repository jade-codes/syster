import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';
import { findServerBinary } from './server-locator';

let client: LanguageClient | undefined;

/**
 * Create server options for launching the LSP server
 */
async function createServerOptions(context: vscode.ExtensionContext, outputChannel: vscode.OutputChannel): Promise<ServerOptions> {
    // Always use the first (root) workspace folder
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    const serverPath = await findServerBinary({ 
        workspaceFolder, 
        outputChannel,
        extensionPath: context.extensionPath
    });

    return {
        command: serverPath,
        transport: TransportKind.stdio,
        options: {
            env: {
                ...process.env,
                RUST_LOG: process.env.RUST_LOG || 'info'
            }
        }
    };
}

/**
 * Create client options for the Language Client
 */
function createClientOptions(context: vscode.ExtensionContext, outputChannel: vscode.OutputChannel): LanguageClientOptions {
    // Read stdlib configuration from VS Code settings
    const config = vscode.workspace.getConfiguration('syster');
    const stdlibEnabled = config.get<boolean>('stdlib.enabled', true);
    let stdlibPath = config.get<string>('stdlib.path', '');

    // If no custom path set, try bundled stdlib
    if (!stdlibPath) {
        const bundledStdlib = path.join(context.extensionPath, 'sysml.library');
        if (fs.existsSync(bundledStdlib)) {
            stdlibPath = bundledStdlib;
            outputChannel.appendLine(`[Client] Using bundled stdlib: ${bundledStdlib}`);
        }
    }

    // Only pass stdlibPath if it's actually set (non-empty)
    const initOptions: { stdlibEnabled: boolean; stdlibPath?: string } = {
        stdlibEnabled
    };
    if (stdlibPath) {
        initOptions.stdlibPath = stdlibPath;
    }

    outputChannel.appendLine(`[Client] Initialization options: ${JSON.stringify(initOptions)}`);

    return {
        documentSelector: [
            { scheme: 'file', language: 'sysml' },
            { scheme: 'file', language: 'kerml' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{sysml,kerml}')
        },
        outputChannel,
        traceOutputChannel: outputChannel,
        revealOutputChannelOn: 4, // Never automatically show output
        initializationOptions: initOptions,
        middleware: {
            provideHover: async (document, position, token, next) => {
                outputChannel.appendLine(`[Client] Hover requested at ${position.line}:${position.character}`);
                return await next(document, position, token);
            },
            provideDocumentFormattingEdits: async (document, options, token, next) => {
                outputChannel.appendLine(`[Client] Format Document requested for ${document.uri.toString()}`);
                const result = await next(document, options, token);
                outputChannel.appendLine(`[Client] Format Document result: ${result ? result.length + ' edits' : 'null'}`);
                return result;
            },
            provideWorkspaceSymbols: async (query, token, next) => {
                outputChannel.appendLine(`[Client] Workspace Symbols requested: ${query}`);
                return await next(query, token);
            }
        }
    };
}

/**
 * Start the Language Client
 */
export async function startClient(context: vscode.ExtensionContext): Promise<LanguageClient> {
    const outputChannel = vscode.window.createOutputChannel('SysML Language Server');
    
    try {
        outputChannel.appendLine('Starting SysML Language Server...');
        
        const serverOptions = await createServerOptions(context, outputChannel);
        const clientOptions = createClientOptions(context, outputChannel);

        client = new LanguageClient(
            'syster-lsp',
            'SysML Language Server',
            serverOptions,
            clientOptions
        );

        // Start the client and wait for it to be ready
        await client.start();
        
        outputChannel.appendLine('✓ SysML Language Server started successfully');
        
        return client;
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        outputChannel.appendLine(`✗ Failed to start language server: ${message}`);
        
        vscode.window.showErrorMessage(
            'Failed to start SysML Language Server. See output for details.',
            'Show Output'
        ).then(selection => {
            if (selection === 'Show Output') {
                outputChannel.show();
            }
        });
        
        throw error;
    }
}

/**
 * Stop the Language Client
 */
export async function stopClient(): Promise<void> {
    if (!client) {
        return;
    }

    try {
        await client.stop();
        client = undefined;
    } catch (error) {
        console.error('Error stopping language client:', error);
    }
}

/**
 * Get the current client instance
 */
export function getClient(): LanguageClient | undefined {
    return client;
}

/**
 * Check if the client is running
 */
export function isClientRunning(): boolean {
    return client !== undefined && client.needsStop();
}
