import * as vscode from 'vscode';
import { startClient, stopClient, getClient } from './client';

// API exported for other extensions
export interface SysterLspApi {
    getClient: typeof getClient;
}

/**
 * Extension activation
 */
export async function activate(context: vscode.ExtensionContext): Promise<SysterLspApi> {
    console.log('SysML Language Support extension is activating...');

    try {
        // Start the LSP client
        await startClient(context);

        // Register restart command
        const restartCommand = vscode.commands.registerCommand('syster.restartServer', async () => {
            try {
                await stopClient();
                await startClient(context);
                vscode.window.showInformationMessage('SysML Language Server restarted successfully');
            } catch (error) {
                const message = error instanceof Error ? error.message : String(error);
                vscode.window.showErrorMessage(`Failed to restart server: ${message}`);
            }
        });

        context.subscriptions.push(restartCommand);

        // Register code lens command handler
        // This converts JSON-serialized arguments from LSP to proper VS Code objects
        const showReferencesCommand = vscode.commands.registerCommand(
            'syster.showReferences',
            async (uriString: string, position: { line: number; character: number }, locations: Array<{ uri: string; range: { start: { line: number; character: number }; end: { line: number; character: number } } }>) => {
                const uri = vscode.Uri.parse(uriString);
                const pos = new vscode.Position(position.line, position.character);
                const locs = locations.map(loc => new vscode.Location(
                    vscode.Uri.parse(loc.uri),
                    new vscode.Range(
                        new vscode.Position(loc.range.start.line, loc.range.start.character),
                        new vscode.Position(loc.range.end.line, loc.range.end.character)
                    )
                ));
                await vscode.commands.executeCommand('editor.action.showReferences', uri, pos, locs);
            }
        );

        context.subscriptions.push(showReferencesCommand);

        // Create status bar item
        const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
        statusBar.text = '$(check) SysML';
        statusBar.tooltip = 'SysML Language Server is running';
        statusBar.show();
        context.subscriptions.push(statusBar);

        console.log('✓ SysML Language Support extension activated');
    } catch (error) {
        console.error('Failed to activate SysML extension:', error);
        // Don't throw - allow extension to activate even if server fails
        // Users can try to restart manually
    }

    // Return the API for other extensions to use
    return {
        getClient
    };
}

/**
 * Extension deactivation
 */
export async function deactivate(): Promise<void> {
    console.log('SysML Language Support extension is deactivating...');
    await stopClient();
    console.log('✓ SysML Language Support extension deactivated');
}
