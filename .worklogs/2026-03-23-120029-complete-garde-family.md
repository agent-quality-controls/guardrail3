# Complete Garde Family

**Date:** 2026-03-23 12:00
**Scope:** `.plans/todo/checks/rs/garde.md`, `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/garde/*`

## Summary
Implemented the new `rs/garde` check family under the current checker architecture and corrected the `garde` plan so it no longer overclaims the old validator state. The new family replaces line-based cargo parsing and fail-open source scanning with typed facts, explicit input-failure reporting, and one rule file plus one rule-specific test file per rule.

## Context & Problem
The old `garde_checks.rs` implementation was only a partial baseline. It used line-based Cargo dependency detection, relied on a small stale ban list, skipped unreadable or unparsable source files silently, and only approximated derive-based validation coverage. The plan also overstated what already existed by marking `RS-GARDE-01..05` as implemented while `06..09` were still design-only and the old code had known enum and parser gaps.

The user explicitly wanted breadth-first family completion without shortcuts, but still with the full architecture shape:
- one rule per production file
- one rule-specific test module per rule
- no grouped concern files
- no fail-open behavior hidden in the orchestrator

That meant `rs/garde` had to be built as a first real implementation rather than a direct migration.

## Decisions Made

### Freeze the plan to the real baseline first
- **Chose:** Update `.plans/todo/checks/rs/garde.md` before implementing code.
- **Why:** The old plan mixed “exists today”, “partially exists”, and “desired future behavior” into one false status story. Writing code against that would guarantee drift.
- **Alternatives considered:**
  - Implement directly from the stale plan — rejected because it would encode false assumptions from the old validator.
  - Treat old `garde_checks.rs` as source of truth — rejected because the old code is line-based, incomplete, and explicitly known-broken in some areas.

### Add `RS-GARDE-10` for input failures
- **Chose:** Introduce `RS-GARDE-10` as a dedicated garde-family input-failure rule.
- **Why:** The old validator failed open on unreadable and unparsable source files. The new architecture should surface those as explicit errors rather than silently skipping files.
- **Alternatives considered:**
  - Keep silent skips — rejected because it weakens the guardrails and hides broken coverage.
  - Fold input failures into `RS-GARDE-01` — rejected because dependency presence and family input integrity are separate concerns.

### Model garde checks per enabled Rust root
- **Chose:** Build root facts for workspace roots and standalone package roots, then apply garde checks only where garde is enabled by policy.
- **Why:** This matches the current root/policy model already used by `clippy` and `deny` without coupling garde to hex architecture.
- **Alternatives considered:**
  - Repo-root-only garde checking — rejected because nested Rust roots would be mis-scoped.
  - Per-crate checking for every workspace member — rejected because the user’s policy roots are workspace roots and standalone packages, not arbitrary member crates.

### Keep clippy baseline lookup local but policy-root aware
- **Chose:** Resolve the nearest covering `clippy.toml` / `.clippy.toml` at allowed roots and use that for `RS-GARDE-02/03/04/06`.
- **Why:** Garde bans live in clippy config, so the family has to read the same policy-root structure rather than assuming a single root file.
- **Alternatives considered:**
  - Hardcode repo-root `clippy.toml` — rejected because it breaks nested workspace/package roots.
  - Import `clippy` family facts directly — rejected for now because it would create family coupling before shared root/config abstractions are stabilized.

### Rebuild source analysis with explicit AST facts
- **Chose:** Add a dedicated `garde/parse.rs` that collects:
  - struct derive boundary targets
  - enum derive boundary targets
  - manual `Deserialize` impls
  - manual `Validate` impls
  - `query_as!` macro invocations
- **Why:** The old derive visitor was too coarse for enum handling and had no support for manual impl or macro bypass cases.
- **Alternatives considered:**
  - Reuse old `ast_helpers::find_derive_attributes()` directly — rejected because it still carries the enum false-positive model.
  - Use grep or string matching for manual impls/macros — rejected because source semantics must stay parser-based.

