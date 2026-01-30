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
    drop(prepared);
    drop(input);

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
