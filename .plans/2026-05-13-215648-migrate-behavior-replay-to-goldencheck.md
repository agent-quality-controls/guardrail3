# Migrate Guardrail3 Behavior Replay To Goldencheck

## Goal

Use `goldencheck` for generic golden-output mechanics.

Keep guardrail3-specific fixture semantics in guardrail3.

End state:

- `goldencheck.yaml` owns suite execution, approved output storage, received output storage, diff storage, and approval workflow.
- guardrail3 keeps fixture design, fixture manifests, coverage matrix, shadowing rules, and CLI-output normalization.
- guardrail3 stops maintaining its own baseline file writer and baseline drift comparator.
- existing replay coverage and fixture minimality checks stay intact.

## Current State

Current behavior replay code lives under:

- `scripts/behavior/baseline_common.py`
- `scripts/behavior/generate-baselines.py`
- `scripts/behavior/verify-baselines.py`
- `scripts/behavior/verify-fixtures.py`
- `scripts/behavior/verify-compaction.py`
- `scripts/behavior/verify-rule-coverage.py`
- `scripts/behavior/verify-ledger.py`
- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-all.sh`

Current replay data lives under:

- `behavior/fixtures/g3rs`
- `behavior/fixtures/g3rs-validate-repo`
- `behavior/baselines/g3rs`
- `behavior/baselines/g3rs-validate-repo`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `behavior/migration/g3rs-test-ledger.toml`

Current fixture manifests live under:

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`

Installed `goldencheck` is `0.1.4`.

Relevant `goldencheck` constraints:

- `goldencheck check --all` runs every suite in `goldencheck.yaml`.
- Fixture expansion is done through `{fixtures}`.
- Command stdout must become JSON.
- `approved_dir/approved.normalized.json` is the committed oracle.
- `.goldencheck/<suite>/diff.txt` and `.goldencheck/<suite>/diff.json` are generated review artifacts.
- `goldencheck approve --suite <suite> --change <path>` records intentional drift in `approved.meta.json`.

## Boundary Decision

`goldencheck` owns:

- fixture glob expansion
- running one suite command
- optional normalizer invocation
- JSON pretty-print normalization
- approved output location
- received output location
- diff output
- approval command
- metadata hashes for fixture, manifest, and normalizer drift
- exit codes for match, diff, and runtime errors

guardrail3 owns:

- which fixture levels exist
- which fixture splits are allowed
- which fixtures must be minimal
- which `g3rs` public commands each fixture runs
- how `g3rs` output is converted to stable JSON
- rule coverage matrix
- closed Error/Warn finding policy
- required Info inventory rows
- test-ledger migration policy
- pre-commit fixture routing exclusions
- fixture compaction rules

Do not move fixture-layer design into `goldencheck`.

Do not teach `goldencheck` what `g3rs validate` means.

Do not make `goldencheck` parse guardrail result text.

## Suite Shape

Create repo-root file:

```text
goldencheck.yaml
```

Initial suites:

```yaml
version: 1
suites:
  g3rs-validate:
    fixtures:
      - "behavior/fixtures/g3rs/*/fixture.toml"
    command:
      argv:
        - "python3"
        - "scripts/behavior/goldencheck-g3rs-replay.py"
        - "--manifest"
        - ".plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml"
        - "{fixtures}"
      ok_exit_codes:
        - 0
    output:
      format: "json"
    storage:
      approved_dir: "behavior/golden/g3rs-validate"
      received_dir: ".goldencheck/g3rs-validate"
      diff_dir: ".goldencheck/g3rs-validate"

  g3rs-validate-repo:
    fixtures:
      - "behavior/fixtures/g3rs-validate-repo/*/fixture.toml"
    command:
      argv:
        - "python3"
        - "scripts/behavior/goldencheck-g3rs-replay.py"
        - "--manifest"
        - ".plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml"
        - "{fixtures}"
      ok_exit_codes:
        - 0
    output:
      format: "json"
    storage:
      approved_dir: "behavior/golden/g3rs-validate-repo"
      received_dir: ".goldencheck/g3rs-validate-repo"
      diff_dir: ".goldencheck/g3rs-validate-repo"
```

