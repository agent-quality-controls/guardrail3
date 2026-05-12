# Goal

Close L30 fixture gaps found by adversarial review.

L30 means:

- workspace root exists
- `guardrail3-rs.toml` exists and parses
- selected family execution is unblocked
- missing required family inputs or activation configs are visible

# Accepted Additions

- Add `g3rs-deps/gitignore-not-ignoring-cargo-lock` to base L30 by adding `.gitignore` that ignores `Cargo.lock`.
- Add a library deps fixture for `g3rs-deps/library-allowlist-present` with no `allowed_deps`.
- Add a garde fixture for `g3rs-garde/dependency-present` with garde enabled in guardrail config but no `garde` dependency.
- Add a garde fixture for clippy-ban inputs missing after `garde` dependency is present:
  - `g3rs-garde/core-method-bans`
  - `g3rs-garde/extractor-type-bans`
  - `g3rs-garde/reqwest-json-ban`
  - `g3rs-garde/additional-method-bans`
- Add a workspace-member fixture for missing declared member roots:
  - `g3rs-cargo/missing-member-cargo`
  - `g3rs-topology/declared-workspace-members-only`
- Add a fmt fixture for nightly rustfmt keys when the toolchain input is missing:
  - `g3rs-fmt/nightly-keys-on-stable`
- Add a valid runtime/assertions test fixture for nextest missing input:
  - `g3rs-test/nextest-timeouts`
- Add file-tree activation fixtures for root config conflicts and misplaced config files:
  - `g3rs-toolchain/legacy-file`
  - `g3rs-toolchain/legacy-file` legacy-only warning branch
  - `g3rs-fmt/dual-file-conflict`
  - `g3rs-fmt/per-crate-override`
  - `g3rs-clippy/same-root-conflict`
  - `g3rs-deny/shadowing`
- Add release ingestion failure fixture for missing declared workspace member:
  - `g3rs-release/config-input-failures`
  - `g3rs-release/filetree-input-failures`
  - `g3rs-release/source-input-failures`
- Add topology file-tree fixtures for workspace membership and nested ownership:
  - `g3rs-topology/member-paths-must-not-escape-root`
  - `g3rs-topology/declared-workspace-members-only` undeclared child branch
  - `g3rs-topology/no-nested-workspaces`
  - `g3rs-topology/no-nested-guardrail3-rs-toml`
  - `g3rs-topology/workspace-local-file-placement`

# Pollution Fix

- Remove L70 test source/file-tree pollution from `L32-test-required-inputs-missing`.
- Do not use an external test harness in L32.
- Keep only the L30 missing input results in its required baseline contract.
- Cover `g3rs-test/nextest-timeouts` only with a valid runtime/assertions test shape so no L70 test architecture findings appear.

# Rejected Additions

- Tool installation findings remain L50/L60.
- Existing but invalid config findings remain L40.
- Source architecture findings remain L70.
- Hooks findings remain in `validate-repo` fixtures, not workspace L30 fixtures.

# Files To Modify

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs/L30-guardrail-config-valid-required-inputs-missing`
- `behavior/fixtures/g3rs/L32-test-required-inputs-missing`
- new `behavior/fixtures/g3rs/L34-deps-library-policy-missing`
- new `behavior/fixtures/g3rs/L35-garde-dependency-missing`
- new `behavior/fixtures/g3rs/L36-garde-clippy-inputs-missing`
- new `behavior/fixtures/g3rs/L37-workspace-member-inputs-missing`
- new `behavior/fixtures/g3rs/L30-toolchain-legacy-file-present`
- new `behavior/fixtures/g3rs/L30-toolchain-legacy-only-file-present`
- new `behavior/fixtures/g3rs/L30-fmt-dual-file-conflict`
- new `behavior/fixtures/g3rs/L30-fmt-per-crate-override`
- new `behavior/fixtures/g3rs/L30-clippy-same-root-conflict`
- new `behavior/fixtures/g3rs/L30-deny-shadowing`
- new `behavior/fixtures/g3rs/L30-release-member-input-failures`
- new `behavior/fixtures/g3rs/L30-topology-member-path-escape`
- new `behavior/fixtures/g3rs/L30-topology-undeclared-workspace-child`
- new `behavior/fixtures/g3rs/L30-topology-nested-workspace`
- new `behavior/fixtures/g3rs/L30-topology-nested-guardrail-config`
- new `behavior/fixtures/g3rs/L30-topology-workspace-local-file-placement`
- `behavior/baselines/g3rs`

# Verification

- `scripts/behavior/verify-all.sh`
- `python3 -m py_compile scripts/behavior/*.py`
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate-repo`
