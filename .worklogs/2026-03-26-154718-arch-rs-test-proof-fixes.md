# Finish Arch RS-TEST Compliance Proof

**Date:** 2026-03-26 15:47
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_*`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/*`, `.plans/todo/rs-test-compliance-handoffs/arch.md`

## Summary
Closed the remaining `RS-TEST` findings on `rs/arch` after the structural split. The family now passes local workspace tests and both product-level validations on the family root: `--family arch` and `--family test` each return zero findings.

## Context & Problem
The previous commit (`cb2b630`) moved `rs/arch` into the required `runtime/assertions/test_support` shape, but the live `rs validate ... --family test` run still reported:
- `RS-TEST-02` because the production files used `mod tests;` instead of the exact owned sidecar module name
- `RS-TEST-03` because each assertions module imported local `crate::common`
- `RS-TEST-07` because proof-site recognition could not connect runtime tests to the owned assertions crate

That meant the structural migration was only partially complete. The remaining job was to satisfy the actual test-family checker, not just compile the local workspace.

## Decisions Made

### Rename cfg-test module declarations to the exact owned sidecar module name
- **Chose:** Changed each runtime rule file from `mod tests;` to `mod rs_arch_<...>_tests;`.
- **Why:** `RS-TEST-02` checks the declaration name, not only the `#[path = ".../mod.rs"]` target. The production module name must resolve directly to the owned `<module>_tests` sidecar.
- **Alternatives considered:**
  - Leave `mod tests;` and rely on `#[path]` only — rejected because the validator explicitly rejects that as ad hoc shape.
  - Relax `RS-TEST-02` — rejected because this task was to make `arch` comply with the existing checker, not weaken the checker.

### Delete the stale empty sidecar directory left from older rule naming
- **Chose:** Removed the empty `rs_arch_05_enablement_coherence_tests` directory under runtime `src/`.
- **Why:** Even empty, it was still detected as a sidecar harness directory missing `mod.rs`.
- **Alternatives considered:**
  - Add a dummy `mod.rs` — rejected because the directory no longer corresponded to a live rule file.
  - Ignore it as harmless residue — rejected because `RS-TEST-02` correctly treats that residue as architecture noise.

### Make each assertions module self-contained instead of importing local `crate::common`
- **Chose:** Deleted `crates/assertions/src/common.rs` and inlined route building, result filtering, and proof-bearing assertions into each `rs_arch_*` assertions module.
- **Why:** `RS-TEST-03` forbids assertions modules from reaching local private code through `crate::`, `self::`, or `super::`. The shared `common` module directly violated that rule.
- **Alternatives considered:**
  - Keep `common.rs` and relax `RS-TEST-03` — rejected because it defeats the point of the runtime/assertions split.
  - Move semantic helpers back into `test_support` — rejected because that would blur the boundary between generic support and proof-bearing assertions again.

### Normalize the assertions package name to the actual imported Rust crate root
- **Chose:** Renamed the assertions package from `guardrail3-app-rs-family-arch-assertions` to `guardrail3_app_rs_family_arch_assertions`, and updated the runtime dev-dependency key to match.
- **Why:** `RS-TEST-07` matches proof-bearing calls against the owned assertions package name. The old hyphenated package name did not line up with the underscore Rust import path used by the sidecar tests, so proof calls were not recognized even after the assertions functions became real proof sites.
- **Alternatives considered:**
  - Keep the hyphenated package name and change all Rust imports — rejected because Rust crate roots cannot use hyphens.
  - Special-case package-name normalization inside `RS-TEST-07` — rejected because the simpler fix is to make the owned package name reflect the actual Rust crate root used by the family.

## Architectural Notes
After this pass, `rs/arch` is compliant at all three layers:
- disk shape: local `runtime/assertions/test_support` workspace
- assertions boundary: no local private imports inside assertions modules
- proof recognition: runtime sidecars call owned public assertion functions that contain direct `assert_eq!` proof sites

This keeps the family aligned with the enforced `rs/test` contract instead of merely imitating its directory layout.

## Information Sources
- `.plans/todo/rs-test-compliance-handoffs/arch.md` — handoff target and expected proof commands
- `apps/guardrail3/crates/app/rs/families/test/README.md` — exact RS-TEST rule contract for `02`, `03`, and `07`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_02_owned_sidecar_shape.rs` — sidecar declaration/name expectations
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — assertions import-boundary rules
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site.rs` — proof-site matching against owned assertions packages
- `.worklogs/2026-03-26-153639-arch-rs-test-compliance.md` — previous structural split checkpoint

## Open Questions / Future Considerations
- The local family workspace still generates `families/arch/Cargo.lock` and `families/arch/target/` during proof runs. They remain untracked and should stay that way.
- The same proof-site/package-name issue may exist in other families being migrated to local runtime/assertions workspaces. `cargo` and `hexarch` should be checked for the same mismatch.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_01_root_classification.rs` — representative fixed `cfg(test)` sidecar declaration shape
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/rs_arch_02_no_misplaced_roots.rs` — representative self-contained proof-bearing assertions module
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/Cargo.toml` — normalized assertions package name used by proof-site matching
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/Cargo.toml` — runtime dev-dependency on the owned assertions crate
- `.plans/todo/rs-test-compliance-handoffs/arch.md` — now reflects the fully passing end state
- `.worklogs/2026-03-26-153639-arch-rs-test-compliance.md` — prior structural split context

## Next Steps / Continuation Plan
1. Stage only the `arch` proof-fix files and this worklog; leave untracked local build artifacts out of the commit.
2. Commit the final `arch` RS-TEST compliance checkpoint.
3. Reuse the same validation loop for the next family handoff:
   - local family workspace tests
   - `guardrail3 rs validate <family-root> --family arch`
   - `guardrail3 rs validate <family-root> --family test`
4. In other migrated families, check early for package-name vs Rust-crate-root mismatches in assertions crates before chasing `RS-TEST-07` warnings elsewhere.
