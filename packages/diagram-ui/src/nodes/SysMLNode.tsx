import React from 'react';
import { Handle, Position } from '@xyflow/react';
import type { SymbolData } from '@syster/diagram-core';

export interface SysMLNodeProps {
  id: string;
  data: SymbolData;
  borderColor: string;
  stereotype: string;
  showFeatures?: boolean;
  showDirection?: boolean;
}

/**
 * Base node component for SysML definitions and usages.
 * Displays stereotype, name, and optional features or direction.
 */
export const SysMLNode: React.FC<SysMLNodeProps> = ({
  data,
  borderColor,
  stereotype,
  showFeatures = false,
  showDirection = false,
}) => {
  const features = (data.features as string[]) || [];
  const direction = data.direction as string | undefined;

  return (
    <div style={{
      padding: '10px',
      border: `2px solid ${borderColor}`,
      borderRadius: '8px',
      background: 'white',
      minWidth: '100px',
      maxWidth: '200px',
    }}>
      <Handle type="target" position={Position.Top} />
      
      <div style={{
        fontSize: '11px',
        color: '#666',
        marginBottom: '4px',
        fontStyle: 'italic',
      }}>
        «{stereotype}»
      </div>
      
      <div style={{
        fontSize: '14px',
        fontWeight: 'bold',
        marginBottom: (showFeatures && features.length > 0) || (showDirection && direction) ? '8px' : '0',
      }}>
        {data.name}
      </div>
      
      {showFeatures && features.length > 0 && (
        <div style={{
          borderTop: '1px solid #e5e7eb',
          paddingTop: '8px',
          fontSize: '12px',
        }}>
          {features.map((feature, index) => (
            <div key={index} style={{ padding: '2px 0' }}>
              {feature}
            </div>
          ))}
        </div>
      )}
      
      {showDirection && direction && (
        <div style={{
          fontSize: '11px',
          color: borderColor,
          fontWeight: '600',
        }}>
          {direction}
        </div>
      )}
      
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};
