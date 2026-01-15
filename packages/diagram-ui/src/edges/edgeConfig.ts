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
}

// ========== Color Palette ==========

const COLORS = {
  /** Core relationships (specialization, typing, subsetting) */
  core: '#475569',       // slate-600
  /** Structural (composition) */
  structural: '#2563eb', // blue-600
  /** Ports, interfaces, connections, flows */
  interface: '#7c3aed',  // violet-600
  /** Behavioral (perform, exhibit, succession) */
  behavioral: '#059669', // emerald-600
  /** Requirements (satisfy, verify) */
  requirement: '#d97706', // amber-600
} as const;

// ========== Config Builders ==========

/** Solid line with closed arrow */
const solid = (color: string, label?: string): EdgeConfig => ({
  strokeColor: color,
  strokeWidth: 2,
  markerEnd: MarkerType.ArrowClosed,
  ...(label && { label }),
});

/** Dashed line with closed arrow */
const dashed = (color: string, label?: string): EdgeConfig => ({
  strokeColor: color,
  strokeWidth: 2,
  strokeDasharray: '5 5',
  markerEnd: MarkerType.ArrowClosed,
  ...(label && { label }),
});

/** Dashed line with open arrow (subsetting-style) */
const dashedOpen = (color: string, label?: string): EdgeConfig => ({
  strokeColor: color,
  strokeWidth: 2,
  strokeDasharray: '5 5',
  markerEnd: MarkerType.Arrow,
  ...(label && { label }),
});

/** Solid line with open arrow */
const solidOpen = (color: string, label?: string): EdgeConfig => ({
  strokeColor: color,
  strokeWidth: 2,
  markerEnd: MarkerType.Arrow,
  ...(label && { label }),
});

// ========== Edge Configurations ==========

/**
 * SysML v2 edge type configurations.
 * 
 * Organized by category with consistent visual patterns:
 * - Solid lines: Direct relationships (specialization, composition, perform)
 * - Dashed lines: Indirect/constraint relationships (typing, subsetting, satisfy)
 */
export const EDGE_CONFIGS: Record<string, EdgeConfig> = {
  // ==================== Core Relationships (Slate) ====================
  // Inheritance, typing, subsetting
  
  [EDGE_TYPES.SPECIALIZATION]:       solid(COLORS.core, 'specializes'),
  [EDGE_TYPES.TYPING]:               dashedOpen(COLORS.core, ':'),
  [EDGE_TYPES.REDEFINITION]:         dashedOpen(COLORS.core, 'redefines'),
  [EDGE_TYPES.SUBSETTING]:           dashedOpen(COLORS.core, 'subsets'),
  [EDGE_TYPES.REFERENCE_SUBSETTING]: dashedOpen(COLORS.core, 'references'),
  [EDGE_TYPES.CROSS_SUBSETTING]:     dashedOpen(COLORS.core, 'cross-subsets'),
  [EDGE_TYPES.DEPENDENCY]:           dashedOpen(COLORS.core, '«dependency»'),
  [EDGE_TYPES.MEMBERSHIP]:           { ...dashedOpen(COLORS.core), strokeWidth: 1, strokeDasharray: '2 2' },

  // ==================== Structural (Blue) ====================
  // Composition
  
  [EDGE_TYPES.COMPOSITION]:          solid(COLORS.structural),
  // Note: Ideally composition would use a filled diamond marker
  // React Flow doesn't have built-in diamond markers
  
  [EDGE_TYPES.ALLOCATION]:           dashed(COLORS.core, '«allocate»'),

  // ==================== Interfaces (Purple) ====================
  // Connections, bindings, flows, conjugation
  
  [EDGE_TYPES.CONNECTION]:           solid(COLORS.interface, 'connect'),
  [EDGE_TYPES.BINDING]:              solid(COLORS.interface, '='),
  [EDGE_TYPES.FLOW]:                 solidOpen(COLORS.interface, 'flow'),
  [EDGE_TYPES.CONJUGATION]:          solidOpen(COLORS.interface, '~'),
  [EDGE_TYPES.INCLUDE]:              dashed(COLORS.interface, '«include»'),
  [EDGE_TYPES.ASSERT]:               dashed(COLORS.interface, '«assert»'),

  // ==================== Behavioral (Green) ====================
  // Perform, exhibit, succession
  
  [EDGE_TYPES.PERFORM]:              solid(COLORS.behavioral, '«perform»'),
  [EDGE_TYPES.EXHIBIT]:              solid(COLORS.behavioral, '«exhibit»'),
  [EDGE_TYPES.SUCCESSION]:           solid(COLORS.behavioral, 'then'),

  // ==================== Requirements (Orange) ====================
  // Satisfy, verify
  
  [EDGE_TYPES.SATISFY]:              dashed(COLORS.requirement, '«satisfy»'),
  [EDGE_TYPES.VERIFY]:               dashed(COLORS.requirement, '«verify»'),
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
