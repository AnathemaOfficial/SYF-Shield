use core::num::NonZeroU32;

/// Unit of State Transition (uST) — thermodynamic accounting unit.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ust(pub u32);

/// Cost of a State Transition — strictly positive (no fractions, no zero).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cost(pub NonZeroU32);

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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Capacity(pub Ust);

impl Capacity {
    /// Zero capacity (Exhaustion).
    pub const fn zero() -> Self {
        Self(Ust(0))
    }

    /// Construct capacity from u32.
    pub const fn new(v: u32) -> Self {
        Self(Ust(v))
    }

    /// True if exhausted (capacity == 0).
    pub const fn is_exhausted(&self) -> bool {
        self.0 .0 == 0
    }

    /// Get the underlying u32.
    pub const fn get(self) -> u32 {
        self.0 .0
    }
}
