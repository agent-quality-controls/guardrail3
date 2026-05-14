# G3RS Release Workflow Replay Fixture

## Goal

Implement Stage 5 from `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`.

End state:

- `g3rs validate --path <fixture>/repo --family release --rules-only --inventory` runs release repo-root checks.
- One new fixture covers the missing release workflow rows through the public CLI boundary:
  - `g3rs-release/release-plz-workflow`
  - `g3rs-release/publish-dry-run-workflow`
  - `g3rs-release/registry-token`
- The plan explicitly decides `g3rs-release/release-profile-inventory`.
- Behavior manifest, baseline, and coverage matrix agree.
- No existing release fixture gets broadened with unrelated workflow behavior.

## Facts From Current Code

Release repo-root checks exist but are not reachable from `g3rs validate`.

Files:

- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/release_plz_workflow/rule.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/publish_dry_run_workflow/rule.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/registry_token/rule.rs`

Current runner bug:

- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs` runs release config, filetree, and source checks.
- It does not call `g3rs_release_ingestion::ingest_for_repo_root_checks`.
- It does not call `g3rs_release_repo_root_checks::check`.
- `apps/guardrail3-rs/Cargo.toml` does not define the workspace dependency `g3rs-release-repo-root-checks`.
- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/Cargo.toml` does not depend on it.

Current ingestion bug:

- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs` implements `repo_root_result` as a stub.
- The stub returns `G3RsReleaseIngestionError::RepoRootChecksNotImplemented`.
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/basic.rs` has a test named `repo_root_checks_stub_returns_not_implemented`.
- That test must be replaced with tests proving repo-root ingestion returns real workflow facts.

Workflow detection exists:

- `workflow_has_release_plz` is true when a workflow step `uses` contains `release-plz`.
- `workflow_has_publish_dry_run` is true when a workflow step run line contains `cargo publish --dry-run`.
- `workflow_has_registry_token` is true when workflow, job, or step env contains `CARGO_REGISTRY_TOKEN`.

Relevant files:

- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/workflow_predicates.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`

## Rule Semantics

### `g3rs-release/release-plz-workflow`

File:

- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/release_plz_workflow/rule.rs`

Behavior:

- Emits `Info` when `input.workflow_flags.has_release_plz_workflow` is true.
- Emits `Warn` when false.
- Missing title: `Release-plz workflow missing`.
- Missing path: `Cargo.toml`.

### `g3rs-release/publish-dry-run-workflow`

File:

- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/publish_dry_run_workflow/rule.rs`

Behavior:

- Emits `Info` when `input.workflow_flags.has_publish_dry_run_workflow` is true.
- Emits `Warn` when false.
- Missing title: `Publish dry-run workflow missing`.
- Missing path: `Cargo.toml`.

### `g3rs-release/registry-token`

File:

- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/registry_token/rule.rs`

Behavior:

- Emits `Info` when `input.workflow_flags.has_registry_token_workflow` is true.
- Emits `Warn` when false.
- Missing title: `CARGO_REGISTRY_TOKEN missing from workflows`.
- Missing path: `Cargo.toml`.

### `g3rs-release/release-profile-inventory`

File:

- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/release_profile_inventory.rs`

Behavior:

- Emits `Info` only.
- It returns early when there are zero publishable crates.
- It returns early when root `[profile.release]` has no tracked settings.
- It has no violation branch.

Decision:

- Do not create a failure fixture for this rule.
- Cover it as intentional positive inventory.
- The fixture should include a publishable root package and non-empty `[profile.release]` settings so this Info row becomes reachable.
- If this Info row appears in the same new fixture, pin it in the baseline and mark it `covered` with `target_replay = "info_inventory"`.

## Implementation Plan

### 1. Make Release Repo-Root Checks Reachable

Modify `apps/guardrail3-rs/Cargo.toml`:

- Add workspace dependency:
  - `g3rs-release-repo-root-checks = { path = "../../packages/rs/release/g3rs-release-repo-root-checks", features = ["api"] }`

Modify `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/Cargo.toml`:

- Add dependency:
  - `g3rs-release-repo-root-checks.workspace = true`

Modify `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs`:

