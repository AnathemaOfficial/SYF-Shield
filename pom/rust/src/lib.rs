#![no_std]
#![forbid(unsafe_code)]

//! Minimal Proof-of-Mechanism for SYF-Shield Engagement Point (Canon v0.2 Phase 4.4)
//!
//! Topology: Reversible Zone (RZ) → Engagement Point (EP) → Irreversible Zone (IZ)
//!
//! Core claims (within instrumented domain):
//! - Token is linear (single-use).
//! - SEALED = structural path absence (typestate).
//! - Insufficient capacity => DENY (Active), not SEAL.

pub mod types;
pub mod action;
pub mod gate;
pub mod shield;
pub mod ep;
pub mod ep_anchor;
pub mod effect;

pub use types::*;
pub use action::*;
pub use gate::*;
pub use shield::*;
pub use ep::*;
pub use ep_anchor::*;
pub use effect::*;
