import React from 'react';
import type { SymbolData } from '@syster/diagram-core';
import { DefinitionNode } from './DefinitionNode';

interface PartDefNodeProps {
  id: string;
  data: SymbolData;
}

/**
 * Custom node component for SysML Part Definition.
 * Displays the part name, stereotype, and features.
 */
export const PartDefNode: React.FC<PartDefNodeProps> = ({ id, data }) => {
  return (
    <DefinitionNode
      id={id}
      data={data}
      borderColor="#2563eb"
      stereotype="part def"
      showFeatures={true}
    />
  );
};
