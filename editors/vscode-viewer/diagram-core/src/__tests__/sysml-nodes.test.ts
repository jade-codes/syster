import { describe, it, expect } from "bun:test";
import {
  NODE_TYPES,
  FEATURE_DIRECTIONS,
  isDefinitionNode,
  isUsageNode,
  type PartDefNode,
  type PortDefNode,
  type ActionDefNode,
  type StateDefNode,
  type RequirementDefNode,
  type ConcernDefNode,
  type CaseDefNode,
  type AnalysisCaseDefNode,
  type VerificationCaseDefNode,
  type UseCaseDefNode,
  type ViewDefNode,
  type ViewpointDefNode,
  type RenderingDefNode,
  type AllocationDefNode,
  type CalculationDefNode,
  type ConnectionDefNode,
  type ConstraintDefNode,
  type EnumerationDefNode,
  type FlowDefNode,
  type IndividualDefNode,
  type InterfaceDefNode,
  type OccurrenceDefNode,
  type MetadataDefNode,
  type ItemDefNode,
  type AttributeDefNode,
  type PartUsageNode,
  type PortUsageNode,
  type ActionUsageNode,
  type ItemUsageNode,
  type AttributeUsageNode,
  type RequirementUsageNode,
  type ConcernUsageNode,
  type CaseUsageNode,
  type ViewUsageNode,
  type EnumerationUsageNode,
  type SatisfyRequirementUsageNode,
  type PerformActionUsageNode,
  type ExhibitStateUsageNode,
  type IncludeUseCaseUsageNode,
  type StateUsageNode,
  type OccurrenceUsageNode,
  type IndividualUsageNode,
  type SnapshotUsageNode,
  type TimesliceUsageNode,
  type ReferenceUsageNode,
  type ConstraintUsageNode,
  type CalculationUsageNode,
  type ConnectionUsageNode,
  type InterfaceUsageNode,
  type AllocationUsageNode,
  type FlowUsageNode,
} from "../sysml-nodes";

