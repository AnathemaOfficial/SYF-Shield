use syf_shield_pom::*;

#[test]
fn token_is_single_use() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(1).unwrap(),
    }
    .prepare();

    let shield = Shield::<Active>::new(Capacity::new(2));
    let (shield2, token) = match engage(&gate, shield, &action) {
        EpResult::Engaged { shield, token } => (shield, token),
        _ => panic!("expected Engaged"),
    };

    assert_eq!(shield2.capacity().get(), 1);

    let mut anchor = MockAnchor::new();
    let effect = NullEffect;

    // First apply: succeeds (token consumed)
    assert!(effect.apply(action, token, &mut anchor).is_ok());
    assert_eq!(anchor.committed_count(), 1);

    // Second apply: impossible (token already consumed — compiler prevents reuse)
    // This is enforced at compile time by token linearity, not runtime checks.
}

#[test]
fn insufficient_capacity_denies_but_does_not_seal() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(10).unwrap(), // cost > capacity
    }
    .prepare();

    let shield = Shield::<Active>::new(Capacity::new(5));
    match engage(&gate, shield, &action) {
        EpResult::Denied(active) => {
            // Capacity unchanged — no burn, no seal
            assert_eq!(active.capacity().get(), 5);
        }
        _ => panic!("expected Denied(Active)"),
    }
}

#[test]
fn exhaustion_seals_after_valid_consumption() {
    let gate = MinimalGate;
    let action1 = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(3).unwrap(),
    }
    .prepare();
    let action2 = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(2).unwrap(),
    }
    .prepare();

    let shield = Shield::<Active>::new(Capacity::new(5));

    // First action: consumes 3 → capacity=2 (still Active)
    let shield2 = match engage(&gate, shield, &action1) {
        EpResult::Engaged { shield, token } => {
            let mut anchor = MockAnchor::new();
            NullEffect.apply(action1, token, &mut anchor).unwrap();
            shield
        }
        _ => panic!("expected Engaged"),
    };
    assert_eq!(shield2.capacity().get(), 2);

    // Second action: consumes 2 → capacity=0 → SEALED
    match engage(&gate, shield2, &action2) {
        EpResult::Sealed(sealed) => {
            assert_eq!(sealed.capacity().get(), 0);
        }
        _ => panic!("expected Sealed"),
    }
}

#[test]
fn gate_deny_preserves_capacity() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Destroy, // MinimalGate rejects Destroy
        cost: Cost::new(1).unwrap(),
    }
    .prepare();

    let shield = Shield::<Active>::new(Capacity::new(10));
    match engage(&gate, shield, &action) {
        EpResult::Denied(active) => {
            // Gate denial => capacity unchanged
            assert_eq!(active.capacity().get(), 10);
        }
        _ => panic!("expected Denied from Gate"),
    }
}

#[test]
fn rz_interruption_leaves_zero_residue() {
    // Simulate interruption in RZ (before EP crossing)
    // RZ = pure preparation (Copy types only) → no observable residue after interruption
    let input = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(1).unwrap(),
    };
    let prepared = input.prepare(); // RZ step

    // Simulate crash: drop all RZ state
    let _ = prepared;
    let _ = input;

    // After "reboot": system state identical to before RZ entry
    // (no persistent residue — verified by absence of side effects in prepare())
    // This is a conceptual test; actual zero-residue requires panic=abort + no Drop types.
}

#[test]
fn multiple_engagements_decrement_capacity() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(1).unwrap(),
    }
    .prepare();

    let mut current_shield = Shield::<Active>::new(Capacity::new(3));
    let mut total_commits = 0u32;

    for i in 0..3 {
        match engage(&gate, current_shield, &action) {
            EpResult::Engaged { shield, token } => {
                let mut anchor = MockAnchor::new();
                NullEffect.apply(action, token, &mut anchor).unwrap();
                total_commits += anchor.committed_count();
                current_shield = shield;
            }
            EpResult::Sealed(_) => {
                // Expected on last iteration when capacity hits 0
                assert_eq!(i, 2, "should seal on third engagement");
                break;
            }
            EpResult::Denied(_) => panic!("unexpected denial"),
        }
    }

    // Note: each MockAnchor is fresh, so total_commits = 2 (last one sealed before commit)
    assert_eq!(total_commits, 2);
}

// =============================================================================
// engage_atomic tests — canon I-EP-1 (Co-localization) enforcement.
//
// Audit 2026-04-16 (S-COMMIT): the non-atomic engage() could burn capacity
// when the subsequent anchor commit failed. engage_atomic() closes that gap
// by co-locating progression with commit. These tests verify the atomicity.
// =============================================================================

#[test]
fn engage_atomic_denies_on_gate_reject() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Destroy, // MinimalGate rejects
        cost: Cost::new(1).unwrap(),
    }
    .prepare();

    let shield = Shield::<Active>::new(Capacity::new(5));
    let mut anchor = MockAnchor::new();

    match engage_atomic(&gate, shield, &action, &mut anchor) {
        AtomicEpResult::Denied(s) => {
            // Gate denied, capacity intact, no commit attempted.
            assert_eq!(s.capacity().get(), 5);
            assert_eq!(anchor.committed_count(), 0);
        }
        _ => panic!("expected Denied on gate reject"),
    }
}

