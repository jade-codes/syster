# SysML v2 Primer for Contributors

This document provides a quick introduction to SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language) concepts relevant to working on Syster.

## What is SysML v2?

**SysML v2** is a modeling language for systems engineering. It's used to describe complex systems (spacecraft, cars, software systems, etc.) using standardized notation.

**Key concepts:**
- **Models** describe systems at various levels of abstraction
- **Relationships** connect elements (specialization, typing, composition)
- **Traceability** tracks requirements through design to implementation
- **Multi-view** supports different perspectives (structure, behavior, requirements)

## What is KerML?

**KerML (Kernel Modeling Language)** is the foundation language underneath SysML v2. Think of it as:
- SysML v2 is to KerML what C++ is to C
- KerML provides core modeling constructs (types, features, relationships)
- SysML v2 adds domain-specific constructs (parts, ports, requirements)

**Syster implements both:**
- KerML parser and AST for foundation constructs
- SysML v2 parser and AST for systems modeling constructs

## KerML Core Concepts

### Types and Classifiers

A **Type** is the fundamental concept—anything that can classify things.

```sysml
type Vehicle;           // Simplest type declaration
```

A **Classifier** is a type that can have features (properties/operations):

```sysml
classifier Vehicle {
    feature speed: Real;
    feature color: String;
}
```

### Specializations (Inheritance)

Types can specialize (inherit from) other types:

```sysml
classifier Car specializes Vehicle {
    feature numWheels: Integer = 4;
}

// Car inherits 'speed' and 'color' from Vehicle
// Car adds 'numWheels'
```

**In Syster:**
- Stored in `RelationshipGraph.specializations`
- Used for type checking and inheritance queries

### Features

**Features** are the building blocks of types—properties, operations, relationships:

```sysml
classifier Engine {
    feature horsepower: Real;        // Attribute
    feature start(): Boolean;        // Operation
    feature fuelType: FuelType;      // Typed feature
}
```

**Feature directions:**
```sysml
feature input voltage: Real;    // Input parameter
feature output power: Real;     // Output result
feature inout state: State;     // Bidirectional
```

**In Syster:**
- Represented as `Symbol::Feature` in symbol table
- Direction stored in AST, used in semantic analysis

### Packages and Namespaces

**Packages** organize models hierarchically:

```sysml
package Automotive {
    package Engine {
        classifier V8Engine;
    }
    package Transmission {
        classifier Automatic;
    }
}

// Qualified name: Automotive::Engine::V8Engine
```

**In Syster:**
- Qualified names like `"Automotive::Engine::V8Engine"` stored in symbol table
- Used for cross-file reference resolution

## SysML v2 Domain Concepts

SysML v2 builds on KerML with domain-specific constructs.

### Definitions vs. Usages

**Key pattern in SysML v2:** Separate **definitions** (types) from **usages** (instances).

```sysml
// Definition: What a vehicle is (type)
part def Vehicle {
    attribute mass: Real;
}

// Usage: A specific vehicle instance
part myVehicle: Vehicle {
    // This specific vehicle weighs 1500 kg
    attribute mass = 1500.0;
}
```

**Analogy:**
- `part def` = class definition in OOP
- `part` = object instance in OOP

### Common Definition Types

```sysml
part def Engine;           // Physical component type
port def DataPort;         // Interface/connection point type
action def Move;           // Behavior type
item def Fuel;            // Resource/flow type
attribute def Speed;       // Data attribute type
requirement def Safety;    // Requirement type
```

**In Syster:**
- All stored as `Symbol::Definition` with different `kind` field
- Kind determines semantic rules and valid relationships

### Common Usage Types

```sysml
part engine: Engine;           // Physical component instance
port dataIn: DataPort;         // Interface instance
action move: Move;             // Behavior instance
item fuel: Fuel;              // Resource instance
attribute speed: Speed;        // Data instance
```

**In Syster:**
- All stored as `Symbol::Usage` with different `kind` field
- May have typing relationship to their definition

### Parts and Composition

**Parts** represent physical or logical components:

```sysml
part def Car {
    part engine: Engine;           // Car HAS-A engine
    part wheels: Wheel[4];         // Car has 4 wheels (multiplicity)
    part transmission: Transmission;
}
```

