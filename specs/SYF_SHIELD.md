# SYF-SHIELD
*Canonical Definition v0.2-phase4.4-sealed*  
**Status:** Canon-SYF · Immutable · Machine World Only

---

## 1. FUNDAMENTAL DEFINITION

SYF-Shield is the **capacity accounting layer** of Anathema-Breaker.  
It ensures that State Transitions within the instrumented domain are:
- **Bounded** by finite, non-regenerative Capacity (in `uST`)
- **Irreversible** once the Engagement Point (EP) is crossed
- **Structurally absent** when Capacity is exhausted (not conditionally denied)

Shield **does not**:
- Enforce identity-based access control
- Evaluate the nature or legality of State Transitions (that is Gate's role)
- Regenerate capacity (capacity is exhaustible only)

Shield **does**:
- Account for State Transitions in discrete `uST` units
- Decrement capacity atomically with Progression at EP
- Enter SEALED state upon Exhaustion (capacity = 0)
- Render the path to Effect structurally absent when SEALED

---

## 2. CORE CONCEPTS

### 2.1 Capacity
Finite resource measured in **uST** (unit of State Transition).  
- Non-regenerative (exhaustible only)  
- Non-temporal (no reset, no periodic renewal)  
- Non-fractional (integer units only)  
- Thermodynamic (consumption = irreversible accounting)

### 2.2 Progression
Monotonic consumption of Capacity triggered **exclusively** by Gate `ALLOW` verdict on a valid State Transition.  
- Atomic coupling to EP crossing  
- Never reversed, never refunded  
- Never consumed without Gate `ALLOW`  
- Never consumed partially (all-or-nothing per State Transition)

### 2.3 State Transition
Discrete, atomic mutation of system state.  
- Binary: occurred / not-occurred (no partial transitions)  
- Observable within Domain Closure  
- Cost expressed in integer `uST` (≥ 1)  
- Examples: write, rotate, destroy (within instrumented domain)

### 2.4 Exhaustion
Thermodynamic state reached when Capacity = 0 **after valid Progression**.  
- Triggers SEALED state  
- Renders path to EP structurally absent  
- **Not** triggered by insufficient capacity for a requested action  
- **Not** reversible by any means within AB-Soft

---

## 3. TOPOLOGICAL ZONES

### 3.1 Reversible Zone (RZ)
Space preceding EP where preparation occurs with **zero persistent residue** upon interruption.  
- Pure mapping (no I/O, no allocation, no side effects)  
- Interruption → total rollback to Input state  
- Observable state identical before/after RZ traversal if interrupted

### 3.2 Engagement Point (EP)
Zero-thickness frontier where:  
1. RZ ends (first Partial Irreversibility begins)  
2. Gate×Shield evaluation resolves to `ALLOW`  
3. Progression consumes Capacity atomically  
4. Trajectory enters Irreversible Zone  

**Invariant:** EP coincides with the **first** Partial Irreversibility in the causal chain (I-EP-6).

### 3.3 Irreversible Zone (IZ)
Trajectory from EP to Effect Leaf where State Transition proceeds deterministically to completion.  
- No suspension, no diversion, no intermediate persistence  
- Monotonic progression toward Effect Leaf  
- No retreat possible after EP crossing

### 3.4 Effect Leaf
Terminal node of State Transition.  
- No descendants (no cascading transitions)  
- No callbacks into Domain Closure  
- Capacity already consumed at EP (not at Leaf)

---

## 4. STATE TRANSITIONS

```
[Input] 
   │
   │ (RZ: pure preparation, zero residue)
   ▼
[Engagement Point (EP)]  ════════════════════════════════╗
   │  • Gate evaluation = ALLOW                         ║
   │  • Shield capacity ≥ cost                          ║
   │  • Progression consumes capacity                   ║
   │  • First Partial Irreversibility begins            ║
   └────────────────────────────────────────────────────╣
   ▼                                                    ║
[Irreversible Zone (IZ)]                               ║
   │ (monotonic, no suspension)                         ║
   ▼                                                    ║
[Effect Leaf] ◄═════════════════════════════════════════╝
```

### 4.1 Capacity State Machine

```
Active (capacity > 0)
   │
   ├──[Gate.DENY]───────────────► Active (capacity unchanged)
   │
   ├──[cost > capacity]─────────► Active (capacity unchanged, Denied)
   │
   ├──[cost ≤ capacity]─────────► Progression
   │                               │
   │                               ├──[next > 0]──► Active (capacity = next)
   │                               │
   │                               └──[next = 0]──► Sealed (Exhaustion)
   │
Sealed (capacity = 0)
   │
   └──[any action]──────────────► Sealed (path absent — not Denied)
```

**Critical Distinction:**  
- `cost > capacity` → **Denied(Active)** (capacity unchanged, path remains open)  
- `capacity - cost = 0` → **Sealed** (Exhaustion — path structurally absent)

---

## 5. CANONICAL INVARIANTS

See `SYF_SHIELD_INVARIANTS.md` for complete invariant set (I-EP-1 to I-EP-6).

---

## 6. SEALED LEXICON

See `SYF_SHIELD_LEXICON.md` for sealed vocabulary and banned terms.

---

## 7. SEALING NOTICE

This document defines the immutable law of SYF-Shield Phase 4.4.  
It SHALL NOT be modified except for typo corrections that preserve semantic meaning.

**Status:** SEALED for Phase 4.4  
**Next Review:** Only upon discovery of structural contradiction with SYF-Core law.

---
*Canonical definition of the Systemic Fire invariant.*  
*Immutable core law. No implementation. No governance.*  
*Machine World Only.*
