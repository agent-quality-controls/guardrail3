# G3RS L80 And Test Ledger Next Stage

## Goal

Move behavior replay from "fixtures exist" to "fixtures can replace old tests safely".

The next implementation stage has two deliverables:

- harden `L80-project-policy-valid-clean` so it is a strong clean replay fixture
- add the temporary migration ledger and verifiers needed before deleting old behavior tests

This stage must not delete old tests until the ledger and deletion verifier can prove the deletion is intentional.

## Current State

- `ab1d95082` pushed `main` with 22 G3RS behavior fixtures.
- `scripts/behavior/verify-all.sh` passes.
- L70 has 6 fixtures:
  - `L70-delegated-policy-valid-project-policy-violated`
  - `L70-workspace-package-policy-violated`
  - `L70-apparch-policy-violated`
  - `L70-garde-boundary-policy-violated`
  - `L70-release-metadata-policy-violated`
  - `L70-release-invalid-semver-policy-violated`
- `L80-project-policy-valid-clean` exists and validates clean.
- `L80-project-policy-valid-clean/repo` has 300 files copied from `packages/rs/deny/g3rs-deny-config-checks`.
- `behavior/migration/` does not exist.
- `verify-all.sh` does not run `verify-ledger.py` or `verify-test-deletion.py`.
- First migration group from the accepted plan is `apps/guardrail3-rs CLI behavior`.
- `apps/guardrail3-rs` currently has 16 test files matching sidecar/integration test patterns.

## Key Decisions

### L80 Is A Clean Realistic Fixture, Not Another Failure Fixture

Do not create more L70-like failure rows for L80.

L80 must prove:

- realistic clean package validates with exit code 0
- clean fixture has no hidden target/build output
- clean fixture remains based on the selected source package until a better clean source is explicitly planned
- clean fixture can be used as a copied workspace baseline for behavior replay

### Add Ledger Infrastructure Before Deleting Tests

Do not delete old tests in the same change that first creates the ledger verifier.

First create:

- `behavior/migration/g3rs-test-ledger.toml`
- `scripts/behavior/verify-ledger.py`
- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-all.sh` wiring for both scripts

Then verify that the ledger can fail closed on:

- old test files missing from the ledger
- invalid `kind`
- invalid `status`
- behavior rows with non-terminal status in a migrated package group
- deleted test files not marked as migrated/deleted
- rows pointing at fixtures that do not exist

### First Ledger Scope Is Apps CLI Only

Start with `apps/guardrail3-rs`, because the migration order says CLI behavior first.

Do not include all 593 currently matched early-order test files in the first ledger change.

The first ledger scope is exactly:

- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli_tests/cases.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli_tests/mod.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run_tests/mod.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text_tests/cases.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text_tests/mod.rs`
- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run_tests/cases.rs`
- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run_tests/mod.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates_tests/cases.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates_tests/mod.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/execute_tests/cases.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/execute_tests/mod.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/marker_pairs_tests/cases.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/marker_pairs_tests/mod.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/selection_tests/cases.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/selection_tests/mod.rs`

### Classification Must Be Conservative

Use these ledger classes:

- `behavior`: CLI-visible output that should be represented by replay fixtures before deletion
- `replay_system`: tests for behavior replay scripts, baseline normalization, fixture metadata, or verifier correctness
- `compile_contract`: public API, trait, or type-shape tests where compiler behavior is the contract
- `private_implementation_only`: tests of private helpers where no public behavior is supposed to be preserved
- `obsolete`: tests for removed behavior or retired compatibility

For the first pass, mark unknown rows as `unclassified`.

Do not guess terminal statuses for test files that have not been audited.

### Use Manifest-Driven Verification

The ledger verifier must not rely on agent summaries.

It must inspect:

- real files under configured migrated package groups
- real ledger rows
- real fixture ids from the behavior manifest
- real deleted/missing files on disk

The verifier output is the implementation status.

## Implementation Plan

### 1. Harden L80 Manifest Contract

Modify:

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `scripts/behavior/verify-fixtures.py`

Changes:

- Add an explicit `fixture_kind = "clean"` or equivalent field to `L80-project-policy-valid-clean`.
- Add explicit L80 checks in `verify-fixtures.py`:
  - `expected_exit = "zero"`
  - `baseline_required = true`
  - `forbidden_paths` includes `repo/target`
  - `fixture.toml` exists
  - `repo/Cargo.toml` exists
  - `repo/guardrail3-rs.toml` exists
  - `repo/Cargo.lock` exists
  - no file under `repo/target`
- Keep `closed_file_list = false` for L80 because the clean fixture is intentionally a full copied workspace.

Do not add source behavior rows to L80.

### 2. Create Ledger File

Create:

- `behavior/migration/g3rs-test-ledger.toml`

Initial ledger shape:

```toml
[[group]]
id = "apps-guardrail3-rs-cli"
root = "apps/guardrail3-rs"
status = "active"

[[test]]
old_test_path = "apps/guardrail3-rs/..."
kind = "unclassified"
status = "unclassified"
fixture = ""
reason = "initial inventory row; not audited yet"
```

Every test file in the first ledger scope must have one row.

Do not classify by test function yet unless the implementation can extract Rust test functions reliably. File-level rows are acceptable for the first ledger verifier because the next deletion gate works at file granularity.

### 3. Add `verify-ledger.py`

Create:

- `scripts/behavior/verify-ledger.py`

The script must:

- parse TOML with `tomllib`
- read `behavior/migration/g3rs-test-ledger.toml`
- read the behavior fixture manifest
- validate allowed `kind` values
- validate allowed `status` values
- validate every `fixture` value when non-empty
- validate every `old_test_path` is repo-relative
- validate every existing first-scope test file has exactly one ledger row
- reject ledger rows for files outside declared active groups unless explicitly allowed
- reject terminal statuses that do not match kind:
  - `behavior` -> `migrated_deleted`
  - `private_implementation_only` -> `deleted_private_implementation`
  - `obsolete` -> `deleted_obsolete`
  - `replay_system` -> `kept_replay_system`
  - `compile_contract` -> `kept_compile_contract`
- allow `unclassified` only before the group is marked complete

Output:

- success: `behavior-ledger: PASS groups:<n> tests:<n>`
- failure: `behavior-ledger: FAIL` plus one concrete line per failure

### 4. Add `verify-test-deletion.py`

Create:

- `scripts/behavior/verify-test-deletion.py`

The script must:

- parse the same ledger
- inspect real filesystem state
- for each ledger row:
  - if `status` starts with `deleted_`, `old_test_path` must not exist
  - if `status` starts with `kept_`, `old_test_path` must exist
  - if `status = "unclassified"`, `old_test_path` must exist
- reject deleted files with non-deleted statuses
- reject existing files with deleted statuses

Output:

- success: `behavior-test-deletion: PASS rows:<n>`
- failure: `behavior-test-deletion: FAIL` plus one concrete line per failure

### 5. Wire `verify-all.sh`

Modify:

- `scripts/behavior/verify-all.sh`

Add:

```sh
python3 "$HERE/verify-ledger.py"
python3 "$HERE/verify-test-deletion.py"
```

Run them after fixture/baseline verification, because ledger validation depends on fixture ids from the accepted fixture manifest.

### 6. Add Negative Controls For Verifier Logic

Do not add a full separate test framework.

Add deterministic self-checks inside the verifier scripts or add small fixture snippets under `behavior/migration/_invalid_examples` only if needed.

Required failure modes to prove manually during implementation:

- duplicate ledger row for same `old_test_path`
- invalid kind
- invalid status
- missing fixture id
- deleted file marked as kept
- existing file marked as deleted

The implementation worklog must list the commands used to prove these failures.

### 7. Run Verification

Required commands:

```sh
python3 -m py_compile scripts/behavior/*.py
scripts/behavior/verify-all.sh
g3rs validate --path apps/guardrail3-rs --inventory
g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
git diff --check
```

Expected:

- all commands exit 0
- `scripts/behavior/verify-all.sh` prints:
  - `behavior-fixtures: PASS fixtures:22`
  - `behavior-baselines: PASS records:41`
  - `behavior-compaction: PASS kept:5 removed:20`
  - `behavior-fixtures: PASS fixtures:6`
  - `behavior-baselines: PASS records:7`
  - `behavior-ledger: PASS ...`
  - `behavior-test-deletion: PASS ...`

## Adversarial Review Requirements

Send reviewers after implementation.

Reviewer A:

- reads this plan
- reads `behavior/migration/g3rs-test-ledger.toml`
- reads `scripts/behavior/verify-ledger.py`
- reads `scripts/behavior/verify-test-deletion.py`
- verifies the first-scope test file inventory is complete and exact

Reviewer B:

- reads this plan
- reads `scripts/behavior/verify-fixtures.py`
- reads the fixture manifest
- verifies L80 checks are meaningful and do not make L80 a closed copied-source tree

Reviewer C:

- tries to break the ledger verifier with duplicate rows, missing fixture ids, wrong statuses, and deleted/kept mismatches
- does not edit committed files
- reports the command or temporary mutation used

No implementation is done until all reviewers return no MUST FIX findings.

## Files To Modify

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/migration/g3rs-test-ledger.toml`
- `scripts/behavior/verify-ledger.py`
- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-all.sh`
- `scripts/behavior/verify-fixtures.py`

Optional only if verifier evidence requires it:

- `behavior/fixtures/g3rs/L80-project-policy-valid-clean/fixture.toml`

## Files Not To Modify In This Stage

- Do not delete old tests.
- Do not modify G3TS fixtures.
- Do not change rule implementation code.
- Do not add new L70 fixtures unless a verifier proves the existing L70 baselines cannot represent a required CLI-visible behavior.

