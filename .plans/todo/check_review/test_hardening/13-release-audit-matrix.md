# Release Hardening Audit Matrix

This began as the starting-state audit for the `rs/release` hardening pass.
It is now also the running matrix for what has already been closed.

It exists to answer three questions before migration work begins:

1. what each rule currently tests
2. what each rule is still missing
3. which family-level implementation bugs must be exposed directly by the rewritten suite

## Structural baseline

Original starting state:

- all 29 production rules in `apps/guardrail3/crates/app/rs/checks/rs/release/` used flat `*_tests.rs` files
- no release-family rule ended in a rule-specific `*_tests/` directory

Current state:

- migrated to `*_tests/` directories:
  - `RS-RELEASE-01`
  - `RS-RELEASE-02`
  - `RS-RELEASE-03`
  - `RS-RELEASE-04`
  - `RS-RELEASE-05`
  - `RS-RELEASE-06`
  - `RS-RELEASE-07`
  - `RS-RELEASE-08`
  - `RS-RELEASE-09`
  - `RS-RELEASE-10`
  - `RS-RELEASE-11`
  - `RS-RELEASE-12`
  - `RS-PUB-01`
  - `RS-PUB-02`
  - `RS-PUB-03`
  - `RS-PUB-04`
  - `RS-PUB-05`
  - `RS-PUB-06`
  - `RS-PUB-07`
  - `RS-PUB-08`
  - `RS-PUB-09`
  - `RS-PUB-10`
  - `RS-PUB-11`
  - `RS-PUB-12`
  - `RS-PUB-13`
  - `RS-PUB-14`
  - `RS-BIN-01`
  - `RS-BIN-02`
  - `RS-BIN-03`
- still flat `*_tests.rs`:
  - none

## Family-level implementation findings

These findings matter before or during test migration because they shape what the hardened suite must expose.

### `readme = false` is currently mishandled

- In `facts.rs`, the README field is read only as a string:
  - `package.get("readme").and_then(toml::Value::as_str)`
- In `release_support.rs`, missing string input falls back to `"README.md"`.
- Result:
  - `readme = false` is treated like “no explicit path, default to README.md”
  - this can incorrectly warn on crates that intentionally disable README packaging

Status:

- fixed in the current pass
- `PublishableCrateFacts` now carries explicit `readme_declared_false`
- `RS-PUB-04` and `RS-PUB-05` now skip explicit opt-out instead of warning

### inherited `workspace = true` local path edges are currently invisible to path-edge rules

- In `release_support.rs`, `dependency_edges` only sets `has_path` from the dependency entry itself.
- It does read inherited versions from `[workspace.dependencies]`, but it does not inherit `path` from workspace dependencies.
- Result:
  - `RS-PUB-10` and `RS-PUB-11` can miss local path edges declared via `workspace = true`

Status:

- fixed in the current pass
- `dependency_edges` now inherits `path` from `[workspace.dependencies]` when the local dependency uses `workspace = true`

### workflow rules still overclaim semantic strength

- `release_plz_step_present` accepts:
  - `uses` containing `release-plz/`
  - any `run` line containing `release-plz`
- `publish_dry_run_step_present` accepts any run line containing `cargo publish --dry-run`
- `registry_token_present` accepts any matching env key or any scalar string containing `CARGO_REGISTRY_TOKEN`
- `linux_target_present` accepts any scalar string containing `ubuntu`, `linux`, `x86_64-unknown-linux`, or `amd64`
- `binary_release_present` only requires:
  - some step mentioning `build` and `--release`
  - some step using `action-gh-release`
- Result:
  - workflow comments, prose, unrelated string fields, and loosely related steps are still the main false-positive surface

Status:

- partially closed in the current pass
- comments, prose/display text, and `echo ...` fake commands are no longer counted as real execution for the hardened workflow rules
- workflow facts now preserve richer step/job facts, but semantic matching is still release-family specific rather than a fuller Actions execution model

### release-family input failures were only partly fail-closed

- `collect_cargo_roots` only reports parse failures for cached Cargo manifests that were successfully read into `ProjectTree`.
- `collect_workflows` only reports parse failures for cached workflow files present in `ProjectTree.content`.
- `project_walker.rs` caches config content only when `fs.read_file(...)` succeeds.
- Result:
  - parse failures are covered
  - unreadable cached config files can still disappear from semantic checks rather than always producing explicit release-family failures

Status:

