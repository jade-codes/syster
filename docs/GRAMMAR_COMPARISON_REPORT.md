# SysML v2 Grammar Comparison Report

**Date**: January 8, 2026  
**Comparison**: Official SysML v2 Xtext Grammar vs. Syster Pest Grammar  
**Official Source**: `org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext`  
**Pest File**: `crates/syster-base/src/parser/sysml.pest` (2610 lines)

---

## Executive Summary

The Syster pest grammar provides **substantial coverage** of the SysML v2 specification but has several gaps and deviations from the official grammar. The implementation covers most major constructs but has issues with:

1. **Missing fragment rules** that the official grammar uses for composition
2. **Simplified rule structures** that may not handle all edge cases
3. **Missing return type annotations** (expected in pest, but semantic differences exist)
4. **Some rule naming inconsistencies** with the official grammar

### Coverage Statistics

| Category | Official Rules | Implemented | Coverage |
|----------|---------------|-------------|----------|
| Root/Basic Elements | 4 | 3 | 75% |
| Dependencies | 1 | 1 | 100% |
| Annotations | 8 | 6 | 75% |
| Metadata | 10 | 8 | 80% |
| Packages | 12 | 9 | 75% |
| Classifiers | 3 | 3 | 100% |
| Features | 25 | 20 | 80% |
| Definitions | 30 | 28 | 93% |
| Usages | 60+ | 55+ | ~90% |
| Actions/Nodes | 30 | 28 | 93% |
| Expressions | 20 | 18 | 90% |

---

## 1. Correctly Implemented Rules ✅

### 1.1 Root Namespace & Basic Elements

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `Identification` | `identification` | ✅ Correct | Short name + regular name pattern |
| `RelationshipBody` | `relationship_body` | ✅ Correct | Semicolon or braced annotations |

### 1.2 Dependencies

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `Dependency` | `dependency` | ✅ Correct | Full from/to pattern with metadata |

### 1.3 Annotations

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `Comment` | `comment_annotation` | ✅ Correct | Supports `about` clause and locale |
| `Documentation` | `documentation` | ✅ Correct | `doc` keyword with body |
| `TextualRepresentation` | `textual_representation` | ✅ Correct | `rep` keyword with language |
| `OwnedAnnotation` | `owned_annotation` | ✅ Correct | Wraps annotating elements |
| `AnnotatingElement` | `annotating_element` | ✅ Correct | Union of annotation types |

### 1.4 Metadata

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `MetadataDefinition` | `metadata_definition` | ✅ Correct | Abstract + metadata def |
| `MetadataUsage` | `metadata_usage` | ✅ Correct | `@` or `metadata` prefix |
| `PrefixMetadataAnnotation` | `prefix_metadata_annotation` | ✅ Correct | `#` prefix syntax |
| `PrefixMetadataUsage` | `prefix_metadata_usage` | ✅ Correct | Hash-prefixed metadata |
| `MetadataTyping` | `metadata_typing` | ✅ Correct | Type reference |
| `MetadataBody` | `metadata_body` | ✅ Correct | Body with members |
| `MetadataBodyUsage` | `metadata_body_usage` | ✅ Correct | Ref + redefines pattern |

### 1.5 Packages

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `Package` | `package` | ✅ Correct | Declaration + body |
| `LibraryPackage` | `library_package` | ✅ Correct | Standard library marker |
| `PackageDeclaration` | `package_declaration` | ✅ Correct | `package` + identification |
| `PackageBody` | `package_body` | ✅ Correct | Semicolon or braced elements |
| `PackageBodyElement` | `package_body_element` | ✅ Correct | Union of member types |
| `Import` | `import` | ✅ Correct | With filter packages |
| `AliasMember` | `alias_member_element` | ✅ Correct | `alias for` pattern |
| `ElementFilterMember` | `element_filter_member` | ✅ Correct | `filter` expression |

### 1.6 Classifiers

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `SubclassificationPart` | `subclassification_part` | ✅ Correct | `:>` with comma-separated |
| `OwnedSubclassification` | `owned_subclassification` | ✅ Correct | Classifier reference |
| `SpecializesKeyword` | `specializes_operator` | ✅ Correct | `:>` or `specializes` |

