# Family Rule CLI Fixtures

## Goal

Replace remaining rule-level unit tests with minimized family-scoped CLI fixtures.

The fixture design is:

```text
behavior/fixtures/g3rs-rules/<family>/<fixture-id>/fixture.toml
behavior/fixtures/g3rs-rules/<family>/<fixture-id>/repo/...
```

Each fixture is a normal `g3rs validate` fixture. It tests the external CLI surface only:

- command
- exit code
- stdout
- stderr
- findings
- inventory output

It does not serialize ingestion structs, rule input structs, helper return values, or internal crate boundaries.

## Current State

Current client-facing fixture suites:

- `g3rs-validate`
- `g3rs-validate-repo`
- `g3rs-cli-output`
- `g3rs-report-output`

Current `behavior/fixtures/g3rs/*` fixtures are broad layered/composite fixtures.

They were designed to expose many guardrail layers at once. That is useful for early CLI replay coverage, but it is not the final rule-coverage corpus because the fixtures are not minimized around one family and their intended rule coverage is not explicit.

Current rule rows not yet fixture-replaced:

- total: 236
- `apparch/g3rs-apparch-config-checks`: 36
- `apparch/g3rs-apparch-source-checks`: 8
- `arch/g3rs-arch-config-checks`: 16
- `arch/g3rs-arch-file-tree-checks`: 7
- `arch/g3rs-arch-source-checks`: 8
- `cargo/g3rs-cargo-config-checks`: 42
- `clippy/g3rs-clippy-config-checks`: 39
- `clippy/g3rs-clippy-filetree-checks`: 4
- `deny/g3rs-deny-config-checks`: 36
- `deny/g3rs-deny-filetree-checks`: 5
- `deps/g3rs-deps-filetree-checks`: 3
- `fmt/g3rs-fmt-filetree-checks`: 1
- `hooks/g3rs-hooks-config-checks`: 2
- `release/g3rs-release-config-checks`: 9
- `release/g3rs-release-filetree-checks`: 11
- `release/g3rs-release-source-checks`: 4
- `toolchain/g3rs-toolchain-filetree-checks`: 5

Current internal rows that stay as unit tests for now:

- `keep_internal_unit_test`: 421

Those are not part of this plan.

## Target Fixture Split

Final fixture ownership:

- `behavior/fixtures/g3rs-global/*`
  - global CLI/adoption/gating states
  - examples: workspace root missing, guardrail config missing, guardrail config invalid, required inputs missing, clean baseline
- `behavior/fixtures/g3rs-rules/<family>/*`
  - minimized rule fixtures for one family
  - each fixture triggers the maximum compatible set of rules inside that family
- `behavior/fixtures/g3rs-validate-repo/*`
  - repo-level adoption and hook behavior
- `behavior/fixtures/g3rs-cli-output/*`
  - command shape, help, rejected arguments, init behavior
- `behavior/fixtures/g3rs-report-output/*`
  - report renderer behavior

Migration rule:

- Existing `behavior/fixtures/g3rs/*` remains temporarily as a broad safety corpus.
- Do not treat broad `g3rs/*` fixtures as the final rule coverage design.
- For each completed family, move rule coverage into `g3rs-rules/<family>`.
- After a family is covered by minimized family-rule fixtures, remove or reduce old broad fixtures whose only remaining purpose was that family's rule coverage.
- Keep only truly global fixtures in the global corpus.

## Fixture Folder Model

Use one folder per rule family:

```text
behavior/fixtures/g3rs-rules/apparch/
behavior/fixtures/g3rs-rules/arch/
behavior/fixtures/g3rs-rules/cargo/
behavior/fixtures/g3rs-rules/clippy/
behavior/fixtures/g3rs-rules/deny/
behavior/fixtures/g3rs-rules/deps/
behavior/fixtures/g3rs-rules/fmt/
behavior/fixtures/g3rs-rules/hooks/
behavior/fixtures/g3rs-rules/release/
behavior/fixtures/g3rs-rules/toolchain/
```

Do not create folders for families with no `needs_rule_fixture_or_golden_output` rows.

Use globally unique fixture IDs because the existing replay harness uses the parent directory name as `fixture_id`.

Fixture ID format:

```text
<family>-R<number>-<short-purpose>
```

Examples:

```text
behavior/fixtures/g3rs-rules/cargo/cargo-R10-workspace-policy/fixture.toml
behavior/fixtures/g3rs-rules/clippy/clippy-R10-managed-config/fixture.toml
behavior/fixtures/g3rs-rules/deny/deny-R10-policy-values/fixture.toml
```

## Fixture Metadata Contract

Each family-rule fixture must define:

```toml
id = "cargo-R10-workspace-policy"
tool = "g3rs"
run_from = "repo"
commands = [
  ["validate", "workspace", "--path", ".", "--family", "cargo", "--rules-only", "--inventory"],
]
expected_exit = "nonzero"
level = "family_rule_policy"

rule_family = "cargo"
target_rules = [
  "g3rs-cargo/workspace-lints",
  "g3rs-cargo/lint-levels",
]

intentionally_invalid = [
  "cargo_workspace_lints_missing",
  "cargo_lint_level_weakened",
]

expected_findings = [
  "g3rs-cargo/workspace-lints",
  "g3rs-cargo/lint-levels",
]
```

Required fields:

- `rule_family`
- `target_rules`
- `expected_findings`

Optional fields:

- `expected_absent_findings`
- `known_overlap_rules`

`target_rules` is the intent inventory. `expected_findings` is the CLI-observed proof.

## Grouping Rule

Create the minimal number of fixtures per family, not one fixture per rule.

Merge rules into the same fixture when:

- the same repo state can trigger all target findings
- no target finding prevents another target finding from being reached
- the expected output remains reviewable
- the fixture does not rely on a parse failure that stops later rule evaluation

