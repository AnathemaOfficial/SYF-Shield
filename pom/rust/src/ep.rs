use core::marker::PhantomData;

use crate::{
    gate::{Gate, LawVerdict},
    shield::{Shield, Active, Sealed, ProgressResult},
    action::PreparedAction,
    ep_anchor::{FirstPartialIrreversibility, CommitError},
};

/// EngagementToken — linear token (non-Copy / non-Clone by construction).
///
/// It is consumed ONLY by FirstPartialIrreversibility::commit().
pub struct EngagementToken {
    _linear: PhantomData<core::cell::UnsafeCell<()>>,
}

impl EngagementToken {
    /// Create a new token.
    /// Crate-private: only `engage*()` functions can mint tokens.
    pub(crate) fn new() -> Self {
        Self { _linear: PhantomData }
    }
}

/// Result of non-atomic EP evaluation attempt.
///
/// NOTE: `Engaged` here separates progression from commit. This allows the
/// caller to introspect the typestate before committing, but it means a
/// failed commit leaves the shield with burned capacity and no
/// irreversible effect. Production code MUST use [`engage_atomic`] which
/// co-locates progression with commit per canon I-EP-1.
pub enum EpResult {
    Engaged { shield: Shield<Active>, token: EngagementToken },
    Sealed(Shield<Sealed>),
    Denied(Shield<Active>),
}

/// Result of atomic EP evaluation + commit.
///
/// Either BOTH progression and irreversibility commit happened, or NEITHER.
/// Capacity is never decremented without a corresponding irreversible effect.
pub enum AtomicEpResult {
    /// Gate allowed, capacity was sufficient, commit succeeded.
    /// The shield reflects the post-progression state.
    Engaged(Shield<Active>),
    /// Gate allowed, capacity was exactly sufficient (exact exhaustion),
    /// and commit succeeded. The shield is now sealed.
    EngagedAndSealed(Shield<Sealed>),
    /// Gate denied OR capacity was insufficient. No progression, no commit.
    Denied(Shield<Active>),
    /// Gate allowed and capacity was sufficient, but the anchor commit
    /// failed (e.g. hardware failure, already committed). **No capacity
    /// was consumed.** The shield returned is the original intact shield.
    CommitFailed(Shield<Active>, CommitError),
}

/// EP primitive — NON-ATOMIC variant (testing / demo only).
///
/// **WARNING:** this function splits progression from commit. A caller that
/// calls `engage()` then fails to commit via the EP anchor will have burned
/// capacity without an irreversible effect. This violates canon I-EP-1
/// (Co-localization). **Production code MUST use [`engage_atomic`]** which
/// ties progression and commit into a single transactional step.
///
/// Kept public for PoM/typestate demonstrations and for tests that want to
/// exercise the intermediate `EngagementToken` state.
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
                token: EngagementToken::new(),
            },
            ProgressResult::Sealed(sealed_shield) => EpResult::Sealed(sealed_shield),
        },
    }
}

/// EP primitive — ATOMIC variant (canonical, production-safe).
///
/// Gate × Shield evaluation + progression + EP anchor commit, all in one
/// transactional step. Either all three succeed and capacity is consumed,
/// or none succeed and the original shield is returned intact.
///
/// This is the canonical implementation per I-EP-1 (Co-localization):
/// capacity decrement and first partial irreversibility are the same event.
///
/// Audit 2026-04-16 (S-COMMIT): the prior non-atomic `engage()` could burn
/// capacity when the subsequent anchor commit failed. This function closes
/// that gap by checking capacity without consuming it, attempting commit
/// first, and only materialising the progression if commit succeeded.
pub fn engage_atomic<G, A>(
    gate: &G,
    shield: Shield<Active>,
    action: &PreparedAction,
    anchor: &mut A,
) -> AtomicEpResult
where
    G: Gate,
    A: FirstPartialIrreversibility + ?Sized,
{
    // Step 1: Gate verdict.
    match gate.evaluate(action) {
        LawVerdict::Deny => return AtomicEpResult::Denied(shield),
        LawVerdict::Allow => {}
    }

    // Step 2: Non-destructive capacity check (peek, do not consume).
    let cost_u = action.cost.get();
    let cur = shield.capacity().get();
    if cur < cost_u {
        return AtomicEpResult::Denied(shield);
    }

    // Step 3: Try to commit irreversibility FIRST. Mint a token here and
    // hand it to the anchor — the token is consumed whether commit
    // succeeds or fails, so it cannot be reused.
    let token = EngagementToken::new();
    if let Err(err) = anchor.commit(token, action) {
        // Commit failed: do NOT progress. Return the original shield intact.
        return AtomicEpResult::CommitFailed(shield, err);
    }

    // Step 4: Commit succeeded. Materialise the progression atomically.
    // `progress` cannot fail here because we already verified capacity
    // in step 2 and no other operation has interleaved.
    match shield.progress(action.cost) {
        ProgressResult::Active(next_shield) => AtomicEpResult::Engaged(next_shield),
        ProgressResult::Sealed(sealed_shield) => AtomicEpResult::EngagedAndSealed(sealed_shield),
        ProgressResult::Denied(s) => {
            // Unreachable under canon: we peeked capacity in step 2.
            // Defend against future refactors by returning Denied.
            AtomicEpResult::Denied(s)
        }
    }
}