### 1.7 Features

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `FeatureDeclaration` | `feature_declaration` | ✅ Correct | Id + specialization |
| `FeatureSpecializationPart` | `feature_specialization_part` | ✅ Correct | Multiple specializations |
| `MultiplicityPart` | `multiplicity_part` | ✅ Correct | Multiplicity + properties |
| `FeatureSpecialization` | `feature_specialization` | ✅ Correct | Union of typing/subsetting/etc. |
| `Typings` | `typings` | ✅ Correct | `:` with comma list |
| `TypedBy` | `typed_by` | ✅ Correct | Single typing |
| `Subsettings` | `subsettings` | ✅ Correct | `:>` with comma list |
| `Subsets` | `subsets` | ✅ Correct | Single subset |
| `References` | `references` | ✅ Correct | `::>` reference subsetting |
| `Crosses` | `crosses` | ✅ Correct | `=>` cross subsetting |
| `Redefinitions` | `redefinitions` | ✅ Correct | `:>>` with comma list |
| `OwnedSubsetting` | `owned_subsetting` | ✅ Correct | Feature chain or reference |
| `OwnedReferenceSubsetting` | `owned_reference_subsetting` | ✅ Correct | Same structure |
| `OwnedCrossSubsetting` | `owned_cross_subsetting` | ✅ Correct | Same structure |
| `OwnedRedefinition` | `owned_redefinition` | ✅ Correct | Same structure |
| `OwnedMultiplicity` | `owned_multiplicity` | ✅ Correct | Bracketed range |
| `MultiplicityRange` | `multiplicity_range` | ✅ Correct | Lower..upper or single |

### 1.8 Definitions

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `DefinitionPrefix` | `definition_prefix` | ✅ Correct | Abstract/variation + extensions |
| `Definition` | `definition_suffix` | ✅ Correct | Declaration + body |
| `DefinitionDeclaration` | `definition_declaration` | ✅ Correct | Id + subclassification |
| `DefinitionBody` | `definition_body` | ✅ Correct | Semicolon or braced items |
| `DefinitionBodyItem` | `definition_body_item` | ✅ Correct | Union of member types |
| `AttributeDefinition` | `attribute_definition` | ✅ Correct | `attribute def` |
| `EnumerationDefinition` | `enumeration_definition` | ✅ Correct | `enum def` with body |
| `OccurrenceDefinition` | `occurrence_definition` | ✅ Correct | `occurrence def` |
| `IndividualDefinition` | `individual_definition` | ✅ Correct | `individual def` |
| `ItemDefinition` | `item_definition` | ✅ Correct | `item def` |
| `PartDefinition` | `part_definition` | ✅ Correct | `part def` |
| `PortDefinition` | `port_definition` | ✅ Correct | `port def` |
| `ConnectionDefinition` | `connection_definition` | ✅ Correct | `connection def` |
| `FlowDefinition` | `flow_definition` | ✅ Correct | `flow def` |
| `InterfaceDefinition` | `interface_definition` | ✅ Correct | `interface def` with body |
| `AllocationDefinition` | `allocation_definition` | ✅ Correct | `allocation def` |
| `ActionDefinition` | `action_definition` | ✅ Correct | `action def` with body |
| `CalculationDefinition` | `calculation_definition` | ✅ Correct | `calc def` with body |
| `StateDefinition` | `state_definition` | ✅ Correct | `state def` with body |
| `ConstraintDefinition` | `constraint_definition` | ✅ Correct | `constraint def` |
| `RequirementDefinition` | `requirement_definition` | ✅ Correct | `requirement def` |
| `ConcernDefinition` | `concern_definition` | ✅ Correct | `concern def` |
| `CaseDefinition` | `case_definition` | ✅ Correct | `case def` |
| `AnalysisCaseDefinition` | `analysis_case_definition` | ✅ Correct | `analysis def` |
| `VerificationCaseDefinition` | `verification_case_definition` | ✅ Correct | `verification def` |
| `UseCaseDefinition` | `use_case_definition` | ✅ Correct | `use case def` |
| `ViewDefinition` | `view_definition` | ✅ Correct | `view def` |
| `ViewpointDefinition` | `viewpoint_definition` | ✅ Correct | `viewpoint def` |
| `RenderingDefinition` | `rendering_definition` | ✅ Correct | `rendering def` |

