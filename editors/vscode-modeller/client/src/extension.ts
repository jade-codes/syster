import * as vscode from 'vscode';
import { ModellerPanel } from './modeller-panel';

/**
 * Extension activation
 */
export async function activate(context: vscode.ExtensionContext): Promise<void> {
    console.log('SysML Diagram Modeller extension is activating...');

    // Register open modeller command
    const openModellerCommand = vscode.commands.registerCommand('syster.openModeller', () => {
        ModellerPanel.createOrShow(context.extensionUri);
    });

    context.subscriptions.push(openModellerCommand);

    console.log('✓ SysML Diagram Modeller extension activated');
}

/**
 * Extension deactivation
 */
export async function deactivate(): Promise<void> {
    console.log('SysML Diagram Modeller extension deactivated');
}
