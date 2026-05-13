# Expand G3RS L70 Project-Policy Behavior Fixtures

## Goal

Expand behavior replay coverage after L60 without turning every unit test into a one-off fixture.

L70 means:

- workspace root exists
- `guardrail3-rs.toml` is valid
- required input files exist
- required input files parse
- delegated tools exist
- delegated tool policy files are valid
- the remaining failures are project policy violations in source, file tree, package shape, dependency shape, app architecture, release metadata, or validation boundary code

The L70 fixture set must cover every existing rule that can fire at this layer without hiding another rule behind an earlier failure.

## Evidence Used

Read-only agents audited:

- `packages/rs/test`
- `packages/rs/code`
- `packages/rs/arch`
- `packages/rs/apparch`
- `packages/rs/topology`
- `packages/rs/cargo`
- `packages/rs/deps`
- `packages/rs/garde`
- `packages/rs/release`
- `behavior/fixtures/g3rs`
- `behavior/baselines/g3rs`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`

Local checks read:

- current L70 fixture files
- current L70 baseline
- behavior baseline scripts
- rule case files under the scoped families

## Current L70 Problem To Fix First

Current fixture:

- `behavior/fixtures/g3rs/L70-delegated-policy-valid-project-policy-violated`

Current intentional source:

- `repo/src/lib.rs`

Current issue:

- It uses `assert!(true, "fixture assertion")`.
- Cargo/clippy can fail on constant assertions before L70 rows remain a clean project-policy replay.

Required first change:

- Keep the inline test trigger.
- Replace the constant assertion with a nonconstant assertion that still compiles and still produces:
  - `g3rs-arch/lib-facade-only`
  - `g3rs-test/owned-sidecar-shape`
  - `g3rs-test/inline-test-bodies`

Allowed pattern:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn inline_test_body_is_project_policy_violation() {
        let observed = std::env::args().count();
        assert!(observed >= 1, "fixture assertion must inspect runtime state");
    }
}
```

Validation:

- Regenerate only this fixture baseline.
- Confirm the three existing rows remain present.
- Confirm no delegated cargo/clippy failure is added.

## Fixture Strategy

Do not force one giant fixture.

Use the minimum number of L70 fixtures needed to avoid shadowing:

- One dense source/file-tree fixture for parseable unreferenced source and simple file-tree policy.
- One workspace/package-policy fixture for Cargo member, dependency allowlist, and topology rows.
- One app-architecture dependency fixture for `apparch` graph and source boundary rows.
- One Garde boundary fixture for validation-source rows.
- One release metadata fixture for publishability metadata rows.

Reason:

- Source-only rows can coexist because G3RS scans parseable unreferenced source files and Cargo ignores them.
- Workspace topology rows change workspace membership and can trigger cargo-family rows if mixed carelessly.
- App-architecture rows need multiple crates and dependency edges.
- Garde rows require Garde activation and boundary-shaped source.
- Release rows require publishable-package activation and release metadata.

## Fixture 1: Existing L70 Dense Source/File-Tree Policy

Keep fixture id:

- `L70-delegated-policy-valid-project-policy-violated`

Purpose:

- Cover source and file-tree policy violations that do not need package graph changes.

Required existing rows:

- `g3rs-arch/lib-facade-only|lib.rs must be facade-only|src/lib.rs`
- `g3rs-test/owned-sidecar-shape|ad hoc cfg(test) module declaration|src/lib.rs`
- `g3rs-test/inline-test-bodies|inline cfg(test) body in src|src/lib.rs`

Add arch rows:

- `g3rs-arch/mod-facade-only`
- Trigger: `src/api/mod.rs` contains a body item or inline module.
- Supporting file: `src/api/leaf.rs` if needed.
- Do not add invalid Rust.

- `g3rs-arch/no-path-attr`
- Trigger: non-test `#[path = "generated.rs"] mod generated;`.
- Supporting file: `src/generated.rs`.
- Do not use test sidecar naming, because test sidecar path attrs are exempt.

