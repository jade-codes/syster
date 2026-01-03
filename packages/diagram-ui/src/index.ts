/**
 * @syster/diagram-ui
 * 
 * Shared React components for SysML v2 diagram visualization.
 * Used by both the viewer (read-only) and modeller (editable) packages.
 */

// Node components
export {
  DefinitionNode,
  PartDefNode,
  PortDefNode,
} from './nodes';

export type {
  DefinitionNodeProps,
  PartDefNodeProps,
  PortDefNodeProps,
} from './nodes';

// Node factory and configuration
export { NODE_CONFIGS } from './nodes/nodeConfig';
export type { NodeConfig } from './nodes/nodeConfig';

export {
  createDefinitionNode,
  nodeTypes,
  getNodeConfig,
} from './nodes/nodeFactory';
