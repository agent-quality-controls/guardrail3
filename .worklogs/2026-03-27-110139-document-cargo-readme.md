# Document Cargo Family README

**Date:** 2026-03-27 11:01
**Scope:** `apps/guardrail3/crates/app/rs/families/cargo/README.md`

## Summary
Added the missing `RS-CARGO` family README so the four actively self-hosted Rust families now all have family-local documentation.

## Context & Problem
The user asked whether `arch`, `hexarch`, `test`, and `cargo` were all documented. The check showed that `arch`, `hexarch`, and `test` already had READMEs, but `cargo` did not. That left the self-hosted Rust family set inconsistent and made `cargo` the only one without a family-local architecture/ownership doc.

## Decisions Made

### Add a family-local `RS-CARGO` README
- **Chose:** create `apps/guardrail3/crates/app/rs/families/cargo/README.md` in the same style as the existing `arch` / `hexarch` / `test` family docs.
- **Why:** the family is already self-hosted and routed through shared `placement` + `FamilyMapper`; it should have a local contract doc that matches the live workspace shape and ownership boundary.
- **Alternatives considered:**
  - leave `cargo` undocumented and rely on the shared Rust README — rejected because the other self-hosted families already have family-local docs
  - write a full rule inventory into the family README — rejected because the immediate gap was architectural documentation, not duplicating every rule plan

## Architectural Notes
The new README documents:
- what `RS-CARGO` owns vs what belongs to `RS-ARCH`, `RS-HEXARCH`, and `RS-CODE`
- that root scope comes from shared `placement` and routing from `FamilyMapper::map_rs_cargo()`
- the current workspace shape: `crates/runtime`, `crates/assertions`, `test_support`
- the local ownership boundaries for runtime, assertions, and test support

This keeps `cargo` aligned with the same documentation pattern already used for the other self-hosted Rust families.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/cargo/Cargo.toml`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/arch/README.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md`
- `apps/guardrail3/crates/app/rs/families/test/README.md`

## Open Questions / Future Considerations
- `cargo` still needs the same deeper adversarial contract audit that `arch` / `hexarch` / `test` already received.
- If the runtime rule inventory changes, the family README should stay focused on architecture and ownership instead of becoming a second rule inventory source of truth.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/cargo/README.md` — new family-local contract doc
- `apps/guardrail3/crates/app/rs/families/cargo/Cargo.toml` — family workspace shape
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lib.rs` — routed runtime entrypoint and active rule list
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust placement and family-mapper source of truth

## Next Steps / Continuation Plan
1. Keep the repo clean by committing this doc-only checkpoint immediately.
2. If the user wants parity beyond documentation, run a dedicated `cargo` family architecture/coverage audit next.
3. When new Rust family docs are added later, keep them in the same family-local ownership-focused style rather than duplicating full rule inventories.