- `g3rs-arch/feature-gated-exports`
- Trigger: public facade export not guarded by a feature.
- Use the same public export to cover `g3rs-arch/feature-contract` if no default feature is configured.

- `g3rs-arch/feature-contract`
- Trigger: public facade export exists and `[features] default` is missing or empty.
- Keep it in this fixture only if it does not require changing package metadata in a way that removes existing rows.

- `g3rs-arch/mod-rs-required`
- Trigger: unreferenced `src/orphan/leaf.rs` without `src/orphan/mod.rs`.
- Cargo ignores it, so delegated gates stay clean.

Add test rows:

- `g3rs-test/ignore-reason`
- Trigger: test with `#[ignore]` and no strong `reason:` comment.

- `g3rs-test/should-panic-expected`
- Trigger: `#[should_panic]` without `expected = "..."`.
- Body must use `std::panic::panic_any("fixture panic")`, not `panic!`, because clippy bans the macro.

- `g3rs-test/tautological-assertions`
- Trigger: unreferenced test sidecar with literal-only assertion.
- Use an unreferenced parseable file to avoid cargo/clippy acting before G3RS.

- `g3rs-test/weak-matches-assert`
- Trigger: unreferenced test sidecar with wildcard `matches!` assertion.

- `g3rs-test/real-proof-site`
- Trigger: test function that exercises code but has no assertion and does not call the shared assertions crate.

- `g3rs-test/assertions-modules-prove`
- Trigger: `assertions/src/lib.rs` with helper code but no proof-bearing export.
- No Cargo manifest is needed for this path in this fixture.

- `g3rs-test/external-harnesses-use-assertions`
- Trigger: `tests/direct.rs` with direct assertion.
- Assertion must be nonconstant.

- `g3rs-test/runtime-assertions-split`
- Trigger: root external harness or root test sidecar outside runtime/assertions split.
- This can be produced by the same `tests/direct.rs`.

- `g3rs-test/test-support-generic`
- Trigger: `test_support/src/lib.rs` exports semantic result constants or helper names that belong in assertions.

Add code rows:

- `g3rs-code/todo-macros`
- `g3rs-code/direct-fs-usage`
- `g3rs-code/panic-macro`
- `g3rs-code/fs-glob-import`
- `g3rs-code/crate-level-allow`
- `g3rs-code/unused-crate-dependencies-allow`
- `g3rs-code/item-level-allow-without-reason`
- `g3rs-code/item-level-allow-with-reason`
- `g3rs-code/deny-forbid-without-reason`
- `g3rs-code/garde-skip-without-comment`
- `g3rs-code/garde-skip-with-comment`
- `g3rs-code/cfg-attr-allow-inventory`
- `g3rs-code/always-true-cfg-attr-bypass`
- `g3rs-code/impl-allow-blast-radius`
- `g3rs-code/extern-allow`
- `g3rs-code/include-bypass`
- `g3rs-code/path-attr-with-reason`
- `g3rs-code/test-expect-message-quality`
- `g3rs-code/too-many-use-imports`
- `g3rs-code/many-use-imports`
- `g3rs-code/too-many-effective-code-lines`
- `g3rs-code/large-type-inventory`
- `g3rs-code/large-trait-surface`
- `g3rs-code/public-struct-named-fields`
- `g3rs-code/public-weak-error-forms`
- `g3rs-code/generic-parameter-cap`
- `g3rs-code/string-dispatch-cap`

Implementation rules for code rows:

- Put production-code probes in parseable unreferenced files under `src/`.
- Put test-expect probes under `tests/`.
- Do not add invalid Rust.
- Do not add compiled code that trips delegated clippy before G3RS rows are captured.
- Use one file per row group, not one file per rule, when rows do not hide each other.

## Fixture 2: L70 Workspace And Package Policy

Create fixture id:

- `L70-workspace-package-policy-violated`

Purpose:

- Cover package-level policy rows that need a workspace member or dependency surface.

Rows to include:

