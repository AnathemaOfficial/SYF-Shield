# API Surface Audit — Control-Surface Leak Standard

**Version:** 1.0
**Applies to:** SYF-Shield, SYF-Gate, Anathema-Breaker (and any future PoM/ref)
**Origin:** Finding from Anathema-Breaker audit — "API renders the law pilotable"

---

## Principle

> If the law says X is impossible, but the API allows representing X,
> the law is _correct_ but _unverifiable at the integration boundary_.

The core risk: internal logic is sound, but public types/constructors/fields
allow an integrator to bypass invariants by operating at the type-surface level.

---

## Checklist (10 points)

### 1. Newtype Inner Fields
- [ ] All newtypes with domain semantics have **private inner fields**
- [ ] `Foo(pub u32)` → `Foo(u32)` + `Foo::new()` + `Foo::get()`
- [ ] No `value.0 = X` possible from outside the crate

### 2. Domain ID Width
- [ ] All domain IDs are `u64` (or minimum `u32`, never `u16`)
- [ ] No truncation (`as u16`, `as u32`) on domain IDs
- [ ] Single canonical mapping documented

### 3. Budget/Capacity = Read-Only from Outside
- [ ] Budget, Capacity, Progression types have private fields
- [ ] Only getters (`get()`, `is_exhausted()`) are public
- [ ] Mutation only via internal law methods (`progress()`, `consume()`)
- [ ] No `set_*`, `with_*`, `from_unchecked` on budget/capacity types

### 4. Constructor Discipline
- [ ] Production constructors return `Self` (not `Option`/`Result` unless needed)
- [ ] Test-only constructors are gated: `#[cfg(any(test, feature = "test-support"))]`
- [ ] No `pub fn new_unchecked()` or equivalent
- [ ] Shield/Gate/Engine cannot be "reset" by creating a new instance mid-flow

### 5. Feedback Surface = Mute
- [ ] Reason codes are a **closed enum** (no free-form strings)
- [ ] No detailed error messages that enable adaptive loops
- [ ] External-facing output: only `ALLOW/DENY` (+ opaque code for audit)
- [ ] `reason_code` + `limits` are for **logging/audit only**, never for retry logic

### 6. Signal/Context Source = Sealed
- [ ] Signal must come from a trusted provider, not the caller
- [ ] `context_min` must be computed by the membrane, not user-provided
- [ ] Document explicitly: "SignalProvider = TCB" in integration docs

### 7. Conversions = Safe
- [ ] Zero `unwrap()` on type conversions in library code
- [ ] Zero `as u16`, `as u32` truncations
- [ ] `try_into()` with proper error handling if needed
- [ ] Panic-free by construction (length checks before copy)

### 8. Configuration = None
- [ ] Zero environment variables controlling behavior
- [ ] Zero feature flags that change semantics (only `test-support` for test helpers)
- [ ] Zero config files or parsers
- [ ] All bounds are hard-coded constants

### 9. Parsing = Strict
- [ ] `deny_unknown_fields` (or equivalent structural check) on all inputs
- [ ] No fallback/default for missing fields
- [ ] Malformed input → immediate DENY (fail-closed)

### 10. Compile-Fail Proof
- [ ] At least one `trybuild` test proving the key invariant cannot be violated
- [ ] Example: "capacity inner field is private" → compile error on `cap.0`
- [ ] Example: "sealed shield cannot engage" → compile error on `engage(sealed, ...)`

---

## Audit Results

### SYF-Shield (Phase 4.4 PoM) — 2026-03-03
- **All 10 points: PASS**
- Hardened: `Ust(u32)`, `Cost(NonZeroU32)`, `Capacity(Ust)` — inners now private
- Compile-fail test added: `capacity_not_writable.rs`
- Existing compile-fail: `sealed_cannot_engage.rs`
- No domain IDs (Shield uses capacity/cost only) — N/A for point 2
- Zero unwrap in lib, zero env vars, zero config

### SYF-Gate (ref/ sketch) — 2026-03-03
- **All 10 points: PASS**
- Hardened: `CanonicalInput`, `Signal`, `Limits`, `FinalityTag`, `GateOutput` — all fields now private
- Constructors: `::new()` + const getters on all types
- Reason codes: closed enum (7 variants), mute responses
- All bounds hard-coded (I-3), zero config, zero env vars
- `RawInput` fields remain pub (intentional: non-canon deserialization boundary)

---

## When to Run This Audit

1. Before sealing any new phase
2. After any change to public types in PoM/ref
3. Before sending code to external auditor
4. When adding new types that represent domain concepts (budget, capacity, domain, signal)

---

## Duplicate PoM Rule

SYF-Shield has two PoM directories that must remain **bit-identical**:
- `SYF-Shield/pom/rust/` (canonical, inside repo)
- `PoM/rust/` (working copy, outside repo)

Any hardening or bug fix applied to one **must** be applied to both.
Divergence = audit invalidation. Future CI should enforce `diff -r` between them.

---

## Canon Alignment

This checklist enforces at the API surface what the canon enforces at the semantic level:
- **TI-2 (Non-Regeneration)** → Capacity inner private, no setter
- **I-3 (Bounded Action)** → Hard-coded bounds, no config
- **I-8 (Finality)** → No retry loop enabled by feedback
- **I-EP-4 (Exhaustion Sealing)** → Shield<Sealed> has no methods (structural)
