import React from 'react';
import type { SymbolData } from '@syster/diagram-core';
import { DefinitionNode } from './DefinitionNode';

export interface PortDefNodeProps {
  id: string;
  data: SymbolData;
}

/**
 * Custom node component for SysML Port Definition.
 * Displays the port name, stereotype, and direction.
 */
export const PortDefNode: React.FC<PortDefNodeProps> = ({ id, data }) => {
  return (
    <DefinitionNode
      id={id}
      data={data}
      borderColor="#7c3aed"
      stereotype="port def"
      showDirection={true}
    />
  );
};
