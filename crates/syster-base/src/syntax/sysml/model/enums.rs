#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementKind {
    Objective,
    Verify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterKind {
    Actor,
    Stakeholder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementConstraintKind {
    Assume,
    Require,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortionKind {
    Timeslice,
    Snapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    When,
    At,
    After,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateSubactionKind {
    Entry,
    Do,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionFeatureKind {
    Trigger,
    Guard,
    Effect,
}
