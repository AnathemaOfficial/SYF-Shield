use crate::{ep::EngagementToken, action::PreparedAction};

/// Error type for irreversibility commitment failure.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CommitError {
    HardwareFailure,
    AlreadyCommitted,
}

/// First Partial Irreversibility frontier — canonical EP anchor.
///
/// The contract here is minimal on purpose:
/// - commit() must be the first partially-irreversible operation in the domain.
/// - no callbacks/cascades (leaf constraint).
pub trait FirstPartialIrreversibility {
    fn commit(&mut self, token: EngagementToken, action: &PreparedAction) -> Result<(), CommitError>;
}

/// Null anchor for PoM testing — simulates irreversibility without side effects.
///
/// WARNING: demo only. Real AB-Soft must anchor this to a real frontier.
pub struct NullAnchor {
    committed: bool,
}

impl NullAnchor {
    pub const fn new() -> Self {
        Self { committed: false }
    }
}

impl FirstPartialIrreversibility for NullAnchor {
    fn commit(&mut self, _token: EngagementToken, _action: &PreparedAction) -> Result<(), CommitError> {
        if self.committed {
            return Err(CommitError::AlreadyCommitted);
        }
        self.committed = true;
        Ok(())
    }
}

/// Deterministic mock anchor for runtime tests.
/// Simulates partial irreversibility via a monotonic counter.
/// Counter survives "interruption" (simulated by dropping and recreating the anchor)
/// to model persistence after crash/reboot.
#[cfg(any(test, feature = "test-support"))]
pub struct MockAnchor {
    committed_count: u32,
}

#[cfg(any(test, feature = "test-support"))]
impl MockAnchor {
    pub const fn new() -> Self {
        Self { committed_count: 0 }
    }

    /// Simulate "interruption" (crash/reboot) — anchor state persists partially
    pub fn simulate_interruption(&self) -> Self {
        // Partial irreversibility: counter survives interruption
        Self {
            committed_count: self.committed_count,
        }
    }

    /// Check how many times irreversibility was committed
    pub fn committed_count(&self) -> u32 {
        self.committed_count
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FirstPartialIrreversibility for MockAnchor {
    fn commit(&mut self, _token: EngagementToken, _action: &PreparedAction) -> Result<(), CommitError> {
        self.committed_count += 1;
        Ok(())
    }
}