describe("SysML Definition Nodes", () => {
  describe("PartDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: PartDefNode = {
        id: "part1",
        type: NODE_TYPES.PART_DEF,
        name: "Vehicle",
        qualifiedName: "Automotive::Vehicle",
        features: []
      };
      
      expect(node.type).toBe(NODE_TYPES.PART_DEF);
      expect(node.name).toBe("Vehicle");
    });

    it("should support features array", () => {
      const node: PartDefNode = {
        id: "part1",
        type: NODE_TYPES.PART_DEF,
        name: "Vehicle",
        qualifiedName: "Automotive::Vehicle",
        features: ["speed", "mass"]
      };
      
      expect(node.features).toHaveLength(2);
      expect(node.features).toContain("speed");
    });
  });

  describe("PortDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: PortDefNode = {
        id: "port1",
        type: NODE_TYPES.PORT_DEF,
        name: "DataPort",
        qualifiedName: "Interfaces::DataPort",
        direction: FEATURE_DIRECTIONS.IN
      };
      
      expect(node.type).toBe(NODE_TYPES.PORT_DEF);
      expect(node.direction).toBe(FEATURE_DIRECTIONS.IN);
    });

    it("should support bidirectional ports", () => {
      const node: PortDefNode = {
        id: "port1",
        type: NODE_TYPES.PORT_DEF,
        name: "StatePort",
        qualifiedName: "Interfaces::StatePort",
        direction: FEATURE_DIRECTIONS.INOUT
      };
      
      expect(node.direction).toBe(FEATURE_DIRECTIONS.INOUT);
    });
  });

  describe("ActionDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: ActionDefNode = {
        id: "action1",
        type: NODE_TYPES.ACTION_DEF,
        name: "Move",
        qualifiedName: "Actions::Move"
      };
      
      expect(node.type).toBe(NODE_TYPES.ACTION_DEF);
      expect(node.name).toBe("Move");
    });
  });

  describe("RequirementDefNode", () => {
    it("should have correct type discriminator with requirement text", () => {
      const node: RequirementDefNode = {
        id: "req1",
        type: NODE_TYPES.REQUIREMENT_DEF,
        name: "SafetyRequirement",
        qualifiedName: "Requirements::SafetyRequirement",
        text: "Vehicle must stop within 100m at 60mph"
      };
      
      expect(node.type).toBe(NODE_TYPES.REQUIREMENT_DEF);
      expect(node.text).toContain("100m");
    });

    it("should allow optional requirement text", () => {
      const node: RequirementDefNode = {
        id: "req1",
        type: NODE_TYPES.REQUIREMENT_DEF,
        name: "SafetyRequirement",
        qualifiedName: "Requirements::SafetyRequirement"
      };
      
      expect(node.text).toBeUndefined();
    });
  });

  describe("ItemDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: ItemDefNode = {
        id: "item1",
        type: NODE_TYPES.ITEM_DEF,
        name: "Fuel",
        qualifiedName: "Resources::Fuel"
      };
      
      expect(node.type).toBe(NODE_TYPES.ITEM_DEF);
    });
  });

  describe("AttributeDefNode", () => {
    it("should have correct type discriminator with data type", () => {
      const node: AttributeDefNode = {
        id: "attr1",
        type: NODE_TYPES.ATTRIBUTE_DEF,
        name: "Speed",
        qualifiedName: "Attributes::Speed",
        dataType: "Real"
      };
      
      expect(node.type).toBe(NODE_TYPES.ATTRIBUTE_DEF);
      expect(node.dataType).toBe("Real");
    });

    it("should allow optional data type", () => {
      const node: AttributeDefNode = {
        id: "attr1",
        type: NODE_TYPES.ATTRIBUTE_DEF,
        name: "Speed",
        qualifiedName: "Attributes::Speed"
      };
      
      expect(node.dataType).toBeUndefined();
    });
  });

  describe("StateDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: StateDefNode = {
        id: "state1",
        type: NODE_TYPES.STATE_DEF,
        name: "EngineState",
        qualifiedName: "States::EngineState"
      };
      
      expect(node.type).toBe(NODE_TYPES.STATE_DEF);
    });
  });

  describe("ConcernDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: ConcernDefNode = {
        id: "concern1",
        type: NODE_TYPES.CONCERN_DEF,
        name: "PerformanceConcern",
        qualifiedName: "Concerns::PerformanceConcern"
      };
      
      expect(node.type).toBe(NODE_TYPES.CONCERN_DEF);
    });
  });

  describe("CaseDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: CaseDefNode = {
        id: "case1",
        type: NODE_TYPES.CASE_DEF,
        name: "TestCase",
        qualifiedName: "Cases::TestCase"
      };
      
      expect(node.type).toBe(NODE_TYPES.CASE_DEF);
    });
  });

  describe("AnalysisCaseDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: AnalysisCaseDefNode = {
        id: "analysis1",
        type: NODE_TYPES.ANALYSIS_CASE_DEF,
        name: "StressAnalysis",
        qualifiedName: "Analysis::StressAnalysis"
      };
      
      expect(node.type).toBe(NODE_TYPES.ANALYSIS_CASE_DEF);
    });
  });

  describe("VerificationCaseDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: VerificationCaseDefNode = {
        id: "verify1",
        type: NODE_TYPES.VERIFICATION_CASE_DEF,
        name: "SafetyVerification",
        qualifiedName: "Verification::SafetyVerification"
      };
      
      expect(node.type).toBe(NODE_TYPES.VERIFICATION_CASE_DEF);
    });
  });

  describe("UseCaseDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: UseCaseDefNode = {
        id: "usecase1",
        type: NODE_TYPES.USE_CASE_DEF,
        name: "DriveVehicle",
        qualifiedName: "UseCases::DriveVehicle"
      };
      
      expect(node.type).toBe(NODE_TYPES.USE_CASE_DEF);
    });
  });

  describe("ViewDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: ViewDefNode = {
        id: "view1",
        type: NODE_TYPES.VIEW_DEF,
        name: "SystemView",
        qualifiedName: "Views::SystemView"
      };
      
      expect(node.type).toBe(NODE_TYPES.VIEW_DEF);
    });
  });

  describe("ViewpointDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: ViewpointDefNode = {
        id: "viewpoint1",
        type: NODE_TYPES.VIEWPOINT_DEF,
        name: "StakeholderViewpoint",
        qualifiedName: "Viewpoints::StakeholderViewpoint"
      };
      
      expect(node.type).toBe(NODE_TYPES.VIEWPOINT_DEF);
    });
  });

  describe("RenderingDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: RenderingDefNode = {
        id: "rendering1",
        type: NODE_TYPES.RENDERING_DEF,
        name: "DiagramRendering",
        qualifiedName: "Renderings::DiagramRendering"
      };
      
      expect(node.type).toBe(NODE_TYPES.RENDERING_DEF);
    });
  });

  describe("AllocationDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: AllocationDefNode = {
        id: "alloc1",
        type: NODE_TYPES.ALLOCATION_DEF,
        name: "ResourceAllocation",
        qualifiedName: "Allocations::ResourceAllocation"
      };
      
      expect(node.type).toBe(NODE_TYPES.ALLOCATION_DEF);
    });
  });

  describe("CalculationDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: CalculationDefNode = {
        id: "calc1",
        type: NODE_TYPES.CALCULATION_DEF,
        name: "VelocityCalculation",
        qualifiedName: "Calculations::VelocityCalculation"
      };
      
      expect(node.type).toBe(NODE_TYPES.CALCULATION_DEF);
    });
  });

  describe("ConnectionDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: ConnectionDefNode = {
        id: "conn1",
        type: NODE_TYPES.CONNECTION_DEF,
        name: "DataConnection",
        qualifiedName: "Connections::DataConnection"
      };
      
      expect(node.type).toBe(NODE_TYPES.CONNECTION_DEF);
    });
  });

  describe("ConstraintDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: ConstraintDefNode = {
        id: "constraint1",
        type: NODE_TYPES.CONSTRAINT_DEF,
        name: "MassConstraint",
        qualifiedName: "Constraints::MassConstraint"
      };
      
      expect(node.type).toBe(NODE_TYPES.CONSTRAINT_DEF);
    });
  });

  describe("EnumerationDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: EnumerationDefNode = {
        id: "enum1",
        type: NODE_TYPES.ENUMERATION_DEF,
        name: "ColorEnum",
        qualifiedName: "Enumerations::ColorEnum"
      };
      
      expect(node.type).toBe(NODE_TYPES.ENUMERATION_DEF);
    });
  });

  describe("FlowDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: FlowDefNode = {
        id: "flow1",
        type: NODE_TYPES.FLOW_DEF,
        name: "DataFlow",
        qualifiedName: "Flows::DataFlow"
      };
      
      expect(node.type).toBe(NODE_TYPES.FLOW_DEF);
    });
  });

  describe("IndividualDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: IndividualDefNode = {
        id: "individual1",
        type: NODE_TYPES.INDIVIDUAL_DEF,
        name: "John",
        qualifiedName: "Individuals::John"
      };
      
      expect(node.type).toBe(NODE_TYPES.INDIVIDUAL_DEF);
    });
  });

  describe("InterfaceDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: InterfaceDefNode = {
        id: "interface1",
        type: NODE_TYPES.INTERFACE_DEF,
        name: "DataInterface",
        qualifiedName: "Interfaces::DataInterface"
      };
      
      expect(node.type).toBe(NODE_TYPES.INTERFACE_DEF);
    });
  });

  describe("OccurrenceDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: OccurrenceDefNode = {
        id: "occurrence1",
        type: NODE_TYPES.OCCURRENCE_DEF,
        name: "CollisionEvent",
        qualifiedName: "Occurrences::CollisionEvent"
      };
      
      expect(node.type).toBe(NODE_TYPES.OCCURRENCE_DEF);
    });
  });

  describe("MetadataDefNode", () => {
    it("should have correct type discriminator", () => {
      const node: MetadataDefNode = {
        id: "metadata1",
        type: NODE_TYPES.METADATA_DEF,
        name: "DocumentationMetadata",
        qualifiedName: "Metadata::DocumentationMetadata"
      };
      
      expect(node.type).toBe(NODE_TYPES.METADATA_DEF);
    });
  });
});

