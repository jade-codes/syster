# SysML v2 Primer for Developers

This document explains SysML v2 and KerML concepts for developers working on the Syster parser.

## What is SysML v2?

**SysML v2** (Systems Modeling Language version 2) is a graphical and textual modeling language for systems engineering. It's built on top of **KerML** (Kernel Modeling Language), which provides the foundational semantics.

```
┌─────────────────────────────────┐
│           SysML v2              │  ← Systems engineering concepts
├─────────────────────────────────┤
│            KerML                │  ← Core semantics
└─────────────────────────────────┘
```

## KerML vs SysML

| Aspect | KerML | SysML v2 |
|--------|-------|----------|
| Purpose | Foundation language | Systems modeling |
| Audience | Language designers | Systems engineers |
| Concepts | Type, Feature, Classifier | Part, Port, Action, etc. |
| Files | `.kerml` | `.sysml` |

## Namespaces and Packages

Everything lives in namespaces. Packages are the primary organizational unit.

```sysml
package Vehicle {
    // Contents go here
}

package Vehicle::Engine {
    // Nested package using qualified name
}
```

## Names and Qualified Names

- **Simple name:** `Engine`
- **Qualified name:** `Vehicle::Engine::Piston`
- **Separator:** `::` (two colons)

```sysml
package A {
    package B {
        part def C;  // Qualified name: A::B::C
    }
}
```

## Definitions vs Usages

SysML has a fundamental distinction:

| Definitions | Usages |
|-------------|--------|
| Define types | Create instances |
| Use `def` suffix | No suffix |
| Like classes | Like objects |

```sysml
// Definition - defines a type
part def Engine {
    attribute power : Real;
}

// Usage - creates an instance
part vehicle : Vehicle {
    part engine : Engine;  // instance of Engine
}
```

## Common Definition Types

```sysml
part def Vehicle;        // Physical thing
port def FuelPort;       // Interface point
action def Drive;        // Behavior/process
attribute def Mass;      // Data type
item def Fuel;           // Non-physical thing
```

## Features

Features are properties or behaviors of classifiers:

```sysml
part def Vehicle {
    // Attribute feature
    attribute mass : Real;
    
    // Part feature (composition)
    part engine : Engine;
    
    // Port feature (interface)
    port fuelPort : FuelPort;
    
    // Reference feature
    ref driver : Person;
}
```

## Relationships

### Specialization (IS-A / Inheritance)

```sysml
part def Car :> Vehicle {
    // Car inherits from Vehicle
}

// Or using 'specializes' keyword
part def Car specializes Vehicle {
}
```

### Typing (INSTANCE-OF)

```sysml
part car : Car;  // car is typed by Car
```

### Subsetting (REFINES)

```sysml
part def Vehicle {
    part powertrain : Powertrain;
}

part def Car :> Vehicle {
    part engine :> powertrain : Engine;  // Subsets powertrain
}
```

### Redefinition (OVERRIDES)

```sysml
part def Vehicle {
    attribute maxSpeed : Real;
}

part def SportsCar :> Vehicle {
    redefines maxSpeed = 250;  // Override value
}
```

## Imports

Bring elements into scope:

```sysml
package MyPackage {
    // Import single element
    import OtherPackage::Element;
    
    // Import all direct children
    import OtherPackage::*;
    
    // Import all descendants (recursive)
    import OtherPackage::*::**;
    
    // Aliased import
    import OtherPackage::LongName as Short;
}
```

## Visibility

```sysml
package MyPackage {
    public part def PublicDef;    // Visible outside
    private part def PrivateDef;  // Only visible inside
    protected part def Protected; // Visible to specializations
}
```

## KerML Foundation Concepts

### Type

The most basic classifier. Everything else specializes from `Type`.

```kerml
type MyType;
```

### Classifier

A type that can have features.

```kerml
classifier MyClassifier {
    feature x;
}
```

### Feature

A property of a classifier.

```kerml
classifier Container {
    feature contents;  // Untyped feature
    feature count : Natural;  // Typed feature
}
```

### Class

A classifier for structured data.

```kerml
class Person {
    feature name : String;
    feature age : Natural;
}
```

## The Standard Library

SysML v2 has a standard library with predefined types:

- `Base::Anything` - Root of type hierarchy
- `ScalarValues::Real`, `Integer`, `Boolean`, `String`
- `ISQ::*` - International System of Quantities
- `SI::*` - SI units

Location in Syster: `crates/syster-base/sysml.library/`

## Comments and Documentation

```sysml
/* Block comment */

// Line comment

/** 
 * Documentation comment
 * Attached to following element
 */
part def Vehicle;

// Or inline doc
part def Engine doc /* Engine documentation */;
```

## Parsing Implications

When parsing SysML/KerML, we need to track:

1. **Hierarchy:** Nested packages and elements
2. **Names:** Both simple and qualified
3. **Relationships:** Specialization, typing, subsetting, redefinition
4. **Imports:** Three types with different resolution rules
5. **Visibility:** Affects what's accessible
6. **Features:** Owned by classifiers

## Common Patterns in Code

### Qualified Name Resolution

```rust
// "A::B::C" → ["A", "B", "C"]
fn split_qualified(name: &str) -> Vec<&str> {
    name.split("::").collect()
}
```

### Checking Definition vs Usage

```rust
fn is_definition(name: &str) -> bool {
    name.ends_with(" def") || 
    name.contains("def ") ||
    // ... check AST node type
}
```

### Import Resolution Order

1. Direct (member) imports: `import A::B`
2. Wildcard imports: `import A::*`
3. Recursive imports: `import A::*::**`

## Glossary

| Term | Definition |
|------|------------|
| **Classifier** | Type that can have features |
| **Definition** | Type declaration (like a class) |
| **Feature** | Property or behavior of classifier |
| **KerML** | Kernel Modeling Language (foundation) |
| **Namespace** | Container for named elements |
| **Package** | Primary namespace container |
| **Qualified Name** | Full path: `A::B::C` |
| **Redefinition** | Overriding inherited feature |
| **Simple Name** | Unqualified name: `C` |
| **Specialization** | Inheritance relationship |
| **Subsetting** | Refinement of inherited feature |
| **SysML** | Systems Modeling Language |
| **Typing** | Instance-of relationship |
| **Usage** | Instance declaration |
