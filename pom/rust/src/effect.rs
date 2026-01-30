use crate::{
    action::PreparedAction,
    ep::EngagementToken,
    ep_anchor::{FirstPartialIrreversibility, CommitError},
};

/// Effect Leaf — terminal node (no descendants).
pub trait EffectLeaf {
    fn apply(
        self,
        action: PreparedAction,
        token: EngagementToken,
        anchor: &mut dyn FirstPartialIrreversibility,
    ) -> Result<(), CommitError>;
}

/// Null effect for PoM — demonstrates topology without real I/O.
pub struct NullEffect;

impl EffectLeaf for NullEffect {
    fn apply(
        self,
        action: PreparedAction,
        token: EngagementToken,
        anchor: &mut dyn FirstPartialIrreversibility,
    ) -> Result<(), CommitError> {
        // Token consumed HERE, atomically with anchor.commit() call.
        anchor.commit(token, &action)
    }
}
