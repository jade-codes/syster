import { describe, it, expect } from "bun:test";
import {
  convertSymbolToNode,
  convertRelationshipToEdge,
  createDiagramFromWorkspace,
  type WorkspaceData,
  type SymbolData,
  type RelationshipData
} from "../converter";
import { NODE_TYPES } from "../sysml-nodes";
import { EDGE_TYPES } from "../sysml-edges";

describe("Symbol to Node Conversion", () => {
  describe("convertSymbolToNode", () => {
    it("should convert PartDef symbol to PartDefNode", () => {
      const symbol: SymbolData = {
        name: "Vehicle",
        qualifiedName: "Automotive::Vehicle",
        kind: "Definition",
        definitionKind: "Part",
        features: ["speed", "mass"]
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PART_DEF);
      expect(node?.name).toBe("Vehicle");
      expect(node?.qualifiedName).toBe("Automotive::Vehicle");
      if (node && 'features' in node) {
        expect(node.features).toEqual(["speed", "mass"]);
      }
    });

    it("should convert PartUsage symbol to PartUsageNode", () => {
      const symbol: SymbolData = {
        name: "engine",
        qualifiedName: "Car::engine",
        kind: "Usage",
        usageKind: "Part",
        typedBy: "Engine"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PART_USAGE);
      expect(node?.name).toBe("engine");
      if (node && 'typedBy' in node) {
        expect(node.typedBy).toBe("Engine");
      }
    });

    it("should convert PortDef symbol to PortDefNode", () => {
      const symbol: SymbolData = {
        name: "DataPort",
        qualifiedName: "Interfaces::DataPort",
        kind: "Definition",
        definitionKind: "Port",
        direction: "in"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PORT_DEF);
      if (node && 'direction' in node) {
        expect(node.direction).toBe("in");
      }
    });

    it("should default PortDef direction to 'in' when not specified", () => {
      const symbol: SymbolData = {
        name: "DefaultPort",
        qualifiedName: "Interfaces::DefaultPort",
        kind: "Definition",
        definitionKind: "Port"
        // direction is omitted
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PORT_DEF);
      if (node && 'direction' in node) {
        expect(node.direction).toBe("in");
      }
    });

    it("should convert PortDef with 'out' direction", () => {
      const symbol: SymbolData = {
        name: "OutputPort",
        qualifiedName: "Interfaces::OutputPort",
        kind: "Definition",
        definitionKind: "Port",
        direction: "out"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PORT_DEF);
      if (node && 'direction' in node) {
        expect(node.direction).toBe("out");
      }
    });

    it("should convert PortDef with 'inout' direction", () => {
      const symbol: SymbolData = {
        name: "BidirectionalPort",
        qualifiedName: "Interfaces::BidirectionalPort",
        kind: "Definition",
        definitionKind: "Port",
        direction: "inout"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PORT_DEF);
      if (node && 'direction' in node) {
        expect(node.direction).toBe("inout");
      }
    });

    it("should convert PortUsage with explicit 'in' direction", () => {
      const symbol: SymbolData = {
        name: "inputPort",
        qualifiedName: "Component::inputPort",
        kind: "Usage",
        usageKind: "Port",
        direction: "in",
        typedBy: "DataPort"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PORT_USAGE);
      if (node && 'direction' in node) {
        expect(node.direction).toBe("in");
      }
    });

    it("should default PortUsage direction to 'in' when not specified", () => {
      const symbol: SymbolData = {
        name: "defaultPort",
        qualifiedName: "Component::defaultPort",
        kind: "Usage",
        usageKind: "Port",
        typedBy: "DataPort"
        // direction is omitted
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PORT_USAGE);
      if (node && 'direction' in node) {
        expect(node.direction).toBe("in");
      }
    });

    it("should convert PortUsage with 'out' direction", () => {
      const symbol: SymbolData = {
        name: "outputPort",
        qualifiedName: "Component::outputPort",
        kind: "Usage",
        usageKind: "Port",
        direction: "out",
        typedBy: "DataPort"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PORT_USAGE);
      if (node && 'direction' in node) {
        expect(node.direction).toBe("out");
      }
    });

    it("should convert PortUsage with 'inout' direction", () => {
      const symbol: SymbolData = {
        name: "bidirectionalPort",
        qualifiedName: "Component::bidirectionalPort",
        kind: "Usage",
        usageKind: "Port",
        direction: "inout",
        typedBy: "DataPort"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.PORT_USAGE);
      if (node && 'direction' in node) {
        expect(node.direction).toBe("inout");
      }
    });

    it("should convert RequirementDef symbol to RequirementDefNode", () => {
      const symbol: SymbolData = {
        name: "SafetyRequirement",
        qualifiedName: "Requirements::SafetyRequirement",
        kind: "Definition",
        definitionKind: "Requirement",
        text: "Must stop within 100m"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.REQUIREMENT_DEF);
      if (node && 'text' in node) {
        expect(node.text).toBe("Must stop within 100m");
      }
    });

    it("should convert AttributeDef with dataType", () => {
      const symbol: SymbolData = {
        name: "MassAttribute",
        qualifiedName: "Attributes::MassAttribute",
        kind: "Definition",
        definitionKind: "Attribute",
        dataType: "Real"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.ATTRIBUTE_DEF);
      if (node && 'dataType' in node) {
        expect(node.dataType).toBe("Real");
      }
    });

    it("should convert AttributeDef without dataType (undefined)", () => {
      const symbol: SymbolData = {
        name: "GenericAttribute",
        qualifiedName: "Attributes::GenericAttribute",
        kind: "Definition",
        definitionKind: "Attribute"
        // dataType is omitted
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.ATTRIBUTE_DEF);
      if (node && 'dataType' in node) {
        expect(node.dataType).toBeUndefined();
      }
    });

    it("should convert AttributeUsage with dataType", () => {
      const symbol: SymbolData = {
        name: "mass",
        qualifiedName: "Vehicle::mass",
        kind: "Usage",
        usageKind: "Attribute",
        dataType: "Real",
        typedBy: "MassAttribute"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.ATTRIBUTE_USAGE);
      if (node && 'dataType' in node) {
        expect(node.dataType).toBe("Real");
      }
    });

    it("should convert AttributeUsage without dataType (undefined)", () => {
      const symbol: SymbolData = {
        name: "genericAttr",
        qualifiedName: "Component::genericAttr",
        kind: "Usage",
        usageKind: "Attribute",
        typedBy: "GenericAttribute"
        // dataType is omitted
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.ATTRIBUTE_USAGE);
      if (node && 'dataType' in node) {
        expect(node.dataType).toBeUndefined();
      }
    });

    it("should handle symbols without optional fields", () => {
      const symbol: SymbolData = {
        name: "Action",
        qualifiedName: "Actions::Move",
        kind: "Definition",
        definitionKind: "Action"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.ACTION_DEF);
      expect(node?.name).toBe("Action");
    });

    it("should convert ActionUsage with typedBy", () => {
      const symbol: SymbolData = {
        name: "moveAction",
        qualifiedName: "Vehicle::moveAction",
        kind: "Usage",
        usageKind: "Action",
        typedBy: "MoveAction"
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.ACTION_USAGE);
      if (node && 'typedBy' in node) {
        expect(node.typedBy).toBe("MoveAction");
      }
    });

    it("should convert ActionUsage without typedBy (undefined)", () => {
      const symbol: SymbolData = {
        name: "genericAction",
        qualifiedName: "Component::genericAction",
        kind: "Usage",
        usageKind: "Action"
        // typedBy is omitted
      };

      const node = convertSymbolToNode(symbol);

      expect(node?.type).toBe(NODE_TYPES.ACTION_USAGE);
      if (node && 'typedBy' in node) {
        expect(node.typedBy).toBeUndefined();
      }
    });

    it("should convert all definition types", () => {
      const defTypes = [
        { kind: "State", expected: NODE_TYPES.STATE_DEF },
        { kind: "Item", expected: NODE_TYPES.ITEM_DEF },
        { kind: "Concern", expected: NODE_TYPES.CONCERN_DEF },
        { kind: "Case", expected: NODE_TYPES.CASE_DEF },
        { kind: "AnalysisCase", expected: NODE_TYPES.ANALYSIS_CASE_DEF },
        { kind: "VerificationCase", expected: NODE_TYPES.VERIFICATION_CASE_DEF },
        { kind: "UseCase", expected: NODE_TYPES.USE_CASE_DEF },
        { kind: "View", expected: NODE_TYPES.VIEW_DEF },
        { kind: "Viewpoint", expected: NODE_TYPES.VIEWPOINT_DEF },
        { kind: "Rendering", expected: NODE_TYPES.RENDERING_DEF },
        { kind: "Allocation", expected: NODE_TYPES.ALLOCATION_DEF },
        { kind: "Calculation", expected: NODE_TYPES.CALCULATION_DEF },
        { kind: "Connection", expected: NODE_TYPES.CONNECTION_DEF },
        { kind: "Constraint", expected: NODE_TYPES.CONSTRAINT_DEF },
        { kind: "Enumeration", expected: NODE_TYPES.ENUMERATION_DEF },
        { kind: "Flow", expected: NODE_TYPES.FLOW_DEF },
        { kind: "Individual", expected: NODE_TYPES.INDIVIDUAL_DEF },
        { kind: "Interface", expected: NODE_TYPES.INTERFACE_DEF },
        { kind: "Occurrence", expected: NODE_TYPES.OCCURRENCE_DEF },
        { kind: "Metadata", expected: NODE_TYPES.METADATA_DEF },
      ] as const;

      for (const { kind, expected } of defTypes) {
        const symbol: SymbolData = {
          name: `Test${kind}`,
          qualifiedName: `Test::${kind}`,
          kind: "Definition",
          definitionKind: kind
        };

        const node = convertSymbolToNode(symbol);
        expect(node?.type).toBe(expected);
      }
    });

    it("should convert all usage types", () => {
      const usageTypes = [
        { kind: "Item", expected: NODE_TYPES.ITEM_USAGE },
        { kind: "Attribute", expected: NODE_TYPES.ATTRIBUTE_USAGE },
        { kind: "Requirement", expected: NODE_TYPES.REQUIREMENT_USAGE },
        { kind: "Concern", expected: NODE_TYPES.CONCERN_USAGE },
        { kind: "Case", expected: NODE_TYPES.CASE_USAGE },
        { kind: "View", expected: NODE_TYPES.VIEW_USAGE },
        { kind: "Enumeration", expected: NODE_TYPES.ENUMERATION_USAGE },
        { kind: "SatisfyRequirement", expected: NODE_TYPES.SATISFY_REQUIREMENT_USAGE },
        { kind: "PerformAction", expected: NODE_TYPES.PERFORM_ACTION_USAGE },
        { kind: "ExhibitState", expected: NODE_TYPES.EXHIBIT_STATE_USAGE },
        { kind: "IncludeUseCase", expected: NODE_TYPES.INCLUDE_USE_CASE_USAGE },
      ] as const;

      for (const { kind, expected } of usageTypes) {
        const symbol: SymbolData = {
          name: `test${kind}`,
          qualifiedName: `Test::${kind}`,
          kind: "Usage",
          usageKind: kind
        };

        const node = convertSymbolToNode(symbol);
        expect(node?.type).toBe(expected);
      }
    });

    it("should return null for unsupported symbol kinds", () => {
      const unsupportedSymbols: SymbolData[] = [
        { name: "MyPackage", qualifiedName: "MyPackage", kind: "Package" },
        { name: "MyClassifier", qualifiedName: "MyClassifier", kind: "Classifier" },
        { name: "MyFeature", qualifiedName: "MyFeature", kind: "Feature" },
        { name: "MyAlias", qualifiedName: "MyAlias", kind: "Alias" },
      ];

      for (const symbol of unsupportedSymbols) {
        const node = convertSymbolToNode(symbol);
        expect(node).toBeNull();
      }
    });

    it("should return null for invalid definition kinds", () => {
      const symbol: SymbolData = {
        name: "Invalid",
        qualifiedName: "Invalid",
        kind: "Definition",
        definitionKind: "InvalidKind" as any
      };

      const node = convertSymbolToNode(symbol);
      expect(node).toBeNull();
    });

    it("should return null for invalid usage kinds", () => {
      const symbol: SymbolData = {
        name: "Invalid",
        qualifiedName: "Invalid",
        kind: "Usage",
        usageKind: "InvalidKind" as any
      };

      const node = convertSymbolToNode(symbol);
      expect(node).toBeNull();
    });

    it("should return null for Definition without definitionKind", () => {
      const symbol: SymbolData = {
        name: "NoKind",
        qualifiedName: "NoKind",
        kind: "Definition"
        // definitionKind is missing
      } as any;

      const node = convertSymbolToNode(symbol);
      expect(node).toBeNull();
    });

    it("should return null for Usage without usageKind", () => {
      const symbol: SymbolData = {
        name: "NoKind",
        qualifiedName: "NoKind",
        kind: "Usage"
        // usageKind is missing
      } as any;

      const node = convertSymbolToNode(symbol);
      expect(node).toBeNull();
    });
  });
});

