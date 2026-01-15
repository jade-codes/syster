"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.DiagramPanel = void 0;
const vscode = __importStar(require("vscode"));
/**
 * Manages the SysML Diagram webview panel
 */
class DiagramPanel {
    static currentPanel;
    static viewType = 'systerDiagram';
    panel;
    extensionUri;
    disposables = [];
    constructor(panel, extensionUri) {
        this.panel = panel;
        this.extensionUri = extensionUri;
        // Set the webview's initial html content
        this.update();
        // Listen for when the panel is disposed
        this.panel.onDidDispose(() => this.dispose(), null, this.disposables);
        // Handle messages from the webview
        this.panel.webview.onDidReceiveMessage((message) => this.handleMessage(message), null, this.disposables);
        // Update diagram when active editor changes
        vscode.window.onDidChangeActiveTextEditor((editor) => {
            if (editor && this.isSysMLFile(editor.document)) {
                this.refreshDiagram(editor.document.uri);
            }
        }, null, this.disposables);
    }
    /**
     * Create or show the diagram panel
     */
    static createOrShow(extensionUri) {
        const column = vscode.ViewColumn.Beside;
        // If we already have a panel, show it
        if (DiagramPanel.currentPanel) {
            DiagramPanel.currentPanel.panel.reveal(column);
            return DiagramPanel.currentPanel;
        }
        // Create a new panel
        const panel = vscode.window.createWebviewPanel(DiagramPanel.viewType, 'SysML Diagram', column, {
            enableScripts: true,
            retainContextWhenHidden: true,
            localResourceRoots: [
                vscode.Uri.joinPath(extensionUri, 'dist'),
                vscode.Uri.joinPath(extensionUri, 'media'),
            ]
        });
        DiagramPanel.currentPanel = new DiagramPanel(panel, extensionUri);
        return DiagramPanel.currentPanel;
    }
    /**
     * Get LSP client from the syster-lsp extension
     */
    async getLspClient() {
        console.log('[DiagramPanel] Looking for jade-codes.syster-lsp extension...');
        const lspExtension = vscode.extensions.getExtension('jade-codes.syster-lsp');
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
    async refreshDiagram(uri) {
        try {
            console.log('[DiagramPanel] Refreshing diagram for:', uri?.toString() || 'whole workspace');
            const client = await this.getLspClient();
            console.log('[DiagramPanel] Got LSP client');
            // Send custom request to LSP
            const result = await client.sendRequest('syster/getDiagram', {
                uri: uri?.toString()
            });
            console.log('[DiagramPanel] LSP response:', JSON.stringify(result, null, 2));
            // Forward to webview
            this.panel.webview.postMessage({
                type: 'diagram',
                data: result
            });
            console.log('[DiagramPanel] Sent diagram to webview');
        }
        catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            this.panel.webview.postMessage({
                type: 'error',
                message: `Failed to get diagram: ${message}`
            });
        }
    }
    isSysMLFile(document) {
        return document.languageId === 'sysml' || document.languageId === 'kerml';
    }
    handleMessage(message) {
        switch (message.type) {
            case 'ready':
                // Webview is ready, send initial diagram
                const editor = vscode.window.activeTextEditor;
                if (editor && this.isSysMLFile(editor.document)) {
                    this.refreshDiagram(editor.document.uri);
                }
                else {
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
        }
    }
    update() {
        this.panel.webview.html = this.getHtmlForWebview();
    }
    getHtmlForWebview() {
        const webview = this.panel.webview;
        // Get the bundled React app assets
        const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(this.extensionUri, 'media', 'index.js'));
        const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(this.extensionUri, 'media', 'index.css'));
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
    dispose() {
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
exports.DiagramPanel = DiagramPanel;
function getNonce() {
    let text = '';
    const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    for (let i = 0; i < 32; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    return text;
}
//# sourceMappingURL=diagram-panel.js.map