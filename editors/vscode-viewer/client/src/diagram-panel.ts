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
}

/**
 * Manages the SysML Diagram webview panel
 */
export class DiagramPanel {
    public static currentPanel: DiagramPanel | undefined;
    private static readonly viewType = 'systerDiagram';

    private readonly panel: vscode.WebviewPanel;
    private readonly extensionUri: vscode.Uri;
    private disposables: vscode.Disposable[] = [];
    private isDisposed = false;

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
     * Create or show the diagram panel
     */
    public static createOrShow(extensionUri: vscode.Uri): DiagramPanel {
        const column = vscode.ViewColumn.Beside;

        // If we already have a panel, show it
        if (DiagramPanel.currentPanel) {
            DiagramPanel.currentPanel.panel.reveal(column);
            return DiagramPanel.currentPanel;
        }

        // Create a new panel
        const panel = vscode.window.createWebviewPanel(
            DiagramPanel.viewType,
            'SysML Diagram',
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

        DiagramPanel.currentPanel = new DiagramPanel(panel, extensionUri);
        return DiagramPanel.currentPanel;
    }

    /**
     * Get LSP client from the syster-lsp extension
     */
    private async getLspClient(): Promise<any> {
        console.log('[DiagramPanel] Looking for jade-codes.sysml-language-support extension...');
        const lspExtension = vscode.extensions.getExtension('jade-codes.sysml-language-support');
        if (!lspExtension) {
            console.error('[DiagramPanel] LSP extension NOT FOUND');
            throw new Error('SysML Language Support extension not found. Please install it first.');
        }
        console.log('[DiagramPanel] Found LSP extension, isActive:', lspExtension.isActive);
        
        if (!lspExtension.isActive) {
            console.log('[DiagramPanel] Activating LSP extension...');
            await lspExtension.activate();
        }
        
        const api = lspExtension.exports;
        console.log('[DiagramPanel] LSP exports:', Object.keys(api || {}));
        if (!api || !api.getClient) {
            console.error('[DiagramPanel] LSP extension does not export getClient');
            throw new Error('LSP extension does not export getClient');
        }
        
        const client = api.getClient();
        if (!client) {
            console.error('[DiagramPanel] Language server not connected');
            throw new Error('Language server not connected');
        }
        console.log('[DiagramPanel] Got LSP client successfully');
        
        return client;
    }

    /**
     * Refresh the diagram for a specific file
     */
    public async refreshDiagram(uri?: vscode.Uri): Promise<void> {
        if (this.isDisposed) {
            return;
        }
        try {
            console.log('[DiagramPanel] Refreshing diagram for:', uri?.toString() || 'whole workspace');
            const client = await this.getLspClient();
            
            // Check again after async operation
            if (this.isDisposed) {
                return;
            }
            console.log('[DiagramPanel] Got LSP client');

            // Send custom request to LSP
            const result: DiagramData = await client.sendRequest('syster/getDiagram', {
                uri: uri?.toString()
            });
            
            // Check again after async operation
            if (this.isDisposed) {
                return;
            }
            console.log('[DiagramPanel] LSP response:', JSON.stringify(result, null, 2));

            // Forward to webview
            this.panel.webview.postMessage({
                type: 'diagram',
                data: result
            });
            console.log('[DiagramPanel] Sent diagram to webview');
        } catch (error) {
            if (this.isDisposed) {
                return;
            }
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
        if (this.isDisposed) {
            return;
        }
        switch (message.type) {
            case 'ready':
                // Webview is ready, send initial diagram for current file only
                const editor = vscode.window.activeTextEditor;
                if (editor && this.isSysMLFile(editor.document)) {
                    this.refreshDiagram(editor.document.uri);
                } else {
                    // No SysML file open - show empty state
                    this.panel.webview.postMessage({
                        type: 'diagram',
                        data: { symbols: [], relationships: [] }
                    });
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
        }
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
    <title>SysML Diagram</title>
    <link rel="stylesheet" href="${styleUri}">
</head>
<body>
    <div id="root"></div>
    <script nonce="${nonce}" src="${scriptUri}"></script>
</body>
</html>`;
    }

    public dispose(): void {
        this.isDisposed = true;
        DiagramPanel.currentPanel = undefined;

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