- `g3rs-cargo/rust-version-policy`
- Trigger: root package profile is `library`, but root `rust-version` is absent.
- This must not make Cargo parse fail.

- `g3rs-cargo/workspace-lints-inherited`
- Trigger: workspace member lacks `[lints] workspace = true`.

- `g3rs-cargo/no-weakened-overrides`
- Trigger: workspace member has `[lints] workspace = true` plus a weaker local lint override.

- `g3rs-cargo/member-edition-drift`
- Trigger: member edition is older than root edition.

- `g3rs-cargo/member-local-allows-forbidden`
- Trigger: member has local `allow` lint entry.

- `g3rs-deps/dependencies-allowlisted`
- Trigger: normal dependency exists but is not in `allowed_deps`.

- `g3rs-deps/build-dependencies-allowlisted`
- Trigger: build dependency exists but is not in `allowed_deps`.

- `g3rs-deps/dev-dependencies-allowlisted`
- Trigger: dev dependency exists but is not in `allowed_deps`.

- `g3rs-arch/crate-has-facade`
- Trigger: member package has no `src/lib.rs` and no `src/main.rs`.
- Keep cargo gates clean by making the member use an explicit valid `[[bin]] path = "src/alt.rs"` if Cargo requires a target.

- `g3rs-topology/declared-workspace-members-only`
- Trigger: child package exists under the workspace root but is not declared as a member.
- Use the undeclared-child branch only.
- Do not use the missing-declared-member branch in L70.

- `g3rs-topology/no-nested-workspaces`
- Trigger: nested descendant `Cargo.toml` has `[workspace]`.
- Do not add it to root workspace members.

- `g3rs-topology/no-nested-guardrail3-rs-toml`
- Trigger: nested descendant contains `guardrail3-rs.toml`.

- `g3rs-topology/workspace-local-file-placement`
- Trigger: workspace-local family config file appears under a member or nested dir.
- Choose a file that does not activate parser/input failures.

Dependency-count rows:

- `g3rs-arch/dependency-count-split`
- `g3rs-deps/direct-dependency-cap`

Plan for these:

- Try to include them in this fixture using local path dependency crates.
- Each dependency must be used so `cargo-machete` stays clean.
- Each dependency must be allowlisted unless the fixture intentionally needs the allowlist row.
- If this creates a 26-crate synthetic fixture that hides simpler rows or makes delegated tools dominate, split it into `L70-dependency-surface-policy-violated`.

Rejected shapes for this fixture:

- Missing declared member, because it overlaps `g3rs-cargo/missing-member-cargo`.
- Escaping member path, because it can break Cargo discovery before L70 rows remain clean.
- Invalid Cargo TOML, because it belongs to L40.

## Fixture 3: L70 App-Architecture Policy

Create fixture id:

- `L70-apparch-policy-violated`

Purpose:

- Cover app-architecture dependency and source-boundary rules that require multiple crates.

Rows to include:

- `g3rs-arch/no-boundary-crossing`
- Trigger: package depends on an internal crate inside another crate boundary.

- `g3rs-arch/shared-flag-required`
- Trigger: package depends on sibling/internal path crate not marked `shared = true`.

- `g3rs-apparch/types-dependency-direction`
- Trigger: `types/...` crate production-depends on another internal apparch crate.

- `g3rs-apparch/logic-dependency-direction`
- Trigger: `logic/...` crate depends on forbidden `logic`, `io/inbound`, or `io/outbound` layer.

- `g3rs-apparch/io-outbound-dependency-direction`
- Trigger: `io/outbound/...` crate depends on forbidden `logic`, `io/inbound`, or `io/outbound` layer.

- `g3rs-apparch/dev-dependency-direction`
- Trigger: dev-dependency crosses forbidden apparch layer and is used by a test target.

- `g3rs-apparch/types-purity`
- Trigger: `types/...` crate has non-dev external dependency not allowed by apparch purity.