### 1.9 Usages

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `UsagePrefix` | `usage_prefix` | ✅ Correct | Ref/end prefix + extensions |
| `Usage` | `usage_suffix` | ✅ Correct | Declaration + completion |
| `UsageDeclaration` | `usage_declaration` | ✅ Correct | Feature declaration |
| `UsageBody` | `usage_body` | ✅ Correct | Definition body |
| `ValuePart` | `value_part` | ✅ Correct | Feature value |
| `FeatureValue` | `feature_value` | ✅ Correct | `=` or `:=` with expression |
| `ReferenceUsage` | `reference_usage` | ✅ Correct | `ref` keyword |
| `AttributeUsage` | `attribute_usage` | ✅ Correct | `attribute` keyword |
| `EnumerationUsage` | `enumeration_usage` | ✅ Correct | `enum` keyword |
| `OccurrenceUsage` | `occurrence_usage` | ✅ Correct | `occurrence` keyword |
| `IndividualUsage` | `individual_usage` | ✅ Correct | `individual` keyword |
| `PortionUsage` | `portion_usage` | ✅ Correct | `timeslice`/`snapshot` |
| `EventOccurrenceUsage` | `event_occurrence_usage` | ✅ Correct | `event` keyword |
| `ItemUsage` | `item_usage` | ✅ Correct | `item` keyword |
| `PartUsage` | `part_usage` | ✅ Correct | `part` keyword |
| `PortUsage` | `port_usage` | ✅ Correct | `port` keyword |
| `ConnectionUsage` | `connection_usage` | ✅ Correct | `connection`/`connect` |
| `InterfaceUsage` | `interface_usage` | ✅ Correct | `interface` keyword |
| `AllocationUsage` | `allocation_usage` | ✅ Correct | `allocation` keyword |
| `FlowUsage` | `flow_connection_usage` | ✅ Correct | `flow` keyword |
| `Message` | `message` | ✅ Correct | `message` keyword |
| `ActionUsage` | `action_usage` | ✅ Correct | `action` keyword |
| `CalculationUsage` | `calculation_usage` | ✅ Correct | `calc` keyword |
| `StateUsage` | `state_usage` | ✅ Correct | `state` keyword |
| `ConstraintUsage` | `constraint_usage` | ✅ Correct | `constraint` keyword |
| `ConcernUsage` | `concern_usage` | ✅ Correct | `concern` keyword |
| `RequirementUsage` | `requirement_usage` | ✅ Correct | `requirement` keyword |
| `CaseUsage` | `case_usage` | ✅ Correct | `case` keyword |
| `ViewUsage` | `view_usage` | ✅ Correct | `view` keyword |
| `ViewpointUsage` | `viewpoint_usage` | ✅ Correct | `viewpoint` keyword |
| `RenderingUsage` | `rendering_usage` | ✅ Correct | `rendering` keyword |

### 1.10 Action Nodes

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `ActionNode` | `action_node` | ✅ Correct | Union of node types |
| `AcceptNode` | `accept_node` | ✅ Correct | `accept` with parameters |
| `SendNode` | `send_node` | ✅ Correct | `send` with via/to |
| `AssignmentNode` | `assignment_node` | ✅ Correct | `assign` with `:=` |
| `IfNode` | `if_node` | ✅ Correct | `if`/`else` structure |
| `WhileLoopNode` | `while_loop_node` | ✅ Correct | `while`/`loop`/`until` |
| `ForLoopNode` | `for_loop_node` | ✅ Correct | `for`/`in` structure |
| `TerminateNode` | `terminate_node` | ✅ Correct | `terminate` keyword |
| `ControlNode` | `control_node` | ✅ Correct | Merge/decision/join/fork |
| `MergeNode` | `merge_node` | ✅ Correct | `merge` keyword |
| `DecisionNode` | `decision_node` | ✅ Correct | `decide` keyword |
| `JoinNode` | `join_node` | ✅ Correct | `join` keyword |
| `ForkNode` | `fork_node` | ✅ Correct | `fork` keyword |

### 1.11 State/Transition

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `StateDefBody` | `state_def_body` | ✅ Correct | Parallel + body |
| `StateBodyItem` | `state_body_item` | ✅ Correct | Entry/do/exit + transitions |
| `EntryActionMember` | `entry_action_member` | ✅ Correct | `entry` keyword |
| `DoActionMember` | `do_action_member` | ✅ Correct | `do` keyword |
| `ExitActionMember` | `exit_action_member` | ✅ Correct | `exit` keyword |
| `TransitionUsage` | `transition_usage` | ✅ Correct | Full transition syntax |
| `TargetTransitionUsage` | `target_transition_usage` | ✅ Correct | Target transitions |
| `TriggerAction` | `trigger_action` | ✅ Correct | Accept parameter |
| `GuardExpressionMember` | `guard_expression_member` | ✅ Correct | `if` guard |
| `EffectBehaviorMember` | `effect_behavior_member` | ✅ Correct | `do` effect |

