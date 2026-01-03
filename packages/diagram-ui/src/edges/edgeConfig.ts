import { EDGE_TYPES } from '@syster/diagram-core';
import { MarkerType } from '@xyflow/react';

/**
 * Configuration for a SysML edge's visual appearance.
 */
export interface EdgeConfig {
  /** Stroke color for the edge */
  strokeColor: string;
  /** Stroke width in pixels */
  strokeWidth: number;
  /** Dash pattern (solid if undefined) */
  strokeDasharray?: string;
  /** Label to display on edge (typically relationship stereotype) */
  label?: string;
  /** Marker type at the end of the edge */
  markerEnd: MarkerType;
  /** Whether the edge should be animated */
  animated?: boolean;
}

/**
 * SysML v2 edge type configurations.
 *
 * Visual conventions:
 * - Specialization: Solid line with hollow triangle (using ArrowClosed as approximation)
 * - Composition: Solid line with filled diamond marker
 * - Typing: Dashed line with open arrow
 * - Subsetting/Redefinition: Dashed line with open arrow
 * - Requirements (satisfy/verify): Dashed line
 * - Behavioral (perform/exhibit): Solid line with arrow
 *
 * Color scheme:
 * - Slate (#475569): Core relationships (specialization, typing)
 * - Blue (#2563eb): Structural (composition)
 * - Orange (#d97706): Requirements (satisfy, verify)
 * - Green (#059669): Behavioral (perform, exhibit)
 * - Purple (#7c3aed): Cases (include, assert)
 */
export const EDGE_CONFIGS: Record<string, EdgeConfig> = {
  // ========== Core Relationships ==========

  [EDGE_TYPES.SPECIALIZATION]: {
    strokeColor: '#475569',
    strokeWidth: 2,
    markerEnd: MarkerType.ArrowClosed,
    label: 'specializes',
  },

  [EDGE_TYPES.TYPING]: {
    strokeColor: '#475569',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.Arrow,
    label: ':',
  },

  [EDGE_TYPES.REDEFINITION]: {
    strokeColor: '#475569',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.Arrow,
    label: 'redefines',
  },

  [EDGE_TYPES.SUBSETTING]: {
    strokeColor: '#475569',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.Arrow,
    label: 'subsets',
  },

  [EDGE_TYPES.REFERENCE_SUBSETTING]: {
    strokeColor: '#475569',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.Arrow,
    label: 'references',
  },

  [EDGE_TYPES.CROSS_SUBSETTING]: {
    strokeColor: '#475569',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.Arrow,
    label: 'subsets',
  },

  // ========== Structural ==========

  [EDGE_TYPES.COMPOSITION]: {
    strokeColor: '#2563eb',
    strokeWidth: 2,
    markerEnd: MarkerType.ArrowClosed,
    // Note: Ideally this would use a filled diamond marker
    // React Flow doesn't have built-in diamond markers
  },

  // ========== Requirements ==========

  [EDGE_TYPES.SATISFY]: {
    strokeColor: '#d97706',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.ArrowClosed,
    label: '«satisfy»',
  },

  [EDGE_TYPES.VERIFY]: {
    strokeColor: '#d97706',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.ArrowClosed,
    label: '«verify»',
  },

  // ========== Behavioral ==========

  [EDGE_TYPES.PERFORM]: {
    strokeColor: '#059669',
    strokeWidth: 2,
    markerEnd: MarkerType.ArrowClosed,
    label: '«perform»',
  },

  [EDGE_TYPES.EXHIBIT]: {
    strokeColor: '#059669',
    strokeWidth: 2,
    markerEnd: MarkerType.ArrowClosed,
    label: '«exhibit»',
  },

  // ========== Cases ==========

  [EDGE_TYPES.INCLUDE]: {
    strokeColor: '#7c3aed',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.ArrowClosed,
    label: '«include»',
  },

  [EDGE_TYPES.ASSERT]: {
    strokeColor: '#7c3aed',
    strokeWidth: 2,
    strokeDasharray: '5 5',
    markerEnd: MarkerType.ArrowClosed,
    label: '«assert»',
  },
};

/**
 * Get the configuration for a specific edge type.
 * Returns a default config if the edge type is not found.
 */
export function getEdgeConfig(edgeType: string): EdgeConfig {
  return EDGE_CONFIGS[edgeType] ?? {
    strokeColor: '#475569',
    strokeWidth: 2,
    markerEnd: MarkerType.ArrowClosed,
  };
}