describe("SysML Usage Nodes", () => {
  describe("PartUsageNode", () => {
    it("should have correct type discriminator with typed-by reference", () => {
      const node: PartUsageNode = {
        id: "usage1",
        type: NODE_TYPES.PART_USAGE,
        name: "engine",
        qualifiedName: "Car::engine",
        typedBy: "Engine"
      };
      
      expect(node.type).toBe(NODE_TYPES.PART_USAGE);
      expect(node.typedBy).toBe("Engine");
    });

    it("should allow optional typedBy", () => {
      const node: PartUsageNode = {
        id: "usage1",
        type: NODE_TYPES.PART_USAGE,
        name: "engine",
        qualifiedName: "Car::engine"
      };
      
      expect(node.typedBy).toBeUndefined();
    });
  });

  describe("PortUsageNode", () => {
    it("should have correct type discriminator with direction", () => {
      const node: PortUsageNode = {
        id: "usage1",
        type: NODE_TYPES.PORT_USAGE,
        name: "dataIn",
        qualifiedName: "Computer::dataIn",
        direction: FEATURE_DIRECTIONS.IN,
        typedBy: "DataPort"
      };
      
      expect(node.type).toBe(NODE_TYPES.PORT_USAGE);
      expect(node.direction).toBe(FEATURE_DIRECTIONS.IN);
    });
  });

  describe("ActionUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: ActionUsageNode = {
        id: "usage1",
        type: NODE_TYPES.ACTION_USAGE,
        name: "accelerate",
        qualifiedName: "Car::accelerate",
        typedBy: "Accelerate"
      };
      
      expect(node.type).toBe(NODE_TYPES.ACTION_USAGE);
      expect(node.typedBy).toBe("Accelerate");
    });
  });

  describe("ItemUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: ItemUsageNode = {
        id: "usage1",
        type: NODE_TYPES.ITEM_USAGE,
        name: "fuel",
        qualifiedName: "Tank::fuel",
        typedBy: "Fuel"
      };
      
      expect(node.type).toBe(NODE_TYPES.ITEM_USAGE);
      expect(node.typedBy).toBe("Fuel");
    });
  });

  describe("AttributeUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: AttributeUsageNode = {
        id: "usage1",
        type: NODE_TYPES.ATTRIBUTE_USAGE,
        name: "speed",
        qualifiedName: "Vehicle::speed",
        dataType: "Real",
        typedBy: "Speed"
      };
      
      expect(node.type).toBe(NODE_TYPES.ATTRIBUTE_USAGE);
      expect(node.dataType).toBe("Real");
    });
  });

  describe("RequirementUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: RequirementUsageNode = {
        id: "usage1",
        type: NODE_TYPES.REQUIREMENT_USAGE,
        name: "safetyReq",
        qualifiedName: "System::safetyReq",
        text: "Must stop within 100m",
        typedBy: "SafetyRequirement"
      };
      
      expect(node.type).toBe(NODE_TYPES.REQUIREMENT_USAGE);
      expect(node.text).toBe("Must stop within 100m");
    });
  });

  describe("ConcernUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: ConcernUsageNode = {
        id: "usage1",
        type: NODE_TYPES.CONCERN_USAGE,
        name: "performanceConcern",
        qualifiedName: "Project::performanceConcern",
        typedBy: "PerformanceConcern"
      };
      
      expect(node.type).toBe(NODE_TYPES.CONCERN_USAGE);
    });
  });

  describe("CaseUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: CaseUsageNode = {
        id: "usage1",
        type: NODE_TYPES.CASE_USAGE,
        name: "testCase1",
        qualifiedName: "TestSuite::testCase1",
        typedBy: "TestCase"
      };
      
      expect(node.type).toBe(NODE_TYPES.CASE_USAGE);
    });
  });

  describe("ViewUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: ViewUsageNode = {
        id: "usage1",
        type: NODE_TYPES.VIEW_USAGE,
        name: "systemView",
        qualifiedName: "Model::systemView",
        typedBy: "SystemView"
      };
      
      expect(node.type).toBe(NODE_TYPES.VIEW_USAGE);
    });
  });

  describe("EnumerationUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: EnumerationUsageNode = {
        id: "usage1",
        type: NODE_TYPES.ENUMERATION_USAGE,
        name: "color",
        qualifiedName: "Vehicle::color",
        typedBy: "ColorEnum"
      };
      
      expect(node.type).toBe(NODE_TYPES.ENUMERATION_USAGE);
    });
  });

  describe("SatisfyRequirementUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: SatisfyRequirementUsageNode = {
        id: "usage1",
        type: NODE_TYPES.SATISFY_REQUIREMENT_USAGE,
        name: "satisfySafety",
        qualifiedName: "System::satisfySafety",
        satisfiedRequirement: "SafetyRequirement"
      };
      
      expect(node.type).toBe(NODE_TYPES.SATISFY_REQUIREMENT_USAGE);
      expect(node.satisfiedRequirement).toBe("SafetyRequirement");
    });
  });

  describe("PerformActionUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: PerformActionUsageNode = {
        id: "usage1",
        type: NODE_TYPES.PERFORM_ACTION_USAGE,
        name: "performMove",
        qualifiedName: "System::performMove",
        performedAction: "Move"
      };
      
      expect(node.type).toBe(NODE_TYPES.PERFORM_ACTION_USAGE);
      expect(node.performedAction).toBe("Move");
    });
  });

  describe("ExhibitStateUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: ExhibitStateUsageNode = {
        id: "usage1",
        type: NODE_TYPES.EXHIBIT_STATE_USAGE,
        name: "exhibitRunning",
        qualifiedName: "Engine::exhibitRunning",
        exhibitedState: "Running"
      };
      
      expect(node.type).toBe(NODE_TYPES.EXHIBIT_STATE_USAGE);
      expect(node.exhibitedState).toBe("Running");
    });
  });

  describe("IncludeUseCaseUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: IncludeUseCaseUsageNode = {
        id: "usage1",
        type: NODE_TYPES.INCLUDE_USE_CASE_USAGE,
        name: "includeDrive",
        qualifiedName: "Scenario::includeDrive",
        includedUseCase: "DriveVehicle"
      };
      
      expect(node.type).toBe(NODE_TYPES.INCLUDE_USE_CASE_USAGE);
      expect(node.includedUseCase).toBe("DriveVehicle");
    });
  });
});

