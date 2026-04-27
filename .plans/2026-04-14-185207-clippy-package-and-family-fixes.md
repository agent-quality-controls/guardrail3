Goal

- Fix the agreed issues blocking `packages/rs/clippy/g3rs-clippy-config-checks` without touching the still-undecided rules.
- Keep package fixes package-local and family fixes in their owning families.

Approach

1. Fix `g3rs-arch/feature-contract`
   - Add failing tests proving crates with feature-gated facade exports do not need an `all` feature.
   - Keep the requirement for explicit feature contracts.
   - Remove the hardcoded `all` + `default = ["all"]` requirement from the rule.

2. Fix hooks applicability
   - Add failing ingestion tests proving nested package workspaces should not require repo-global `.githooks/pre-commit` or `core.hooksPath`.
   - Make hooks filetree coverage apply only at the Git repo root, not nested package workspaces.

3. Fix `g3rs-code/ast-24-path-attr-with-reason` in the clippy package
   - Keep the existing sidecar test directories.
   - Remove `#[path = "rule_tests/mod.rs"]` and switch rule files to normal `mod rule_tests;`.
   - This preserves the sidecar directory shape while removing the redirected module-resolution attribute.

4. Fix package boundary/facade issues in the clippy package
   - `g3rs-arch/no-boundary-crossing`
   - `g3rs-arch/shared-flag-required`
   - `g3rs-arch/lib-facade-only`
   - Inspect the exact dependency graph in `crates/runtime` and `crates/assertions`.
   - Move or mark dependencies at the architecturally correct place instead of adding exemptions.
   - Make `crates/assertions/src/lib.rs` facade-only.

5. Fix release metadata for the clippy package workspace
   - Add missing per-crate metadata: keywords, categories, docs.rs metadata, include/exclude.
   - Add missing subcrate README files.
   - Add workspace `LICENSE`, `release-plz.toml`, and `cliff.toml`.
   - Add the missing release workflows only if the release family truly expects standalone package workspaces to self-host release.
   - Re-run the CLI after metadata changes to confirm exactly which release findings remain.

Key decisions

- Do not touch `RS-CODE-FILETREE-35` or `g3rs-arch/structural-split` in this slice.
  - Those are intentionally deferred pending rule review.
- Do not touch `g3rs-code/ast-31-public-struct-named-fields` in this slice.
  - Test/assertion DTO surfaces are still undecided.
- Ignore `test` family findings in this slice by user direction.
- Keep sidecar directories; only remove `#[path]`.

Files to modify

- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_08b_feature_contract.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_08b_feature_contract_tests/mod.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/*/rule.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/types/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/README.md`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/README.md`
- `packages/rs/clippy/g3rs-clippy-config-checks/LICENSE`
- `packages/rs/clippy/g3rs-clippy-config-checks/release-plz.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/cliff.toml`