- `g3rs-apparch/logic-purity`
- Trigger: `logic/...` crate has non-dev external dependency not allowed by apparch purity.

- `g3rs-apparch/patch-replace-bypass`
- Trigger: root `[patch]` or `[replace]` points at internal apparch crate without waiver.

- `g3rs-apparch/types-public-surface`
- Trigger: `types/...` crate exposes public behavior function or inherent method.

- `g3rs-apparch/io-traits-in-types`
- Trigger: `io/inbound` or `io/outbound` crate defines public trait that should live in `types`.

Implementation rules:

- Use local path crates for dependency edges.
- Every dependency must be used so `cargo-machete` does not dominate.
- Avoid reciprocal dependency cycles unless a probe proves `g3rs-apparch/same-layer-cycles` can fire without Cargo failing first.

Excluded from initial L70 apparch fixture:

- `g3rs-apparch/same-layer-cycles`

Reason:

- Real package cycles can fail Cargo before the G3RS row is useful.
- Add it only if an inactive target-cfg dependency probe proves the row can fire without cargo-gate failure.

## Fixture 4: L70 Garde Boundary Policy

Create fixture id:

- `L70-garde-boundary-policy-violated`

Purpose:

- Cover validation-boundary source rules after Garde is fully enabled.

Setup:

- Add `garde` dependency.
- Add `garde` to the dependency allowlist.
- Keep clippy Garde method/type bans valid.
- Keep all input files parseable.

Rows to include:

- `g3rs-garde/struct-derive-validate`
- Trigger: boundary struct derives deserialization-like boundary macro but does not derive `Validate`.

- `g3rs-garde/enum-derive-validate`
- Trigger: boundary enum carries non-primitive payload but does not derive `Validate`.

- `g3rs-garde/manual-deserialize-impl`
- Trigger: manual `Deserialize` impl exists for boundary type without `Validate`.

- `g3rs-garde/field-level-constraints`
- Trigger: validated boundary field has no meaningful `#[garde(...)]` constraint.

- `g3rs-garde/nested-validation-dive`
- Trigger: nested validated field misses `#[garde(dive)]`.

- `g3rs-garde/context-validation-surface`
- Trigger: field validator uses `ctx`, but boundary type lacks `#[garde(context(...))]`.

- `g3rs-garde/query-as-inventory`
- Trigger: `sqlx::query_as!` appears without matching waiver.

Implementation rules:

- Prefer unreferenced parseable source files if the rule scans them.
- Compile only the minimum code needed to keep delegated gates clean.
- Do not introduce source parse failures.

## Fixture 5: L70 Release Metadata Policy

Create fixture id:

- `L70-release-metadata-policy-violated`

Purpose:

- Cover publishability metadata rows that are not delegated release-config baselines and not repo-root workflow rows.

Setup:

- Use a publishable crate shape.
- Keep `LICENSE`, `README.md`, `release-plz.toml`, and `cliff.toml` present and parseable unless the row being tested specifically requires a source-quality issue.
- Keep delegated release config policy valid.

Rows to include if probe confirms they are visible without dry-run domination:

- `g3rs-release/publish-must-be-explicit`
- `g3rs-release/description-present`
- `g3rs-release/license-present`
- `g3rs-release/repository-present`
- `g3rs-release/keywords-present`
- `g3rs-release/categories-present`
- `g3rs-release/valid-semver`
- `g3rs-release/docs-rs-metadata`
- `g3rs-release/binstall-metadata`
- `g3rs-release/accidentally-publishable`
- `g3rs-release/include-exclude-inventory`
- `g3rs-release/no-path-deps-to-unpublishable`
- `g3rs-release/interdependent-version-consistency`
- `g3rs-release/readme-quality`

Split rules:

- If `valid-semver` breaks Cargo metadata before other release rows, split it into its own L70 release fixture.
- If path-dependency release rows require multi-crate structure that hides simple metadata rows, split them into `L70-release-path-dependency-policy-violated`.
- If publish dry-run output dominates, keep static release metadata rows only and move dry-run behavior to a later fixture layer.

