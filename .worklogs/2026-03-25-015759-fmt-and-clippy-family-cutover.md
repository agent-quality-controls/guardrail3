# Cut Over Fmt And Clippy Family Crates

**Date:** 2026-03-25 01:57
**Scope:** `apps/guardrail3/crates/app/rs/families/fmt/src/*`, `apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/clippy/src/*`

## Summary
Promoted `fmt` and `clippy` from shim crates into real self-owned family crates under `families/*/src`. Both crates now compile and run their own family tests directly, which gives another pair of fast standalone Rust test loops without going through the root `guardrail3` test harness.

## Context & Problem
After the runtime/toolchain cutover, the split was still only partially aligned with the plan. `fmt` and `clippy` existed as workspace members, but they still depended on `#[path = ...]` inclusion of the old `checks/rs/*` tree. That preserved the same ownership lie the split is supposed to remove: Cargo saw crates, but the crates did not actually own their code.

The plan’s family order starts with `fmt`, then `toolchain`, then `clippy`. `toolchain` was already cut over in the previous batch, so this pass synchronized the implementation to that order by finishing `fmt` and then promoting `clippy`.

## Decisions Made

### Promote `fmt` exactly like `toolchain`
- **Chose:** copy the full `checks/rs/fmt` family sources into `families/fmt/src`, replace the shim `lib.rs` with a real entrypoint, and delete the copied `mod.rs` wrapper file.
- **Why:** `fmt` has the same small dependency profile as `toolchain` and is the next intended family in the plan order.
- **Alternatives considered:**
  - Skip `fmt` and jump to a larger family — rejected because it would drift away from the planned sequence.
  - Leave `fmt` as a path shim until hook-shell is extracted — rejected because `fmt` does not depend on that substrate.

### Promote `clippy` next, but cut it cleaner than the old shim
- **Chose:** copy the full `checks/rs/clippy` family tree into `families/clippy/src`, rewrite imports to direct crate owners, and remove the fake `domain` / `app` / `hook_shell` wrapper from the family crate root.
- **Why:** `clippy` is next in the plan order, and unlike several later families it does not actually need the shared hook-shell parser. That made it possible to remove more legacy compatibility than the original wrapper exposed.
- **Alternatives considered:**
  - Extract hook-shell first — rejected for this batch because `clippy` does not need it, so that would delay the straightforward planned-order family cut.
  - Keep the fake `hook_shell` and nested `app` tree for consistency with other family shims — rejected because it would preserve an unnecessary monolith edge in a crate that no longer needs it.

### Fix family-owned tests rather than weakening the split
- **Chose:** update `clippy` test support to find fixtures relative to the new crate location and update two garde-disabled tests to derive expected ban messages from the current canonical baseline helpers.
- **Why:** once the crate owns its own tests, those tests must validate the real current family behavior and the new crate layout, not the old root package assumptions.
- **Alternatives considered:**
  - Hard-code the new expected ban lists again — rejected because the lists already drifted once and would drift again.
  - Keep fixture paths rooted at the old package layout — rejected because that breaks standalone crate tests and hides ownership problems.

## Architectural Notes
`fmt` and `clippy` now follow the same pattern as `toolchain`: the family crate owns discovery/facts/inputs/rules/tests under its own `src/` tree, and the runtime depends on the crate instead of a facade wrapper.

`clippy` is an important specimen because it proves a medium-sized family can be promoted without preserving fake nested crate trees. The family crate now depends directly on `guardrail3_app_core`, `guardrail3_domain_modules`, `guardrail3_domain_project_tree`, and `guardrail3_domain_report`, while test-only FS access lives in dev dependencies instead of polluting the library dependency surface.

This materially improves test topology: `cargo test -p guardrail3-app-rs-family-fmt --lib` and `cargo test -p guardrail3-app-rs-family-clippy --lib` are now real standalone loops rather than root-facade illusions.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — family-crate end state and the split-phase rationale.
- `.plans/todo/check_review/test_hardening/30-workspace-split-phase1-agent-brief.md` — earlier phase constraints and the requirement that crate boundaries become real, not facade-preserved.
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/*` — original family sources promoted into the `fmt` crate.
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/*` — original family sources promoted into the `clippy` crate.
- `.worklogs/2026-03-25-011916-runtime-shim-and-toolchain-cutover.md` — previous batch that removed the runtime shim and promoted `toolchain`.

## Open Questions / Future Considerations
- Several later family crates still path-include `checks/hooks/shell.rs`; that shared substrate should be extracted before or during the next set of promotions.
- The root test harness at `apps/guardrail3/tests/unit.rs` is still the main source of monolithic rebuild pressure.
- `fmt`, `toolchain`, and `clippy` are now real family crates, but many other families still preserve fake nested `domain/app` trees and `#[path = ...]` wrappers.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/fmt/src/lib.rs` — real `fmt` family entrypoint after cutover.
- `apps/guardrail3/crates/app/rs/families/clippy/src/lib.rs` — real `clippy` family entrypoint after cutover.
- `apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml` — cleaned library vs dev dependency boundary for the standalone family crate.
- `apps/guardrail3/crates/app/rs/families/clippy/src/test_support.rs` — fixture-rooting and family-local test harness for the new crate location.
- `apps/guardrail3/crates/app/rs/families/toolchain/src/lib.rs` — prior specimen used as the model for these promotions.
- `.worklogs/2026-03-25-011916-runtime-shim-and-toolchain-cutover.md` — prior split context.

## Next Steps / Continuation Plan
1. Promote the next planned family crate, starting with `deny` or `cargo`, depending on which has the smaller remaining dependency surface after inspecting the hook-shell fanout.
2. Extract the shared hook-shell parser out of `checks/hooks/shell.rs` into a real shared owner so the remaining family crates can stop path-including that old tree.
3. Keep converting root-facade tests into family-owned crate tests so the fast per-family loops continue replacing the monolithic root harness rather than just supplementing it.