describe("New SysML Usage Nodes", () => {
  describe("StateUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: StateUsageNode = {
        id: "state1",
        type: NODE_TYPES.STATE_USAGE,
        name: "running",
        qualifiedName: "Engine::running",
        typedBy: "States::EngineState"
      };
      
      expect(node.type).toBe(NODE_TYPES.STATE_USAGE);
      expect(node.typedBy).toBe("States::EngineState");
    });
  });

  describe("OccurrenceUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: OccurrenceUsageNode = {
        id: "occ1",
        type: NODE_TYPES.OCCURRENCE_USAGE,
        name: "event1",
        qualifiedName: "Events::event1"
      };
      
      expect(node.type).toBe(NODE_TYPES.OCCURRENCE_USAGE);
    });
  });

  describe("IndividualUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: IndividualUsageNode = {
        id: "ind1",
        type: NODE_TYPES.INDIVIDUAL_USAGE,
        name: "car1",
        qualifiedName: "Fleet::car1",
        typedBy: "Vehicles::Car"
      };
      
      expect(node.type).toBe(NODE_TYPES.INDIVIDUAL_USAGE);
    });
  });

  describe("SnapshotUsageNode", () => {
    it("should have correct type discriminator with snapshotOf", () => {
      const node: SnapshotUsageNode = {
        id: "snap1",
        type: NODE_TYPES.SNAPSHOT_USAGE,
        name: "car1AtTime1",
        qualifiedName: "Snapshots::car1AtTime1",
        snapshotOf: "Fleet::car1"
      };
      
      expect(node.type).toBe(NODE_TYPES.SNAPSHOT_USAGE);
      expect(node.snapshotOf).toBe("Fleet::car1");
    });
  });

  describe("TimesliceUsageNode", () => {
    it("should have correct type discriminator with timesliceOf", () => {
      const node: TimesliceUsageNode = {
        id: "slice1",
        type: NODE_TYPES.TIMESLICE_USAGE,
        name: "car1Morning",
        qualifiedName: "Timeslices::car1Morning",
        timesliceOf: "Fleet::car1"
      };
      
      expect(node.type).toBe(NODE_TYPES.TIMESLICE_USAGE);
      expect(node.timesliceOf).toBe("Fleet::car1");
    });
  });

  describe("ReferenceUsageNode", () => {
    it("should have correct type discriminator with direction", () => {
      const node: ReferenceUsageNode = {
        id: "ref1",
        type: NODE_TYPES.REFERENCE_USAGE,
        name: "input",
        qualifiedName: "Action::input",
        direction: FEATURE_DIRECTIONS.IN,
        typedBy: "Data::Signal"
      };
      
      expect(node.type).toBe(NODE_TYPES.REFERENCE_USAGE);
      expect(node.direction).toBe(FEATURE_DIRECTIONS.IN);
    });
  });

  describe("ConstraintUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: ConstraintUsageNode = {
        id: "cons1",
        type: NODE_TYPES.CONSTRAINT_USAGE,
        name: "speedLimit",
        qualifiedName: "Vehicle::speedLimit",
        typedBy: "Constraints::MaxSpeed"
      };
      
      expect(node.type).toBe(NODE_TYPES.CONSTRAINT_USAGE);
    });
  });

  describe("CalculationUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: CalculationUsageNode = {
        id: "calc1",
        type: NODE_TYPES.CALCULATION_USAGE,
        name: "distance",
        qualifiedName: "Physics::distance",
        typedBy: "Calculations::DistanceCalc"
      };
      
      expect(node.type).toBe(NODE_TYPES.CALCULATION_USAGE);
    });
  });

  describe("ConnectionUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: ConnectionUsageNode = {
        id: "conn1",
        type: NODE_TYPES.CONNECTION_USAGE,
        name: "link1",
        qualifiedName: "System::link1",
        typedBy: "Connections::DataLink"
      };
      
      expect(node.type).toBe(NODE_TYPES.CONNECTION_USAGE);
    });
  });

  describe("InterfaceUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: InterfaceUsageNode = {
        id: "iface1",
        type: NODE_TYPES.INTERFACE_USAGE,
        name: "api",
        qualifiedName: "System::api",
        typedBy: "Interfaces::RestAPI"
      };
      
      expect(node.type).toBe(NODE_TYPES.INTERFACE_USAGE);
    });
  });

  describe("AllocationUsageNode", () => {
    it("should have correct type discriminator", () => {
      const node: AllocationUsageNode = {
        id: "alloc1",
        type: NODE_TYPES.ALLOCATION_USAGE,
        name: "funcToComp",
        qualifiedName: "System::funcToComp",
        typedBy: "Allocations::FunctionToComponent"
      };
      
      expect(node.type).toBe(NODE_TYPES.ALLOCATION_USAGE);
    });
  });

  describe("FlowUsageNode", () => {
    it("should have correct type discriminator with itemType", () => {
      const node: FlowUsageNode = {
        id: "flow1",
        type: NODE_TYPES.FLOW_USAGE,
        name: "fuelFlow",
        qualifiedName: "Engine::fuelFlow",
        itemType: "Resources::Fuel",
        typedBy: "Flows::LiquidFlow"
      };
      
      expect(node.type).toBe(NODE_TYPES.FLOW_USAGE);
      expect(node.itemType).toBe("Resources::Fuel");
    });
  });
});

