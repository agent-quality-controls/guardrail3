# G3RS L45 Source And Filetree Input Failure Fixtures

## Goal

Implement Stage 4 from `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`.

End state:

- one or more L45 fixtures replay source/filetree ingestion failures through the public `g3rs validate` boundary
- these four planned rule IDs become covered:
  - `g3rs-code/input-failures`
  - `g3rs-garde/input-failures`
  - `g3rs-test/source-input-failures`
  - `g3rs-test/filetree-input-failures`
- behavior fixture manifests pin every emitted `Error`, `Warn`, and intended `Info`
- `behavior/coverage/g3rs-rule-coverage.toml` moves those rows from `planned_fixture` to `covered`

## Layer

Fixture layer:

- `L45-source-and-filetree-input-failures`

This layer sits after:

- workspace root found
- guardrail config valid
- required root input files present
- required root input files parseable

This layer sits before:

- delegated tool checks
- delegated tool policy checks
- project policy checks

Reason:

- malformed Rust source can make `cargo check` fail
- this fixture must prove G3RS source/filetree input failure reporting, not delegated cargo gate behavior
- therefore the fixture command must use `--rules-only`

## Public Command

Use:

```sh
g3rs validate --path . --rules-only --inventory
```

Do not use a private rule harness.

Do not use family ingestion tests.

Do not use `cargo check`, `cargo test`, or delegated tools for this fixture.

## Fixture Shape

Start from the valid L80 shape unless implementation proves a smaller existing fixture is cleaner:

- `behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo`

Create:

- `behavior/fixtures/g3rs/L45-source-and-filetree-input-failures`

Fixture metadata:

- `id = "L45-source-and-filetree-input-failures"`
- `commands = [["validate", "--path", ".", "--rules-only", "--inventory"]]`
- `expected_exit = "nonzero"`
- `level = "required_inputs_valid_source_filetree_inputs_invalid"`
- `valid_state`:
  - `workspace_root_found`
  - `guardrail_config_valid`
  - `required_inputs_present`
  - `required_inputs_valid`
- `intentionally_invalid`:
  - `source_inputs_invalid`
  - `filetree_inputs_invalid`

## Required Mutations

Add malformed Rust files that are still readable files in the workspace.

Do not make root config files malformed.

Do not remove root required inputs.

Do not use unreadable file permissions unless adversarial review proves readable malformed files cannot trigger one of the four target rule IDs. File permissions are not stable in git fixtures.

### Code Input Failure

Target:

- `g3rs-code/input-failures`

Mutation:

- add `src/broken_code.rs`
- content: syntactically invalid Rust, for example `pub fn broken( {\n`

Expected finding:

- severity: `Error`
- title: `code-family input failure`
- path: `src/broken_code.rs`

Reason:

- `g3rs-code-ingestion` stores `syn::parse_file` failures as `G3RsCodeParsedSourceState::Invalid`
- `g3rs-code-source-checks` reports that through `g3rs-code/input-failures`

### Garde Input Failure

Target:

- `g3rs-garde/input-failures`

Mutation:

- same malformed `src/broken_code.rs` should be sufficient if garde is active
- keep `garde` dependency or `[checks].garde = true` exactly as needed by the copied valid fixture

Expected finding:

- severity: `Error`
- title: `garde-family input failure`
- path: `src/broken_code.rs`

Reason:

- `g3rs-garde-ingestion` parses Rust source through `syn`
- malformed source is recorded as an input failure and reported by `g3rs-garde/input-failures`

If this does not fire from `src/broken_code.rs`, first inspect garde activation in the copied fixture. Do not add malformed `guardrail3-rs.toml`, because that belongs to L20/L40 and would hide this layer.

### Test Source Input Failure

Target:

- `g3rs-test/source-input-failures`

Mutation:

- add `tests/broken_source.rs`
- content: syntactically invalid Rust, for example `#[test]\nfn broken( {\n`

Expected finding:

- severity: `Error`
- title: `failed to read test input`
- path: `tests/broken_source.rs`

Reason:

- test source ingestion parses test files for semantic test checks
- malformed test source is reported through `g3rs-test/source-input-failures`

### Test Filetree Input Failure

Target:

- `g3rs-test/filetree-input-failures`

Mutation:

- try to reuse `tests/broken_source.rs`
- if the same file emits both test source and filetree input failures, keep one fixture
- if one parse failure suppresses the other in the filetree lane, split into a second L45 fixture with a different malformed file path

Expected finding:

- severity: `Error`
- title: `failed to read test input`
- path: `tests/broken_source.rs` or the second fixture path

Reason:

- test filetree ingestion separately analyzes classified test files for ownership/shape checks
- malformed filetree source is reported through `g3rs-test/filetree-input-failures`

## Hiding Boundary

One fixture is allowed only if all four target IDs emit independently.

Split if any mutation does one of these:

- prevents another family from running
- changes root config validity
- causes a non-target L40 or earlier rule to emit
- makes delegated cargo gates relevant
- forces unreadable file permissions

Do not split just because each rule has a separate unit test.

## Manifest Updates

Update:

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`

Add the fixture with:

- `baseline_required = true`
- `closed_file_list = true`
- exact `files = [...]`
- `required_results = [...]` for every emitted `Error`, `Warn`, and intentional `Info`

The fixture must not emit unlisted `Error` or `Warn`.

If it emits intentional `Info` rows used for coverage, pin exact counted `Info|rule|title|path` rows.

## Baseline Updates

Generate:

- `behavior/baselines/g3rs/L45-source-and-filetree-input-failures/command-00.json`

Use the existing behavior baseline writer pattern.

Do not hand-edit stdout or stderr.

## Coverage Matrix Updates

Update:

- `behavior/coverage/g3rs-rule-coverage.toml`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md.manifest.toml`

Rows to move to covered:

- `g3rs-code/input-failures`
- `g3rs-garde/input-failures`
- `g3rs-test/source-input-failures`
- `g3rs-test/filetree-input-failures`

Expected row values:

- `coverage_status = "covered"`
- `current_replay = "error_or_warn"`
- `target_replay = "error_or_warn"`
- `fixture = "L45-source-and-filetree-input-failures"` unless split is required
- reason must state the concrete malformed readable Rust file that triggers the input failure

Expected count change if exactly these four absent rules become covered:

- `covered`: `225 -> 229`
- `planned`: `41 -> 37`
- `baseline_rule_ids`: `256 -> 260`
- `baseline_error_warn_rule_ids`: `209 -> 213`
- `absent_rule_ids`: `10 -> 6`

Do not update counts by hand before running `verify-rule-coverage.py`; if the actual numbers differ, inspect the emitted baselines before changing the manifest.

## Required Verification

Run:

```sh
python3 -m py_compile scripts/behavior/*.py
scripts/behavior/verify-all.sh
g3rs validate --path behavior/fixtures/g3rs/L45-source-and-filetree-input-failures/repo --rules-only --inventory
g3rs validate --path apps/guardrail3-rs --inventory
g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
git diff --check
```

If multiple L45 fixtures are created, run `g3rs validate` for each one.

## Adversarial Review

Reviewer A:

- compare fixture output against the four target rule IDs
- verify every target emits through the public CLI boundary
- verify no target is only covered by a private unit test

Reviewer B:

- attack fixture minimality
- verify one fixture does not hide another target
- verify split decisions are caused by actual hiding, not habit

Reviewer C:

- inspect manifest, baseline, and coverage matrix
- verify every new `Error`, `Warn`, and intended `Info` row is pinned
- verify `verify-rule-coverage.py` rejects stale counts and missing coverage rows

No implementation stage is done until adversarial review returns no `MUST FIX`.