describe("Relationship to Edge Conversion", () => {
  describe("convertRelationshipToEdge", () => {
    it("should convert specialization relationship to SpecializationEdge", () => {
      const relationship: RelationshipData = {
        type: "specialization",
        source: "Car",
        target: "Vehicle"
      };

      const edge = convertRelationshipToEdge(relationship);

      expect(edge.type).toBe(EDGE_TYPES.SPECIALIZATION);
      expect(edge.source).toBe("Car");
      expect(edge.target).toBe("Vehicle");
    });

    it("should convert typing relationship to TypingEdge", () => {
      const relationship: RelationshipData = {
        type: "typing",
        source: "myCar",
        target: "Car"
      };

      const edge = convertRelationshipToEdge(relationship);

      expect(edge.type).toBe(EDGE_TYPES.TYPING);
    });

    it("should convert satisfy relationship to SatisfyEdge", () => {
      const relationship: RelationshipData = {
        type: "satisfy",
        source: "BrakingSystem",
        target: "SafetyRequirement"
      };

      const edge = convertRelationshipToEdge(relationship);

      expect(edge.type).toBe(EDGE_TYPES.SATISFY);
    });

    it("should generate unique edge ID", () => {
      const relationship: RelationshipData = {
        type: "specialization",
        source: "Car",
        target: "Vehicle"
      };

      const edge1 = convertRelationshipToEdge(relationship);
      const edge2 = convertRelationshipToEdge(relationship);

      expect(edge1.id).toBeDefined();
      expect(edge2.id).toBeDefined();
      expect(edge1.id).not.toBe(edge2.id);
    });

    it("should convert all relationship types", () => {
      const relationshipTypes = [
        { type: "redefinition", expected: EDGE_TYPES.REDEFINITION },
        { type: "subsetting", expected: EDGE_TYPES.SUBSETTING },
        { type: "reference_subsetting", expected: EDGE_TYPES.REFERENCE_SUBSETTING },
        { type: "cross_subsetting", expected: EDGE_TYPES.CROSS_SUBSETTING },
        { type: "perform", expected: EDGE_TYPES.PERFORM },
        { type: "exhibit", expected: EDGE_TYPES.EXHIBIT },
        { type: "include", expected: EDGE_TYPES.INCLUDE },
        { type: "assert", expected: EDGE_TYPES.ASSERT },
        { type: "verify", expected: EDGE_TYPES.VERIFY },
      ] as const;

      for (const { type, expected } of relationshipTypes) {
        const relationship: RelationshipData = {
          type,
          source: "source1",
          target: "target1"
        };

        const edge = convertRelationshipToEdge(relationship);
        expect(edge.type).toBe(expected);
        expect(edge.source).toBe("source1");
        expect(edge.target).toBe("target1");
      }
    });
  });
});