`scripts/behavior/verify-all.sh` must call:

```sh
goldencheck check --all
```

## Replay Harness

Create:

```text
scripts/behavior/goldencheck-g3rs-replay.py
```

Purpose:

- Read one fixture manifest.
- Receive one or more fixture metadata paths from `goldencheck`.
- For each fixture, run every command listed in that fixture's `fixture.toml`.
- Emit one stable JSON object to stdout.
- Exit `0` if replay generation succeeded, even when the wrapped `g3rs` command exits nonzero.
- Exit nonzero only when the harness cannot generate replay output.

Required output shape:

```json
{
  "schema_version": "g3rs-replay-v1",
  "tool": "g3rs",
  "manifest": ".plans/...",
  "records": [
    {
      "fixture_id": "L30-guardrail-config-valid-required-inputs-missing",
      "fixture_hash": "sha256:...",
      "command_index": 0,
      "command": ["g3rs", "validate", "--path", "repo", "--inventory"],
      "cwd": "repo",
      "exit_code": 1,
      "stdout": "...",
      "stderr": "..."
    }
  ]
}
```

The harness must preserve behavior-sensitive fields:

- fixture ID
- fixture hash
- command index
- command argv
- cwd
- wrapped command exit code
- normalized stdout
- normalized stderr

The harness must not include:

- current git commit
- wall-clock time
- generated timestamp
- absolute temp paths
- absolute repo paths

Reason:

- `goldencheck` already stores approval metadata.
- Including commit and timestamps in approved output creates avoidable churn.

## Code To Reuse

Move these functions out of `baseline_common.py` into the new harness or a harness-local module:

- `load_toml`
- `load_fixture_metadata`
- `command_cwd`
- `g3rs_candidate_binary`
- `tool_executable`
- `normalize_output`
- `fixture_hash`
- `run_command`
- `cargo_subcommand_blocker_path`
- `rust_toolchain_path_without_delegated_tools`
- `prepare_runtime_fixture`
- `copy_shared_fixture_inputs`
- `fixture_copy_ignore`

Delete or retire these baseline-specific functions after migration:

- `baseline_path`
- `read_json`
- `write_json`
- `git_head`

Delete or retire these scripts after migration:

- `scripts/behavior/generate-baselines.py`
- `scripts/behavior/verify-baselines.py`

Do not delete these scripts in the same step:

- `scripts/behavior/verify-fixtures.py`
- `scripts/behavior/verify-compaction.py`
- `scripts/behavior/verify-rule-coverage.py`
- `scripts/behavior/verify-ledger.py`
- `scripts/behavior/verify-test-deletion.py`

They enforce guardrail3-specific semantics that `goldencheck` does not own.

## Baseline Migration

Old committed baseline roots:

- `behavior/baselines/g3rs`
- `behavior/baselines/g3rs-validate-repo`

New committed golden roots:

- `behavior/golden/g3rs-validate/approved.normalized.json`
- `behavior/golden/g3rs-validate/approved.meta.json`
- `behavior/golden/g3rs-validate-repo/approved.normalized.json`
- `behavior/golden/g3rs-validate-repo/approved.meta.json`

Migration procedure:

1. Add `goldencheck.yaml`.
2. Add `scripts/behavior/goldencheck-g3rs-replay.py`.
3. Run `goldencheck check --suite g3rs-validate`.
4. Approve with `goldencheck approve --suite g3rs-validate --change .plans/2026-05-13-215648-migrate-behavior-replay-to-goldencheck.md`.
5. Run `goldencheck check --suite g3rs-validate-repo`.
6. Approve with `goldencheck approve --suite g3rs-validate-repo --change .plans/2026-05-13-215648-migrate-behavior-replay-to-goldencheck.md`.
7. Delete old `behavior/baselines/g3rs`.
8. Delete old `behavior/baselines/g3rs-validate-repo`.
9. Delete old baseline generation and baseline verification scripts.
10. Update coverage and fixture verifiers to read `behavior/golden/.../approved.normalized.json`.

