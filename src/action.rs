use core::marker::PhantomData;
use crate::types::Cost;

/// Kinds of State Transitions permitted in the instrumented domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionKind {
    Write,
    Rotate,
    Destroy,
}

/// InputAction: outside instrumented domain (user-provided).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InputAction {
    pub kind: ActionKind,
    pub cost: Cost,
}

/// PreparedAction: inside Reversible Zone (RZ).
///
/// Invariants:
/// - Pure mapping (no I/O, no allocation, no side effects).
/// - Copy => no Drop => zero residue on interruption (in the model).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PreparedAction {
    pub kind: ActionKind,
    pub cost: Cost,
    _no_drop: PhantomData<fn() -> !>,
}

impl InputAction {
    /// Pure preparation step — reversible by construction.
    pub const fn prepare(self) -> PreparedAction {
        PreparedAction {
            kind: self.kind,
            cost: self.cost,
            _no_drop: PhantomData,
        }
    }
}
