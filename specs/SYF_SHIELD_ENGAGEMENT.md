# SYF_SHIELD_ENGAGEMENT.md
*Canonical Definition v0.2-phase4.4-sealed*  
**Status:** Canon-SYF · Immutable · Machine World Only

---

## 1. DEFINITIONS

### 1.1 Domain Closure
The complete set of State Transitions subject to Capacity accounting within AB-Soft. No effect observable outside this set may be produced by an action passing through Gate×Shield.

### 1.2 Partial Irreversibility
A state modification is **partially irreversible** if it persists through:
- Loss of execution context (interruption);
- System restart without full state reset (crash + reboot);
- Power cycle (within hardware persistence limits).

Structures exhibiting partial irreversibility include (but are not limited to): cache writes, kernel buffers, disk journals, and register spills.

### 1.3 Reversible Zone (RZ)
The structural space preceding the Engagement Point where preparation occurs with **zero persistent residue** upon interruption. Any interruption within RZ results in total retreat to Input state with no observable effect in the Domain Closure.

### 1.4 Engagement Point (EP)
The singular boundary where:
1. The Reversible Zone ends (first Partial Irreversibility begins);
2. Gate×Shield evaluation resolves to ALLOW;
3. Progression consumes Capacity (atomic coupling);
4. The trajectory enters the Irreversible Zone.

**Invariant:** EP is the **only** locus where Capacity decrements. It coincides with the first physical irreversibility, not merely the logical verdict.

### 1.5 Irreversible Zone (IZ)
The trajectory from EP to Effect Leaf where the State Transition proceeds to completion deterministically. No retreat possible.

### 1.6 Effect Leaf
Terminal node of the State Transition. Exhaustion of the effect (completion) occurs here with Capacity already consumed at EP.

---

## 2. STRUCTURAL INVARIANTS

**I-EP-1: Co-localization at Irreversibility Frontier**  
Gate evaluation, Shield verification, and Progression consumption occupy the **same coordinates** as the first Partial Irreversibility (the EP boundary). No spatial or causal separation exists between "verdict ALLOW" and "Capacity decremented".

**I-EP-2: Absence of Transitional States**  
No structure exhibiting Partial Irreversibility may exist between the Input and the EP. All such structures (buffers, caches, journals) are either:
- Located **within** the Irreversible Zone (after EP), or
- **Structurally excluded** (forbidden transitional states in RZ).

**I-EP-3: Monotonic Trajectory (IZ)**  
Once the EP is crossed, the State Transition proceeds to Effect Leaf without suspension, diversion, or intermediate persistence. The path is a **directed edge** (single causal arrow).

**I-EP-4: Exhaustion Sealing**  
Upon Capacity = 0 (Exhaustion), the path to EP becomes structurally unreachable. The Domain Closure remains (system continues), but the entry into IZ is **physically absent** (no edge exists from Input to EP).

**I-EP-5: Leaf Constraint**  
The Effect Leaf is a terminal node. No State Transition may induce secondary State Transitions (no cascading Progression; no callback into Domain Closure).

**I-EP-6: Alignment of Irreversibility Frontier (NEW)**  
The EP **must** coincide with the **first** point of Partial Irreversibility in the causal chain. Progression is coupled structurally to this frontier. No Partial Irreversibility may precede Progression consumption.

---

## 3. PROOF OF IMPOSSIBILITY

**Theorem:** The state "Engaged but not Progressed" is structurally impossible within AB-Soft's instrumented domain.

**Proof by Structural Construction:**

1. **Definition of Engagement (Revised):** By Definition 1.4, Engagement **is** the crossing of the Partial Irreversibility frontier coupled with atomic Capacity consumption (Progression). The EP is a **boundary**, not a node with duration.

2. **Hypothesis of Separation:** Assume a state exists where Partial Irreversibility has occurred (system state persists after interruption), but Capacity remains undecremented.
   - By I-EP-6, Partial Irreversibility **requires** Progression coupling.
   - Therefore, occurrence of Partial Irreversibility without Progression violates the structural alignment of the EP.

3. **Topology Contradiction:**
   - If Partial Irreversibility exists before EP, the EP is misaligned (not at first irreversibility).
   - If Progression occurs after Partial Irreversibility, a Transitional State exists (irreversible effect without accounting), violating I-EP-2.

4. **Co-localization Enforcement:**
   - Gate×Shield evaluation occurs **at** the EP boundary.
   - If evaluation = DENY, the trajectory never crosses into Partial Irreversibility (remains in RZ).
   - If evaluation = ALLOW, Progression consumes Capacity **simultaneously** with the irreversible step (IZ entry).

**Conclusion:** There is no structural location where Partial Irreversibility (Engagement) exists without accompanying Progression. The two are the **same boundary event** observed from causal (Engagement) and thermodynamic (Progression) perspectives.

**Q.E.D.**

---

## 4. FORBIDDEN STATES

The following configurations are structurally impossible and must not appear:

| Forbidden State | Description | Violation Sealed |
|-----------------|-------------|------------------|
| **Staged Action** | Partial Irreversibility (buffer, cache) preceding EP. | I-EP-6 (EP must be first irreversibility) |
| **Deferred Progression** | Capacity decremented after Partial Irreversibility observed. | I-EP-1 (Co-localization) |
| **Callback Cascade** | Effect Leaf triggering new evaluation within Domain Closure. | I-EP-5 (Leaf Constraint) |
| **Conditional Sealing** | Exhaustion verified by "check" rather than structural path absence. | I-EP-4 (SEALED = path absence, not verdict) |
| **RZ Residue** | Observable state surviving interruption within Reversible Zone. | I-EP-2 (RZ = zero residue) |

---

## 5. MINIMAL TOPOLOGY (V0.2)

```
[Input] 
   │
   │ (Reversible Zone: zero residue upon interruption)
   ▼
[Engagement Point (EP)]  ════════════════════════════════╗
   │                                                    ║
   │ Gate×Shield evaluation                             ║
   │ Progression consumption (Capacity → Capacity-1)   ║
   │ First Partial Irreversibility                      ║
   │────────────────────────────────────────────────────╣
   ▼                                                    ║
[Irreversible Zone (IZ)]                               ║
   │ (No exit, no suspension, monotonic)                ║
   ▼                                                    ║
[Effect Leaf] ◄═════════════════════════════════════════╝
   │
   │ (Capacity already consumed at EP)
   ▼
[Structural Termination]
```

**Topological Properties:**
- **Depth:** 3 boundaries (Input→EP→Effect→Termination).
- **Atomic Boundary:** EP is a **zero-thickness frontier** (not a node with duration).
- **No Edges Around EP:** If Progression fails (Capacity=0), no edge connects Input to IZ (Fail-Closed = structural absence of path).

---

## 6. SEALING NOTICE

This document v0.2 corrects the misalignment identified in v0.1:
- **EP relocated** from logical evaluation to **physical irreversibility frontier** (CE-01…CE-05 sealed).
- **Progression coupled** to first Partial Irreversibility (CE-03, CE-05 sealed).
- **Reversible Zone** formally defined as zero-residue space (CE-01 sealed).
- **Leaf Constraint** structurally prevents reentry/callbacks (CE-02, CE-04 sealed).

**Status:** SEALED for Phase 4.4  
**Next Review:** Upon physical manifestation of Partial Irreversibility preceding Progression (violation of I-EP-6).

---
*Canonical definition of the Systemic Fire invariant.*  
*Immutable core law. No implementation. No governance.*  
*Machine World Only.*
