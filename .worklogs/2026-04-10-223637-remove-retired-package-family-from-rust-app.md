Summary
- Removed the retired Rust package family from live app Rust code, plans, and docs.
- Updated old app bridges and naming so the touched app families still compile against the extracted packages.

Decisions made
- Replaced dead topology package ownership references with `arch`, because the retired package family no longer exists as a live Rust app family.
- Kept fixes in old app code only after the package boundary clarification. Extracted package implementations were not changed.
- Rewired old app `garde` and `deps` bridges to current extracted package APIs instead of preserving stale references.

Key files for context
- .plans/2026-04-10-222657-remove-retired-package-family-everywhere.md
- apps/guardrail3/crates/app/rs/placement/src/classification.rs
- apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/facts.rs
- apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/run.rs
- apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/run.rs

Next steps
- Finish retiring stale references outside Rust app scope if they still matter elsewhere in the repo.
- Keep old app bridges aligned with renamed extracted packages until the old app path is removed.
