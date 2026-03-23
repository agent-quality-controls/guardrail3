# Plan Libarch Family

**Date:** 2026-03-23 21:21
**Scope:** `.plans/todo/checks/rs/libarch.md`

## Summary
This change created the first real `rs/libarch` family plan and aligned it with the same structural checker architecture used by the other Rust families. The plan now defines when flat libraries are allowed, when they must escalate to a multi-crate workspace, and what folder/rule/test layout the eventual implementation must use.

## Context & Problem
The project already had strong architecture stories for apps (`hexarch`) and content sites, but libraries were still the main place where code could accumulate without any hard architectural shape. We had already frozen several quality rules in `rs/code` and `rs/deps`, but those only create pressure; they do not define what a library should become once it outgrows a flat crate. The user wanted that gap closed and also wanted the new family plan to explicitly carry the same “one rule per file, one sidecar test module per rule” implementation contract used elsewhere.

## Decisions Made

### Define a two-mode library model
- **Chose:** Keep two valid library modes:
  - flat single-crate library while under complexity thresholds
  - layered multi-crate workspace once any threshold is crossed
- **Why:** Requiring all libraries to be workspaces would add too much ceremony for small crates, but doing nothing leaves libraries as junk drawers.
- **Alternatives considered:**
  - Always require layered workspaces — rejected as overkill for genuinely small libraries.
  - Keep only code-quality pressure rules with no library architecture — rejected because it would not actually force a structural split.

### Use early escalation thresholds
- **Chose:** Escalate to layered library architecture when a flat library exceeds any of:
  - direct deps `> 12`
  - module depth `> 3`
  - sibling subdirs `> 4`
  - sibling `.rs` files `> 6`
- **Why:** These thresholds are intentionally lower than the generic anti-sprawl caps so libraries are forced to split before they become extreme garbage.
- **Alternatives considered:**
  - Reuse the very high global anti-sprawl caps — rejected because those are too late for library architectural intervention.

### Keep the layered library shape minimal
- **Chose:** The required layered structure is:
  - `crates/api`
  - `crates/core`
  - optional `crates/infra`
- **Why:** This is the smallest crate-based architecture that still gives enforceable separation between public surface, pure logic, and integration glue.
- **Alternatives considered:**
  - More layers — rejected as too flexible and too easy to weaken.
  - Pure module-level layering — rejected because hard dependency-direction enforcement is not robust enough without crate boundaries.

### Carry the standard checker-family structure into the plan itself
- **Chose:** Add an explicit “Target family shape” section requiring:
  - one production file per rule
  - one sidecar test module directory per rule
  - `mod.rs` / `facts.rs` / `inputs.rs`
- **Why:** The user explicitly asked that `libarch` be planned with the same concrete implementation structure as `hexarch` and the newer families.
- **Alternatives considered:**
  - Rely on the global architecture doc alone — rejected because the family plan should be self-describing and implementation-ready.

## Architectural Notes
`rs/libarch` is deliberately the library analogue of `rs/hexarch`, but simpler:
- it is crate/workspace-based, not AST-heavy
- it enforces workspace shape and dependency direction
- it relies on `rs/code` and `rs/deps` for the lower-level quality pressure signals

The family is intended to reuse patterns already proven in:
- `rs/hexarch` for workspace membership and dependency edges
- `rs/code` / `rs/deps` for complexity measurements that act as escalation triggers

## Information Sources
- `.plans/todo/checks/rs/hexarch.md` — reference for crate/workspace architecture enforcement style
- `.plans/todo/checks/rs/code.md` — source-level pressure rules and structural cap direction
- `.plans/todo/checks/rs/deps.md` — direct dependency cap and dependency-policy ownership
- session discussion clarifying:
  - no per-rule bypasses
  - libraries need a real architecture family
  - flat libraries should only be allowed while still small

## Open Questions / Future Considerations
- The plan currently assumes `api -> infra` is forbidden. That is intentionally strict, but it is still an implementation-time policy checkpoint.
- The root package may act as a published facade crate or just a workspace root; the plan currently allows either as long as the public surface exports from `api`.
- No rule IDs have been implemented yet; this is a planning-only addition.

## Key Files for Context
- `.plans/todo/checks/rs/libarch.md` — newly created library architecture family plan
- `.plans/todo/checks/rs/hexarch.md` — architectural enforcement reference model
- `.plans/todo/checks/rs/code.md` — source-level library pressure and quality rules
- `.plans/todo/checks/rs/deps.md` — dependency pressure and direct-count cap

## Next Steps / Continuation Plan
1. Review the `rs/libarch` rule set for any policy ambiguities before implementation starts.
2. When implementing, build the family with the standard structure:
   - `mod.rs`
   - `facts.rs`
   - `inputs.rs`
   - one rule file per `RS-LIBARCH-*`
   - one `*_tests/` directory per rule
3. Reuse `rs/hexarch` dependency-edge and workspace-membership patterns rather than inventing new ad hoc enforcement logic.