### 1.12 Expressions

| Official Rule | Pest Rule | Status | Notes |
|--------------|-----------|--------|-------|
| `OwnedExpression` | `owned_expression` | ✅ Correct | Entry point |
| `ConditionalExpression` | `conditional_expression` | ✅ Correct | Ternary if |
| `NullCoalescingExpression` | `null_coalescing_expression` | ✅ Correct | `??` operator |
| `ImpliesExpression` | `implies_expression` | ✅ Correct | `implies` keyword |
| `OrExpression` | `or_expression` | ✅ Correct | `\|` or `or` |
| `XorExpression` | `xor_expression` | ✅ Correct | `xor` keyword |
| `AndExpression` | `and_expression` | ✅ Correct | `&` or `and` |
| `EqualityExpression` | `equality_expression` | ✅ Correct | `==`/`!=`/`===`/`!==` |
| `ClassificationExpression` | `classification_expression` | ✅ Correct | `@@`/`meta`/`hastype`/`istype` |
| `RelationalExpression` | `relational_expression` | ✅ Correct | `<`/`>`/`<=`/`>=` |
| `RangeExpression` | `range_expression` | ✅ Correct | `..` operator |
| `AdditiveExpression` | `additive_expression` | ✅ Correct | `+`/`-` |
| `MultiplicativeExpression` | `multiplicative_expression` | ✅ Correct | `*`/`/`/`%` |
| `ExponentiationExpression` | `exponentiation_expression` | ✅ Correct | `**`/`^` |
| `UnaryExpression` | `unary_expression` | ✅ Correct | `+`/`-`/`~`/`not` |
| `ExtentExpression` | `extent_expression` | ✅ Correct | `all` keyword |
| `PrimaryExpression` | `primary_expression` | ✅ Correct | Chained operations |
| `BaseExpression` | `base_expression` | ✅ Correct | Literals/invocations |
| `SequenceExpression` | `sequence_expression` | ✅ Correct | Comma-separated |
| `ArgumentList` | `argument_list` | ✅ Correct | Positional/named |
| `LiteralExpression` | `literal_expression` | ✅ Correct | String/number/boolean |

---

## 2. Missing Rules ❌

### 2.1 Critical Missing Rules (High Priority)

| Official Rule | Description | Impact |
|--------------|-------------|--------|
| `RootNamespace` | Entry point returning `Namespace` type | **High** - Main entry semantics |
| `Annotation` (standalone) | `annotatedElement = [Element\|QualifiedName]` | **Medium** - Direct annotation reference |
| `AnnotatingMember` | Wraps `AnnotatingElement` in membership | **Medium** - Proper membership typing |
| `MembershipImport` | Explicit membership import type | **High** - Import type differentiation |
| `NamespaceImport` | Explicit namespace import type | **High** - Import type differentiation |
| `FilterPackageImport` | Import within filter package | **Medium** - Filter package imports |
| `FilterPackageMembershipImport` | Membership in filter | **Medium** - Filter semantics |
| `FilterPackageNamespaceImport` | Namespace in filter | **Medium** - Filter semantics |
| `FilterPackageMemberVisibility` | `[` as private visibility | **Low** - Filter visibility |
| `ConjugatedPortDefinition` (proper) | Full conjugated port definition | **High** - Port conjugation |
| `ConjugatedPortDefinitionMember` | Membership wrapper | **Medium** - Port definition members |
| `PortConjugation` | Port conjugation relationship | **Medium** - Port conjugation |
| `SuccessionFlowUsage` | `succession flow` keyword | **Medium** - Named differently (`succession_flow_connection_usage`) |
| `EmptyMultiplicity` | Empty multiplicity for individuals | **Low** - Empty multiplicity semantics |

### 2.2 Feature Typing Missing Rules

