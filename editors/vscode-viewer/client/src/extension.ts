import * as vscode from 'vscode';
import { DiagramPanel } from './diagram-panel';

/**
 * Extension activation
 */
export async function activate(context: vscode.ExtensionContext): Promise<void> {
    console.log('SysML Diagram Viewer extension is activating...');

    // Register show diagram command
    const showDiagramCommand = vscode.commands.registerCommand('syster.showDiagram', () => {
        vscode.window.showInformationMessage('Opening SysML Diagram...');
        console.log('[Viewer] showDiagram command executed');
        DiagramPanel.createOrShow(context.extensionUri);
    });

    context.subscriptions.push(showDiagramCommand);

    console.log('✓ SysML Diagram Viewer extension activated');
}

/**
 * Extension deactivation
 */
export async function deactivate(): Promise<void> {
    console.log('SysML Diagram Viewer extension deactivated');
}
