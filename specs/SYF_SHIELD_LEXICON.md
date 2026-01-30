# SYF_SHIELD_LEXICON.md
*Sealed Vocabulary v0.2-phase4.4-sealed*  
**Status:** Canon-SYF · Immutable · Machine World Only

---

## ✅ SEALED TERMS (Authorized Usage)

| Term | Definition | Usage Example |
|------|------------|---------------|
| **Capacity** | Finite resource in `uST`, non-regenerative | "Shield contains Capacity 5 uST" |
| **Progression** | Monotonic consumption of Capacity at EP | "Action induces Progression of 1 uST" |
| **State Transition** | Atomic mutation (occurred/not-occurred) | "Deploy is a State Transition" |
| **Exhaustion** | Thermodynamic state (Capacity = 0) | "Exhaustion triggers SEALED state" |
| **Engagement Point (EP)** | Frontier at first Partial Irreversibility | "EP coincides with buffer write" |
| **Reversible Zone (RZ)** | Zero-residue preparation space | "RZ guarantees rollback on interruption" |
| **Irreversible Zone (IZ)** | Monotonic trajectory to Effect Leaf | "IZ begins at EP crossing" |
| **Effect Leaf** | Terminal node (no descendants) | "Effect Leaf consumes token" |
| **uST** | Unit of State Transition (atomic) | "Cost expressed in uST" |
| **Domain Closure** | Complete set of instrumented transitions | "No effect outside Domain Closure" |

---

## ❌ BANNED TERMS (Explicitly Forbidden)

| Term | Reason for Ban | Canonical Substitution |
|------|----------------|------------------------|
| **TTL** | Temporal reference (time-to-live) | **RC** (Remaining Capacity) |
| **Time** | External temporal dimension | Not applicable (time-orphaned) |
| **Expiry** | Temporal instant | **Exhaustion** (thermodynamic state) |
| **Timeout** | Temporal wait | **Depletion** (capacity exhaustion) |
| **Duration** | Temporal measure | **Scope** (structural extent) |
| **Period** | Temporal recurrence | **Batch** (fixed transition set) |
| **Interval** | Temporal span | **Span** (structural, not temporal) |
| **Clock** | External time source | Forbidden (I-6: No Oracle) |
| **Lifetime** | Temporal existence | **Action Budget** (capacity-based) |
| **Reset** | Capacity regeneration | Forbidden (non-regenerative) |
| **Refund** | Capacity restoration | Forbidden (irreversible Progression) |
| **Regenerate** | Capacity renewal | Forbidden (exhaustible only) |

---

## ⚠️ AMBIGUITY LOCKS

### Capacity × Progression
- **Ambiguity:** "Capacity progression" could imply capacity growth.
- **Lock:** Progression **only** consumes capacity. Never increases it.
- **Correct:** "Progression consumes Capacity" (unidirectional).

### State Transition × Progression
- **Ambiguity:** "Transition progression" could imply partial transition.
- **Lock:** State Transition is atomic. Progression is counter increment.
- **Correct:** "Transition induces Progression" (cause → effect).

### Capacity × State Transition
- **Ambiguity:** "Capacity per transition" vs "Transition consumes capacity".
- **Lock:** Capacity is global resource. Transition has cost.
- **Correct:** "Transition cost: 1 uST" / "Remaining Capacity: 4 uST".

---

## SEALING NOTICE

This lexicon is **sealed**. No new terms may be added without Phase 5 review.  
No banned terms may appear in any canonical document or implementation commentary.

**Status:** SEALED for Phase 4.4  
**Enforcement:** All documentation and code comments must comply with this lexicon.

---
*Immutable core law. No implementation. No governance.*  
*Machine World Only.*
