import React from 'react';
import {
  BaseEdge,
  EdgeLabelRenderer,
  getSmoothStepPath,
  type EdgeProps,
} from '@xyflow/react';
import { EDGE_TYPES } from '@syster/diagram-core';
import { getEdgeConfig, type EdgeConfig } from './edgeConfig';

/**
 * Props for SysML edge components.
 * Extends React Flow EdgeProps with optional SysML-specific data.
 */
export interface SysMLEdgeProps extends EdgeProps {
  data?: {
    /** Override label from config */
    label?: string;
    /** Multiplicity for composition edges */
    multiplicity?: string;
  };
}

/**
 * Generate a unique displayName for an edge type.
 * Converts SCREAMING_SNAKE_CASE to PascalCase with "Edge" suffix.
 * 
 * @param edgeType - Edge type in SCREAMING_SNAKE_CASE format
 * @returns Display name in PascalCase format (e.g., "SpecializationEdge")
 * 
 * @example
 * generateDisplayName('SPECIALIZATION') // => 'SpecializationEdge'
 * generateDisplayName('CROSS_SUBSETTING') // => 'CrossSubsettingEdge'
 */
function generateDisplayName(edgeType: string): string {
  return edgeType
    .split('_')
    .map(part => part.charAt(0).toUpperCase() + part.slice(1).toLowerCase())
    .join('') + 'Edge';
}

/**
 * Factory function to create a styled SysML edge component.
 *
 * @param config - Visual configuration for the edge
 * @param edgeType - Type of edge for displayName
 * @returns A React component for rendering the edge
 */
export function createSysMLEdge(config: EdgeConfig, edgeType?: string): React.FC<SysMLEdgeProps> {
  const SysMLEdge: React.FC<SysMLEdgeProps> = ({
    id,
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
    data,
    selected,
  }) => {
    const [edgePath, labelX, labelY] = getSmoothStepPath({
      sourceX,
      sourceY,
      targetX,
      targetY,
      sourcePosition,
      targetPosition,
      borderRadius: 8,
    });

    // Use data label if provided, otherwise use config label
    const label = data?.label ?? config.label;
    const multiplicity = data?.multiplicity;

    // Highlight on selection
    const strokeColor = selected ? '#3b82f6' : config.strokeColor;
    const strokeWidth = selected ? config.strokeWidth + 1 : config.strokeWidth;

    return (
      <>
        <BaseEdge
          id={id}
          path={edgePath}
          style={{
            stroke: strokeColor,
            strokeWidth,
            strokeDasharray: config.strokeDasharray,
          }}
          markerEnd={config.markerEnd ? `url(#${config.markerEnd})` : undefined}
        />
        {(label || multiplicity) && (
          <EdgeLabelRenderer>
            <div
              style={{
                position: 'absolute',
                transform: `translate(-50%, -50%) translate(${labelX}px, ${labelY}px)`,
                background: 'white',
                padding: '2px 6px',
                borderRadius: '4px',
                fontSize: '11px',
                color: config.strokeColor,
                border: `1px solid ${config.strokeColor}`,
                pointerEvents: 'all',
              }}
              className="nodrag nopan"
            >
              {label}
              {multiplicity && <span style={{ marginLeft: 4 }}>[{multiplicity}]</span>}
            </div>
          </EdgeLabelRenderer>
        )}
      </>
    );
  };

  // Create unique displayName for debugging
  SysMLEdge.displayName = edgeType ? generateDisplayName(edgeType) : 'SysMLEdge';
  return SysMLEdge;
}

/**
 * Pre-built edge components for all SysML edge types.
 * Use this map with React Flow's edgeTypes prop.
 *
 * @example
 * ```tsx
 * import { edgeTypes } from '@syster/diagram-ui';
 *
 * <ReactFlow
 *   edges={edges}
 *   edgeTypes={edgeTypes}
 * />
 * ```
 */
export const edgeTypes: Record<string, React.FC<SysMLEdgeProps>> = Object.fromEntries(
  Object.values(EDGE_TYPES).map((edgeType) => [
    edgeType,
    createSysMLEdge(getEdgeConfig(edgeType), edgeType),
  ])
);