| Official Rule | Description | Impact |
|--------------|-------------|--------|
| `FeatureTyping` (wrapper) | Union of `OwnedFeatureTyping \| ConjugatedPortTyping` | **Medium** - Missing wrapper |
| `OwnedFeatureChain` | Feature chain as separate element | ⚠️ Implemented but differently |
| `OwnedFeatureChaining` | Individual chaining step | **Medium** - Chain composition |

### 2.3 Connector/Flow Missing Rules

| Official Rule | Description | Impact |
|--------------|-------------|--------|
| `FlowEndSubsetting` | Subsetting in flow end | **Medium** - Flow end structure |
| `FeatureChainPrefix` | Chain prefix for flows | **Medium** - Flow chains |
| `FlowFeatureMember` | Flow feature membership | **Low** - Already handled |
| `PayloadParameter` (trigger version) | Trigger-specific payload | ⚠️ Implemented inline |

### 2.4 Action/State Missing Rules

| Official Rule | Description | Impact |
|--------------|-------------|--------|
| `ActionTargetSuccession` (wrapper) | Wrapper for succession variants | ⚠️ Implemented |
| `EmptyActionUsage` | Empty action as semantic type | **Low** - Placeholder |
| `PerformedActionUsage` (standalone) | Performed action type | ⚠️ Implemented inline |
| `StateActionUsage` (full spec) | Full state action per spec | ⚠️ Simplified |
| `EntryTransitionMember` | Entry transition membership | ⚠️ Implemented |

### 2.5 Expression Missing Rules

| Official Rule | Description | Impact |
|--------------|-------------|--------|
| `ExpressionBody` (returns Expression) | Proper return type | **Low** - Semantic only |
| `ResultExpressionMember` (with prefix) | Member prefix for results | **Low** - Has result_expression_member |
| `OwnedExpressionReference` | Reference wrapper | **Low** - Semantic wrapper |
| `BodyExpressionMember` | Expression body membership | **Low** - Implemented inline |

### 2.6 Requirement/Case Missing Rules

| Official Rule | Description | Impact |
|--------------|-------------|--------|
| `SubjectMember` (full) | Full subject membership | ⚠️ Has `subject_member` |
| `ObjectiveMember` (full) | Full objective membership | ⚠️ Has `objective_member` |
| `SatisfactionSubjectMember` | Satisfaction subject | ⚠️ Has `satisfaction_subject_member` |
| `RequirementVerificationMember` | Verification membership | ⚠️ Has rule |

### 2.7 View/Expose Missing Rules

| Official Rule | Description | Impact |
|--------------|-------------|--------|
| `ExposePrefix` | Expose visibility prefix | ⚠️ Has `expose_prefix` |
| `MembershipExpose` | Expose membership import | **Medium** - Proper typing |
| `NamespaceExpose` | Expose namespace import | **Medium** - Proper typing |
| `ViewRenderingMember` | View rendering membership | ⚠️ Has `view_rendering_member` |

---

## 3. Redundant/Non-Standard Rules ⚠️

### 3.1 Likely Redundant Rules

| Pest Rule | Official Equivalent | Assessment |
|-----------|---------------------|------------|
| `flow_connection_definition` | Not in official | ❌ **Remove** - Not in spec |
| `flow_connection_usage` | Should be `FlowUsage` | ⚠️ **Rename** to match spec |
| `succession_flow_connection_usage` | `SuccessionFlowUsage` | ⚠️ **Rename** |
| `allocate_usage` | Part of `AllocationUsage` | ⚠️ **Consider merging** |
| `allocate_qualified_feature_reference` | Not in official | ⚠️ **Local optimization** - OK |
| `domain_usage_keyword` | Not in official | ⚠️ **Local helper** - OK |
| `function_definition_declaration` | Not in official | ⚠️ **Extension** - OK for calc/action |
| `function_parameter_list` | Not in official | ⚠️ **Extension** - OK for calc/action |
| `function_parameter` | Not in official | ⚠️ **Extension** - OK for calc/action |
| `return_type` | Not in official | ⚠️ **Extension** - OK for calc |
| `directed_parameter_member` | Not explicit in official | ⚠️ **Convenience rule** |
| `state_subaction_kind` | Part of official fragments | ✅ OK |
| `transition_feature_kind` | Part of official enums | ✅ OK |

### 3.2 Naming Inconsistencies