- In the `SupportedFamily::Release` branch:
  - call `g3rs_release_ingestion::ingest_for_repo_root_checks(crawl)`
  - map errors to `FamilyRunError` in the same style as existing release ingestion calls
  - extend results with `g3rs_release_repo_root_checks::check(&repo_root_input)`
- Keep config, filetree, source, and repo-root result ordering stable:
  - config
  - filetree
  - source
  - repo-root

Reason:

- The missing workflow rules are not fixture problems until the CLI can run the repo-root checks package.

### 2. Replace Release Repo-Root Ingestion Stub

Modify `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`:

- Replace `repo_root_result` stub with real ingestion.
- It must:
  - call `require_pointed_workspace_root(crawl)?`
  - call existing `collect(crawl, current_path_env().as_deref())`
  - return `Ok(collected.config.repos[0].clone())` using a non-panicking access pattern
- If no repo config exists, return a concrete `IngestionError` variant already used for missing root Cargo input.

Do not:

- duplicate workflow scanning logic
- parse workflow files in the runner
- add a new parser

Reason:

- `collect` already builds `G3RsReleaseConfigRepo` with workflow flags and workflow paths.
- Repo-root checks need the same repo facts as config checks.

Modify `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/basic.rs`:

- Delete or rewrite `repo_root_checks_stub_returns_not_implemented`.
- Add a test proving `repo_root_result` returns real facts when workflows are present.
- The test fixture must include:
  - root `Cargo.toml` with `[workspace]`
  - `.github/workflows/release.yml` containing:
    - `uses: release-plz/action@...`
    - `run: cargo publish --dry-run`
    - `env: CARGO_REGISTRY_TOKEN: ...`
- Assert:
  - `has_release_plz_workflow == true`
  - `has_publish_dry_run_workflow == true`
  - `has_registry_token_workflow == true`
  - rel path fields point at `.github/workflows/release.yml`

Add a second test only if needed:

- A no-workflow workspace returns repo config with all three flags false.
- This is useful if the rewritten stub test would otherwise only prove the positive branch.

### 3. Add One Minimal Fixture

Create:

- `behavior/fixtures/g3rs/L70-release-workflow-policy-violated`

Start from:

- `behavior/fixtures/g3rs/L70-release-invalid-semver-policy-violated/repo`

Then change it to a valid publishable package:

- root `Cargo.toml` must have valid semver
- keep `publish = true`
- keep required release metadata valid enough to avoid unrelated metadata errors
- include non-empty `[profile.release]` settings to trigger `release-profile-inventory`

Required files:

- `fixture.toml`
- `repo/Cargo.toml`
- `repo/LICENSE`
- `repo/README.md`
- `repo/cliff.toml`
- `repo/guardrail3-rs.toml`
- `repo/release-plz.toml`
- `repo/src/lib.rs`

Do not add `.github/workflows`.

Reason:

- The three repo-root workflow rules emit missing-workflow warnings when workflows are absent.
- Adding wrong workflows would test workflow parser edge cases, not the intended missing workflow policy.
- Omitting workflows gives three independent missing warnings without hiding config, filetree, or source checks.

`fixture.toml`:

```toml
id = "L70-release-workflow-policy-violated"
tool = "g3rs"
run_from = "repo"
commands = [
  ["validate", "--path", ".", "--family", "release", "--inventory", "--rules-only"],
]
expected_exit = "zero"
level = "delegated_policy_valid_project_policy_violated"

valid_state = [
  "workspace_root_found",
  "guardrail_config_valid",
  "required_inputs_present",
  "required_inputs_valid",
  "delegated_tools_present",
  "delegated_policy_valid",
]

intentionally_invalid = [
  "project_policy_violated",
]
```

Expected Error/Warn rows:

- `Warn|g3rs-release/release-plz-workflow|Release-plz workflow missing|Cargo.toml`
- `Warn|g3rs-release/publish-dry-run-workflow|Publish dry-run workflow missing|Cargo.toml`
- `Warn|g3rs-release/registry-token|CARGO_REGISTRY_TOKEN missing from workflows|Cargo.toml`

Expected intentional Info row:

- `Info|g3rs-release/release-profile-inventory|Release profile inventory|Cargo.toml`

Other Info rows may appear because the package is otherwise valid.

- Pin all intentional Info rows that are used for coverage.
- Do not pin incidental Info rows unless verifier requires it.
- The fixture must not emit unrelated `Error` rows.

### 4. Update Fixture Manifest

Modify:

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`

Add `[[fixture]]`:

- `id = "L70-release-workflow-policy-violated"`
- `expected_exit = "zero"`
- `baseline_required = true`
- `closed_file_list = true`
- exact `files = [...]`
- `required_results = [...]` containing at least the three Warn rows and the release profile Info row if it appears.

Closure requirement:

- The fixture must reject unlisted `Error` and `Warn`.
- If `verify-baselines.py` already closes by `required_results`, do not add a fixture-prefix special case.

### 5. Generate Baseline

Generate:

- `behavior/golden/g3rs-validate/approved.normalized.json`

Use:

```sh
python3 scripts/behavior/generate-baselines.py --manifest .plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml
```

Then restore all unrelated pre-existing tracked baseline files if their only change is `created_at` churn.

Do not hand-edit the JSON baseline.

### 6. Update Coverage Matrix

Modify:

- `behavior/coverage/g3rs-rule-coverage.toml`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md.manifest.toml`

Rows:

- `g3rs-release/release-plz-workflow`
- `g3rs-release/publish-dry-run-workflow`
- `g3rs-release/registry-token`

Set:

- `coverage_status = "covered"`
- `current_replay = "error_or_warn"`
- `target_replay = "error_or_warn"`
- `fixture = "L70-release-workflow-policy-violated"`
- `reason = "covered by L70 release workflow fixture with valid release config and no workflows"`

Row:

- `g3rs-release/release-profile-inventory`

Set:

- `coverage_status = "covered"`
- `current_replay = "info_only"`
- `target_replay = "info_inventory"`
- `fixture = "L70-release-workflow-policy-violated"`
- `reason = "intentional positive inventory; rule has no warning/error branch and emits only when publishable crates and root profile.release settings exist"`

Expected count changes if exactly the three absent workflow IDs become covered:

- `covered`: `229 -> 233`
- `planned`: `37 -> 33`
- `baseline_rule_ids`: `260 -> 263`
- `baseline_error_warn_rule_ids`: `213 -> 216`
- `info_only_rule_ids`: unchanged unless the new fixture is the first replay for `release-profile-inventory`
- `absent_rule_ids`: `6 -> 3`

Do not update counts by hand before running `verify-rule-coverage.py`.

### 7. Required Verification

Run:

```sh
cargo test --manifest-path packages/rs/release/g3rs-release-ingestion/crates/runtime/Cargo.toml
cargo test --manifest-path packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/Cargo.toml
cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-family-runner-process
python3 -m py_compile scripts/behavior/*.py
scripts/behavior/verify-all.sh
apps/guardrail3-rs/target/debug/g3rs validate --path behavior/fixtures/g3rs/L70-release-workflow-policy-violated/repo --family release --rules-only --inventory
apps/guardrail3-rs/target/debug/g3rs validate --path apps/guardrail3-rs --inventory
apps/guardrail3-rs/target/debug/g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory
git diff --check
```

The direct L70 fixture command should exit zero because workflow misses are warnings. Its stdout must contain the three workflow warnings.

### 8. Adversarial Review

Send reviewers after implementation.

Reviewer A:

- Verify repo-root checks are actually called by the public CLI.
- Verify the fixture would fail before the runner/ingestion fix and succeeds after it.
- Verify no private rule harness is used for coverage.

Reviewer B:

- Verify fixture minimality.
- Verify no workflow file is needed to trigger the intended missing-workflow warnings.
- Verify the fixture does not emit unrelated release metadata errors.

Reviewer C:

- Verify manifest, baseline, and coverage matrix agree.
- Verify all new `Error` and `Warn` rows are pinned.
- Verify `release-profile-inventory` is correctly treated as Info inventory, not a missing violation branch.

No implementation is complete until all MUST FIX findings are resolved and a final adversarial pass returns no MUST FIX.
