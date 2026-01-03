// Mock Handle component for isolated node component tests
// In actual ReactFlow context, real Handles work fine and are needed for edges
export const Handle = () => null;
export const Position = {
  Top: 'top',
  Bottom: 'bottom',
  Left: 'left',
  Right: 'right',
} as const;
