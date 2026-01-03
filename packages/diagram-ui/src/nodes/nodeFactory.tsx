import { FC } from 'react';
import { NodeProps } from '@xyflow/react';
import { DefinitionNode } from './DefinitionNode';
import { NodeConfig, NODE_CONFIGS } from './nodeConfig';
import type { SysMLNodeData } from '@syster/diagram-core';

/**
 * Creates a definition node component from configuration.
 * 
 * This factory function generates React Flow node components for each
 * SysML node type, using the provided configuration to customize
 * appearance (border color, stereotype label, etc.).
 * 
 * @param config - Visual configuration for the node type
 * @returns A React Flow node component
 * 
 * @example
 * ```tsx
 * const MyPartDefNode = createDefinitionNode({
 *   borderColor: '#2563eb',
 *   stereotype: 'part def',
 *   showFeatures: true,
 * });
 * ```
 */
export function createDefinitionNode(config: NodeConfig): FC<NodeProps<SysMLNodeData>> {
  const NodeComponent: FC<NodeProps<SysMLNodeData>> = ({ id, data }) => (
    <DefinitionNode
      id={id}
      data={data}
      borderColor={config.borderColor}
      stereotype={config.stereotype}
      showFeatures={config.showFeatures}
      showDirection={config.showDirection}
    />
  );
  NodeComponent.displayName = `${config.stereotype.replace(/\s+/g, '')}Node`;
  return NodeComponent;
}

/**
 * Generated React Flow node types map.
 * 
 * Maps each SysML node type string to its corresponding React component.
 * Use this object directly with React Flow's `nodeTypes` prop.
 * 
 * @example
 * ```tsx
 * import { nodeTypes } from '@syster/diagram-ui';
 * 
 * <ReactFlow
 *   nodes={nodes}
 *   edges={edges}
 *   nodeTypes={nodeTypes}
 * />
 * ```
 */
export const nodeTypes: Record<string, FC<NodeProps<SysMLNodeData>>> = Object.fromEntries(
  Object.entries(NODE_CONFIGS).map(([type, config]) => [type, createDefinitionNode(config)])
);

/**
 * Gets the configuration for a specific node type.
 * 
 * @param nodeType - The SysML node type string
 * @returns The node configuration, or undefined if not found
 */
export function getNodeConfig(nodeType: string): NodeConfig | undefined {
  return NODE_CONFIGS[nodeType];
}
