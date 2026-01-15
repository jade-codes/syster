import * as vscode from 'vscode';
/**
 * Manages the SysML Diagram webview panel
 */
export declare class DiagramPanel {
    static currentPanel: DiagramPanel | undefined;
    private static readonly viewType;
    private readonly panel;
    private readonly extensionUri;
    private disposables;
    private constructor();
    /**
     * Create or show the diagram panel
     */
    static createOrShow(extensionUri: vscode.Uri): DiagramPanel;
    /**
     * Get LSP client from the syster-lsp extension
     */
    private getLspClient;
    /**
     * Refresh the diagram for a specific file
     */
    refreshDiagram(uri?: vscode.Uri): Promise<void>;
    private isSysMLFile;
    private handleMessage;
    private update;
    private getHtmlForWebview;
    dispose(): void;
}
//# sourceMappingURL=diagram-panel.d.ts.map