describe("Node Type Discrimination", () => {
  it("should distinguish between definition and usage nodes", () => {
    const def: PartDefNode = {
      id: "def1",
      type: NODE_TYPES.PART_DEF,
      name: "Vehicle",
      qualifiedName: "Automotive::Vehicle",
      features: []
    };

    const usage: PartUsageNode = {
      id: "usage1",
      type: NODE_TYPES.PART_USAGE,
      name: "myCar",
      qualifiedName: "Garage::myCar",
      typedBy: "Vehicle"
    };

    expect(def.type).not.toBe(usage.type);
    expect(def.type).toBe(NODE_TYPES.PART_DEF);
    expect(usage.type).toBe(NODE_TYPES.PART_USAGE);
  });
});

describe("Type Guard Functions", () => {
  const partDef: PartDefNode = {
    id: "def1",
    type: NODE_TYPES.PART_DEF,
    name: "Vehicle",
    qualifiedName: "Automotive::Vehicle",
    features: []
  };

  const partUsage: PartUsageNode = {
    id: "usage1",
    type: NODE_TYPES.PART_USAGE,
    name: "myCar",
    qualifiedName: "Garage::myCar"
  };

  const portDef: PortDefNode = {
    id: "port1",
    type: NODE_TYPES.PORT_DEF,
    name: "DataPort",
    qualifiedName: "Interfaces::DataPort",
    direction: FEATURE_DIRECTIONS.IN
  };

  const requirementDef: RequirementDefNode = {
    id: "req1",
    type: NODE_TYPES.REQUIREMENT_DEF,
    name: "SafetyRequirement",
    qualifiedName: "Requirements::SafetyRequirement"
  };

  it("should correctly identify definition nodes", () => {
    expect(isDefinitionNode(partDef)).toBe(true);
    expect(isDefinitionNode(portDef)).toBe(true);
    expect(isDefinitionNode(requirementDef)).toBe(true);
    expect(isDefinitionNode(partUsage)).toBe(false);
  });

  it("should correctly identify usage nodes", () => {
    expect(isUsageNode(partUsage)).toBe(true);
    expect(isUsageNode(partDef)).toBe(false);
    expect(isUsageNode(portDef)).toBe(false);
  });
});
