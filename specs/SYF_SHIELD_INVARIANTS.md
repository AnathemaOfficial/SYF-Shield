# SYF_SHIELD_INVARIANTS.md
*Canonical Invariants v0.2-phase4.4-sealed*  
**Status:** Canon-SYF · Immutable · Machine World Only

---

## INVARIANT SET (I-EP-1 to I-EP-6)

| ID | Name | Statement | Enforcement |
|----|------|-----------|-------------|
| **I-EP-1** | Co-localization at Irreversibility Frontier | Gate evaluation, Shield verification, and Progression consumption occupy the same coordinates as the first Partial Irreversibility. | Structural (topology) |
| **I-EP-2** | Absence of Transitional States | No structure exhibiting Partial Irreversibility may exist between Input and EP. | Structural (RZ definition) |
| **I-EP-3** | Monotonic Trajectory | Once EP is crossed, State Transition proceeds to Effect Leaf without suspension or diversion. | Structural (IZ definition) |
| **I-EP-4** | Exhaustion Sealing | Upon Capacity = 0 (Exhaustion), path to EP becomes structurally unreachable (not conditionally denied). | Typestate (SEALED = no path) |
| **I-EP-5** | Leaf Constraint | Effect Leaf is terminal; no State Transition may induce secondary transitions. | Structural (no descendants) |
| **I-EP-6** | Alignment of Irreversibility Frontier | EP must coincide with the first point of Partial Irreversibility in the causal chain. | Structural (definition 1.4) |

---

## ADDITIONAL INVARIANTS (Cross-Layer)

| ID | Name | Statement | Source |
|----|------|-----------|--------|
| **I-1** | Fail-Closed | Ambiguity = DENY (no resolver) | SYF-Gate |
| **I-2** | Identity ≠ Capacity | Signature ≠ authorization | SYF-Gate |
| **I-3** | Bounded Action | Limits hard-coded or derived from invariants (R, cadence) — never configurable | SYF-Gate |
| **I-4** | Determinism | One input → one verdict (always) | SYF-Gate |
| **I-5** | No Governance | No admin, override, multisig, human control | SYF-Gate |
| **I-6** | No Oracle | No external data, clocks, probabilities | SYF-Gate |

---

## THERMODYNAMIC INVARIANTS

| ID | Name | Statement |
|----|------|-----------|
| **TI-1** | Conservation | Capacity consumed = State Transitions executed (no burn without effect) |
| **TI-2** | Non-Regeneration | Capacity never increases (exhaustible only) |
| **TI-3** | Atomicity | Progression occurs in integer units (no fractional consumption) |
| **TI-4** | Exhaustion | SEALED state triggered only by capacity reaching 0 through valid Progression |

---

## SEALING NOTICE

These invariants are **non-negotiable**. Any implementation violating even one invariant is **not** SYF-Shield.

**Status:** SEALED for Phase 4.4  
**Verification:** All PoM implementations must demonstrate invariant compliance via structural proof (not runtime checks).

---
*Immutable core law. No implementation. No governance.*  
*Machine World Only.*
