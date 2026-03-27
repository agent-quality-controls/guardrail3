# Start RS-CODE Stabilization

**Date:** 2026-03-27 13:23
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/code/**`, `.plans/todo/checks/rs/code-family-stabilization.md`

## Summary
Started the `RS-CODE` family stabilization work by turning the family root into a workspace, moving the existing runtime source tree to `crates/runtime`, and adding family-local documentation plus a concrete stabilization plan. Restored the moved runtime package to a green unit-test state and re-ran `RS-TEST`, which now exposes the real remaining migration debt instead of the old single-crate shape.

## Context & Problem
After stabilizing `RS-TEST`, `RS-ARCH`, `RS-HEXARCH`, and `RS-CARGO`, the next most useful family for repo-wide leverage was `RS-CODE`. Analysis showed:

- `RS-CODE` already passed `RS-ARCH`
- `RS-CODE` family tests were green
- the family already consumed `RsCodeRoute`
- but it still lived as one single crate with no family README
- `RS-TEST` on the family showed only structural debt at that stage:
  - `RS-TEST-02`: 31
  - `RS-TEST-03`: 30

That made `RS-CODE` a good next stabilization target: semantics were already alive, but the family itself was not yet self-hosted under the same structure enforced on the earlier families.

## Decisions Made

### Start with the workspace split before semantic extraction
- **Chose:** convert the family root into a workspace first, then move the existing runtime source tree intact, and only after that inspect the new `RS-TEST` fallout.
- **Why:** this exposes the real post-split debt instead of guessing which assertions or test-support boundaries will matter after the move.
- **Alternatives considered:**
  - design the assertions split first without moving files — rejected because it would be speculative and easy to get wrong before seeing the post-split validator output
  - defer the workspace split and only write docs — rejected because the user explicitly wanted to move forward, not stop at planning

### Tell the truth in the new family README
- **Chose:** add `apps/guardrail3/crates/app/rs/families/code/README.md` as a transitional contract doc that describes the current partial migration and the target end state.
- **Why:** the family had no local contract doc, and once the split started it was important to document that the family is mid-migration rather than pretending it is already stabilized.
- **Alternatives considered:**
  - wait to write the README until the family fully passes `RS-TEST` — rejected because the contract is needed during the migration, not just after it
  - only update `.plans/todo/checks/rs/code.md` — rejected because that file is rule inventory, not family architecture

### Keep the existing runtime package identity
- **Chose:** keep `guardrail3-app-rs-family-code` as the runtime crate package name and repoint workspace dependencies to `families/code/crates/runtime`.
- **Why:** this minimizes downstream churn in the shared Rust runtime and follows the same pattern already used by `arch`, `cargo`, `hexarch`, and `test`.
- **Alternatives considered:**
  - rename the runtime package during the split — rejected because it adds needless dependency churn during an already large structural move

### Add placeholder sibling crates before extracting semantics
- **Chose:** create `crates/assertions` and `test_support` immediately, even though they are still placeholder crates.
- **Why:** the shape itself is part of the stabilization target, and `RS-TEST` only reveals the real remaining boundary debt once the family is actually in that shape.
- **Alternatives considered:**
  - postpone creating sibling crates until after moving semantic helpers — rejected because it hides the true validator fallout and delays the architecture cutover

### Restore the moved runtime test suite before proceeding
- **Chose:** fix the moved test fixture path and runtime package paths immediately so the existing `179` family tests were green again before reading more validator fallout.
- **Why:** the runtime suite is the safety net during the migration. Without restoring it, every later change would be harder to trust.
- **Alternatives considered:**
  - leave the suite red and rely on `RS-TEST` output only — rejected because the family’s own behavior would become much harder to preserve during extraction work

## Architectural Notes
This checkpoint moves `RS-CODE` into the same outer family shape as the stabilized families:

- family root workspace
- `crates/runtime`
- `crates/assertions`
- `test_support`

But only the outer shape is done. The semantic ownership split is not:

- runtime still contains `test_support.rs`
- runtime sidecars still own semantic result-shape proof
- the sibling assertions crate exists but is only a placeholder
- the sibling test-support crate exists but is only a placeholder

That distinction matters because the family is now in the right structural shell, and the validator can finally report the real remaining work:

- `RS-TEST-02`: 31
- `RS-TEST-03`: 778
- `RS-TEST-16`: 99

So the migration has crossed the “single-crate structural debt” boundary and is now in the “extract proof/test-support ownership” phase.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/Cargo.toml`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/test_support.rs`
- `apps/guardrail3/crates/app/rs/Cargo.toml`
- `apps/guardrail3/Cargo.toml`
- `.plans/todo/checks/rs/code.md`
- `.plans/todo/check_review/test_hardening/02-code.md`
- `.plans/todo/check_review/test_hardening/12-code-execution-plan.md`
- prior recent worklogs:
  - `.worklogs/2026-03-27-110139-document-cargo-readme.md`
  - `.worklogs/2026-03-27-084852-hexarch-ports-surface-rule.md`
  - `.worklogs/2026-03-26-224750-hexarch-attack-fixes.md`

## Open Questions / Future Considerations
- `RS-TEST-03` now reports `778` sidecar boundary escapes. The next pass needs to determine whether those are mostly shared helper imports, direct runtime helper imports, or sidecar-to-sidecar patterns.
- `RS-TEST-16` now reports `99` sidecar-owned semantic assertions. Those need to be migrated into the new assertions crate by rule cluster rather than file-by-file chaos.
- `test_support` needs a real extraction plan because the current runtime-local helper module still imports mapper/placement/reporting types directly.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/README.md` — current family-local contract, including transitional status
- `.plans/todo/checks/rs/code-family-stabilization.md` — current execution plan for stabilizing the family itself
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/Cargo.toml` — new runtime package manifest
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs` — active routed runtime entrypoint after the move
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/test_support.rs` — still-active runtime-local support module that must be extracted
- `apps/guardrail3/crates/app/rs/families/code/crates/assertions/Cargo.toml` — placeholder assertions crate that the next pass must make real
- `apps/guardrail3/crates/app/rs/Cargo.toml` — shared Rust runtime dependency path now pointing at `families/code/crates/runtime`
- `apps/guardrail3/Cargo.toml` — workspace membership changes for the new family crates

## Next Steps / Continuation Plan
1. Group the new `RS-TEST-03` failures by import pattern so the largest boundary-escape classes are known before editing hundreds of files blindly.
2. Extract runtime-local `test_support.rs` into `test_support/src/lib.rs` and update runtime tests to use that sibling crate.
3. Create real assertions modules in `crates/assertions/src/` for the highest-volume rule clusters, then migrate runtime sidecars off inline result-shape assertions until `RS-TEST-16` starts dropping materially.
4. Re-run:
   - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-code --lib`
   - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family test --inventory`
   - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family arch --inventory`
5. Only after `RS-CODE` passes `RS-TEST` should the next adversarial family-level audit begin.
