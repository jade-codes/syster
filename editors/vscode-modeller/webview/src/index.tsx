import React, { useState, useCallback, useEffect } from 'react';
import { createRoot } from 'react-dom/client';
import {
  ReactFlow,
  ReactFlowProvider,
  Controls,
  Background,
  BackgroundVariant,
  MiniMap,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
  Node,
  Edge,
  OnConnect,
} from '@xyflow/react';
import { nodeTypes, edgeTypes } from '@syster/diagram-ui';
import type { SymbolData } from '@syster/diagram-core';
import '@xyflow/react/dist/style.css';

// VS Code API for messaging
declare function acquireVsCodeApi(): {
  postMessage(message: unknown): void;
  getState(): unknown;
  setState(state: unknown): void;
};

const vscode = acquireVsCodeApi();

// Types from LSP
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

// Convert LSP symbol to React Flow node
function symbolToNode(symbol: DiagramSymbol, index: number): Node<SymbolData> {
  const cols = 4;
  const nodeWidth = 180;
  const nodeHeight = 100;
  const gap = 40;
  
  const row = Math.floor(index / cols);
  const col = index % cols;
  
  // Determine node type based on kind
  let nodeType = 'default';
  if (symbol.kind === 'Definition' && symbol.definitionKind) {
    nodeType = `${symbol.definitionKind}Def`;
  } else if (symbol.kind === 'Usage' && symbol.usageKind) {
    nodeType = `${symbol.usageKind}Usage`;
  }
  
  return {
    id: symbol.qualifiedName.replace(/::/g, '_'),
    type: nodeType,
    position: { 
      x: col * (nodeWidth + gap) + gap, 
      y: row * (nodeHeight + gap) + gap 
    },
    data: {
      name: symbol.name,
      qualifiedName: symbol.qualifiedName,
      features: symbol.features,
      typedBy: symbol.typedBy,
      direction: symbol.direction as 'in' | 'out' | 'inout' | undefined,
    },
  };
}

// Convert LSP relationship to React Flow edge
function relationshipToEdge(rel: DiagramRelationship, index: number): Edge {
  return {
    id: `edge_${index}`,
    source: rel.source.replace(/::/g, '_'),
    target: rel.target.replace(/::/g, '_'),
    type: rel.type,
    label: rel.type,
  };
}

function ModellerApp() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [stats, setStats] = useState({ nodes: 0, edges: 0 });

  // Handle new connections (creating relationships)
  const onConnect: OnConnect = useCallback(
    (connection: Connection) => {
      setEdges((eds) => addEdge({ ...connection, type: 'Typing' }, eds));
      // Notify VS Code extension about the new relationship
      vscode.postMessage({
        type: 'createRelationship',
        payload: {
          source: connection.source,
          target: connection.target,
          relationshipType: 'Typing', // Default, could be user-selectable
        }
      });
    },
    [setEdges]
  );

  // Handle messages from extension
  useEffect(() => {
    const handleMessage = (event: MessageEvent) => {
      const message = event.data;
      
      switch (message.type) {
        case 'diagram': {
          const data = message.data as DiagramData;
          const newNodes = data.symbols.map((s, i) => symbolToNode(s, i));
          const newEdges = data.relationships.map((r, i) => relationshipToEdge(r, i));
          
          setNodes(newNodes);
          setEdges(newEdges);
          setStats({ nodes: newNodes.length, edges: newEdges.length });
          setLoading(false);
          setError(null);
          break;
        }
        case 'error':
          setError(message.message);
          setLoading(false);
          break;
      }
    };

    window.addEventListener('message', handleMessage);
    return () => window.removeEventListener('message', handleMessage);
  }, [setNodes, setEdges]);

  // Request initial data
  useEffect(() => {
    vscode.postMessage({ type: 'ready' });
  }, []);

  const handleRefresh = useCallback(() => {
    setLoading(true);
    vscode.postMessage({ type: 'refresh' });
  }, []);

  const handleAddNode = useCallback(() => {
    // TODO: Open dialog to select node type
    vscode.postMessage({
      type: 'createElement',
      payload: {
        kind: 'Definition',
        definitionKind: 'Part',
        name: 'NewPart',
      }
    });
  }, []);

  if (error) {
    return (
      <div className="error-container">
        <div className="error-message">{error}</div>
        <button onClick={handleRefresh}>Retry</button>
      </div>
    );
  }

  return (
    <div className="diagram-app">
      <div className="toolbar">
        <button onClick={handleRefresh} disabled={loading}>
          {loading ? 'Loading...' : '↻ Refresh'}
        </button>
        <button onClick={handleAddNode} disabled={loading}>
          + Add Node
        </button>
        <span className="stats">
          {stats.nodes} nodes, {stats.edges} edges
        </span>
        <span className="mode-indicator">Edit Mode</span>
      </div>
      <div className="diagram-container">
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          nodeTypes={nodeTypes}
          edgeTypes={edgeTypes}
          fitView
          minZoom={0.1}
          maxZoom={2}
          // Enable editing features
          nodesDraggable={true}
          nodesConnectable={true}
          elementsSelectable={true}
          deleteKeyCode={['Backspace', 'Delete']}
        >
          <Controls />
          <MiniMap />
          <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
        </ReactFlow>
      </div>
    </div>
  );
}