### Split struct, enum, manual-impl, and macro concerns cleanly
- **Chose:** Keep:
  - `RS-GARDE-05` for struct derive validation
  - `RS-GARDE-07` for manual `Deserialize` impl bypass
  - `RS-GARDE-08` for enum derive validation
  - `RS-GARDE-09` for `query_as!` inventory
- **Why:** Each rule now has one narrow local assertion instead of one large “input validation audit” blob.
- **Alternatives considered:**
  - Bundle all source boundary checks into one rule — rejected because it violates the one-rule/one-concern design and makes testing ambiguous.

## Architectural Notes
`rs/garde` follows the same family pattern as the completed Rust families:
- `mod.rs` orchestrates root discovery, source parsing, and fan-out
- `facts.rs` owns policy-root resolution, clippy config coverage lookup, and source fact collection
- `inputs.rs` exposes minimal typed rule inputs
- `parse.rs` owns syn-based source extraction for garde-specific semantics
- each `RS-GARDE-*` rule lives in its own file with its own test file

Notable design choices:
- deeper rules (`02/03/04/06/05/07/08/09`) only run for roots where `garde` is actually present
- source files are assigned to the nearest active garde root
- test paths are excluded from derive validation checks
- parse/read/config failures surface via `RS-GARDE-10`
- C-like enums are explicitly treated as safe, avoiding the old “all enums are suspect” false-positive behavior

## Information Sources
- `.plans/todo/checks/rs/garde.md` — active rule inventory and migration notes
- `apps/guardrail3/crates/app/rs/validate/garde_checks.rs` — old validator behavior and limitations
- `apps/guardrail3/tests/unit/test_garde_checks.rs` — old adversarial seed cases for `01..05`
- `apps/guardrail3/crates/app/rs/validate/ast_visitors.rs` and `ast_helpers.rs` — old derive/skip behavior and known enum limitation
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs` — policy-root/clippy coverage pattern
- `apps/guardrail3/crates/domain/modules/clippy/*` — canonical clippy baseline context for garde-related bans
- Prior worklogs:
  - `.worklogs/2026-03-23-104643-complete-deps-family.md`
  - `.worklogs/2026-03-23-110636-tighten-deps-lockfile-scope.md`

## Open Questions / Future Considerations
- `rs/garde` currently resolves manual `Deserialize` impl targets by simple type-name matching within a parsed file result. That is acceptable for the breadth-first pass, but fully qualified type-resolution across modules may be needed in the deeper hardening pass.
- The family locally re-implements allowed clippy config lookup rather than consuming a shared root/config service. If several later families need the same lookup, that should be extracted.
- The canonical clippy generator currently contains only the older smaller garde-related ban subset. `rs/garde` now enforces the broader plan-level set. That generator/checker alignment still needs a dedicated hardening pass.

## Key Files for Context
- `AGENTS.md` — active project instructions and checker architecture rules
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — current checker architecture contract
- `.plans/todo/checks/rs/garde.md` — active garde rule inventory and implementation target
- `apps/guardrail3/crates/app/rs/checks/rs/garde/mod.rs` — garde family orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/garde/facts.rs` — garde root discovery, policy resolution, clippy lookup, and source fact collection
- `apps/guardrail3/crates/app/rs/checks/rs/garde/parse.rs` — garde-specific AST extraction
- `apps/guardrail3/crates/app/rs/validate/garde_checks.rs` — old baseline for comparison only
- `apps/guardrail3/tests/unit/test_garde_checks.rs` — old adversarial seed tests
- `.worklogs/2026-03-23-110636-tighten-deps-lockfile-scope.md` — previous family checkpoint immediately before garde

## Next Steps / Continuation Plan
1. Send an adversarial audit over `rs/garde` against `.plans/todo/checks/rs/garde.md`, with special attention to:
   - rule coverage vs plan
   - enum/manual-impl/query-as bypasses
   - whether any rule is still indirectly depending on old clippy baseline assumptions
2. If the audit finds medium/high gaps, fix `rs/garde` before starting the next family.
3. Then move to `rs/test` using the same breadth-first standard:
   - one rule file per rule
   - one rule-specific test module per rule
   - real orchestrator/facts/input design, not placeholders.
