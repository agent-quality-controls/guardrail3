# Tighten RS Test Validator Checkpoint

**Date:** 2026-03-26 19:45
**Scope:** `apps/guardrail3/crates/app/rs/families/test/**`, `apps/guardrail3/crates/app/rs/families/arch/**`, `apps/guardrail3/crates/app/rs/families/cargo/**`, `apps/guardrail3/crates/app/rs/families/hexarch/**`, `.plans/todo/checks/rs/test.md`, `apps/guardrail3/Cargo.lock`

## Summary
Tightened the `RS-TEST` validator so it catches boundary violations that were previously slipping through, then carried the fallout into the self-hosted `test`, `arch`, and `cargo` families until they were green again. This checkpoint intentionally includes unfinished `hexarch` fallout: the validator is stricter and correct there now, but the family still needs a broad assertions-layer rewrite and currently has both `RS-TEST-16` failures and unit-test regressions.

## Context & Problem
The earlier `RS-TEST` pass was too easy to satisfy structurally while still allowing semantic leakage:
- package-name/import matching used Cargo names directly, so hyphenated crates could bypass local-boundary checks
- proof-bearing assertions were tracked too loosely
- sidecars could still own semantic result checks in ways the README contract intended to ban
- `test_support` genericity checks were too weak

After hardening the validator, real hidden violations appeared in the family crates themselves. `arch` and `cargo` were tractable. `hexarch` turned out to be much larger and was already leaning on runtime-side sidecar assertions heavily, so the stricter validator exposed a much deeper migration surface there.

## Decisions Made

### Normalize Rust crate identifiers from Cargo manifests
- **Chose:** Normalize package/dependency names from `Cargo.toml` into Rust import identifiers by replacing `-` with `_` during test-family discovery.
- **Why:** `RS-TEST-03` compares Rust import paths, not Cargo package names. Without normalization, local-crate direct imports could bypass the boundary rule.
- **Alternatives considered:**
  - Compare raw package names only — rejected because Rust import syntax does not use hyphens.
  - Add one-off special cases in `RS-TEST-03` — rejected because the mismatch is a discovery/input issue, not a rule-local issue.

### Tighten semantic ownership checks instead of only structural ownership
- **Chose:** Extend parser facts and strengthen `RS-TEST-16` / `RS-TEST-18` to notice direct result-shape assertions and semantic helper leakage.
- **Why:** The previous rules allowed thin assertions facades and generic-helper crates that still encoded family semantics.
- **Alternatives considered:**
  - Leave `RS-TEST-16` as “any proof-bearing export exists” — rejected because it did not force assertions to own semantic proof.
  - Infer “reusable semantics” heuristically — rejected because it would be vague and unstable; field/path/string-signal checks are stricter and reproducible.

### Checkpoint the whole tree before finishing `hexarch`
- **Chose:** Commit the current state even though `hexarch` is not done.
- **Why:** The user explicitly asked for a commit, and the validator-side work plus `test`/`arch`/`cargo` fallout fixes are substantial, coherent progress worth preserving. The unfinished state is localized and documented.
- **Alternatives considered:**
  - Wait until `hexarch` is fully green — rejected because the user asked for a checkpoint now.
  - Split the commit into validator-only and family-fallout-only pieces — rejected because the current worktree already contains intertwined fallout fixes across families.

## Architectural Notes
- The main validator changes live in the self-hosted `rs/test` family runtime:
  - `discover.rs` now normalizes package/dependency names to Rust crate identifiers for boundary analysis.
  - `parse.rs` now exposes richer semantic signals such as path uses, field accesses, string literals, and `CheckResult`-shaped helper signatures.
  - `RS-TEST-03` is stricter about local crate imports/calls and assertion-orchestration leakage.
  - `RS-TEST-07` / `RS-TEST-16` use richer proof-bearing identity.
  - `RS-TEST-18` is stricter about semantic helper leakage from `test_support`.
- `arch` and `cargo` were pulled back into compliance by moving more semantic proof into their assertions crates and stopping runtime sidecars from importing or asserting on disallowed surfaces directly.
- `hexarch` is only partially migrated. The remaining failures are concentrated in runtime sidecars that still inspect `CheckResult` collections directly for counts, titles, messages, files, and cross-rule ownership.

## Information Sources
- Recent worklogs:
  - `.worklogs/2026-03-26-193046-rs-test-sidecar-semantic-helper-refactor.md`
  - `.worklogs/2026-03-26-192344-cargo-rs-test-validator-fix.md`
  - `.worklogs/2026-03-26-185943-fix-arch-rs-test-16-sidecar-assertions.md`
- Contract/docs:
  - `.plans/todo/checks/rs/test.md`
  - `apps/guardrail3/crates/app/rs/families/test/README.md`
- Validator implementation:
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs`
- Fallout families:
  - `apps/guardrail3/crates/app/rs/families/arch/**`
  - `apps/guardrail3/crates/app/rs/families/cargo/**`
  - `apps/guardrail3/crates/app/rs/families/hexarch/**`

## Open Questions / Future Considerations
- `hexarch` still needs a large semantic-proof migration. The validator appears correct there; the open question is just how to batch the family rewrite most efficiently.
- `RS-TEST-16` may still want another pass later once `hexarch` is green, to see whether any remaining allowed sidecar patterns still feel too permissive.
- Two restarted `hexarch` workers were shut down after not converging; their work was not merged. Any future delegation should be on much narrower rule bands.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — crate-name normalization and test-family discovery rules
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs` — richer semantic parse signals used by tightened rules
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — runtime/assertions boundary enforcement
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — sidecar semantic-proof enforcement
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs` — generic test-support enforcement
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/**` — fallout-fixed arch sidecars and test entrypoints
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/**` — cargo proof-bearing assertion helpers added during fallout cleanup
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/**` — unfinished fallout area; many runtime sidecars still own semantic checks
- `.worklogs/2026-03-26-193046-rs-test-sidecar-semantic-helper-refactor.md` — self-hosted `rs/test` cleanup context
- `.worklogs/2026-03-26-192344-cargo-rs-test-validator-fix.md` — cargo fallout cleanup context

## Next Steps / Continuation Plan
1. Finish `hexarch` compliance under the hardened validator by moving runtime-side semantic result assertions into the matching assertions modules, starting with the heaviest remaining files:
   - `rs_hexarch_07_workspace_members_match_crate_dirs_tests/ownership.rs`
   - `rs_hexarch_02_exact_contents_tests/root_loose_files.rs`
   - `rs_hexarch_06_leaf_valid_tests/broad_attacks.rs`
   - `rs_hexarch_10_members_within_app_boundary_tests/ownership.rs`
2. Re-run:
   - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/hexarch --family test --inventory --format json`
   - `CARGO_TARGET_DIR=/tmp/guardrail3-hexarch cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
3. Once `hexarch` is green, rerun the four-family matrix:
   - `arch`, `cargo`, `hexarch`, `test` under `RS-TEST`
   - `arch`, `cargo`, `hexarch` under `RS-ARCH`
4. If the tree is then stable, write a follow-up worklog specifically for the `hexarch` fallout cleanup and commit that as a separate checkpoint.
