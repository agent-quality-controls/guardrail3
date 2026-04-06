# Stabilize RS-GARDE Boundary Rules

**Date:** 2026-03-28 12:39
**Scope:** `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/{facts.rs,parse.rs,rs_garde_ast_05_field_level_constraints.rs,rs_garde_ast_05_field_level_constraints_tests/false_positives.rs,rs_garde_ast_06_nested_validation_dive.rs,rs_garde_ast_06_nested_validation_dive_tests/false_positives.rs}`, `apps/guardrail3/crates/domain/project-tree/Cargo.toml`, `apps/guardrail3/crates/domain/project-tree/src/lib.rs`

## Summary
Brought live `RS-GARDE` on `apps/guardrail3` to zero by fixing two checker false positives in the field-validation / nested-dive rules and making `ProjectTree`/`DirEntry` satisfy the derive inventory rule. The family test suite now covers skipped fields and primitive-only nested validated structs explicitly, and the live app-root garde run is clean again.

## Context & Problem
After the `RS-CLIPPY` checkpoint, live `RS-GARDE` still showed 11 errors concentrated in two areas:

- `RS-GARDE-AST-01` reported `ProjectTree` and `DirEntry` because they derive `Deserialize` and have non-primitive fields but did not derive `Validate`
- `RS-GARDE-AST-05` reported a set of fields that already had explicit garde handling:
  - `#[garde(skip)]` fields such as CLI subcommands and config maps
  - `#[garde(dive)]` fields pointing at nested validated config structs

The second bucket contradicted the plan. `RS-GARDE-AST-05` is supposed to require meaningful field-level validators only when a field actually needs runtime validation and is not already handled by `skip` or nested validation via `dive`.

## Decisions Made

### Fix the garde checker semantics instead of decorating code with bogus validators
- **Chose:** Teach the garde parser/runtime to treat `#[garde(skip)]` as explicit field handling, and make `RS-GARDE-AST-05` skip fields that already use `#[garde(dive)]`.
- **Why:** The failing fields were not missing validation policy; the checker was misclassifying them. Adding fake validators just to satisfy the checker would weaken the guardrail by teaching developers to cargo-cult meaningless annotations.
- **Alternatives considered:**
  - Add synthetic `length`/`custom` validators to CLI/config fields that are already skipped or dived — rejected because it would encode nonsense semantics and hide the rule bug.
  - Relax `RS-GARDE-AST-05` broadly by making fewer field types require validation — rejected because the core rule is still valuable for real unvalidated boundary fields.

### Extend the same skip semantics to nested-dive enforcement
- **Chose:** Add explicit `has_garde_skip` tracking and make `RS-GARDE-AST-06` ignore skipped nested fields.
- **Why:** Once `ProjectTree` started deriving `Validate`, its skipped `structure` field was being misread as a nested validated field that needed `#[garde(dive)]`. That is the same false-positive class: skipped fields should not be forced into recursive validation.
- **Alternatives considered:**
  - Add `#[garde(dive)]` to skipped internal fields — rejected because `skip` and `dive` are contradictory intents.
  - Stop deriving `Validate` on `ProjectTree` entirely — rejected because `RS-GARDE-AST-01` is explicitly inventorying exactly this derive contract.

### Satisfy the derive inventory rule for `ProjectTree` honestly, with skipped internal fields
- **Chose:** Add `garde` as a dependency to `guardrail3-domain-project-tree`, derive `Validate` on `ProjectTree` and `DirEntry`, and annotate their internal walker-owned fields with `#[garde(skip)]`.
- **Why:** These types are serialized/deserialized project snapshots and they do trip the derive-inventory pattern. Skipping their internal fields is appropriate because they are constructed by the walker, not validated as direct external request boundaries.
- **Alternatives considered:**
  - Special-case `ProjectTree` in `RS-GARDE-AST-01` — rejected because the rule should stay syntax-driven and general.
  - Add ad hoc custom validators to `PathBuf`/`Vec<String>` walker fields — rejected because the types are not external user input and would again be bogus validation just to silence the checker.

## Architectural Notes
This checkpoint sharpens the garde family in the intended direction:

- `RS-GARDE-AST-05` now distinguishes between “missing validator” and “field deliberately handled by garde already”
- `RS-GARDE-AST-06` no longer conflicts with `#[garde(skip)]`
- `ProjectTree` remains checker-visible as a deserializable boundary type without pretending that its internal snapshot fields are external validated inputs

The important principle is preserved: real boundary fields still need meaningful validators, but explicit garde escape hatches (`skip`, `dive`) are first-class semantics, not false positives.

## Information Sources
- `.plans/todo/checks/rs/garde.md`
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/rs_garde_ast_05_field_level_constraints.rs`
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/rs_garde_ast_06_nested_validation_dive.rs`
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/parse.rs`
- live validation:
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family garde --inventory --format json`
- verification commands:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-garde --lib`
  - `cargo build --manifest-path apps/guardrail3/Cargo.toml -p guardrail3`

## Open Questions / Future Considerations
- The remaining large families (`test`, `code`, `release`) still likely contain adversarial fixtures or old structural assumptions that will surface once their turn comes. The “explicit garde semantics should not count as missing validation” lesson may recur in other checker families.
- `ProjectTree` now derives `Validate` mainly to satisfy the current inventory model. If the long-term product view is that walker-owned snapshots are not boundary types at all, that should be expressed in the garde plan/rules explicitly rather than via per-type exceptions.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/parse.rs` — garde attribute summarization and boundary-field extraction
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/facts.rs` — boundary-field facts and nested-validation resolution
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/rs_garde_ast_05_field_level_constraints.rs` — field-level validator rule
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/rs_garde_ast_06_nested_validation_dive.rs` — nested `#[garde(dive)]` rule
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/rs_garde_ast_05_field_level_constraints_tests/false_positives.rs` — new skip/dive regression tests
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/rs_garde_ast_06_nested_validation_dive_tests/false_positives.rs` — skip regression for nested validated fields
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs` — deserializable project snapshot types now deriving `Validate` with explicit skipped internal fields
- `.worklogs/2026-03-28-123327-stabilize-rs-clippy-root-policy.md` — prior checkpoint that restored the clippy baseline garde depends on

## Next Steps / Continuation Plan
1. Stage and commit the garde-family/runtime changes together with the `project-tree` derive update and this worklog.
2. Move to `RS-TEST` next; it is the largest remaining structural family and the current counts suggest most of the repo-wide debt is there.
3. Use the same pattern on `RS-TEST`: live app-root baseline first, then concentrated specimen migrations, then adversarial attacks before commit.
