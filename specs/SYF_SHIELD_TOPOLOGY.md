# SYF_SHIELD_TOPOLOGY.md
*Structural Diagrams v0.2-phase4.4-sealed*  
**Status:** Canon-SYF · Immutable · Machine World Only

---

## 1. ZONE TOPOLOGY (RZ → EP → IZ)

```
┌─────────────────────────────────────────────────────────────────────┐
│                        DOMAIN CLOSURE                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────┐     ┌──────────────┐     ┌─────────────────────┐  │
│  │ Reversible  │     │  Engagement  │     │  Irreversible       │  │
│  │    Zone     │────▶│     Point    │────▶│       Zone          │  │
│  │    (RZ)     │     │     (EP)     │     │       (IZ)          │  │
│  └─────────────┘     └──────────────┘     └─────────────────────┘  │
│        │                    │                        │             │
│        │ Pure preparation   │ Atomic coupling:       │ Monotonic   │
│        │ (zero residue)     │ • Gate.ALLOW           │ trajectory  │
│        │                    │ • Shield verification  │ to Effect   │
│        │                    │ • Progression          │ Leaf        │
│        │                    │ • First irreversibility│             │
│        ▼                    ▼                        ▼             │
│  Interruption ──────► Total rollback          No retreat possible  │
│  (crash/reboot)      (state identical to                          │
│                       before RZ entry)                             │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. CAPACITY STATE MACHINE

```
                    ┌───────────────────┐
                    │   Shield<Active>  │
                    │   capacity = N>0  │
                    └─────────┬─────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
  Gate = DENY        cost > capacity        cost ≤ capacity
        │                     │                     │
        │                     │                     ▼
        │                     │              Progression
        │                     │                     │
        │                     │        ┌────────────┼────────────┐
        │                     │        │            │            │
        │                     │        ▼            ▼            │
        │                     │   next > 0      next = 0        │
        │                     │        │            │            │
        │                     │        │            │            │
        │                     │        ▼            ▼            │
        │                     │  ┌──────────┐  ┌──────────┐      │
        └─────────────────────┴─▶│ Active   │  │ Sealed   │◄─────┘
                                 │ capacity │  │ capacity │
                                 │ = next   │  │ = 0      │
                                 └──────────┘  └──────────┘
```

**Critical Path Semantics:**
- `cost > capacity` → **Denied(Active)** (capacity unchanged, path remains open)
- `cost ≤ capacity` → **Progression** → `next = 0` → **Sealed** (Exhaustion)

---

## 3. PATH ABSENCE (SEALED STATE)

```
Shield<Active> ────────────────┐
                               │
                               ▼
                          ┌─────────┐
                          │  EP     │───▶ IZ ──▶ Effect Leaf
                          └─────────┘

Shield<Sealed> ────────────X  (NO PATH)
                           │
                           │  Structural absence:
                           │  • No method to reach EP
                           │  • Not "DENY" verdict
                           │  • Not conditional check
                           │  • Path literally does not exist
                           ▼
                      [System Continues]
                      (but no State Transitions possible)
```

---

## 4. TOKEN LINEARITY (Single-Use Guarantee)

```
EngagementToken
       │
       │ Created at EP crossing (Gate.ALLOW + Progression)
       ▼
┌──────────────┐
│ Linear Token │───▶ Cannot be copied (non-Copy)
└──────────────┘     Cannot be cloned (non-Clone)
       │             Must be consumed by value
       ▼
   enter_irreversible_zone(token, ...)
       │
       │ Token consumed HERE (dropped)
       ▼
   [Token ceases to exist]
       │
       X───▶ No second use possible (compile-time enforced)
```

---

## 5. THERMODYNAMIC ACCOUNTING

```
Initial State:     Capacity = 5 uST
                   │
Action 1 (cost=2): │ Progression consumes 2 uST
                   ▼
                   Capacity = 3 uST  (Active)
                   │
Action 2 (cost=3): │ Progression consumes 3 uST
                   ▼
                   Capacity = 0 uST  (Exhaustion → Sealed)
                   │
Action 3 (any):    X  Path absent — not DENIED, not checked
                   │  (structurally impossible to reach EP)
                   ▼
              [No effect possible]
```

**Non-Example (Forbidden):**
```
Action (cost=10) on Capacity=5:
   ✗ NOT Sealed (no burn without effect)
   ✓ Denied(Active) — capacity remains 5 uST
```

---

## SEALING NOTICE

These diagrams represent the **structural truth** of SYF-Shield Phase 4.4.  
They SHALL NOT be altered to accommodate implementation limitations.

**Status:** SEALED for Phase 4.4  
**Reference:** All implementations must conform to these topologies.

---
*Immutable core law. No implementation. No governance.*  
*Machine World Only.*
