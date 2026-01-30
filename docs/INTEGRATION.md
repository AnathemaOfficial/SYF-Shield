# Integrating SYF-Shield into Anathema-Breaker

## Overview

SYF-Shield provides the **capacity accounting layer** for Anathema-Breaker. It must be integrated with SYF-Gate to form the complete safety module.

## Integration Topology

```
[Input Action]
       │
       ▼
┌──────────────┐
│  SYF-Gate    │───▶ ALLOW/DENY (invariant check only)
└──────────────┘
       │
       ▼ (if ALLOW)
┌──────────────┐
│ SYF-Shield   │───▶ Progression + EP crossing
└──────────────┘
       │
       ▼ (token)
┌──────────────┐
│  Anchor      │───▶ First Partial Irreversibility
│  (syscall)   │     (e.g., write to kernel buffer)
└──────────────┘
       │
       ▼
┌──────────────┐
│ Effect Leaf  │───▶ Terminal state mutation
└──────────────┘
```

## Step-by-Step Integration

### 1. Instantiate Gate and Shield

```rust
use syf_shield_pom::*;

let gate = MinimalGate;  // Or your SYF-Gate implementation
let shield = Shield::<Active>::new(Capacity::new(100)); // 100 uST initial capacity
```

### 2. Prepare Action (RZ)

```rust
let input = InputAction {
    kind: ActionKind::Write,
    cost: Cost::new(5).unwrap(),
};
let prepared = input.prepare(); // Pure preparation (RZ)
```

### 3. Engage EP

```rust
match engage(&gate, shield, &prepared) {
    EpResult::Engaged { shield: next_shield, token } => {
        // EP crossed successfully — token granted for IZ entry
        // next_shield has capacity decremented by 5 uST
    }
    EpResult::Denied(active_shield) => {
        // Gate denied OR insufficient capacity
        // active_shield capacity unchanged
    }
    EpResult::Sealed(sealed_shield) => {
        // Exhaustion — path structurally absent
        // No further State Transitions possible
    }
}
```

### 4. Anchor to Physical Irreversibility (Production)

In production, replace `NullAnchor` with a real anchor that performs the first partially irreversible operation:

```rust
// Example: syscall-based anchor (pseudo-code)
pub struct SyscallAnchor {
    fd: i32, // File descriptor for instrumented buffer
}

impl FirstPartialIrreversibility for SyscallAnchor {
    fn commit(&mut self, _token: EngagementToken, action: &PreparedAction) -> Result<(), CommitError> {
        // This syscall write is the FIRST partially irreversible operation
        // (persists after crash/reboot within hardware limits)
        unsafe {
            libc::write(self.fd, action.payload_ptr(), action.payload_len());
        }
        Ok(())
    }
}
```

### 5. Apply Effect (Leaf)

```rust
let effect = NullEffect;
effect.apply(prepared, token, &mut anchor)?; // Token consumed here
```

## Critical Integration Rules

1. **Never separate token consumption from irreversibility onset**  
   The token MUST be consumed atomically with the first partially irreversible operation.

2. **Never implement callbacks from Effect Leaf**  
   Violates I-EP-5 (Leaf Constraint) — no descendants allowed.

3. **Never regenerate capacity**  
   Violates thermodynamic invariant TI-2 — capacity is exhaustible only.

4. **Never check capacity before Gate evaluation**  
   Gate must evaluate purely on invariants (I-6: No Oracle).

5. **Anchor must be minimal**  
   Only the first partially irreversible operation — no preparatory steps.

## Testing Your Integration

1. **Compile-fail test:** Verify `Shield<Sealed>` cannot reach EP
2. **Token linearity test:** Verify token cannot be duplicated/reused
3. **Exhaustion test:** Verify capacity=0 → path absent (not DENY)
4. **Insufficiency test:** Verify `cost > capacity` → `Denied(Active)` (capacity unchanged)
5. **RZ interruption test:** Verify interruption in RZ leaves zero residue

## Production Considerations

- **Anchor selection:** Must correspond to actual first Partial Irreversibility in your substrate
- **Buffer boundaries:** Kernel buffers, journal writes, cache coherency — all must be analyzed for partial irreversibility
- **Crash simulation:** Test interruption at every point between RZ and IZ to verify zero residue / proper sealing
- **Side channels:** Audit all paths capable of producing effects — must be instrumented or explicitly excluded from Domain Closure

## Exclusions (Anathema-Hard)

The following are **explicitly out of scope** for AB-Soft:

- DMA paths outside instrumented domain
- Hardware rollback / snapshot resurrection
- Microcode-level universal gating
- Universal material impossibility

These require Anathema-Hard (future phase) with hardware co-design.