describe("Workspace to Diagram Conversion", () => {
  describe("createDiagramFromWorkspace", () => {
    it("should convert workspace with symbols and relationships", () => {
      const workspace: WorkspaceData = {
        symbols: [
          {
            name: "Vehicle",
            qualifiedName: "Automotive::Vehicle",
            kind: "Definition",
            definitionKind: "Part",
            features: []
          },
          {
            name: "Car",
            qualifiedName: "Automotive::Car",
            kind: "Definition",
            definitionKind: "Part",
            features: []
          }
        ],
        relationships: [
          {
            type: "specialization",
            source: "Automotive::Car",
            target: "Automotive::Vehicle"
          }
        ]
      };

      const diagram = createDiagramFromWorkspace(workspace);

      expect(diagram.nodes).toHaveLength(2);
      expect(diagram.edges).toHaveLength(1);
      expect(diagram.nodes[0].data.name).toBe("Vehicle");
      expect(diagram.nodes[1].data.name).toBe("Car");
      expect(diagram.edges[0].type).toBe(EDGE_TYPES.SPECIALIZATION);
    });

    it("should handle empty workspace", () => {
      const workspace: WorkspaceData = {
        symbols: [],
        relationships: []
      };

      const diagram = createDiagramFromWorkspace(workspace);

      expect(diagram.nodes).toHaveLength(0);
      expect(diagram.edges).toHaveLength(0);
    });

    it("should handle workspace with only symbols", () => {
      const workspace: WorkspaceData = {
        symbols: [
          {
            name: "Vehicle",
            qualifiedName: "Automotive::Vehicle",
            kind: "Definition",
            definitionKind: "Part",
            features: []
          }
        ],
        relationships: []
      };

      const diagram = createDiagramFromWorkspace(workspace);

      expect(diagram.nodes).toHaveLength(1);
      expect(diagram.edges).toHaveLength(0);
    });

    it("should filter out unsupported symbol kinds", () => {
      const workspace: WorkspaceData = {
        symbols: [
          {
            name: "Vehicle",
            qualifiedName: "Automotive::Vehicle",
            kind: "Definition",
            definitionKind: "Part",
            features: []
          },
          {
            name: "MyPackage",
            qualifiedName: "MyPackage",
            kind: "Package"
          }
        ],
        relationships: []
      };

      const diagram = createDiagramFromWorkspace(workspace);

      // Should only include the Part definition, not the Package
      expect(diagram.nodes).toHaveLength(1);
      expect(diagram.nodes[0].data.name).toBe("Vehicle");
    });

    it("should convert complex workspace with multiple types", () => {
      const workspace: WorkspaceData = {
        symbols: [
          {
            name: "Vehicle",
            qualifiedName: "Automotive::Vehicle",
            kind: "Definition",
            definitionKind: "Part",
            features: []
          },
          {
            name: "engine",
            qualifiedName: "Car::engine",
            kind: "Usage",
            usageKind: "Part",
            typedBy: "Engine"
          },
          {
            name: "SafetyRequirement",
            qualifiedName: "Requirements::SafetyRequirement",
            kind: "Definition",
            definitionKind: "Requirement",
            text: "Must stop within 100m"
          }
        ],
        relationships: [
          {
            type: "typing",
            source: "Car::engine",
            target: "Engine"
          },
          {
            type: "satisfy",
            source: "BrakingSystem",
            target: "Requirements::SafetyRequirement"
          }
        ]
      };

      const diagram = createDiagramFromWorkspace(workspace);

      expect(diagram.nodes).toHaveLength(3);
      expect(diagram.edges).toHaveLength(2);
      
      // Check node types are correct
      const partDef = diagram.nodes.find(n => n.data.name === "Vehicle");
      const partUsage = diagram.nodes.find(n => n.data.name === "engine");
      const reqDef = diagram.nodes.find(n => n.data.name === "SafetyRequirement");

      expect(partDef?.data.type).toBe(NODE_TYPES.PART_DEF);
      expect(partUsage?.data.type).toBe(NODE_TYPES.PART_USAGE);
      expect(reqDef?.data.type).toBe(NODE_TYPES.REQUIREMENT_DEF);
    });
  });
});
