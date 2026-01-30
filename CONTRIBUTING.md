# Contributing to SYF-Shield

## ⚠️ Critical Boundary

This repository contains **two distinct layers** with different mutability rules:

| Layer | Path | Mutability | Contribution Rules |
|-------|------|------------|-------------------|
| **Law (Canon)** | `specs/` | 🔒 **IMMUTABLE** | **NO CHANGES ALLOWED** after Phase 4.4 sealing. Only corrections of typos/spelling that do not alter semantic meaning. |
| **Mechanism (PoM)** | `pom/` | ⚙️ **EVOLUTIVE** | Bug fixes, optimizations, test improvements welcome. Must conform to `specs/`. |
| **Documentation** | `docs/` | 📚 **EVOLUTIVE** | Clarifications, examples, integration guides welcome. Must not contradict `specs/`. |

## 🚫 Forbidden Contributions

- **ANY** modification to `specs/` that alters semantic meaning of the canon
- Introduction of temporal concepts (`TTL`, `timeout`, `expiry`) into the law layer
- Mixing implementation concerns into specification documents
- Adding governance, policy, or dynamic configuration to the core canon

## ✅ Allowed Contributions

- Bug fixes in `pom/rust/` that preserve canonical semantics
- Additional test cases proving canonical properties
- Documentation improvements in `docs/`
- Typos/spelling corrections in `specs/` (no semantic change)

## 🔒 Submission Process

1. Fork the repository
2. Make changes **only** in `pom/` or `docs/` (unless typo fix in `specs/`)
3. Submit PR with clear statement: *"This change conforms to specs/ canon"*
4. PR will be rejected if it violates canonical boundaries

## 📜 Canonical Reference

All contributions must reference the sealed canon in `specs/`:
- `SYF_SHIELD_ENGAGEMENT.md` v0.2-phase4.4-sealed
- `SYF_SHIELD_LEXICON.md` (sealed vocabulary)
- `SYF_SHIELD_INVARIANTS.md` (I-EP-1 to I-EP-6)

**Remember:** The law is immutable. Implementation adapts to the law — never the reverse.