**Composition hierarchy:**
```
Car
├── engine: Engine
├── wheels: Wheel[4]
│   ├── wheels[0]
│   ├── wheels[1]
│   ├── wheels[2]
│   └── wheels[3]
└── transmission: Transmission
```

### Ports and Connections

**Ports** are interaction points between components:

```sysml
part def Computer {
    port usb: USBPort;
    port network: EthernetPort;
}

part def Printer {
    port usb: USBPort;
}

// Connection between ports
connection usbConnection connect
    computer.usb to printer.usb;
```

**In Syster:**
- Ports stored as `Symbol::Usage` with `kind = "port"`
- Connections stored as relationship in graph

### Requirements

**Requirements** describe what the system must do:

```sysml
requirement def SafetyRequirement {
    doc /* The vehicle must stop within 100m at 60mph */
    
    subject vehicle: Vehicle;      // What the requirement applies to
    
    require constraint {
        // Formal constraint (future: OCL-like)
        vehicle.brakingDistance <= 100.0
    }
}

// Requirement usage
requirement safetyReq: SafetyRequirement {
    subject = myVehicle;
}
```

**Satisfaction:** Link implementation to requirements
```sysml
part brakingSystem satisfies safetyReq;
```

**In Syster:**
- Requirements stored as `Symbol::Definition` or `Symbol::Usage`
- Satisfaction stored in `RelationshipGraph.satisfactions`

### Actions and Behaviors

**Actions** describe what the system does:

```sysml
action def Accelerate {
    in input throttle: Real;
    out output speed: Real;
}

action accelerate: Accelerate {
    // Behavior: increase speed based on throttle
}
```

**State machines:**
```sysml
state def EngineState {
    entry / startEngine();
    
    state off;
    state running;
    state stalled;
    
    transition off to running on ignitionKey;
    transition running to stalled when fuelEmpty;
}
```

## Import System

SysML v2 has a sophisticated import system for reusing models.

### Namespace Imports

Import all members of a package:

```sysml
import Automotive::*;

// Now can use "Engine" instead of "Automotive::Engine"
part myEngine: Engine;
```

**In Syster:**
- Processed in first pass of import resolution
- All symbols in target namespace become visible

### Member Imports

Import specific members:

```sysml
import Automotive::Engine;
import Automotive::Transmission;

// Only Engine and Transmission are visible
part myEngine: Engine;          // OK
part myTrans: Transmission;     // OK
part myWheel: Wheel;           // Error: Wheel not imported
```

### Recursive Imports

Import nested packages:

```sysml
import Automotive::**;

// Imports:
// - Automotive::Engine::V8Engine
// - Automotive::Engine::V6Engine
// - Automotive::Transmission::Automatic
// - etc.
```

**In Syster:**
- Processed in third pass (requires fully populated namespaces)
- Recursively walks package hierarchy

### Aliases

Rename imports to avoid conflicts:

```sysml
import Automotive::Engine as AutoEngine;
import Aircraft::Engine as AircraftEngine;

part carEngine: AutoEngine;
part jetEngine: AircraftEngine;
```

**In Syster:**
- Stored as `Symbol::Alias` with `target` field
- Resolved during name lookup

## Relationships in SysML v2

### Specialization (Inheritance)

```sysml
part def ElectricCar specializes Car {
    part battery: Battery;
}
```

**Semantic meaning:** ElectricCar IS-A Car (inherits all features)

### Typing

```sysml
part myCar: Car;
```

**Semantic meaning:** myCar is an instance of type Car

### Subsetting

```sysml
part def Vehicle {
    part components: Component[*];  // 0 or more components
}

part def Car specializes Vehicle {
    part wheels: Wheel[4] subsets components;  // Wheels are a subset of components
}
```

**Semantic meaning:** `wheels` refines the more general `components` feature

### Redefinition

```sysml
part def Vehicle {
    part engine: Engine;
}

part def ElectricVehicle specializes Vehicle {
    part engine: ElectricEngine redefines engine;  // Replace with electric engine
}
```

**Semantic meaning:** Overrides inherited feature with more specific type

### Feature Membership

