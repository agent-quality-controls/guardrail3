# Stabilize RS-CLIPPY Root Policy

**Date:** 2026-03-28 12:33
**Scope:** `apps/guardrail3/clippy.toml`, `apps/guardrail3/crates/app/rs/families/clippy/**`, `apps/guardrail3/tests/adversarial_config_tests.rs`, `apps/guardrail3/tests/fixtures/adversarial-configs/incomplete-clippy/*`

## Summary
Brought live `RS-CLIPPY` on `apps/guardrail3` to zero by aligning the app-root `clippy.toml` with the stricter canonical baseline, removing the now-forbidden family-local `clippy.toml`, and converting the old adversarial fixture into an inert asset that gets materialized only inside temp copies during tests. The checkpoint also captures the in-flight clippy family runtime/assertions hardening that now passes its library test suite.

## Context & Problem
After the app-root workspace-boundary fix and the `RS-CARGO` policy checkpoint, live `RS-CLIPPY` still reported only root-policy drift plus two placement violations:

- `apps/guardrail3/clippy.toml` was a stale generated file that predated the stricter clippy family baseline
- `apps/guardrail3/crates/app/rs/families/clippy/clippy.toml` was still a live family-local policy root, which `RS-CLIPPY-12` now correctly forbids inside the single app workspace
- `apps/guardrail3/tests/fixtures/adversarial-configs/incomplete-clippy/clippy.toml` was also a literal live `clippy.toml` inside the repo tree, so live validation treated an adversarial test fixture as an active policy root

The user’s sweep goal is repo-root zero, not “make the rule stop complaining.” So the fix needed to remove the violating shapes and update the real app policy root, not weaken placement or coverage semantics.

## Decisions Made

### Promote the stricter generated baseline to the app-root `clippy.toml`
- **Chose:** Replace the stale root `apps/guardrail3/clippy.toml` content with the richer managed baseline already reflected in the clippy family’s current runtime/generator work.
- **Why:** The live errors were exactly “root file drifted behind the managed rule set” (`max-fn-params-bools`, `excessive-nesting-threshold`, test-relaxation booleans, macro bans, extra garde-owned method/type bans, extractor bans, `std::process::abort`, `std::any::Any`, `avoid-breaking-exported-api = false`).
- **Alternatives considered:**
  - Relax the runtime rules back down to match the old root file — rejected because the stricter baseline is the intended hardening direction and already has runtime/assertion coverage.
  - Patch only the specific missing keys and leave the rest of the stale comments/layout alone — rejected because the file is generated policy and should match the canonical rendered shape, not accumulate manual drift.

### Remove the family-local `clippy.toml` instead of exempting it
- **Chose:** Delete `apps/guardrail3/crates/app/rs/families/clippy/clippy.toml`.
- **Why:** Under the now-restored single-workspace app boundary, the family root is neither the app validation root, nor a workspace root, nor a standalone non-member package root. Keeping the file would make `RS-CLIPPY-12` dishonest.
- **Alternatives considered:**
  - Special-case family roots in `RS-CLIPPY-12` — rejected because it would reopen hidden nested policy roots just after `hexarch` closed the same topology hole for workspaces.
  - Keep the file under another live Clippy filename — rejected because the rule owns both `clippy.toml` and `.clippy.toml` and the problem is the live policy root, not only the exact filename.

### Materialize adversarial fixture configs in temp copies instead of storing live policy files in the repo tree
- **Chose:** Rename `tests/fixtures/adversarial-configs/incomplete-clippy/clippy.toml` to `clippy.fixture.toml` and teach `tests/adversarial_config_tests.rs` to copy the fixture into a temp directory, then rename hidden fixture policy files back to their live names before running validation.
- **Why:** The fixture still needs to exercise “broken `clippy.toml`” behavior, but it should not poison live repo validation. Moving the file behind a hidden fixture name preserves the adversarial scenario while keeping the repo tree itself honest.
- **Alternatives considered:**
  - Exclude `tests/fixtures` from `RS-CLIPPY` discovery — rejected because it would teach the rule to ignore real in-repo policy files based on path convention rather than policy-root semantics.
  - Delete the fixture coverage entirely — rejected because the incomplete-policy regression is still useful and now easier to keep isolated.

## Architectural Notes
This checkpoint keeps the single-workspace app model coherent:

- app-root `clippy.toml` is the real live policy root for `apps/guardrail3`
- family containers are no longer allowed to carry their own live Clippy roots inside that workspace
- adversarial config fixtures stop being live repo policy roots and become materialized test assets instead

One nuance remains: validating `apps/guardrail3/crates/app/rs/families/clippy` directly now reports uncovered package roots under `RS-CLIPPY-01`, because there is no local `clippy.toml` anymore and the family subtree cannot see the app-root ancestor policy file. I did not reintroduce a forbidden nested policy root just to keep family-local self-validation green. That tension belongs to the broader “self-hosting vs single app policy root” design discussion, not this repo-root-zero checkpoint.

## Information Sources
- `.plans/todo/checks/rs/clippy.md`
- `apps/guardrail3/clippy.toml`
- `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement.rs`
- live validation:
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family clippy --inventory --format json`
  - `apps/guardrail3/target/debug/guardrail3 rs validate <temp materialized incomplete-clippy fixture> --family clippy --inventory --format json`
- verification commands:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
  - `cargo build --manifest-path apps/guardrail3/Cargo.toml -p guardrail3`

## Open Questions / Future Considerations
- Direct family-root `RS-CLIPPY` self-validation is now red because the family subtree no longer has a local policy root. If family-by-family self-validation remains a hard requirement under the one-workspace model, the clippy coverage semantics need a principled answer that does not reintroduce forbidden nested live policy files.
- The same “inert fixture materialization” pattern will likely be needed for other families that still keep live config files under `tests/fixtures`.
- `RS-GARDE` is the next immediate family and already shows boundary-derive / field-validator debt that now sits on top of the corrected clippy baseline.

## Key Files for Context
- `apps/guardrail3/clippy.toml` — live app-root clippy policy root after baseline promotion
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — placement/coverage logic that made the family-local file illegal under the app root
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement.rs` — explicit forbidden-location rule
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — updated family shape and current-state notes
- `apps/guardrail3/tests/adversarial_config_tests.rs` — temp-copy fixture materialization logic for hidden policy files
- `apps/guardrail3/tests/fixtures/adversarial-configs/incomplete-clippy/clippy.fixture.toml` — inert adversarial Clippy fixture asset
- `.worklogs/2026-03-28-110213-finish-hexarch-workspace-boundary.md` — prior checkpoint that made nested family policy/workspace roots no longer acceptable
- `.worklogs/2026-03-28-122223-stabilize-rs-cargo-root-policy.md` — prior checkpoint that restored the cargo-side workspace policy surface clippy depends on

## Next Steps / Continuation Plan
1. Stage and commit the full clippy-family checkpoint: app-root `clippy.toml`, family runtime/assertions/test-support changes already in the worktree, README update, and inert fixture materialization changes.
2. Move directly to `RS-GARDE`, starting from the current live errors in `project-tree`, CLI boundary enums/vectors, and `domain/config/types.rs`.
3. Reuse the same pattern for `garde` that just worked for clippy: live app-root validation first, family tests second, adversarial temp-repo check third, then commit before touching the next family.