Split fixtures when:

- malformed config prevents other rules from reading the config
- missing file prevents rules that require the file contents
- a fail-closed input error stops the family lane
- a clean non-hit assertion would be hidden by deliberate invalid state
- the fixture output becomes too noisy to identify the target rule regressions

## Suite Wiring

Add the family-rule fixtures to the existing `g3rs-validate` suite while the migration is in progress.

`fixture3.yaml` should contain both:

```yaml
fixtures:
  - "behavior/fixtures/g3rs/*/fixture.toml"
  - "behavior/fixtures/g3rs-rules/*/*/fixture.toml"
```

Do not create one suite per family unless fixture3 output becomes too large to review. The product boundary is still the same CLI command.

Final `g3rs-validate` fixture input should be:

```yaml
fixtures:
  - "behavior/fixtures/g3rs-global/*/fixture.toml"
  - "behavior/fixtures/g3rs-rules/*/*/fixture.toml"
```

The old glob is transitional:

```yaml
fixtures:
  - "behavior/fixtures/g3rs/*/fixture.toml"
```

Remove it after global fixtures are renamed/reduced and family-rule coverage has replaced the broad composite rule fixtures.

## Coverage Verification

Add a verifier:

```text
scripts/behavior/verify-family-rule-fixtures.py
```

The verifier must:

1. Read `behavior/migration/g3rs-kept-test-disposition.toml`.
2. Select rows with `disposition = "needs_rule_fixture_or_golden_output"`.
3. Extract each row's semantic rule name from the test path or sidecar rule file.
4. Read all `behavior/fixtures/g3rs-rules/*/*/fixture.toml`.
5. Build the set of `target_rules`.
6. Build the set of `expected_findings`.
7. Fail if any target rule is missing from `expected_findings`.
8. Fail if any `needs_rule_fixture_or_golden_output` rule has no fixture target.
9. Fail if a fixture declares a target rule that does not exist in the rule inventory.
10. Print exact missing and extra rules.

This verifier does not decide whether a fixture's contents are minimal. It verifies declared coverage.

During incremental migration, the verifier must fail on malformed family-rule fixture metadata and unknown rule IDs for every fixture that exists. It must not fail just because an unimplemented family has no `g3rs-rules/<family>` folder yet.

For test-disposition coverage, the verifier must only require rows for families listed as completed in the manifest. A family is completed only after its fixture set covers all CLI-exposable rows for that family and non-CLI-exposable rows have been reclassified to `keep_internal_unit_test`.

## Ledger Update Rule

When a family-rule fixture proves a row:

- update the row disposition from `needs_rule_fixture_or_golden_output` to `covered_by_cli_output`
- update the reason to name the exact fixture ID
- delete the corresponding unit test only after `verify-test-deletion.py` passes

Do not mark a row covered just because a fixture targets the rule. It must be present in approved CLI output.

## Implementation Order

Use this order:

1. `cargo`
   - 42 rows
   - highest count
   - mostly config-policy findings
   - implemented with 3 fixtures:
     - `cargo-R00-clean-golden`
     - `cargo-R10-policy-violations`
     - `cargo-R21-root-metadata-missing`
   - 26 rows are covered by CLI output
   - 16 rows stay as internal unit tests because the CLI cannot expose them as cargo-rule findings without first failing Cargo TOML or guardrail config ingestion
2. `clippy`
   - 43 rows
   - validates dense config fixtures and file-tree fixture edge cases
3. `deny`
   - 41 rows
   - validates policy variants and file-tree coverage
4. `apparch`
   - 44 rows
   - validates config and source checks
5. `arch`
   - 31 rows
   - validates config, file-tree, and source checks
6. `release`
   - 24 rows
   - validates config, file-tree, source checks
7. `toolchain`
   - 5 rows
8. `deps`
   - 3 rows
9. `hooks`
   - 2 rows
10. `fmt`
    - 1 row

## First Family Target: Cargo

Start with `cargo` because it has the most remaining rule rows and mostly exercises config findings through the CLI.

Process:

1. Read every `needs_rule_fixture_or_golden_output` row under `packages/rs/cargo/g3rs-cargo-config-checks`.
2. Map each row to its semantic rule ID.
3. Group rules by required repo state.
4. Create the smallest set of `behavior/fixtures/g3rs-rules/cargo/*` fixtures that covers all cargo target rules.
5. Add those fixtures to `fixture3.yaml`.
6. Run `fixture3 check --suite g3rs-validate`.
7. Approve output only after verifying target findings appear.
8. Update the ledger for covered cargo rows.
9. Delete cargo unit tests only after the deletion verifier allows it.
10. Identify any old `behavior/fixtures/g3rs/*` fixtures whose only remaining purpose was cargo rule coverage and either remove them or reduce them to global behavior only.

## Verification Commands

After adding or changing fixtures:

```bash
fixture3 check --suite g3rs-validate
python3 scripts/behavior/verify-family-rule-fixtures.py
python3 scripts/behavior/verify-test-deletion.py
bash scripts/behavior/verify-all.sh
g3rs validate repo --path "$PWD"
git diff --check
```

If a family package changes:

```bash
g3rs validate workspace --path packages/rs/<family>/<package> --inventory
cargo test --manifest-path packages/rs/<family>/<package>/Cargo.toml --workspace
cargo clippy --manifest-path packages/rs/<family>/<package>/Cargo.toml --workspace --all-targets --all-features
```

## Non-Goals

Do not:

- add serialized ingestion fixtures
- add rule-input JSON snapshots
- test private helper outputs
- preserve unit tests only because they existed
- create one fixture per rule when rules can be grouped without hiding output
- create family-specific fixture3 suites unless one shared `g3rs-validate` suite becomes unreviewable