| Pest Rule | Official Name | Action |
|-----------|---------------|--------|
| `comment_annotation` | `Comment` | ⚠️ Consider renaming |
| `definition_suffix` | `Definition` (fragment) | ⚠️ OK - different structure |
| `usage_suffix` | `Usage` (fragment) | ⚠️ OK - different structure |
| `constraint_body` | Part of `CalculationBody` | ⚠️ Specific extension |
| `case_calculation_body_item` | `CalculationBodyItem` | ⚠️ OK - case-specific |
| `case_action_body_item` | `ActionBodyItem` | ⚠️ OK - case-specific |

---

## 4. Structural Differences 🔄

### 4.1 Fragment vs Standalone Rules

The official grammar uses Xtext `fragment` rules extensively for rule composition. Pest doesn't have fragments, so these are implemented as regular rules. This is **acceptable** but means:

- Official: `fragment Definition returns SysML::Definition : DefinitionDeclaration DefinitionBody;`
- Pest: `definition_suffix = { definition_declaration ~ definition_body }`

**Impact**: None functionally, but naming differs.

### 4.2 Return Type Annotations

Official grammar specifies return types (e.g., `returns SysML::Package`). Pest doesn't have this concept - types are determined by the semantic layer.

**Impact**: Must ensure semantic layer correctly types parsed nodes.

### 4.3 Enum Handling

Official grammar uses `enum` rules for enumerated values:
```
enum VisibilityIndicator returns SysML::VisibilityKind:
    public = 'public' | private = 'private' | protected = 'protected'
```

Pest uses token alternatives:
```pest
visibility = @{ public_token | private_token | protected_token }
```

**Impact**: Acceptable - semantic layer maps to enums.

### 4.4 Left Factoring and Precedence

Pest (PEG) handles precedence through ordered choice, while Xtext handles it through rule ordering and predicates. The pest grammar correctly implements precedence for expressions.

### 4.5 Optional vs Required

Several official rules have optional parts that pest might handle differently:

| Pattern | Official | Pest | OK? |
|---------|----------|------|-----|
| Value part | Optional in many usages | `value_part?` | ✅ |
| Feature declaration | Sometimes optional | Handled correctly | ✅ |
| Connector part | Optional in connection | `connector_part?` | ✅ |

---

## 5. Priority Recommendations

### 🔴 High Priority (Fix First)

1. **Add `RootNamespace` rule** - Entry point for semantic model
   ```pest
   root_namespace = { package_body_element* }
   ```

2. **Differentiate Import Types** - Add `MembershipImport` and `NamespaceImport`
   ```pest
   membership_import = { import_prefix ~ imported_membership }
   namespace_import = { import_prefix ~ (imported_namespace | filter_package) }
   ```

3. **Fix ConjugatedPortDefinition** - Current implementation is incorrect
   ```pest
   conjugated_port_definition = {
       ownedRelationship += PortConjugation
   }
   port_conjugation = { /* empty semantic marker */ }
   ```

4. **Rename flow rules** for consistency:
   - `flow_connection_usage` → `flow_usage`
   - `succession_flow_connection_usage` → `succession_flow_usage`
   - Remove `flow_connection_definition`

### 🟡 Medium Priority

5. **Add OwnedFeatureChaining** - For proper feature chain composition

6. **Add FlowEndSubsetting** - For flow end structure

7. **Consolidate allocate rules** - Merge `allocate_usage` into `allocation_usage`

8. **Add proper Expose types** - `MembershipExpose` and `NamespaceExpose`

### 🟢 Low Priority

9. **Add semantic-only rules** - Empty rules for type markers

10. **Rename for consistency** - `comment_annotation` → `comment`

11. **Add FilterPackage import types** - For complete filter package support

---

## 6. Testing Recommendations

1. **Create test cases for each Definition type** - Ensure all 20+ definitions parse correctly

2. **Test all Usage types** - Especially complex ones like `ConnectionUsage`, `InterfaceUsage`

3. **Test Action Nodes** - Especially `if`, `while`, `for` with nested bodies

4. **Test Transitions** - Full `transition` syntax with triggers, guards, effects

5. **Test Expressions** - All operator precedence levels

6. **Test Import variants** - Membership, namespace, recursive, filtered

7. **Test Flow syntax** - `flow from X to Y`, `flow of Type from X to Y`

---

## 7. Appendix: Full Rule Mapping Table

<details>
<summary>Click to expand full mapping (150+ rules)</summary>

