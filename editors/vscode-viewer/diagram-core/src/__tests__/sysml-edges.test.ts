import { describe, it, expect } from "bun:test";
import {
  EDGE_TYPES,
  isRelationshipEdge,
  isStructuralEdge,
  type SpecializationEdge,
  type RedefinitionEdge,
  type SubsettingEdge,
  type TypingEdge,
  type ReferenceSubsettingEdge,
  type CrossSubsettingEdge,
  type ConjugationEdge,
  type SatisfyEdge,
  type PerformEdge,
  type ExhibitEdge,
  type IncludeEdge,
  type AssertEdge,
  type VerifyEdge,
  type DependencyEdge,
  type CompositionEdge,
  type ConnectionEdge,
  type BindingEdge,
  type AllocationEdge,
  type FlowEdge,
  type SuccessionEdge,
  type MembershipEdge,
} from "../sysml-edges";

describe("SysML Core Relationship Edges", () => {
  describe("SpecializationEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: SpecializationEdge = {
        id: "edge1",
        type: EDGE_TYPES.SPECIALIZATION,
        source: "Car",
        target: "Vehicle",
        label: "specializes"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.SPECIALIZATION);
      expect(edge.label).toBe("specializes");
    });

    it("should allow optional label", () => {
      const edge: SpecializationEdge = {
        id: "edge1",
        type: EDGE_TYPES.SPECIALIZATION,
        source: "Car",
        target: "Vehicle"
      };
      
      expect(edge.label).toBeUndefined();
    });
  });

  describe("RedefinitionEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: RedefinitionEdge = {
        id: "edge1",
        type: EDGE_TYPES.REDEFINITION,
        source: "ElectricVehicle::engine",
        target: "Vehicle::engine",
        label: "redefines"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.REDEFINITION);
    });
  });

  describe("SubsettingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: SubsettingEdge = {
        id: "edge1",
        type: EDGE_TYPES.SUBSETTING,
        source: "Car::wheels",
        target: "Vehicle::components",
        label: "subsets"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.SUBSETTING);
    });
  });

  describe("TypingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: TypingEdge = {
        id: "edge1",
        type: EDGE_TYPES.TYPING,
        source: "myCar",
        target: "Car",
        label: "typed by"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.TYPING);
    });
  });

  describe("ReferenceSubsettingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: ReferenceSubsettingEdge = {
        id: "edge1",
        type: EDGE_TYPES.REFERENCE_SUBSETTING,
        source: "ref1",
        target: "ref2",
        label: "reference subsets"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.REFERENCE_SUBSETTING);
    });
  });

  describe("CrossSubsettingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: CrossSubsettingEdge = {
        id: "edge1",
        type: EDGE_TYPES.CROSS_SUBSETTING,
        source: "ref1",
        target: "ref2",
        label: "cross subsets"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.CROSS_SUBSETTING);
    });
  });

  describe("ConjugationEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: ConjugationEdge = {
        id: "edge1",
        type: EDGE_TYPES.CONJUGATION,
        source: "ConjugatedPort",
        target: "OriginalPort",
        label: "conjugates"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.CONJUGATION);
    });
  });
});

describe("SysML Domain-Specific Relationship Edges", () => {
  describe("SatisfyEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: SatisfyEdge = {
        id: "edge1",
        type: EDGE_TYPES.SATISFY,
        source: "BrakingSystem",
        target: "SafetyRequirement",
        label: "satisfies"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.SATISFY);
    });
  });

  describe("PerformEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: PerformEdge = {
        id: "edge1",
        type: EDGE_TYPES.PERFORM,
        source: "System",
        target: "Accelerate",
        label: "performs"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.PERFORM);
    });
  });

  describe("ExhibitEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: ExhibitEdge = {
        id: "edge1",
        type: EDGE_TYPES.EXHIBIT,
        source: "Engine",
        target: "Running",
        label: "exhibits"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.EXHIBIT);
    });
  });

  describe("IncludeEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: IncludeEdge = {
        id: "edge1",
        type: EDGE_TYPES.INCLUDE,
        source: "MainScenario",
        target: "DriveVehicle",
        label: "includes"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.INCLUDE);
    });
  });

  describe("AssertEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: AssertEdge = {
        id: "edge1",
        type: EDGE_TYPES.ASSERT,
        source: "VerificationCase",
        target: "Constraint",
        label: "asserts"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.ASSERT);
    });
  });

  describe("VerifyEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: VerifyEdge = {
        id: "edge1",
        type: EDGE_TYPES.VERIFY,
        source: "TestCase",
        target: "Requirement",
        label: "verifies"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.VERIFY);
    });
  });

  describe("DependencyEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: DependencyEdge = {
        id: "edge1",
        type: EDGE_TYPES.DEPENDENCY,
        source: "Client",
        target: "Server",
        label: "depends on"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.DEPENDENCY);
    });
  });
});

