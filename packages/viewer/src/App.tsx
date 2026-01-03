import React from 'react';
import { Viewer } from './Viewer';
import { NODE_TYPES } from '@syster/diagram-core';

/**
 * Demo application wrapper for the Viewer component.
 * Shows a sample SysML diagram for development.
 */
export const App: React.FC = () => {
  // Sample diagram with Vehicle hierarchy
  const sampleDiagram = {
    nodes: [
      {
        id: "Vehicle",
        type: NODE_TYPES.PART_DEF,
        position: { x: 250, y: 50 },
        data: {
          id: "Vehicle",
          name: "Vehicle",
          qualifiedName: "Automotive::Vehicle",
          type: NODE_TYPES.PART_DEF,
          features: ["speed", "mass"]
        }
      },
      {
        id: "Car",
        type: NODE_TYPES.PART_DEF,
        position: { x: 100, y: 200 },
        data: {
          id: "Car",
          name: "Car",
          qualifiedName: "Automotive::Car",
          type: NODE_TYPES.PART_DEF,
          features: ["numWheels"]
        }
      },
      {
        id: "Truck",
        type: NODE_TYPES.PART_DEF,
        position: { x: 400, y: 200 },
        data: {
          id: "Truck",
          name: "Truck",
          qualifiedName: "Automotive::Truck",
          type: NODE_TYPES.PART_DEF,
          features: ["cargoCapacity"]
        }
      },
      {
        id: "engine",
        type: NODE_TYPES.PORT_DEF,
        position: { x: 250, y: 350 },
        data: {
          id: "engine",
          name: "engine",
          qualifiedName: "Car::engine",
          type: NODE_TYPES.PORT_DEF,
          direction: "inout"
        }
      }
    ],
    edges: [
      {
        id: "edge1",
        source: "Car",
        target: "Vehicle"
      },
      {
        id: "edge2",
        source: "Truck",
        target: "Vehicle"
      }
    ]
  };

  return (
    <div style={{ width: '100vw', height: '100vh' }}>
      <Viewer diagram={sampleDiagram} />
    </div>
  );
};

export default App;