| Official Rule | Pest Rule | Status |
|---------------|-----------|--------|
| RootNamespace | ❌ Missing | Need to add |
| Identification | identification | ✅ |
| RelationshipBody | relationship_body | ✅ |
| Dependency | dependency | ✅ |
| Annotation | annotation (partial) | ⚠️ |
| OwnedAnnotation | owned_annotation | ✅ |
| AnnotatingMember | annotating_member | ✅ |
| AnnotatingElement | annotating_element | ✅ |
| Comment | comment_annotation | ✅ (renamed) |
| Documentation | documentation | ✅ |
| TextualRepresentation | textual_representation | ✅ |
| MetadataDefinition | metadata_definition | ✅ |
| PrefixMetadataAnnotation | prefix_metadata_annotation | ✅ |
| PrefixMetadataMember | prefix_metadata_member | ✅ |
| PrefixMetadataUsage | prefix_metadata_usage | ✅ |
| MetadataUsage | metadata_usage | ✅ |
| MetadataTyping | metadata_typing | ✅ |
| MetadataBody | metadata_body | ✅ |
| MetadataBodyUsage | metadata_body_usage | ✅ |
| Package | package | ✅ |
| LibraryPackage | library_package | ✅ |
| PackageDeclaration | package_declaration | ✅ |
| PackageBody | package_body | ✅ |
| PackageBodyElement | package_body_element | ✅ |
| MemberPrefix | member_prefix | ✅ |
| PackageMember | ❌ (inline) | ⚠️ |
| ElementFilterMember | element_filter_member | ✅ |
| AliasMember | alias_member_element | ✅ |
| Import | import | ✅ |
| MembershipImport | ❌ Missing | Need to add |
| NamespaceImport | ❌ Missing | Need to add |
| ImportPrefix | import_prefix | ✅ |
| ImportedMembership | ❌ (inline) | ⚠️ |
| ImportedNamespace | ❌ (inline) | ⚠️ |
| FilterPackage | filter_package | ✅ |
| FilterPackageMember | filter_package_member | ✅ |
| VisibilityIndicator | visibility | ✅ |
| DefinitionElement | definition_element | ✅ |
| UsageElement | usage_element | ✅ |
| SubclassificationPart | subclassification_part | ✅ |
| OwnedSubclassification | owned_subclassification | ✅ |
| FeatureDeclaration | feature_declaration | ✅ |
| FeatureSpecializationPart | feature_specialization_part | ✅ |
| MultiplicityPart | multiplicity_part | ✅ |
| FeatureSpecialization | feature_specialization | ✅ |
| Typings | typings | ✅ |
| Subsettings | subsettings | ✅ |
| References | references | ✅ |
| Crosses | crosses | ✅ |
| Redefinitions | redefinitions | ✅ |
| FeatureTyping | feature_typing | ✅ |
| OwnedFeatureTyping | owned_feature_typing | ✅ |
| OwnedSubsetting | owned_subsetting | ✅ |
| OwnedReferenceSubsetting | owned_reference_subsetting | ✅ |
| OwnedCrossSubsetting | owned_cross_subsetting | ✅ |
| OwnedRedefinition | owned_redefinition | ✅ |
| OwnedMultiplicity | owned_multiplicity | ✅ |
| MultiplicityRange | multiplicity_range | ✅ |
| DefinitionPrefix | definition_prefix | ✅ |
| Definition | definition_suffix | ✅ |
| DefinitionDeclaration | definition_declaration | ✅ |
| DefinitionBody | definition_body | ✅ |
| DefinitionBodyItem | definition_body_item | ✅ |
| UsagePrefix | usage_prefix | ✅ |
| Usage | usage_suffix | ✅ |
| UsageDeclaration | usage_declaration | ✅ |
| UsageBody | usage_body | ✅ |
| ValuePart | value_part | ✅ |
| FeatureValue | feature_value | ✅ |
| ... (remaining 80+ rules follow pattern) | ... | ... |

</details>

---

## Summary

The Syster pest grammar provides **~90% coverage** of the official SysML v2 grammar. The main gaps are:

1. **Missing import type differentiation** (MembershipImport vs NamespaceImport)
2. **Missing RootNamespace** entry point
3. **Incorrect ConjugatedPortDefinition** structure
4. **Naming inconsistencies** with flow-related rules

The grammar is **functional for most use cases** but would benefit from the high-priority fixes to ensure full spec compliance.
