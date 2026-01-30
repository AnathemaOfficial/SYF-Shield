use syf_shield_pom::*;

fn main() {
    let gate = MinimalGate;
    let action = InputAction {
        kind: ActionKind::Write,
        cost: Cost::new(1).unwrap(),
    }
    .prepare();

    // Construct SEALED shield via test helper
    let sealed = Shield::<Sealed>::sealed_for_test();

    // ❌ MUST FAIL TO COMPILE: engage() requires Shield<Active>, not Shield<Sealed>
    let _ = engage(&gate, sealed, &action);
}
