use core::marker::PhantomData;

use crate::{
    gate::{Gate, LawVerdict},
    shield::{Shield, Active, Sealed, ProgressResult},
    action::PreparedAction,
};

/// EngagementToken — linear token (non-Copy / non-Clone by construction).
///
/// It is consumed ONLY by FirstPartialIrreversibility::commit().
pub struct EngagementToken {
    _linear: PhantomData<core::cell::UnsafeCell<()>>,
}

/// Result of EP evaluation attempt.
pub enum EpResult {
    Engaged { shield: Shield<Active>, token: EngagementToken },
    Sealed(Shield<Sealed>),
    Denied(Shield<Active>),
}

/// EP primitive: Gate×Shield evaluation + progression coupling.
///
/// NOTE (canon v0.2): the canonical EP (first partial irreversibility)
/// occurs when the token is consumed by the EP anchor commit().
pub fn engage<G: Gate>(
    gate: &G,
    shield: Shield<Active>,
    action: &PreparedAction,
) -> EpResult {
    match gate.evaluate(action) {
        LawVerdict::Deny => EpResult::Denied(shield),
        LawVerdict::Allow => match shield.progress(action.cost) {
            ProgressResult::Denied(s) => EpResult::Denied(s),
            ProgressResult::Active(next_shield) => EpResult::Engaged {
                shield: next_shield,
                token: EngagementToken { _linear: PhantomData },
            },
            ProgressResult::Sealed(sealed_shield) => EpResult::Sealed(sealed_shield),
        },
    }
}
