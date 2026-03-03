use syf_shield_pom::*;

fn main() {
    let cap = Capacity::new(10);

    // ❌ MUST FAIL TO COMPILE: inner field is private (TI-2 Non-Regeneration)
    // An integrator must NOT be able to directly set/mutate capacity.
    let _ = cap.0;

    let ust = Ust::new(42);
    // ❌ MUST FAIL TO COMPILE: inner field is private
    let _ = ust.0;

    let cost = Cost::new(1).unwrap();
    // ❌ MUST FAIL TO COMPILE: inner field is private
    let _ = cost.0;
}
