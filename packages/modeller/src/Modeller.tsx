import React, { useCallback } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
  Edge,
  BackgroundVariant,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { Palette } from './components/Palette';
import { PropertiesPanel } from './components/PropertiesPanel';
import { useLspConnection } from './hooks/useLspConnection';
import './Modeller.css';

/**
 * Main Modeller component that provides a visual modelling canvas
 * for SysML v2 diagrams with editing capabilities.
 */
export const Modeller: React.FC = () => {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const lspConnection = useLspConnection();

  const onConnect = useCallback(
    (params: Edge | Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  return (
    <div className="modeller">
      <Palette />
      <div className="modeller-canvas">
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          fitView
        >
          <Controls />
          <MiniMap />
          <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
        </ReactFlow>
      </div>
      <PropertiesPanel />
    </div>
  );
};