- partially closed in the current pass
- malformed config and partial-facts coverage are now exercised via synthetic `ProjectTree` tests
- unreadable README permission failure is now exercised through a real on-disk fixture mutation
- unreadable Cargo/workflow/release config files now fail closed when structure shows the file but cached content is missing

### aggregate shape is mostly acceptable, but workflow semantic matching is still family-specific

- Rule inputs themselves are small:
  - `RepoReleaseInput`
  - `PublishableCrateReleaseInput`
  - `ReleaseEdgeInput`
  - `ReleaseInputFailureInput`
- The main problem is not input size.
- The remaining problem is not data loss anymore.
- `WorkflowFacts` now preserves parsed workflow structure, but release semantics are still recognized through family-specific helper matching rather than a fuller Actions execution model.

Status:

- partially closed in the current pass
- workflow rules no longer consume pre-collapsed booleans
- deeper Actions execution semantics are still a later-hardening target

### new adversarial pressure from the current test-attack pass

- shell-wrapper execution, workspace-package inheritance, `publish = []`, canonical root license-name validation, split-job binary workflow linkage, and nested `[package.metadata.docs.rs]` handling have now been added and implementation has been tightened for those cases
- remaining architectural limitation: workflow semantics are still recognized by release-family helper matching, even though parsed jobs/steps are now preserved and consumed directly by the rules

## Legacy corpus seed map

These old files still matter as attack-vector sources:

- `apps/guardrail3/tests/unit/test_release_repo_checks.rs`
  - seeds `RS-RELEASE-01..08`
- `apps/guardrail3/tests/unit/test_release_crate_checks.rs`
  - seeds `RS-PUB-01..05` and `RS-PUB-08`
- `apps/guardrail3/tests/unit/test_release_crate_deps.rs`
  - seeds `RS-PUB-06..11`
- `apps/guardrail3/tests/unit/test_release_bin_checks.rs`
  - seeds `RS-BIN-01..03`
- `apps/guardrail3/tests/unit/test_release_checks.rs`
  - seeds `RS-PUB-12`

The legacy corpus is useful for attack ideas and expected policy edges.
It is not acceptable as-is because it uses old IDs, broad assertions, and weaker workflow semantics.

## Rule matrix

Legend:

- `Current` = what the current release-family test file actually proves
- `Missing` = attack vectors or exactness still required before the rule can be called hardened

## Repo rules

### `RS-RELEASE-01` — `rs_release_01_license_file.rs`

- Current:
  - flat `rs_release_01_license_file_tests.rs`
  - missing-license negative
  - license-present inventory positive
- Missing:
  - `*_tests/` directory migration
  - golden fixture coverage
  - exact owned-hit and owned-non-hit assertions
  - attack proving alternate accepted license filenames behave correctly from the family fixture

### `RS-RELEASE-02` — `rs_release_02_release_plz_exists.rs`

- Current:
  - flat `rs_release_02_release_plz_exists_tests.rs`
  - missing-file warning
  - file-present inventory positive
- Missing:
  - `*_tests/` directory migration
  - golden fixture coverage
  - exact owned-hit and owned-non-hit assertions
  - malformed-file interaction with `RS-RELEASE-12`

### `RS-RELEASE-03` — `rs_release_03_release_plz_coverage.rs`

- Current:
  - directory-based `rs_release_03_release_plz_coverage_tests/`
  - missing `[workspace]` warning
  - canonical `[workspace]` baseline checks for `changelog_config`, `git_release_enable`, and `release_always`
  - per-crate package coverage warnings
  - inventory only when both semantic baseline and package coverage are complete
- Missing:
  - none beyond broader workflow-fact richness outside this rule

### `RS-RELEASE-04` — `rs_release_04_cliff_exists.rs`

- Current:
  - directory-based `rs_release_04_cliff_exists_tests/`
  - missing-file warning
  - canonical `[git]` baseline checks for `conventional_commits`, `filter_unconventional`, and parser coverage for `feat/fix/doc/perf/refactor/style/test/chore`
  - inventory only when the semantic baseline is complete
- Missing:
  - none beyond broader workflow-fact richness outside this rule

### `RS-RELEASE-05` — `rs_release_05_release_plz_workflow.rs`

- Current:
  - flat `rs_release_05_release_plz_workflow_tests.rs`
  - no-step warning
  - boolean-driven inventory positive
- Missing:
  - `*_tests/` directory migration
  - golden fixture coverage using real workflow YAML
  - fake-hit attacks via comments
  - fake-hit attacks via prose or display fields
  - fake-hit attacks via step names or unrelated scalar strings
  - exact executable-step detection
  - exact path ownership when multiple workflows exist