```sysml
part def Car {
    feature mass: Real;    // 'mass' is a member of 'Car'
    part engine: Engine;   // 'engine' is a member of 'Car'
}
```

**In Syster:**
- Members stored in nested scopes
- Qualified name: `Car::mass`, `Car::engine`

## Key Terminology Glossary

| Term | Definition | Example |
|------|-----------|---------|
| **Qualified Name** | Full path to symbol | `Automotive::Engine::V8` |
| **Scope** | Context where declarations are visible | Inside a package or classifier |
| **Feature** | Property/operation of a type | `attribute speed: Real` |
| **Classifier** | Type that can have features | `classifier Vehicle { }` |
| **Definition** | Type (template) | `part def Engine` |
| **Usage** | Instance | `part myEngine: Engine` |
| **Specialization** | IS-A relationship | `Car specializes Vehicle` |
| **Typing** | INSTANCE-OF relationship | `myCar: Car` |
| **Subsetting** | REFINES relationship | `wheels subsets components` |
| **Redefinition** | OVERRIDES relationship | `engine redefines inherited::engine` |
| **Namespace** | Named container (package) | `package Automotive { }` |

## Reading SysML Models

### Example 1: Simple System

```sysml
package VehicleModel {
    // Define a vehicle type
    part def Vehicle {
        attribute mass: Real;
        attribute speed: Real;
    }
    
    // Define a specific vehicle
    part myCar: Vehicle {
        attribute mass = 1500.0;  // kg
        attribute speed = 0.0;    // m/s (initially stopped)
    }
}
```

**Syster representation:**
- Package: `VehicleModel` (scope 0)
- Definition: `VehicleModel::Vehicle` (scope 1)
- Feature: `VehicleModel::Vehicle::mass`
- Feature: `VehicleModel::Vehicle::speed`
- Usage: `VehicleModel::myCar` (typed by `Vehicle`)

### Example 2: Composition

```sysml
package AutomotiveModel {
    part def Engine {
        attribute horsepower: Real;
    }
    
    part def Car {
        part engine: Engine;       // Composition
        part wheels: Wheel[4];
    }
    
    part myCar: Car {
        part engine: Engine {
            attribute horsepower = 300.0;  // Specific value
        }
    }
}
```

**Containment tree:**
```
AutomotiveModel
├── Engine (definition)
├── Car (definition)
│   ├── engine: Engine (usage)
│   └── wheels: Wheel[4] (usage)
└── myCar: Car (usage)
    └── engine: Engine (usage)
        └── horsepower = 300.0
```

### Example 3: Specialization

```sysml
classifier Vehicle {
    feature speed: Real;
}

classifier Car specializes Vehicle {
    feature numWheels: Integer = 4;
}

classifier SportsCar specializes Car {
    feature turbo: Boolean = true;
}
```

**Inheritance hierarchy:**
```
Vehicle (speed)
  └── Car (speed, numWheels)
        └── SportsCar (speed, numWheels, turbo)
```

**Syster graph:**
- `specializations["Vehicle"]` = `["Car"]`
- `specializations["Car"]` = `["SportsCar"]`

## Working with Syster

### How Syster Represents SysML

1. **Parse phase:** SysML text → Pest parse tree
2. **Syntax phase:** Parse tree → AST (e.g., `PartDef`, `PartUsage`)
3. **Semantic phase:** AST → Symbol table + Relationship graphs

### Symbol Table Lookup

```rust
// Look up a symbol by qualified name
let symbol = symbol_table.lookup("Automotive::Engine::V8");

match symbol {
    Some(Symbol::Classifier { name, qualified_name, .. }) => {
        println!("Found classifier: {}", name);
    }
    Some(Symbol::Feature { name, feature_type, .. }) => {
        println!("Found feature: {} of type {:?}", name, feature_type);
    }
    None => {
        println!("Symbol not found");
    }
}
```

## Further Reading

- [SysML v2 Specification (OMG)](https://www.omg.org/spec/SysML/2.0/)
- [SysML v2 Release](https://github.com/Systems-Modeling/SysML-v2-Release)
- [ARCHITECTURE.md](../ARCHITECTURE.md) - Syster implementation details
- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute
