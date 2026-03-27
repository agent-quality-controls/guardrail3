# RS Code Parse Shape Cleanup

**Date:** 2026-03-27 14:28
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/*`

## Summary
Split the large `parse.rs` facade into sibling parse submodules so the file drops well below the `RS-CODE-09` length threshold and no longer carries self-inventory `allow` attributes. The change preserves parse behavior while moving the helper implementation into smaller modules.

## Context & Problem
`rs/code` was still tripping self-hosting checks on the parse surface:
- `parse.rs` was too long for `RS-CODE-09`
- `parse.rs` carried `#[allow(...)]` attributes that triggered `RS-CODE-04`

The user asked for a parse-shape cleanup only, with no rule behavior change, and specifically wanted the work limited to `parse.rs` plus any sibling parse modules needed to make the split clean.

## Decisions Made

### Split the parse implementation into sibling submodules
- **Chose:** keep `parse.rs` as a thin facade and move implementation into `parse/attrs.rs`, `parse/comments.rs`, `parse/core.rs`, `parse/helpers.rs`, `parse/types.rs`, and `parse/visitors.rs`.
- **Why:** this shrinks the facade below the length threshold without changing the parse API or rule logic.
- **Alternatives considered:**
  - leave the file long and only remove `allow` attributes - rejected because `RS-CODE-09` would still fire
  - move logic into unrelated modules - rejected because the user asked to keep the change localized to parse-shape self-hosting

### Remove self-inventory `allow` attributes from the parse surface
- **Chose:** eliminate all `#[allow(...)]` attributes from `parse.rs` and the new sibling parse submodules.
- **Why:** `RS-CODE-04` was flagging the parse surface’s own inventory allowances, which should not live there if we want the family to self-host cleanly.
- **Alternatives considered:**
  - leave the allowances in place and accept the rule hits - rejected because the goal was specifically to stop tripping them
  - move the allowances to module-level scope - rejected because it would still be self-inventory debt

### Preserve behavior, not line-for-line structure
- **Chose:** keep the same parse responsibilities and helper semantics while reorganizing the code.
- **Why:** the user explicitly asked not to change rule behavior.
- **Alternatives considered:**
  - rewrite the parser algorithms more aggressively - rejected because that would increase the risk of behavioral drift

## Architectural Notes
The new shape is:
- `parse.rs` = public facade and type aliases
- `parse/core.rs` = parse entrypoints and top-level counts
- `parse/comments.rs` = comment-aware line counting and line text helpers
- `parse/helpers.rs` = shared AST helpers
- `parse/attrs.rs` = attribute-based finders
- `parse/visitors.rs` = visitor-based finders
- `parse/types.rs` = small parse-specific data types

That keeps the parse surface self-hosted without making the facade itself the place where policy helpers and lint suppressions accumulate.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/*`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-code --lib`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family code --inventory --format json`

## Open Questions / Future Considerations
- The broader `rs/code` family still has unrelated test failures elsewhere in the crate, but they were outside this parse-only cleanup.
- If `parse` grows again, the next split should preserve the same “thin facade + sibling modules” pattern instead of reintroducing a large central file.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse.rs` - thin facade after the split
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs` - comment-aware helpers
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs` - shared AST helpers
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/attrs.rs` - attribute-based detection
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs` - visitor-based detection
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/types.rs` - parse-local data types

## Next Steps / Continuation Plan
1. Keep `parse.rs` as a façade only; add any new parse logic to sibling submodules instead of the top-level file.
2. If future code-family work needs more parse helpers, prefer a new sibling module rather than re-expanding `parse.rs`.
3. Re-run `RS-CODE` inventory after any future parse changes to ensure no new self-inventory `allow` attributes appear in the parse subtree.
