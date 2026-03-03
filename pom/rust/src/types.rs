use core::num::NonZeroU32;

/// Unit of State Transition (uST) — thermodynamic accounting unit.
///
/// Inner field is private to prevent direct mutation.
/// Use `get()` for read access, `new()` for construction.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ust(u32);

impl Ust {
    /// Construct from u32.
    pub const fn new(v: u32) -> Self {
        Self(v)
    }

    /// Get the underlying u32 (read-only).
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Cost of a State Transition — strictly positive (no fractions, no zero).
///
/// Inner field is private: cost > 0 is guaranteed by NonZeroU32.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cost(NonZeroU32);

impl Cost {
    /// Construct Cost from u32 (returns None for 0).
    pub const fn new(v: u32) -> Option<Self> {
        match NonZeroU32::new(v) {
            Some(nz) => Some(Cost(nz)),
            None => None,
        }
    }

    /// Get the underlying u32.
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

/// Remaining capacity in uST units.
///
/// Inner field is private to enforce TI-2 (Non-Regeneration):
/// capacity is set once at initialization and only decremented by Progression.
/// No external code can mutate or regenerate capacity.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Capacity(Ust);

impl Capacity {
    /// Zero capacity (Exhaustion).
    pub const fn zero() -> Self {
        Self(Ust(0))
    }

    /// Construct capacity from u32.
    ///
    /// This sets the initial capacity. Once a Shield is created,
    /// capacity only decreases via `progress()` — never increases.
    pub const fn new(v: u32) -> Self {
        Self(Ust(v))
    }

    /// True if exhausted (capacity == 0).
    pub const fn is_exhausted(&self) -> bool {
        self.0.get() == 0
    }

    /// Get the underlying u32 (read-only).
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}
