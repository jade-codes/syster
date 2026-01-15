import * as vscode from 'vscode';

/**
 * Types for diagram data from LSP
 */
interface DiagramSymbol {
    name: string;
    qualifiedName: string;
    kind: string;
    definitionKind?: string;
    usageKind?: string;
    features?: string[];
    typedBy?: string;
    direction?: string;
}

interface DiagramRelationship {
    type: string;
    source: string;
    target: string;
}

interface DiagramData {
    symbols: DiagramSymbol[];
    relationships: DiagramRelationship[];
}

interface WebviewMessage {
    type: string;
    uri?: string;
    position?: { line: number; character: number };
    // Modeller-specific messages
    action?: string;
    payload?: unknown;
}

/**
 * Manages the SysML Diagram Modeller webview panel
 * 
 * Unlike the Viewer, the Modeller supports:
 * - Creating new elements
 * - Editing existing elements
 * - Creating/deleting relationships
 * - Syncing changes back to source files
 */
export class ModellerPanel {
    public static currentPanel: ModellerPanel | undefined;
    private static readonly viewType = 'systerModeller';

    private readonly panel: vscode.WebviewPanel;
    private readonly extensionUri: vscode.Uri;
    private disposables: vscode.Disposable[] = [];

    private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri) {
        this.panel = panel;
        this.extensionUri = extensionUri;

        // Set the webview's initial html content
        this.update();

        // Listen for when the panel is disposed
        this.panel.onDidDispose(() => this.dispose(), null, this.disposables);

        // Handle messages from the webview
        this.panel.webview.onDidReceiveMessage(
            (message: WebviewMessage) => this.handleMessage(message),
            null,
            this.disposables
        );

        // Update diagram when active editor changes
        vscode.window.onDidChangeActiveTextEditor(
            (editor: vscode.TextEditor | undefined) => {
                if (editor && this.isSysMLFile(editor.document)) {
                    this.refreshDiagram(editor.document.uri);
                }
            },
            null,
            this.disposables
        );
    }

    /**
     * Create or show the modeller panel
     */
    public static createOrShow(extensionUri: vscode.Uri): ModellerPanel {
        const column = vscode.ViewColumn.Beside;

        // If we already have a panel, show it
        if (ModellerPanel.currentPanel) {
            ModellerPanel.currentPanel.panel.reveal(column);
            return ModellerPanel.currentPanel;
        }

        // Create a new panel
        const panel = vscode.window.createWebviewPanel(
            ModellerPanel.viewType,
            'SysML Modeller',
            column,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [
                    vscode.Uri.joinPath(extensionUri, 'dist'),
                    vscode.Uri.joinPath(extensionUri, 'media'),
                ]
            }
        );

        ModellerPanel.currentPanel = new ModellerPanel(panel, extensionUri);
        return ModellerPanel.currentPanel;
    }

    /**
     * Get LSP client from the syster-lsp extension
     */
    private async getLspClient(): Promise<any> {
        const lspExtension = vscode.extensions.getExtension('jade-codes.sysml-language-support');
        if (!lspExtension) {
            throw new Error('SysML Language Support extension not found. Please install it first.');
        }
        
        if (!lspExtension.isActive) {
            await lspExtension.activate();
        }
        
        const api = lspExtension.exports;
        if (!api || !api.getClient) {
            throw new Error('LSP extension does not export getClient');
        }
        
        const client = api.getClient();
        if (!client) {
            throw new Error('Language server not connected');
        }
        
        return client;
    }

    /**
     * Refresh the diagram for a specific file
     */
    public async refreshDiagram(uri?: vscode.Uri): Promise<void> {
        try {
            const client = await this.getLspClient();

            // Send custom request to LSP
            const result = await client.sendRequest<DiagramData>('syster/getDiagram', {
                uri: uri?.toString()
            });

            // Forward to webview
            this.panel.webview.postMessage({
                type: 'diagram',
                data: result
            });
        } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            this.panel.webview.postMessage({
                type: 'error',
                message: `Failed to get diagram: ${message}`
            });
        }
    }

    private isSysMLFile(document: vscode.TextDocument): boolean {
        return document.languageId === 'sysml' || document.languageId === 'kerml';
    }

    private handleMessage(message: WebviewMessage): void {
        switch (message.type) {
            case 'ready':
                // Webview is ready, send initial diagram
                const editor = vscode.window.activeTextEditor;
                if (editor && this.isSysMLFile(editor.document)) {
                    this.refreshDiagram(editor.document.uri);
                } else {
                    this.refreshDiagram(); // Get whole workspace
                }
                break;
            case 'refresh':
                this.refreshDiagram(message.uri ? vscode.Uri.parse(message.uri) : undefined);
                break;
            case 'navigate':
                // Navigate to symbol in editor
                if (message.uri && message.position) {
                    const uri = vscode.Uri.parse(message.uri);
                    const position = new vscode.Position(message.position.line, message.position.character);
                    vscode.window.showTextDocument(uri, {
                        selection: new vscode.Range(position, position)
                    });
                }
                break;
            // TODO: Modeller-specific actions
            case 'createElement':
                this.handleCreateElement(message.payload);
                break;
            case 'updateElement':
                this.handleUpdateElement(message.payload);
                break;
            case 'deleteElement':
                this.handleDeleteElement(message.payload);
                break;
            case 'createRelationship':
                this.handleCreateRelationship(message.payload);
                break;
        }
    }

    // TODO: Implement modeller actions
    private async handleCreateElement(payload: unknown): Promise<void> {
        vscode.window.showInformationMessage('Create element: Not yet implemented');
    }

    private async handleUpdateElement(payload: unknown): Promise<void> {
        vscode.window.showInformationMessage('Update element: Not yet implemented');
    }

    private async handleDeleteElement(payload: unknown): Promise<void> {
        vscode.window.showInformationMessage('Delete element: Not yet implemented');
    }

    private async handleCreateRelationship(payload: unknown): Promise<void> {
        vscode.window.showInformationMessage('Create relationship: Not yet implemented');
    }

    private update(): void {
        this.panel.webview.html = this.getHtmlForWebview();
    }

    private getHtmlForWebview(): string {
        const webview = this.panel.webview;

        // Get the bundled React app assets
        const scriptUri = webview.asWebviewUri(
            vscode.Uri.joinPath(this.extensionUri, 'media', 'index.js')
        );
        const styleUri = webview.asWebviewUri(
            vscode.Uri.joinPath(this.extensionUri, 'media', 'index.css')
        );

        // Use a nonce to only allow specific scripts to be run
        const nonce = getNonce();

        // Load the bundled React Flow diagram app
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';">
    <title>SysML Modeller</title>
    <link rel="stylesheet" href="${styleUri}">
</head>
<body>
    <div id="root"></div>
    <script nonce="${nonce}" src="${scriptUri}"></script>
</body>
</html>`;
    }

    public dispose(): void {
        ModellerPanel.currentPanel = undefined;

        this.panel.dispose();

        while (this.disposables.length) {
            const disposable = this.disposables.pop();
            if (disposable) {
                disposable.dispose();
            }
        }
    }
}

function getNonce(): string {
    let text = '';
    const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    for (let i = 0; i < 32; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    return text;
}
