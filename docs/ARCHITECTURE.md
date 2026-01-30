# SYF-Shield Architecture

## Core Principle

SYF-Shield enforces **physical impossibility** (not logical denial) within the instrumented domain by making the path to effect **structurally absent** when capacity is exhausted.

## Topological Zones

### Reversible Zone (RZ)
- Pure preparation space before EP
- Zero residue guarantee: interruption → total rollback
- Implemented via:
  - `panic=abort` (no unwinding)
  - Copy-only types (no Drop)
  - No I/O, no allocation, no side effects

### Engagement Point (EP)
- Zero-thickness frontier (not a node with duration)
- Coincides with **first Partial Irreversibility**
- Atomic coupling of:
  1. Gate evaluation (`ALLOW`)
  2. Shield verification (capacity ≥ cost)
  3. Progression (capacity decrement)
  4. Irreversibility onset (buffer write, etc.)

### Irreversible Zone (IZ)
- Monotonic trajectory to Effect Leaf
- No suspension, no diversion, no callbacks
- Token consumed immediately at IZ entry

## Typestate Enforcement

```
Shield<Active>  ──[progress]──▶  Shield<Active>  (capacity > 0)
                     │
                     └─[exhaustion]──▶  Shield<Sealed>  (capacity = 0)

Shield<Sealed>  ──X──  (no path to EP — structurally absent)
```

**Key Insight:** `Shield<Sealed>` has **no methods** to reach EP. This is not a conditional `if capacity == 0: deny` — it is a structural absence of the path itself.

## Token Linearity

```
EngagementToken ──[consumed by value]──▶ enter_irreversible_zone()
                                          │
                                          X──▶ Cannot be reused (compile-time)
```

- Non-Copy, non-Clone by construction (`UnsafeCell`)
- Consumed exactly once at IZ entry
- Guarantees single-use transition (no duplication → no N effects for 1 progression)

## Thermodynamic Accounting

| Scenario | Capacity Before | Action Cost | Capacity After | State |
|----------|-----------------|-------------|----------------|-------|
| Valid progression | 5 | 2 | 3 | Active |
| Exhaustion | 2 | 2 | 0 | Sealed |
| Insufficient capacity | 1 | 5 | 1 | Active (Denied) |
| Gate denial | 5 | 2 (Destroy) | 5 | Active (Denied) |

**Critical:** Insufficient capacity → `Denied(Active)` (capacity unchanged).  
Only valid progression to zero → `Sealed` (Exhaustion).

## Integration Points

1. **Gate Layer:** Provides `ALLOW`/`DENY` based on SYF-Core invariants (never knows capacity)
2. **Shield Layer:** Accounts capacity, enforces path absence when exhausted
3. **Anchor Layer:** Defines first Partial Irreversibility (syscall boundary in production)
4. **Effect Layer:** Consumes token at Effect Leaf (terminal node)

## Non-Goals

- Universal material impossibility (DMA, hardware rollback)
- Temporal constraints (clocks, TTL)
- Governance or dynamic policy
- Performance optimization

These belong to Anathema-Hard (future phase).