#[test]
fn engage_atomic_denies_on_insufficient_capacity() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(10).unwrap(),
    }
    .prepare();

    let shield = Shield::<Active>::new(Capacity::new(5));
    let mut anchor = MockAnchor::new();

    match engage_atomic(&gate, shield, &action, &mut anchor) {
        AtomicEpResult::Denied(s) => {
            // Insufficient capacity, shield intact, no commit attempted.
            assert_eq!(s.capacity().get(), 5);
            assert_eq!(anchor.committed_count(), 0);
        }
        _ => panic!("expected Denied on insufficient capacity"),
    }
}

#[test]
fn engage_atomic_engages_on_sufficient_capacity() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(2).unwrap(),
    }
    .prepare();

    let shield = Shield::<Active>::new(Capacity::new(5));
    let mut anchor = MockAnchor::new();

    match engage_atomic(&gate, shield, &action, &mut anchor) {
        AtomicEpResult::Engaged(s) => {
            // Progression happened atomically with commit.
            assert_eq!(s.capacity().get(), 3);
            assert_eq!(anchor.committed_count(), 1);
        }
        _ => panic!("expected Engaged"),
    }
}

#[test]
fn engage_atomic_seals_on_exact_exhaustion() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(3).unwrap(),
    }
    .prepare();

    // Exact match: capacity == cost
    let shield = Shield::<Active>::new(Capacity::new(3));
    let mut anchor = MockAnchor::new();

    match engage_atomic(&gate, shield, &action, &mut anchor) {
        AtomicEpResult::EngagedAndSealed(s) => {
            // Last unit consumed, commit happened, shield sealed.
            assert_eq!(s.capacity().get(), 0);
            assert_eq!(anchor.committed_count(), 1);
        }
        _ => panic!("expected EngagedAndSealed on exact exhaustion"),
    }
}

/// Regression test for S-COMMIT audit finding.
///
/// This anchor fails every commit. Under non-atomic `engage()`, the shield's
/// capacity would be decremented before the failure was known, burning
/// capacity with no effect. `engage_atomic()` must return the shield
/// intact because commit happens BEFORE progression is materialised.
#[test]
fn engage_atomic_does_not_burn_capacity_on_commit_failure() {
    struct FailingAnchor;
    impl FirstPartialIrreversibility for FailingAnchor {
        fn commit(
            &mut self,
            _token: EngagementToken,
            _action: &PreparedAction,
        ) -> Result<(), CommitError> {
            Err(CommitError::HardwareFailure)
        }
    }

    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(2).unwrap(),
    }
    .prepare();

    let shield = Shield::<Active>::new(Capacity::new(5));
    let mut anchor = FailingAnchor;

    match engage_atomic(&gate, shield, &action, &mut anchor) {
        AtomicEpResult::CommitFailed(s, err) => {
            // CRITICAL: capacity MUST be intact. No burn without effect.
            assert_eq!(
                s.capacity().get(),
                5,
                "capacity was burned without an irreversible effect — \
                 violates I-EP-1 (Co-localization)"
            );
            assert_eq!(err, CommitError::HardwareFailure);
        }
        _ => panic!("expected CommitFailed"),
    }
}

/// Full exhaustion with atomic engages — verifies the audit-finding regression
/// from `multiple_engagements_decrement_capacity`: exact-exhaustion must
/// produce a commit for the final unit, not seal without committing.
#[test]
fn engage_atomic_full_exhaustion_commits_every_unit() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(1).unwrap(),
    }
    .prepare();

    let mut current_shield: Option<Shield<Active>> = Some(Shield::<Active>::new(Capacity::new(3)));
    let mut anchor = MockAnchor::new();

    // 3 units of capacity → 3 commits expected (canon I-EP-1).
    // Prior non-atomic engage() produced only 2 commits for 3 capacity,
    // codifying the S-COMMIT bug in the test suite.
    let shield = current_shield.take().unwrap();
    match engage_atomic(&gate, shield, &action, &mut anchor) {
        AtomicEpResult::Engaged(s) => current_shield = Some(s),
        other => panic!("unit 1: expected Engaged, got {:?}", match other {
            AtomicEpResult::Engaged(_) => "Engaged",
            AtomicEpResult::EngagedAndSealed(_) => "EngagedAndSealed",
            AtomicEpResult::Denied(_) => "Denied",
            AtomicEpResult::CommitFailed(_, _) => "CommitFailed",
        }),
    }

    let shield = current_shield.take().unwrap();
    match engage_atomic(&gate, shield, &action, &mut anchor) {
        AtomicEpResult::Engaged(s) => current_shield = Some(s),
        _ => panic!("unit 2: expected Engaged"),
    }

    let shield = current_shield.take().unwrap();
    match engage_atomic(&gate, shield, &action, &mut anchor) {
        AtomicEpResult::EngagedAndSealed(_) => {
            // Final unit: commit happens, then seal. No lost commit.
        }
        _ => panic!("unit 3: expected EngagedAndSealed"),
    }

    // All 3 units of capacity produced 3 irreversible commits.
    // This is the canonical post-fix behavior per I-EP-1.
    assert_eq!(
        anchor.committed_count(),
        3,
        "3 capacity units must produce 3 commits (was 2 under S-COMMIT bug)"
    );
}