Excluded from L70:

- `g3rs-release/publish-dry-run`
- `g3rs-release/semver-checks-installed`
- `g3rs-release/publish-dry-run-workflow`
- `g3rs-release/registry-token`
- `g3rs-release/release-plz-workflow`

Reason:

- These are deeper delegated execution or repo-root workflow rows, not per-workspace L70 project-policy rows.

## Explicit Exclusions From L70

Do not add these to L70:

- `g3rs-code/input-failures`
- `g3rs-test/source-input-failures`
- `g3rs-test/filetree-input-failures`
- `g3rs-garde/input-failures`
- `g3rs-release/config-input-failures`
- `g3rs-release/filetree-input-failures`
- `g3rs-release/source-input-failures`
- `g3rs-topology/required-inputs-fail-closed`

Reason:

- They require missing, unreadable, or invalid inputs.
- They belong to L30-L40 coverage.
- They hide source and project-policy checks for the same file or family.

Do not add inventory-only rows as required L70 failures:

- `g3rs-cargo/approved-allow-inventory`
- `g3rs-release/publish-status-inventory`
- `g3rs-release/release-profile-inventory`
- `g3rs-release/crate-inventory`
- `g3rs-release/binary-release-workflow`
- `g3rs-release/linux-release-target`

Reason:

- Inventory rows can appear in baselines, but the fixture should not exist just to prove an Info row.

## Probe Protocol

For every candidate row:

1. Modify a scratch fixture copy, not the committed fixture.
2. Run the fixture command:

```sh
python3 scripts/behavior/generate-baselines.py
python3 scripts/behavior/verify-baselines.py
```

3. Inspect the generated baseline stdout.
4. Keep the mutation only if:
   - all existing fixture required rows still appear
   - the candidate row appears
   - no L00-L60 row appears
   - no parse/input failure appears unless the fixture is explicitly for that layer
   - no delegated command output dominates the row set
5. Split the mutation into a separate L70 fixture if:
   - it changes workspace graph or package activation
   - it requires publishable release activation
   - it requires Garde activation
   - it makes another L70 candidate disappear
6. Reject the mutation from L70 if it only works by making inputs invalid.

## Manifest Updates

After fixture probes pass:

- Add new fixture entries to `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`.
- Mark each L70 fixture:
  - `expected_exit = "nonzero"`
  - `baseline_required = true`
  - `closed_file_list = true`
- Add every committed fixture file to `files`.
- Add every Error/Warn row that defines the fixture purpose to `required_results`.
- Do not add Info-only rows to `required_results` unless the fixture exists to prove inventory output.

## Verification

Required commands after implementation:

```sh
python3 -m py_compile scripts/behavior/*.py
scripts/behavior/verify-all.sh
g3rs validate --path apps/guardrail3-rs --inventory
g3rs validate-repo
git diff --check
```

Commit gate:

- Run the normal pre-commit hook.
- Do not bypass hooks.

## Adversarial Review After Implementation

Send read-only adversarial agents after implementation.

Agent 1:

- Read this plan.
- Read all L70 fixtures and baselines.
- Read the manifest.
- Report every planned row missing from `required_results`.
- Report every required row that is hidden by a lower-layer failure.

Agent 2:

- Read rule case files under `packages/rs/test`, `packages/rs/code`, `packages/rs/arch`, `packages/rs/apparch`, `packages/rs/topology`, `packages/rs/cargo`, `packages/rs/deps`, `packages/rs/garde`, and `packages/rs/release`.
- Compare implemented L70 coverage against rule tests.
- Report every rule test behavior that can fit L70 but is still not represented.

Agent 3:

- Run `scripts/behavior/verify-all.sh`.
- Inspect generated baseline stdout for L70 fixtures.
- Report lower-layer rows, delegated-tool domination, redundant fixtures, and mergeable fixtures.

Implementation is not done until all adversarial findings are either fixed or moved to an explicit later-layer plan with a concrete reason.