### `RS-RELEASE-06` — `rs_release_06_publish_dry_run_workflow.rs`

- Current:
  - flat `rs_release_06_publish_dry_run_workflow_tests.rs`
  - no-step warning
  - boolean-driven inventory positive
- Missing:
  - `*_tests/` directory migration
  - golden fixture coverage using real workflow YAML
  - fake-hit attacks via comments and prose
  - fake-hit attacks via non-executable string fields
  - real shell-command detection instead of substring approval
  - exact owned-hit and owned-non-hit assertions

### `RS-RELEASE-07` — `rs_release_07_registry_token.rs`

- Current:
  - flat `rs_release_07_registry_token_tests.rs`
  - missing-token warning
  - boolean-driven inventory positive
- Missing:
  - `*_tests/` directory migration
  - golden fixture coverage using real workflow YAML
  - comment/prose false-positive attacks
  - unrelated env-key or scalar-string false-positive attacks
  - real release-flow token wiring test
  - exact owned-hit and owned-non-hit assertions

### `RS-RELEASE-08` — `rs_release_08_semver_checks_installed.rs`

- Current:
  - flat `rs_release_08_semver_checks_installed_tests.rs`
  - tool-missing warning
  - tool-installed inventory positive
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - exact result assertions in final family style

### `RS-RELEASE-09` — `rs_release_09_publish_status_inventory.rs`

- Current:
  - flat `rs_release_09_publish_status_inventory_tests.rs`
  - one inventory case when `publish_setting` is present
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - exact inventory assertions from the family fixture
  - absent-setting behavior coverage

### `RS-RELEASE-10` — `rs_release_10_release_profile_inventory.rs`

- Current:
  - flat `rs_release_10_release_profile_inventory_tests.rs`
  - one inventory case when release profile settings exist
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - exact inventory assertions from the family fixture
  - absent-setting behavior coverage

### `RS-RELEASE-11` — `rs_release_11_accidentally_publishable_internal_crates.rs`

- Current:
  - flat `rs_release_11_accidentally_publishable_internal_crates_tests.rs`
  - one warning case for a publishable crate missing description, license, and repository
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - false-positive protection for non-publishable crates
  - false-positive protection for crates with partial valid metadata
  - exact owned-hit and owned-non-hit assertions

### `RS-RELEASE-12` — `rs_release_12_input_failures.rs`

- Current:
  - flat `rs_release_12_input_failures_tests.rs`
  - one generic failure-input error
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - source-specific attacks for Cargo parse failure
  - source-specific attacks for workflow YAML parse failure
  - source-specific attacks for README read failure
  - source-specific attacks for `release-plz.toml` parse failure
  - source-specific attacks for `cliff.toml` parse failure
  - partial-facts assertions proving one failure does not silently erase unrelated release outputs

## Publishable-crate rules

### `RS-PUB-01` — `rs_pub_01_description_present.rs`

- Current:
  - flat `rs_pub_01_description_present_tests.rs`
  - one missing-description error
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - positive inventory coverage
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-02` — `rs_pub_02_license_present.rs`

- Current:
  - flat `rs_pub_02_license_present_tests.rs`
  - one missing-license error
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - positive inventory coverage
  - `license-file` success coverage in new-family tests
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-03` — `rs_pub_03_repository_present.rs`

- Current:
  - flat `rs_pub_03_repository_present_tests.rs`
  - one missing-repository error
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - positive inventory coverage
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-04` — `rs_pub_04_readme_exists.rs`

- Current:
  - flat `rs_pub_04_readme_exists_tests.rs`
  - one missing-README warning
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - explicit README-path success case
  - default `README.md` success case
  - `readme = false` policy case
  - false-positive protection for non-publishable crates
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-05` — `rs_pub_05_readme_quality.rs`

- Current:
  - flat `rs_pub_05_readme_quality_tests.rs`
  - stub README warning
  - good README inventory positive
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - heading-less README attack
  - unreadable README fail-closed interplay
  - `readme = false` policy case
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-06` — `rs_pub_06_keywords_present.rs`

- Current:
  - flat `rs_pub_06_keywords_present_tests.rs`
  - too-many-keywords warning
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - missing-keywords warning
  - valid-keywords inventory positive
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-07` — `rs_pub_07_categories_present.rs`

