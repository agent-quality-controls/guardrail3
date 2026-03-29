# Close RS-CODE Proof Gaps

**Date:** 2026-03-29 23:36
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_03_item_level_allow_without_reason_tests/bypasses.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason_tests/{mod.rs,bypasses.rs}`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_17_impl_allow_blast_radius_tests/direct.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_20_extern_allow_tests/{mod.rs,bypasses.rs}`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_23_include_bypass_tests/{mod.rs,bypasses.rs}`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr_tests/{mod.rs,bypasses.rs}`

## Summary
Closed the remaining `RS-CODE` proof gaps left after the shared parser fixes by making the shared `#[expect(...)]` ownership explicit in the test surface and by adding dedicated later-rule bypass sidecars for `20`, `23`, and `24`. The code family still passes its library tests, and the family root remains clean under `arch`, `test`, and `code` validation.

## Context & Problem
The previous `RS-CODE` fixes had already made the family green and removed the substantive correctness bugs described in `FIXES.md`, but the last adversarial pass still pointed out two proof-shape weaknesses:

1. explicit `#[expect(...)]` regressions were only pinned in the `RS-CODE-20` extern lane, even though the shared collectors now treat `expect` as owned lint-policy surface for other rules too
2. later-numbered rules still leaned on generic `direct.rs` and `inventory.rs` test files instead of having dedicated bypass-focused sidecars like the earlier rules

Those were not new check bugs, but they were real evidence gaps. Leaving them open would have made the family look “green” while still proving less than the written contract.

## Decisions Made

### Add explicit `expect` proofs where shared ownership already exists
- **Chose:** Add `#[expect(...)]` coverage to `RS-CODE-03`, `RS-CODE-04`, and `RS-CODE-17`.
- **Why:** The implementation already owns `expect` through shared collectors; the tests needed to prove that consistently so future parser changes cannot silently regress it.
- **Alternatives considered:**
  - Rely on the existing `RS-CODE-20` extern-block `expect` proof only — rejected because that does not prove item-level or impl-level ownership.
  - Leave the behavior implicit because the implementation branches on `attr_name()` — rejected because the entire purpose of the adversarial follow-up was to lock that behavior in.

### Add bypass-focused sidecars for later exploit-heavy rules
- **Chose:** Introduce dedicated `bypasses.rs` sidecars for `RS-CODE-20`, `RS-CODE-23`, and `RS-CODE-24`.
- **Why:** Those rules are exploit-shaped by nature and were still proving their adversarial cases only via broad `direct.rs` coverage. A dedicated bypass lane makes the intent obvious and keeps parity with the earlier rules.
- **Alternatives considered:**
  - Rename existing `direct.rs` files wholesale — rejected because it would create large churn for little value.
  - Add bypass sidecars for every rule `20+` immediately — rejected because the adversarial note was about uneven proof shape, not a requirement to rename the whole later-rule suite in one commit.

### Keep this slice inside the code family only
- **Chose:** Commit only the `families/code` proof follow-up and ignore the unrelated dirty `clippy` / `arch` / lockfile work in the tree.
- **Why:** The user asked specifically for the code-family fixes from `FIXES.md`, and other agents are actively working in adjacent lanes.
- **Alternatives considered:**
  - Bundle other dirty files while committing — rejected because that would make the checkpoint incoherent and risk conflicts with in-flight work.

## Architectural Notes
- `RS-CODE-03` and `RS-CODE-04` now explicitly prove that item-level `expect` is owned by the same shared lint-policy parsing lane as item-level `allow`.
- `RS-CODE-17` now proves impl-level `expect` blast-radius handling, not just impl-level `allow`.
- `RS-CODE-20`, `RS-CODE-23`, and `RS-CODE-24` now each have a dedicated bypass file for one focused exploit class:
  - `cfg_attr(..., expect(...))` on extern blocks
  - nested `OUT_DIR` concat traversal in build-script include patterns
  - path attributes with forged reason spellings and non-parent `..` substrings
- Family-level self-hosting remains unchanged:
  - `arch` clean
  - `test` clean
  - `code` clean

## Information Sources
- Audit backlog and follow-up scope:
  - `apps/guardrail3/crates/app/rs/families/code/FIXES.md`
- Prior RS-CODE checkpoints:
  - `.worklogs/2026-03-29-225525-harden-rs-code-shared-parsers.md`
  - `.worklogs/2026-03-29-232925-finish-rs-code-fixes-tail.md`
- Live implementation and tests:
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_03_item_level_allow_without_reason.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_17_impl_allow_blast_radius.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_20_extern_allow.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_23_include_bypass.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr.rs`
- Verification:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-code --lib`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family arch --inventory --format json`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family test --inventory --format json`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family code --inventory --format json`

## Open Questions / Future Considerations
- The code family’s correctness backlog from `FIXES.md` is now closed, but `EXPANSION.md` still contains optional policy ideas that were intentionally not pulled into this work.
- Repo-root `RS-CODE` debt outside the family still exists and should be treated as normal source cleanup, not code-family correctness work.
- Untracked local docs (`FIXES.md`, `EXPANSION.md`) remain intentionally outside commits unless the user explicitly wants them versioned.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/FIXES.md` — the audit backlog that defined this lane
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_03_item_level_allow_without_reason_tests/bypasses.rs` — explicit item-level `expect` ownership proof
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason_tests/bypasses.rs` — explicit documented item-level `expect` inventory proof
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_17_impl_allow_blast_radius_tests/direct.rs` — explicit impl-level `expect` blast-radius proof
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_20_extern_allow_tests/bypasses.rs` — later-rule bypass proof for cfg-driven `expect` on extern blocks
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_23_include_bypass_tests/bypasses.rs` — later-rule bypass proof for nested `OUT_DIR` traversal
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr_tests/bypasses.rs` — later-rule bypass proof for forged reasons and non-parent `..` substrings
- `.worklogs/2026-03-29-232925-finish-rs-code-fixes-tail.md` — the previous checkpoint that left only these proof gaps open

## Next Steps / Continuation Plan
1. Treat `RS-CODE` family correctness as finished unless a new adversarial pass finds a concrete check bug.
2. If the user wants broader policy growth, work from `apps/guardrail3/crates/app/rs/families/code/EXPANSION.md` separately from the correctness lane.
3. Keep future code-family commits narrow and avoid mixing them with the active `clippy` and `arch` worktree changes.
