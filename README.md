# SYF-Shield — Phase 4.4 Sealed

**Status:** ✅ CANON IMMUTABLE · Phase 4.4 · Machine World Only  
**Repository:** https://github.com/AnathemaOfficial/SYF-Shield

> *Capacity accounting layer for Anathema-Breaker.  
> Physical impossibility within instrumented domain via structural path absence.*

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