- Current:
  - flat `rs_pub_07_categories_present_tests.rs`
  - missing-categories warning
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - positive inventory coverage
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-08` — `rs_pub_08_valid_semver.rs`

- Current:
  - flat `rs_pub_08_valid_semver_tests.rs`
  - invalid-version error
  - `workspace = true` inventory positive
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - normal valid semver positive in the new-family suite
  - malformed version edge cases from the family fixture
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-09` — `rs_pub_09_publish_dry_run.rs`

- Current:
  - directory-based `rs_pub_09_publish_dry_run_tests/`
  - direct rule-scope checks for no-dry-run and non-publishable crates
  - richer-fixture success inventory for a real `cargo publish --dry-run`
  - richer-fixture failure coverage by breaking a real crate body and asserting exact owned failure
- Missing:
  - explicit family-level `thorough = false` non-hit coverage if we want the full orchestrator path tested rather than rule-local scope only

### `RS-PUB-10` — `rs_pub_10_no_path_deps_to_unpublishable.rs`

- Current:
  - flat `rs_pub_10_no_path_deps_to_unpublishable_tests.rs`
  - one target-specific path-to-unpublishable error
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - normal dependency-table attack
  - build-dependency-table attack
  - inherited `workspace = true` path-edge attack
  - publishable local-path non-hit case
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-11` — `rs_pub_11_interdependent_version_consistency.rs`

- Current:
  - flat `rs_pub_11_interdependent_version_consistency_tests.rs`
  - one incompatible-version error
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - compatible local-edge non-hit case
  - inherited `workspace = true` edge attack
  - non-path edge non-hit case
  - non-publishable dependency non-hit case
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-12` — `rs_pub_12_crate_inventory.rs`

- Current:
  - flat `rs_pub_12_crate_inventory_tests.rs`
  - one inventory count case
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - exact inventory assertions from the family fixture

### `RS-PUB-13` — `rs_pub_13_docs_rs_metadata.rs`

- Current:
  - flat `rs_pub_13_docs_rs_metadata_tests.rs`
  - one info result when docs.rs metadata is missing for a library
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - present-metadata inventory case
  - non-library false-positive protection
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

### `RS-PUB-14` — `rs_pub_14_include_exclude_inventory.rs`

- Current:
  - flat `rs_pub_14_include_exclude_inventory_tests.rs`
  - one info result when include/exclude is missing
- Missing:
  - `*_tests/` directory migration
  - golden coverage
  - present-metadata inventory case
  - non-publishable false-positive protection
  - exact owned-hit and owned-non-hit assertions

## Binary workflow rules

### `RS-BIN-01` — `rs_bin_01_binary_release_workflow.rs`

- Current:
  - directory-based `rs_bin_01_binary_release_workflow_tests/`
  - same-job binary release positive
  - split build/publish `needs:` positive
  - non-binary false-positive protection
  - comment/prose and unrelated-job negatives
- Missing:
  - no known rule-local bug remains
  - broader workflow-fact richness still remains outside this rule

### `RS-BIN-02` — `rs_bin_02_linux_target.rs`

- Current:
  - directory-based `rs_bin_02_linux_target_tests/`
  - same-job Linux positive
  - split `needs:` Linux positive
  - matrix-driven `runs-on` Linux positive
  - non-binary false-positive protection
  - comment/prose and unrelated-job negatives
- Missing:
  - no known rule-local bug remains
  - broader workflow-fact richness still remains outside this rule

### `RS-BIN-03` — `rs_bin_03_binstall_metadata.rs`

- Current:
  - directory-based `rs_bin_03_binstall_metadata_tests/`
  - missing-metadata warning
  - hand-built inventory positive
  - manifest-backed inventory positive
  - wrong-shape metadata negative
  - non-binary and non-publishable false-positive protection
- Missing:
  - no known rule-local bug remains

## Migration order implied by the audit

This audit confirms the execution order already recorded in `13-release-execution-plan.md`.

The first implementation batches should be:

1. workflow semantics
   - `RS-RELEASE-05`
   - `RS-RELEASE-06`
   - `RS-RELEASE-07`
   - `RS-BIN-01`
   - `RS-BIN-02`
2. README and publishability semantics
   - `RS-PUB-04`
   - `RS-PUB-05`
   - `RS-RELEASE-11`
3. inherited local-edge semantics
   - `RS-PUB-10`
   - `RS-PUB-11`
4. fail-closed coverage
   - `RS-RELEASE-12`
5. then the remaining repo, metadata, inventory, and binary rules in full structural conversion order

The reason is not prioritization in the product sense.
The reason is dependency of effort:

- these batches are most likely to force changes in facts/support code
- the rest of the rule migrations can then target the corrected family semantics instead of encoding the current bugs