describe("SysML Structural Edges", () => {
  describe("CompositionEdge", () => {
    it("should have correct type discriminator for part containment", () => {
      const edge: CompositionEdge = {
        id: "edge1",
        type: EDGE_TYPES.COMPOSITION,
        source: "Car",
        target: "Engine",
        label: "contains"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.COMPOSITION);
    });

    it("should support multiplicity information", () => {
      const edge: CompositionEdge = {
        id: "edge1",
        type: EDGE_TYPES.COMPOSITION,
        source: "Car",
        target: "Wheel",
        multiplicity: "4"
      };
      
      expect(edge.multiplicity).toBe("4");
    });
  });

  describe("ConnectionEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: ConnectionEdge = {
        id: "edge1",
        type: EDGE_TYPES.CONNECTION,
        source: "ComponentA::port1",
        target: "ComponentB::port2",
        label: "connected to"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.CONNECTION);
    });

    it("should support connection definition reference", () => {
      const edge: ConnectionEdge = {
        id: "edge1",
        type: EDGE_TYPES.CONNECTION,
        source: "ComponentA::port1",
        target: "ComponentB::port2",
        connectionDef: "Connections::DataConnection"
      };
      
      expect(edge.connectionDef).toBe("Connections::DataConnection");
    });
  });

  describe("BindingEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: BindingEdge = {
        id: "edge1",
        type: EDGE_TYPES.BINDING,
        source: "x",
        target: "y",
        label: "="
      };
      
      expect(edge.type).toBe(EDGE_TYPES.BINDING);
    });
  });

  describe("AllocationEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: AllocationEdge = {
        id: "edge1",
        type: EDGE_TYPES.ALLOCATION,
        source: "Function::process",
        target: "Component::processor",
        label: "allocated to"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.ALLOCATION);
    });

    it("should support allocation definition reference", () => {
      const edge: AllocationEdge = {
        id: "edge1",
        type: EDGE_TYPES.ALLOCATION,
        source: "Function::process",
        target: "Component::processor",
        allocationDef: "Allocations::FuncToComp"
      };
      
      expect(edge.allocationDef).toBe("Allocations::FuncToComp");
    });
  });

  describe("FlowEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: FlowEdge = {
        id: "edge1",
        type: EDGE_TYPES.FLOW,
        source: "Tank::outlet",
        target: "Engine::inlet",
        label: "flows to"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.FLOW);
    });

    it("should support item type specification", () => {
      const edge: FlowEdge = {
        id: "edge1",
        type: EDGE_TYPES.FLOW,
        source: "Tank::outlet",
        target: "Engine::inlet",
        itemType: "Resources::Fuel"
      };
      
      expect(edge.itemType).toBe("Resources::Fuel");
    });
  });

  describe("SuccessionEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: SuccessionEdge = {
        id: "edge1",
        type: EDGE_TYPES.SUCCESSION,
        source: "State::idle",
        target: "State::running",
        label: "then"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.SUCCESSION);
    });

    it("should support guard condition", () => {
      const edge: SuccessionEdge = {
        id: "edge1",
        type: EDGE_TYPES.SUCCESSION,
        source: "State::idle",
        target: "State::running",
        guard: "ignitionKey"
      };
      
      expect(edge.guard).toBe("ignitionKey");
    });
  });

  describe("MembershipEdge", () => {
    it("should have correct type discriminator", () => {
      const edge: MembershipEdge = {
        id: "edge1",
        type: EDGE_TYPES.MEMBERSHIP,
        source: "Vehicle",
        target: "Vehicle::engine",
        label: "owns"
      };
      
      expect(edge.type).toBe(EDGE_TYPES.MEMBERSHIP);
    });
  });
});

describe("Edge Type Discrimination", () => {
  it("should distinguish between different edge types", () => {
    const specialization: SpecializationEdge = {
      id: "e1",
      type: EDGE_TYPES.SPECIALIZATION,
      source: "Car",
      target: "Vehicle"
    };

    const typing: TypingEdge = {
      id: "e2",
      type: EDGE_TYPES.TYPING,
      source: "myCar",
      target: "Car"
    };

    const composition: CompositionEdge = {
      id: "e3",
      type: EDGE_TYPES.COMPOSITION,
      source: "Car",
      target: "Engine"
    };

    expect(specialization.type).not.toBe(typing.type);
    expect(typing.type).not.toBe(composition.type);
    expect(specialization.type).toBe(EDGE_TYPES.SPECIALIZATION);
    expect(typing.type).toBe(EDGE_TYPES.TYPING);
    expect(composition.type).toBe(EDGE_TYPES.COMPOSITION);
  });
});

describe("Type Guard Functions", () => {
  const specializationEdge: SpecializationEdge = {
    id: "e1",
    type: EDGE_TYPES.SPECIALIZATION,
    source: "Car",
    target: "Vehicle"
  };

  const satisfyEdge: SatisfyEdge = {
    id: "e2",
    type: EDGE_TYPES.SATISFY,
    source: "System",
    target: "Requirement"
  };

  const compositionEdge: CompositionEdge = {
    id: "e3",
    type: EDGE_TYPES.COMPOSITION,
    source: "Car",
    target: "Engine"
  };

  const connectionEdge: ConnectionEdge = {
    id: "e4",
    type: EDGE_TYPES.CONNECTION,
    source: "A::port",
    target: "B::port"
  };

  const successionEdge: SuccessionEdge = {
    id: "e5",
    type: EDGE_TYPES.SUCCESSION,
    source: "action1",
    target: "action2"
  };

  const conjugationEdge: ConjugationEdge = {
    id: "e6",
    type: EDGE_TYPES.CONJUGATION,
    source: "ConjPort",
    target: "OrigPort"
  };

  it("should correctly identify relationship edges", () => {
    expect(isRelationshipEdge(specializationEdge)).toBe(true);
    expect(isRelationshipEdge(satisfyEdge)).toBe(true);
    expect(isRelationshipEdge(conjugationEdge)).toBe(true);
    expect(isRelationshipEdge(compositionEdge)).toBe(false);
    expect(isRelationshipEdge(connectionEdge)).toBe(false);
    expect(isRelationshipEdge(successionEdge)).toBe(false);
  });

  it("should correctly identify structural edges", () => {
    expect(isStructuralEdge(compositionEdge)).toBe(true);
    expect(isStructuralEdge(connectionEdge)).toBe(true);
    expect(isStructuralEdge(successionEdge)).toBe(true);
    expect(isStructuralEdge(specializationEdge)).toBe(false);
    expect(isStructuralEdge(conjugationEdge)).toBe(false);
  });
});
