use core::marker::PhantomData;
use crate::types::{Capacity, Cost};

/// Active state — path to EP exists.
pub struct Active;

/// Sealed state — path to EP structurally absent.
pub struct Sealed;

/// Shield<S> — typestate encodes path availability.
pub struct Shield<S> {
    cap: Capacity,
    _state: PhantomData<S>,
}

impl Shield<Active> {
    /// Construct Active Shield with initial capacity.
    pub const fn new(capacity: Capacity) -> Self {
        Self { cap: capacity, _state: PhantomData }
    }

    /// Remaining capacity.
    pub const fn capacity(&self) -> Capacity {
        self.cap
    }

    /// Progression: consume capacity (uST) if sufficient.
    ///
    /// Canon correction:
    /// - If insufficient capacity => DENY (Active) (no progression, no sealing)
    /// - Sealed only on Exhaustion (capacity becomes 0) after valid consumption
    pub fn progress(self, cost: Cost) -> ProgressResult {
        let cost_u = cost.get();
        let cur = self.cap.get();

        if cur < cost_u {
            // Fail-closed: insufficient capacity => no effect, no progression, no seal
            return ProgressResult::Denied(self);
        }

        let next = cur - cost_u;

        if next == 0 {
            ProgressResult::Sealed(Shield::<Sealed> {
                cap: Capacity::zero(),
                _state: PhantomData,
            })
        } else {
            ProgressResult::Active(Shield::<Active> {
                cap: Capacity::new(next),
                _state: PhantomData,
            })
        }
    }
}

impl Shield<Sealed> {
    /// Remaining capacity (always zero).
    pub const fn capacity(&self) -> Capacity {
        self.cap
    }

    // CRITICAL: No progress() / engage() methods here.
}

/// Result of progression attempt.
pub enum ProgressResult {
    Active(Shield<Active>),
    Sealed(Shield<Sealed>),
    Denied(Shield<Active>),
}

// --- TEST HELPERS (never in production path) ---
#[cfg(any(test, feature = "test-support"))]
impl Shield<Sealed> {
    /// Construct a SEALED shield for UI compile-fail tests only.
    /// Never used in production code — purely for proving "path absence" at compile time.
    pub fn sealed_for_test() -> Self {
        Shield::<Sealed> {
            cap: Capacity::zero(),
            _state: core::marker::PhantomData,
        }
    }
}
