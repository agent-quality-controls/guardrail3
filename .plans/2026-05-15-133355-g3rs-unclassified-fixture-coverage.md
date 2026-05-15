# Goal

Remove the remaining unclassified Rust rule-test rows from `behavior/migration/g3rs-test-fixture-ledger.toml` by adding the smallest fixture states that expose those behaviors without hiding each other.

# Approach

- Extend existing source-policy and package-policy fixtures where the missing behavior is already at that layer.
- Add new deny fixtures only where one deny config state would hide another deny config state.
- Add new fmt fixtures for mutually exclusive Cargo edition precedence states.
- Add one repo-level hook fixture for path-qualified tools and safe `--no-verify` comment parsing.
- Add one topology fixture for nested-context topology rows because the existing root topology fixtures do not expose the nested workspace as the validation root.
- Update the ledger classifier only where the existing fixture output already proves a test but the test name is too specific for the generic keyword classifier.
- Verify the work through `fixture3 check --all`, fixture manifest verification, and strict test fixture ledger verification.

# Fixture states

- Existing `L70-delegated-policy-valid-project-policy-violated` covers:
  - public re-export many-use warning
  - public re-export too-many-use error
  - crate-level `unused_crate_dependencies` allow inventory
  - inline-module `unused_crate_dependencies` allow inventory
  - other crate-level allow non-hit
- Existing `L70-workspace-package-policy-violated` covers:
  - unauthorized build dependency while dependency policy is present
- New `L60-deny-missing-sections-policy-invalid` covers:
  - missing `[advisories]`
  - missing `[graph]`
  - missing `[bans]` non-hit inventory for bans rules
- New `L60-deny-wrong-values-policy-invalid` covers:
  - wrong advisory baseline values
  - wrong graph `all-features`
  - wrong graph `no-default-features`
- New `L60-deny-nonstricter-values-policy-invalid` covers:
  - valid but non-stricter advisory inventory cases
- Existing `L60-deny-deprecated-advisories-policy-invalid` covers:
  - deprecated advisory fields
- Existing `L80-project-policy-valid-clean` covers:
  - clean deny baseline cases
- New `L60-fmt-package-edition-fallback-policy-invalid` covers:
  - package edition fallback when workspace package edition is absent
  - empty `skip_macro_invocations` inventory
- New `L60-fmt-workspace-edition-precedence-policy-invalid` covers:
  - workspace package edition precedence over package edition
  - nonempty `skip_macro_invocations` inventory
- New `R18-hooks-path-qualified-safe-comments` covers:
  - path-qualified required tools
  - concrete lockfile commands
  - escaped hash comment
  - escaped space before hash comment
  - quoted hash
  - no real bypass comment
- New `L38-topology-non-root-nested-context` covers:
  - nested workspace context under a non-root path
  - nested `guardrail3-rs.toml` context under a non-root path
  - undeclared child member under the nested workspace
  - escaping member path under the nested workspace
  - workspace-local `rustfmt.toml` placement under the nested workspace

# Files to modify

- `.plans/2026-05-15-133355-g3rs-unclassified-fixture-coverage.md.manifest.toml`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `behavior/fixtures/g3rs/**`
- `behavior/fixtures/g3rs-validate-repo/**`
- `behavior/golden/g3rs-validate/approved.normalized.json`
- `behavior/golden/g3rs-validate-repo/approved.normalized.json`
- `scripts/behavior/classify-test-fixture-ledger.py`
- `scripts/behavior/verify-unclassified-fixture-coverage.py`
- `scripts/behavior/verify-all.sh`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
