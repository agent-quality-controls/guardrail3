# G3RS Info-Only Coverage Closure

## Goal

Close Stage 3 of `.plans/2026-05-14-110900-g3rs-fixture-coverage-closure.md`.

Every remaining `planned_existing_fixture_expansion` row in `behavior/coverage/g3rs-rule-coverage.toml` must either:

- have one public replay fixture that emits `Error` or `Warn` for that rule ID, or
- be marked as intentional info-only inventory with a rule-specific reason based on the implementation.

## Current Input

Current remaining planned rows:

- `g3rs-cargo/disallowed-macros-deny`
- `g3rs-cargo/priority-order`
- `g3rs-cargo/workspace-metadata`
- `g3rs-clippy/avoid-breaking-exported-api`
- `g3rs-clippy/ban-reason-quality`
- `g3rs-clippy/duplicate-bans`
- `g3rs-clippy/library-global-state`
- `g3rs-clippy/macro-bans`
- `g3rs-clippy/missing-method-ban`
- `g3rs-clippy/missing-type-ban`
- `g3rs-clippy/policy-context-parseable`
- `g3rs-clippy/unknown-keys`
- `g3rs-code/unsafe-code-lint`

`g3rs-clippy/extra-method-ban` and `g3rs-clippy/extra-type-ban` are covered by `L44-clippy-typed-config-invalid`, not by L61. Their only Error branch is malformed ban-section syntax, so the branch belongs to the required-input-invalid fixture layer.
- `g3rs-code/unused-crate-dependencies-allow`
- `g3rs-deny/highlight-inventory`
- `g3rs-fmt/rustfmt-extra-settings-inventory`
- `g3rs-hooks/local-override-inventory`
- `g3rs-hooks/modular-directory-inventory`
- `g3rs-hooks/modular-scripts-inventory`
- `g3rs-hooks/no-bypass-instructions`
- `g3rs-hooks/pre-commit-file-size-inventory`
- `g3rs-hooks/script-stats-inventory`
- `g3rs-release/binary-release-workflow`
- `g3rs-release/cliff-baseline`
- `g3rs-release/crate-inventory`
- `g3rs-release/linux-release-target`
- `g3rs-release/publish-status-inventory`
- `g3rs-release/release-plz-baseline`
- `g3rs-release/semver-checks-installed`

## Implementation

### New G3RS validate fixture

Add one small fixture:

- `behavior/fixtures/g3rs/L61-cargo-clippy-code-config-branches`

It must run:

```sh
g3rs validate --path . --family cargo --family clippy --family code --rules-only --inventory
```

It must contain only the files required for Cargo, Clippy, and Code config branches:

- `fixture.toml`
- `repo/Cargo.toml`
- `repo/clippy.toml`
- `repo/guardrail3-rs.toml`
- `repo/src/lib.rs`

It must emit `Error` or `Warn` rows for:

- `g3rs-cargo/disallowed-macros-deny`
- `g3rs-cargo/priority-order`
- `g3rs-cargo/workspace-metadata`
- `g3rs-clippy/avoid-breaking-exported-api`
- `g3rs-clippy/ban-reason-quality`
- `g3rs-clippy/duplicate-bans`
- `g3rs-clippy/extra-method-ban`
- `g3rs-clippy/extra-type-ban`
- `g3rs-clippy/library-global-state`
- `g3rs-clippy/macro-bans`
- `g3rs-clippy/missing-method-ban`
- `g3rs-clippy/missing-type-ban`
- `g3rs-code/unsafe-code-lint`

Do not cover `g3rs-clippy/policy-context-parseable` in this fixture if doing so would require making `guardrail3-rs.toml` unparsable, because that changes the policy context for other Clippy rules.

Do not cover `g3rs-clippy/unknown-keys` in this fixture. The strict clippy parser treats the near-miss managed key as a typed config parse error, so the warning belongs to `L44-clippy-typed-config-invalid`.

### Existing validate-repo fixture update

Update `R16-hooks-required-steps-present-but-weakened` by adding one shell comment containing `--no-verify`.

This must expose the existing non-inventory `Info` branch for:

- `g3rs-hooks/no-bypass-instructions`

It must not change hook reachability or step weakening behavior.

### Release replay fixture update

Update the smallest existing release fixture that already runs release config checks and has valid release inputs.

If the current fixtures can expose warning branches without hiding metadata checks, update `L70-release-invalid-semver-policy-violated`:

- make `release-plz.toml` violate the baseline
- make `cliff.toml` violate the baseline

This must emit `Warn` rows for:

- `g3rs-release/cliff-baseline`
- `g3rs-release/release-plz-baseline`

Add `L50-release-semver-checks-missing` for `g3rs-release/semver-checks-installed`.

- run only the release family
- use `runner_mode = "path_without_delegated_tools"`
- keep release metadata, release workflow, README, LICENSE, and semver valid
- emit only `Warn|g3rs-release/semver-checks-installed|cargo-semver-checks missing|Cargo.toml`
- do not strip delegated tools inside `L70-release-invalid-semver-policy-violated`

### Pure inventory rows

Mark these as covered with specific reasons if implementation confirms they cannot emit `Warn` or `Error`:

- `g3rs-code/unused-crate-dependencies-allow`
- `g3rs-deny/highlight-inventory`
- `g3rs-fmt/rustfmt-extra-settings-inventory`
- `g3rs-hooks/local-override-inventory`
- `g3rs-hooks/modular-directory-inventory`
- `g3rs-hooks/modular-scripts-inventory`
- `g3rs-hooks/pre-commit-file-size-inventory`
- `g3rs-hooks/script-stats-inventory`
- `g3rs-release/binary-release-workflow`
- `g3rs-release/crate-inventory`
- `g3rs-release/linux-release-target`
- `g3rs-release/publish-status-inventory`

For `g3rs-clippy/policy-context-parseable`, either:

- add an isolated fixture if the public CLI can expose its parse-error branch without hiding every other Clippy rule, or
- mark it intentional info-only only if public replay cannot expose the error branch without making the family input invalid before branch-level checks run.

## Files To Modify

- `behavior/fixtures/g3rs/L61-cargo-clippy-code-config-branches/**`
- `behavior/fixtures/g3rs/L50-release-semver-checks-missing/**`
- `behavior/fixtures/g3rs-validate-repo/R16-hooks-required-steps-present-but-weakened/repo/.githooks/pre-commit`
- `behavior/fixtures/g3rs/L70-release-invalid-semver-policy-violated/repo/release-plz.toml`
- `behavior/fixtures/g3rs/L70-release-invalid-semver-policy-violated/repo/cliff.toml`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `behavior/golden/g3rs-validate/approved.normalized.json`
- `behavior/golden/g3rs-validate-repo/approved.normalized.json`

## Verification

Run:

```sh
python3 -m py_compile scripts/behavior/*.py
scripts/behavior/verify-all.sh
fixture3 check --all
g3rs validate --path apps/guardrail3-rs --inventory
g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
g3rs validate-repo
git diff --check
```

Expected coverage verifier state:

```text
behavior-rule-coverage: PASS source:266 covered:266 planned:0
```