// Styles
const styles = document.createElement('style');
styles.textContent = `
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }
  
  html, body, #root {
    height: 100%;
    width: 100%;
    overflow: hidden;
  }
  
  body {
    font-family: var(--vscode-font-family, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif);
    background: var(--vscode-editor-background, #1e1e1e);
    color: var(--vscode-editor-foreground, #d4d4d4);
  }
  
  .diagram-app {
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  .toolbar {
    padding: 8px 12px;
    border-bottom: 1px solid var(--vscode-panel-border, #3c3c3c);
    display: flex;
    gap: 12px;
    align-items: center;
    background: var(--vscode-editor-background, #1e1e1e);
  }
  
  .toolbar button {
    background: var(--vscode-button-background, #0e639c);
    color: var(--vscode-button-foreground, #ffffff);
    border: none;
    padding: 6px 14px;
    border-radius: 2px;
    cursor: pointer;
    font-size: 13px;
  }
  
  .toolbar button:hover:not(:disabled) {
    background: var(--vscode-button-hoverBackground, #1177bb);
  }
  
  .toolbar button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .stats {
    font-size: 12px;
    color: var(--vscode-descriptionForeground, #8c8c8c);
  }
  
  .mode-indicator {
    margin-left: auto;
    font-size: 11px;
    padding: 2px 8px;
    background: var(--vscode-badge-background, #4d4d4d);
    color: var(--vscode-badge-foreground, #ffffff);
    border-radius: 3px;
  }
  
  .diagram-container {
    flex: 1;
    position: relative;
  }
  
  .error-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 16px;
  }
  
  .error-message {
    color: var(--vscode-errorForeground, #f48771);
    padding: 16px;
    max-width: 400px;
    text-align: center;
  }
  
  /* React Flow overrides for VS Code theme */
  .react-flow__node {
    background: var(--vscode-editor-background, #1e1e1e);
    border: 2px solid var(--vscode-textLink-foreground, #3794ff);
    border-radius: 8px;
    padding: 10px;
    font-size: 12px;
  }
  
  .react-flow__node.selected {
    border-color: var(--vscode-focusBorder, #007fd4);
    box-shadow: 0 0 0 2px var(--vscode-focusBorder, #007fd4);
  }
  
  .react-flow__edge-path {
    stroke: var(--vscode-textLink-foreground, #3794ff);
  }
  
  .react-flow__controls {
    background: var(--vscode-editor-background, #1e1e1e);
    border: 1px solid var(--vscode-panel-border, #3c3c3c);
    border-radius: 4px;
  }
  
  .react-flow__controls-button {
    background: var(--vscode-editor-background, #1e1e1e);
    border-bottom: 1px solid var(--vscode-panel-border, #3c3c3c);
    fill: var(--vscode-editor-foreground, #d4d4d4);
  }
  
  .react-flow__controls-button:hover {
    background: var(--vscode-list-hoverBackground, #2a2d2e);
  }
  
  .react-flow__background {
    background: var(--vscode-editor-background, #1e1e1e);
  }
  
  .react-flow__minimap {
    background: var(--vscode-editor-background, #1e1e1e);
    border: 1px solid var(--vscode-panel-border, #3c3c3c);
    border-radius: 4px;
  }
`;
document.head.appendChild(styles);

// Mount app
const root = createRoot(document.getElementById('root')!);
root.render(
  <ReactFlowProvider>
    <ModellerApp />
  </ReactFlowProvider>
);
