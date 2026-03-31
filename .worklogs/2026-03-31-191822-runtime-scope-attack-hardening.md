# Runtime Scope Attack Hardening

**Date:** 2026-03-31 19:18
**Scope:** `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs`, `apps/guardrail3/crates/app/rs/family_selection/assertions/src/selection.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs_tests/mod.rs`, `apps/guardrail3/crates/app/rs/legality/src/lib.rs`, `apps/guardrail3/crates/app/rs/runtime/Cargo.toml`, `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`, `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs`, `apps/guardrail3/crates/app/rs/runtime/assertions/src/runtime.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots_tests/enablement_matrix.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts/policy.rs`

## Summary
This patch set hardens Rust runtime routing after the surface/legality split. It makes `arch` always-on in runtime selection, aligns global-vs-local routing with the current scope contract, expands runtime attack coverage across the remaining unproven families, and fixes a real `code` bug where Rust files outside discovered Cargo roots were silently invisible.

## Context & Problem
The current architecture depends on a strict split:
- shared structure discovers roots and owned files
- shared legality decides what is legal
- family mapper slices routed family inputs
- runtime invokes global families once and workspace-local families once per legal workspace

That split had two kinds of risk after the recent refactors:
- contractual drift, where docs said a family was global but mapper/runtime still narrowed it
- false green runtime tests, where a family might return nothing or miss files and still look fine

The user explicitly asked for attack-style validation, not just green tests. The required proof was:
- `arch` is always run
- global families are actually global
- workspace-local families really fan out per legal workspace
- scoped runs do not leak into siblings
- runtime tests prove actual execution, not just absence

## Decisions Made

### Make `arch` unconditional in Rust family selection
- **Chose:** always insert `RustValidateFamily::Arch` during selection and treat it as runtime-enabled regardless of config.
- **Why:** placement/topology legality must not disappear in family-only runs. The architecture now assumes `arch` is part of every Rust validation surface.
- **Alternatives considered:**
  - Keep `arch` user-selectable and let family-only runs hide topology/placement issues — rejected because that undermines the legality-first model.
  - Leave `arch` optional but bolt on special runtime exceptions — rejected as weaker and harder to reason about than making `arch` mandatory.

### Treat `code` and `test` as truly global routes
- **Chose:** map `code` and `test` as global families in the mapper/runtime and stop narrowing them by validation scope.
- **Why:** current docs and plans already describe them as repo-global. Narrowing them by root or scope creates invisible files and turns “global” into a lie.
- **Alternatives considered:**
  - Keep them pseudo-global over discovered roots only — rejected because non-root Rust files can then escape validation.
  - Push root-placement ownership into those families — rejected because legality/placement belongs before family execution, not inside family logic.

### Fix `code` so rootless Rust files are still checked
- **Chose:** run `code` on the full routed project surface and stop filtering files by owning Cargo root inside code facts.
- **Why:** the attack test using `tools/stray.rs` proved a real hole. A repo-global family cannot depend on “has an owning Cargo root” just to see a Rust file.
- **Alternatives considered:**
  - Keep the current route and only special-case some directories — rejected because it preserves the wrong contract.
  - Teach `arch` to reject every rootless Rust file instead of letting `code` see it — rejected because `code` still owns source-level policy over those files.

### Expand runtime attack coverage to the remaining risky families
- **Chose:** add runtime tests for `garde`, `hexarch`, and `libarch` proving per-workspace fanout and scoped isolation, and strengthen route/multi-section assertions.
- **Why:** earlier runtime coverage was heavily skewed toward a few families. The rest could still have routing bugs or false greens.
- **Alternatives considered:**
  - Leave them to family-local tests only — rejected because runner/mapper bugs can still break runtime while family-local tests stay green.
  - Assert only section presence — rejected because that does not prove the family actually executed the intended rule surface.

### Keep ancestor cargo-config legality for clippy
- **Chose:** preserve the legality fix that treats ancestor `.cargo/config.toml` / `config` as a legal owning input for clippy when it covers descendant legal workspaces.
- **Why:** clippy legitimately depends on ancestor cargo config for `CLIPPY_CONF_DIR`-style behavior, and mapper/runtime tests were already exposing that gap.
- **Alternatives considered:**
  - Force clippy to ignore ancestor cargo config entirely — rejected because it contradicts actual tool behavior and weakens enforcement.

## Architectural Notes
- `arch` is now a mandatory repo-global legality/reporting family, not a selectable optional add-on.
- Global Rust families currently are:
  - `arch`
  - `fmt`
  - `code`
  - `test`
- Workspace-local Rust families currently are:
  - `toolchain`
  - `clippy`
  - `deny`
  - `cargo`
  - `garde`
  - `deps`
  - `release`
  - `hexarch`
  - `libarch`
- The runner contract is now exercised directly for all of those scope classes.
- The most important functional bug uncovered in this pass was not in legality or mapper. It was in the `code` family’s own fact collection, which still quietly assumed source files needed an owning Cargo root.

## Information Sources
- Current architecture and family scope docs:
  - `AGENTS.md`
  - `apps/guardrail3/crates/app/rs/README.md`
  - `.plans/by_family/rs/README.md`
  - `.plans/by_family/rs/code.md`
  - `.plans/by_family/rs/test.md`
  - `.plans/by_family/rs/garde.md`
  - `.plans/by_family/rs/hexarch.md`
  - `.plans/by_family/rs/libarch.md`
- Runtime/mapping code:
  - `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs`
- Family implementations exercised by attack tests:
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/lib.rs`
- Prior worklogs:
  - `.worklogs/2026-03-31-171258-close-project-tree-family-leak.md`
  - `.worklogs/2026-03-31-173328-rs-runtime-attack-hardening.md`

## Open Questions / Future Considerations
- `test` is now treated as global at runtime, but it still models per-root activation internally. That is acceptable for now, but the family should keep being attacked for any root-based blind spots.
- `libarch` runtime attack coverage now proves local execution over legal routed package roots, but the family still depends heavily on legal topology coming from `arch`; broader cross-family attack pressure there is still useful.
- Hooks families are still global in selection/runtime, but this pass did not add equivalent runtime attack coverage for them.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — mandatory `arch` selection and global-family classification
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — global vs workspace-local route shaping for `arch`, `code`, `test`, and local families
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — actual runtime surface construction for global and local families
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` — runtime attack suite proving fanout/globality/scoped isolation
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs` — fixed `code` bug where rootless Rust files were dropped
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs` — misplaced-root activation now depends on owner architecture families, not `arch` config
- `.worklogs/2026-03-31-173328-rs-runtime-attack-hardening.md` — earlier runtime attack checkpoint this pass extends and corrects

## Next Steps / Continuation Plan
1. Add the same style of runtime attack coverage for hooks families if they remain in the active Rust runtime matrix.
2. Keep attacking `test` and `code` for any remaining non-root blind spots, especially files that are legal Rust-owned files but sit outside conventional Cargo workspace layouts.
3. Once the scope contract stabilizes, remove any remaining stale docs/tests that still describe routed-root behavior for global families.