Do not keep both baseline systems.

Do not add compatibility mode for old `behavior/baselines`.

## Updating Existing Verifiers

### `verify-rule-coverage.py`

Current behavior:

- reads per-command JSON files under `behavior/baselines/g3rs`
- reads per-command JSON files under `behavior/baselines/g3rs-validate-repo`

Required behavior:

- read `behavior/golden/g3rs-validate/approved.normalized.json`
- read `behavior/golden/g3rs-validate-repo/approved.normalized.json`
- iterate `records`
- parse `stdout` from each record exactly as today

### `verify-fixtures.py`

Keep current responsibility:

- fixture directory set
- fixture metadata shape
- closed file list
- pre-commit excludes `behavior/fixtures/`

Add one check:

- every manifest fixture with `baseline_required = true` has at least one matching record in the relevant approved golden output.

### `verify-compaction.py`

Keep current responsibility:

- fixture split/minimality rules
- no hidden mergeable fixtures
- no copied full trees in lower layers unless explicitly allowed

No `goldencheck` dependency belongs here.

### `verify-ledger.py`

Keep current responsibility:

- old test migration ledger status
- no behavior rows left unmigrated after migration stages

No `goldencheck` dependency belongs here unless it checks that migrated behavior has a matching approved record.

### `verify-test-deletion.py`

Keep current responsibility:

- deleted test files stay deleted
- kept tests are only allowed categories

No `goldencheck` dependency belongs here.

### `verify-all.sh`

Required order:

```sh
python3 "$HERE/verify-fixtures.py"
python3 "$HERE/verify-compaction.py"
goldencheck check --all
python3 "$HERE/verify-rule-coverage.py"
python3 "$HERE/verify-ledger.py"
python3 "$HERE/verify-test-deletion.py"
```

Reason:

- fixture structure must be valid before running replay.
- `goldencheck` must prove approved behavior before coverage verifiers inspect approved output.
- coverage and ledger verifiers depend on approved output, not received output.

## Output Normalization Policy

Keep these normalizations from current `baseline_common.py`:

- fixture repo path to `$REPO`
- fixture path to `$FIXTURE`
- fixture root path to `$FIXTURE_ROOT`
- target directory path to `$TARGET`
- path separators to `/`
- macOS `/private` temp prefix cleanup
- cargo package-cache lock wait line removal
- cargo timing replacement
- rust test timing replacement
- Rust test binary hash replacement

Remove these from approved output:

- `baseline_commit`
- `created_at`
- `runner_version`
- `normalizer_version`
- `output_schema_version`

Replace them with:

- `schema_version = "g3rs-replay-v1"` inside harness output
- `fixture_hash` per record

Reason:

- `goldencheck` owns approval metadata.
- harness schema version is behavior-relevant because it changes approved JSON shape.

## Fixture Hash Policy

Keep `fixture_hash` in each record.

Reason:

- `goldencheck` hashes fixture paths passed through its manifest, but it sees `fixture.toml` files as fixtures.
- guardrail3 fixture behavior depends on the entire fixture directory, not only `fixture.toml`.
- the harness-level `fixture_hash` catches hidden fixture directory changes.

## Approval Policy

All behavior drift must be approved through:

```sh
goldencheck approve --suite <suite> --change <path>
```

The `<path>` must point to a plan, worklog, issue, or PR.

Do not add a custom guardrail3 approval command.

Do not write approved JSON manually.

## Removed Responsibilities

Delete from guardrail3:

- custom approved-vs-actual JSON comparison
- custom per-command baseline file naming
- custom baseline write command
- custom baseline commit metadata
- custom baseline timestamp validation
- custom baseline drift message

Keep in guardrail3:

- fixture policy checks
- output normalization
- wrapped command execution details
- coverage matrix verification
- ledger verification
- test deletion verification

## Implementation Stages

### Stage 1: Harness And Manifest

Deliverables:

- `goldencheck.yaml`
- `scripts/behavior/goldencheck-g3rs-replay.py`
- updated `scripts/behavior/verify-all.sh` that runs both `goldencheck` suites

Verification:

```sh
python3 -m py_compile scripts/behavior/*.py
goldencheck check --suite g3rs-validate
goldencheck check --suite g3rs-validate-repo
```

Expected result before approval:

- `goldencheck check` exits `2` if approved output is missing.
- after approval, both checks exit `0`.

### Stage 2: Approve New Golden Output

Deliverables:

- `behavior/golden/g3rs-validate/approved.normalized.json`
- `behavior/golden/g3rs-validate/approved.meta.json`
- `behavior/golden/g3rs-validate-repo/approved.normalized.json`
- `behavior/golden/g3rs-validate-repo/approved.meta.json`

Verification:

```sh
goldencheck check --suite g3rs-validate
goldencheck check --suite g3rs-validate-repo
```

Expected result:

- both commands exit `0`

### Stage 3: Move Verifiers To Golden Output

Deliverables:

- `scripts/behavior/verify-rule-coverage.py` reads approved golden output
- `scripts/behavior/verify-fixtures.py` verifies each baseline-required fixture has approved records
- `scripts/behavior/verify-all.sh` no longer calls `verify-baselines.py`

Verification:

```sh
scripts/behavior/verify-all.sh
```

Expected result:

- all existing fixture, compaction, coverage, ledger, and deletion checks pass against golden output

### Stage 4: Remove Old Baseline System

Deliverables:

- delete `behavior/baselines/g3rs`
- delete `behavior/baselines/g3rs-validate-repo`
- delete `scripts/behavior/generate-baselines.py`
- delete `scripts/behavior/verify-baselines.py`
- remove old baseline helper code that is no longer imported

Verification:

```sh
rg "behavior/baselines|generate-baselines|verify-baselines" behavior scripts .plans .worklogs
scripts/behavior/verify-all.sh
```

Expected result:

- no active runtime reference to old baseline paths
- worklogs may still mention old paths
- `verify-all.sh` exits `0`

### Stage 5: Full Guardrail Verification

Verification:

```sh
python3 -m py_compile scripts/behavior/*.py
scripts/behavior/verify-all.sh
g3rs validate --path apps/guardrail3-rs --inventory
g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
git diff --check
```

Expected result:

- all commands exit `0`

## Manifest Requirements For This Migration

Create:

```text
.plans/2026-05-13-215648-migrate-behavior-replay-to-goldencheck.md.manifest.toml
```

Minimum manifest rows:

- `goldencheck.yaml` exists
- `goldencheck.yaml` has suites `g3rs-validate` and `g3rs-validate-repo`
- both suites use `scripts/behavior/goldencheck-g3rs-replay.py`
- both suites use `output.format = "json"`
- both suites store approved output under `behavior/golden`
- `scripts/behavior/verify-all.sh` calls `goldencheck check --all`
- old `behavior/baselines/g3rs` is absent
- old `behavior/baselines/g3rs-validate-repo` is absent
- `scripts/behavior/generate-baselines.py` is absent
- `scripts/behavior/verify-baselines.py` is absent
- `verify-rule-coverage.py` reads `behavior/golden`
- `verify-fixtures.py` checks approved records for every baseline-required fixture

## Adversarial Review Requirements

Reviewer A:

- Compare this plan to the implementation.
- Verify `goldencheck` owns only generic golden mechanics.
- Verify guardrail3 still owns fixture semantics and coverage semantics.

Reviewer B:

- Verify no old baseline system remains active.
- Verify no compatibility path allows `behavior/baselines` to stay in use.

Reviewer C:

- Verify replay output is stable.
- Verify approved output has no timestamps, commits, absolute paths, or volatile cargo/test timing.

Reviewer D:

- Verify every fixture previously covered by `behavior/baselines` appears in the new approved golden output.
- Verify every rule coverage verifier still sees all rule IDs it saw before migration.

No stage is done until all reviewers report no `MUST FIX`.
