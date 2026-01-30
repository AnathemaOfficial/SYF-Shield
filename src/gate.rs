use crate::action::{PreparedAction, ActionKind};

/// Gate verdict — purely structural (does action violate invariants?).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LawVerdict {
    Allow,
    Deny,
}

/// Gate trait — law-layer interface (does not know Capacity).
pub trait Gate {
    fn evaluate(&self, action: &PreparedAction) -> LawVerdict;
}

/// Minimal Gate for PoM — rejects Destroy, allows others.
pub struct MinimalGate;

impl Gate for MinimalGate {
    fn evaluate(&self, action: &PreparedAction) -> LawVerdict {
        match action.kind {
            ActionKind::Destroy => LawVerdict::Deny,
            _ => LawVerdict::Allow,
        }
    }
}
