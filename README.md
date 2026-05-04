<img width="256" height="256" alt="syf shield" src="https://github.com/user-attachments/assets/9c7b60af-be45-4e54-9548-4fe1f61f4af7" />

# SYF-Shield — Phase 4.4 Sealed

**Status:** ✅ CANON IMMUTABLE · Phase 4.4 · Machine World Only  
**Repository:** https://github.com/AnathemaOfficial/SYF-Shield

> *Capacity accounting layer for Anathema-Breaker.  
> Physical impossibility within instrumented domain via structural path absence.*

---

## Public Engine Layer

SYF-Shield is part of the **public canonical engine layer** of the CoreXalt
ecosystem:

```text
SYF-Core -> SYF-Gate -> SYF-Shield -> Anathema-Breaker -> SLIME-Core
```

This layer remains open so the law, lineage, and audit surface are inspectable.
Applied systems built from these engines, including SAFA, SLIME-Enterprise, and
SLIME-APP, are controlled product editions. Enterprise or government deployments
are expected to be sealed, signed, SBOM-backed, and distributed separately from
the public engine layer.

---

## 🔐 Canonical Boundary

| Layer | Location | Mutability | Purpose |
|-------|----------|------------|---------|
| **Law** | `specs/` | 🔒 SEALED | Immutable specification (v0.2) |
| **Mechanism** | `pom/` | ⚙️ EVOLUTIVE | Proof-of-Mechanism (Rust `no_std`) |
| **Integration** | `docs/` | 📚 DOCUMENTATION | Usage guidance (non-binding) |

**Critical Principle:**  
The law (`specs/`) is **never** modified by implementation concerns.  
Implementation (`pom/`) must conform to the law — never the reverse.

---

## 📜 Core Canon (Phase 4.4)

- **Capacity** = finite resource in `uST` (non-regenerative, non-temporal)
- **Progression** = monotonic consumption triggered **only** by Gate `ALLOW`
- **Engagement Point (EP)** = zero-thickness frontier at **first Partial Irreversibility**
- **Reversible Zone (RZ)** = zero residue on interruption (pure preparation)
- **Irreversible Zone (IZ)** = monotonic trajectory to Effect Leaf (no suspension)
- **SEALED** = structural path absence (not conditional denial)

**Sealed Lexicon:**  
✅ `Capacity` | `Progression` | `State Transition` | `Exhaustion` | `EP` | `RZ` | `IZ`  
❌ `TTL` | `Time` | `Expiry` | `Timeout` | `Duration` | `Clock`

---

## 🧭 Position in Lineage

SYF-Shield is an **intermediate primitive**, complementary to Gate — not a
terminal core and not a membrane.

Canonical chronology of the ecosystem:

1. **SYF-Core** — upstream thermodynamic theory (`R = (F × E) / K`)
2. **SYF-Gate** — structural admissibility primitive
3. **SYF-Shield** — capacity, progression, and irreversibility primitive (this repo)
4. **Anathema-Breaker** — sealed synthesis of **Gate + Shield** into a resolution core
5. **SLIME-Core** — canonical execution membrane built from that core

### Complementarity with Gate

- **Gate** determines whether progression **may begin**.
- **Shield** governs how capacity is **consumed** once engagement has begun.
- Shield's progression is triggered **only** by a Gate `ALLOW`.
- Their sealed synthesis becomes **Anathema-Breaker**.

Gate and Shield are not competing deny layers. Gate owns admissibility at the
boundary; Shield owns capacity, progression, and irreversibility inside the
admitted trajectory.

### Scope clarification

Shield enforces **structural impossibility of a traversable execution path
within a bounded instrumented domain**. It does not claim universal material
impossibility — DMA paths, hardware rollback, and similar concerns belong to
the future Anathema-Hard tier, not AB-Soft.

---

## ⚙️ Proof-of-Mechanism (PoM)

Rust `no_std` implementation demonstrating structural impossibility:

```bash
cd pom/rust
cargo test               # Runtime tests and checked-in compile-fail suite
cargo test --test compile_fail
```

**Key Properties Verified:**
- ✅ Token linear (non-Copy/non-Clone) → single-use transition
- ✅ `Shield<Sealed>` has no path to EP (compile-time proof)
- ✅ Insufficiency ≠ Exhaustion (`Denied(Active)` vs `Sealed`)
- ✅ RZ = zero residue (`panic=abort` + Copy types)

---

## 🚫 Non-Goals (Explicitly Excluded)

- Universal material impossibility (DMA paths, hardware rollback)
- Temporal constraints (clocks, timestamps, block height)
- Governance or dynamic policy
- Optimization or performance concerns

*These belong to Anathema-Hard (future phase), not AB-Soft.*

---

## 📜 License

Apache License 2.0 — with explicit clause:  
**"The canonical specifications in `specs/` are immutable law.  
Implementation may evolve, but must never alter the sealed canon."**
