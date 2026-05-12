# Behavior Fixture Gap Audit

## Goal

Record the adversarial audit results for behavior replay fixture coverage.

The current fixture stack is not enough to replace the existing behavior tests.

The current stack proves:

```text
G3RS L00-L80 directory shape
basic validate --path command shape
one clean deny-family package
one minimal project-policy violation
fixture exclusion from live validation routing
```

It does not prove most CLI-visible behavior now covered by tests.

## Method

Four read-only adversarial agents audited tests against:

```text
.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md
.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml
behavior/fixtures/g3rs
scripts/behavior/verify-fixtures.py
```

The agents were split by ownership area:

```text
Agent A: G3RS CLI, shared crawl, topology, hooks
Agent B: G3RS fmt, toolchain, cargo, clippy, deny
Agent C: G3RS code, apparch, deps, garde, test, release
Agent D: G3TS, TypeScript packages, parsers, Astro families
```

## Agent A Plan: CLI, Shared Crawl, Topology, Hooks

### Scope Read

Agent A read:

```text
apps/guardrail3-rs
packages/shared/g3-workspace-crawl
packages/rs/topology
packages/rs/hooks
```

### Current Coverage Gap

Current G3RS fixtures only run:

```text
g3rs validate --path <fixture>/repo --inventory
```

They do not run:

```text
g3rs validate-repo
g3rs validate --path <fixture>/repo --family <family> --inventory
g3rs validate --path <fixture>/repo --staged --inventory
```

They do not contain:

```text
hook scripts
marker-pair failures
staged Git state
topology-invalid trees
ignored recoverable files
core.hooksPath state
executable-bit-sensitive files
```

### Missing Behaviors

- `validate-repo` marker-pair behavior.
  - Tests: `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/marker_pairs_tests/cases.rs`
  - Old tests: `marker_pairs_ignore_behavior_fixtures`, `marker_pairs_still_report_real_incomplete_adoption`
  - Fixture level: L70
  - Mutation: add a real incomplete adoption under `packages/demo/guardrail3-rs.toml` and an ignored nested fixture adoption under `behavior/fixtures/.../guardrail3-rs.toml`
  - Command: `g3rs validate-repo --inventory`

- CLI family selection.
  - Tests: `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli_tests/cases.rs`, `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/selection_tests/cases.rs`
  - Old tests: `parse_command_accepts_family_and_inventory_flags`, `selected_families_follow_canonical_order`
  - Fixture level: L80
  - Mutation: add command with repeated out-of-order family flags
  - Command: `g3rs validate --path repo --family release --family fmt --family toolchain --inventory`

- Staged mode and cargo gate filtering.
  - Tests: `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates_tests/cases.rs`
  - Old tests: `cargo_gate_commands_skips_dupes_exclude_tests_when_staged_without_rust_source`, `rust_relevant_path_detection`, `paths_under_workspace_filters_correctly`, `rust_relevance_helpers`
  - Fixture level: L80 for clean non-Rust staged skip, L50 for missing delegated tools when Rust files are staged
  - Mutation: fixture metadata for staged files under and outside `repo`
  - Command: `g3rs validate --path repo --staged --inventory`

- Workspace crawl ignore and recovery.
  - Tests: `packages/shared/g3-workspace-crawl/crates/runtime/src/run_tests/ignore_state.rs`
  - Old tests: `marks_gitignored_files_as_included_via_recovery`, `recovery_finds_ignored_config_in_non-banned_directory`, `recovery_uses_guardrail3_rs_toml_and_not_dead_guardrail3_toml`, `behavior_fixtures_are_excluded_from_phase1_without_gitignore`
  - Fixture level: L40 or L70, depending on whether recovered files are malformed or policy-invalid
  - Mutation: add `.gitignore` that ignores recoverable configs, includes ignored non-recoverable junk, and contains nested `behavior/fixtures/...`
  - Command: `g3rs validate --path repo --inventory`

- Topology file-tree violations.
  - Tests: `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run_tests/pipeline.rs`, `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/*/rule_tests/cases.rs`
  - Old tests: `nested_workspace_fires_end_to_end`, `exact_membership_fires_end_to_end`, `escaping_member_path_fires_end_to_end`, `illegal_family_file_placement_fires_end_to_end`, `nested_guardrail3_rs_toml_under_adopted_outer_fires`
  - Fixture level: L70
  - Mutation: nested workspace, undeclared child crate, extra missing member, escaping member path, member-local family files, nested `guardrail3-rs.toml`
  - Command: `g3rs validate --path repo --family topology --inventory`

- Hook source contract.
  - Tests: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run_tests/cases.rs`
  - Old tests: `real_repo_pre_commit_hook_with_validate_repo_stripped_fires_calls_validate_repo`, `real_repo_pre_commit_hook_with_per_unit_validate_stripped_fires_dispatches`, `real_repo_pre_commit_hook_with_marker_stripped_fires_error`, `family_owned_commands_warn_when_hook_neither_delegates_nor_runs_them`
  - Fixture level: L60
  - Mutation: weakened `.githooks/pre-commit` missing `g3rs validate-repo`, per-unit staged validation, marker-pair discovery, or family commands
  - Command: `g3rs validate-repo --inventory`

- Hook file-tree contract.
  - Tests: `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run_tests/pipeline.rs`
  - Old tests: `pipeline_file_tree_reports_missing_pre_commit_hook`, `pipeline_file_tree_reports_layout_stats_permissions_and_overrides`, `pipeline_file_tree_reports_trust_risk`
  - Fixture level: L60 for invalid layout, L80 for clean layout inventory
  - Mutation: `.githooks/pre-commit`, `.githooks/pre-commit.d/*.sh`, `.guardrail3/overrides/pre-commit.d/*.sh`, executable bits, and competing `.git/hooks/pre-commit`
  - Command: `g3rs validate-repo --inventory`

- Hook tool availability.
  - Tests: `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/*/rule_tests/*.rs`, `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run_tests/pipeline.rs`
  - Old tests: `reports_missing_g3rs_when_validation_is_required`, `reports_missing_cargo_dupes_when_required`, `pipeline_config_reports_missing_g3rs_binary_when_required`, `pipeline_config_reports_tool_inventory_and_missing_cargo_dupes`
  - Fixture level: L50
  - Mutation: hook requiring `g3rs`, `gitleaks`, `cargo-deny`, `cargo-machete`, and `cargo dupes`
  - Command: `g3rs validate-repo --inventory`

### Agent A Fixture Additions

- Add commands to L80:
  - `g3rs validate-repo --inventory`
  - `g3rs validate --path repo --family release --family fmt --family toolchain --inventory`
  - `g3rs validate --path repo --staged --inventory`

- Add `L50-required-inputs-valid-delegated-tools-missing-repo-hooks`.

- Add `L60-delegated-tools-present-hook-policy-invalid`.

- Add `L70-delegated-policy-valid-topology-filetree-violated`.

- Add `L70-delegated-policy-valid-marker-pair-violated`.

- Add `L70-delegated-policy-valid-crawl-ignore-recovery`.

### Agent A Runner Requirements

- Fixture runner must support staged-file setup.
- Fixture runner must preserve executable bits.
- Fixture runner must support repo-level Git config such as `core.hooksPath`.

## Agent B Plan: Rust Config And Tooling Families

### Scope Read

Agent B read:

```text
packages/rs/fmt
packages/rs/toolchain
packages/rs/cargo
packages/rs/clippy
packages/rs/deny
```

Inventory:

```text
fmt: 34 files / 75 tests
toolchain: 19 files / 58 tests
cargo: 42 files / 105 tests
clippy: 44 files / 99 tests
deny: 118 files / 196 tests
```

### Current Coverage Gap

- L40 covers syntax-invalid required files only.
- L60 covers one Clippy threshold mutation and incidental Cargo lint-table errors.
- L70 covers source/test policy, not config/tooling policy breadth.
- L80 is one clean library-profile deny package.

### Missing Behaviors

- Fmt stable-channel policy.
  - Tests: `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/pipeline.rs`
  - Old tests: `pipeline_reports_nightly_rustfmt_keys_on_stable`, `pipeline_reports_rustfmt_ignore_waiver_from_guardrail3_rs_toml`, `pipeline_uses_root_dot_rustfmt_toml_for_config_checks`
  - Fixture level: L60
  - Mutation: add nightly rustfmt key on stable, add ignored generated path with strong waiver, add dotfile-selection variant
  - Command: `g3rs validate --path repo --family fmt --inventory`

- Fmt file-tree inventory.
  - Tests: `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/dual_file_conflict/rule_tests/cases.rs`, `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/per_crate_override/rule_tests/cases.rs`
  - Old tests: `warns_for_root_dual_file_conflict`, `errors_for_nested_rustfmt_toml`, `errors_for_nested_dot_rustfmt_toml`
  - Fixture level: L70
  - Mutation: root `.rustfmt.toml`, `crates/api/rustfmt.toml`, `crates/api/.rustfmt.toml`
  - Command: `g3rs validate --path repo --family fmt --inventory`

- Toolchain legacy/conflict/MSRV behavior.
  - Tests: `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run_tests/filetree.rs`, `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/channel_and_components/rule_tests/malformed.rs`, `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/msrv_consistency/rule_tests/older_than_msrv.rs`
  - Old tests: `pipeline_reports_legacy_only_as_warn_plus_missing_modern`, `pipeline_reports_both_toolchain_files_conflict`, `errors_when_channel_head_is_malformed_stable`, `warns_when_pinned_toolchain_is_older_than_msrv`
  - Fixture level: L60
  - Mutation: add legacy `rust-toolchain`, malformed or older modern channel, keep Cargo `rust-version = "1.85"`
  - Command: `g3rs validate --path repo --family toolchain --inventory`

- Cargo workspace member policy.
  - Tests: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/no_weakened_overrides_tests/cases.rs`, `workspace_lints_inherited_tests/cases.rs`, `member_edition_drift_tests/cases.rs`
  - Old tests: `errors_when_member_weakens_workspace_lints`, `errors_when_member_does_not_inherit_workspace_lints`, `warns_when_member_edition_is_older_than_workspace`
  - Fixture level: L70
  - Mutation: member `crates/api/Cargo.toml` with edition drift, missing lint inheritance, weakened local lint
  - Command: `g3rs validate --path repo --family cargo --inventory`

- Cargo root policy weakening.
  - Tests: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/workspace_lints/rule_tests/missing_clippy.rs`, `resolver/rule_tests/wrong.rs`, `lint_levels/rule_tests/weakens.rs`
  - Old tests: `errors_when_clippy_lints_table_is_missing`, `errors_when_workspace_resolver_is_unsupported`, `errors_when_expected_deny_is_weakened`
  - Fixture level: L60
  - Mutation: unsupported resolver, missing or weakened lint tables, weakened deny lint
  - Command: `g3rs validate --path repo --family cargo --inventory`

- Clippy ban inventory.
  - Tests: `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/missing_method_ban_tests/cases.rs`, `missing_type_ban_tests/cases.rs`, `macro_bans_tests/cases.rs`
  - Old tests: `reports_missing_baseline_method_ban`, `reports_library_profile_specific_missing_type_ban`, `reports_missing_macro_bans`, `reports_malformed_method_section`
  - Fixture level: L60
  - Mutation: empty or malformed `disallowed-methods`, `disallowed-types`, `disallowed-macros`
  - Command: `g3rs validate --path repo --family clippy --inventory`

- Clippy config override and shadowing.
  - Tests: `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/forbid_clippy_conf_dir_override_tests/cases.rs`, `g3rs-clippy-filetree-checks/.../same_root_conflict/rule_tests/cases.rs`, `unknown_keys_tests/cases.rs`
  - Old tests: `errors_on_override_surface`, `errors_for_shadowed_plain_clippy_toml_when_dotfile_wins`, `warns_on_managed_key_typos`
  - Fixture level: L70 for extra config, L60 for typo
  - Mutation: `.clippy.toml`, `.cargo/config.toml` with `CLIPPY_CONF_DIR`, misspelled managed key
  - Command: `g3rs validate --path repo --family clippy --inventory`

- Deny file-tree shadowing.
  - Tests: `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/run_tests/filetree.rs`, `g3rs-deny-filetree-checks/.../shadowing_tests/same_root_conflicts.rs`
  - Old tests: `pipeline_reports_same_root_conflicts`, `pipeline_reports_shadowed_root_parse_failures`, `errors_on_same_root_precedence_conflict`
  - Fixture level: L70
  - Mutation: `.deny.toml`, `.cargo/deny.toml`, and one shadowed parse-invalid file if selected-file parse hides it
  - Command: `g3rs validate --path repo --family deny --inventory`

- Deny policy breadth.
  - Tests: `allow_override_channel_tests/overrides.rs`, `sources/unknown_keys/rule_tests/unknown_top_level.rs`, `licenses/license_allow_baseline/rule_tests/extra_license.rs`, `sources/allow_git_inventory/rule_tests/has_entries.rs`, `sources/ignore_hygiene/rule_tests/weak_reason.rs`
  - Old tests: `errors_on_non_empty_allow_list_and_deny_overrides`, `unknown_top_level_section_warns`, `unexpected_license_produces_error`, `non_empty_allow_git_warns_and_inventories`, `weak_reason_errors`
  - Fixture level: L60
  - Mutation: non-empty bans allowlist, unknown top-level sections, extra license, `allow-git`, weak advisory ignore or bans skip reason
  - Command: `g3rs validate --path repo --family deny --inventory`

### Agent B Fixture Additions

- Add `L40-required-inputs-present-typed-invalid`.
- Add or replace with `L60-config-tooling-policy-invalid`.
- Add `L70-extra-config-inventory-violated`.
- Add `L70-cargo-workspace-member-policy-violated`.
- Add `L80-project-policy-valid-clean-service`.

### Agent B Risks

- Unreadable files and delete-after-crawl races need runner mutation support, not static fixtures.
- Some missing workspace-member cases may be hidden by Cargo metadata before family checks.

## Agent C Plan: Rust Source, Architecture, Dependency, Garde, Test, Release

### Scope Read

Agent C read:

```text
packages/rs/code
packages/rs/apparch
packages/rs/deps
packages/rs/garde
packages/rs/test
packages/rs/release
```

Inventory:

```text
code: 230 tests
apparch: 74 tests
deps: 55 tests
garde: 71 tests
test: 150 tests
release: 91 tests
```

### Current Coverage Gap

- L70 has only one project source violation: inline `#[cfg(test)] mod tests`.
- L80 is a clean deny package and does not activate garde, release, apparch-layered workspace behavior, dependency-policy edges, broad code AST rules, or mutation/nextest config.

### Missing Behaviors

- Code source AST violations and false-positive boundaries.
  - Tests: `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/*/rule_tests/*.rs`
  - Old tests include direct `std::fs`, `todo!`, `panic!`, weak allow/expect, `include!`, too many fields/imports, weak public error types
  - Fixture level: L70
  - Mutation: compound source files with each violation plus clean/test-owned false positives
  - Command: `g3rs validate --path repo --family code --inventory`

- Code parse/input failure with continued findings.
  - Tests: `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run_tests/pipeline.rs`, `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/input_failures/rule_tests/direct.rs`
  - Old tests: `pipeline_emits_explicit_input_failure_for_parse_error`, `pipeline_keeps_other_findings_when_one_file_fails_to_parse`, `emits_code_family_input_failure_on_parse_error`
  - Fixture level: L70
  - Mutation: malformed `src/broken.rs` plus another valid violating source file
  - Command: `g3rs validate --path repo --family code --inventory`

- Code config exception and unsafe-code inventory.
  - Tests: `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/*/rule_tests/*.rs`
  - Old tests: `emits_warn_for_each_exception_comment`, `emits_inventory_info_for_forbid`, `emits_error_for_deny`
  - Fixture level: L60 for policy weakening, L70 for inventory
  - Mutation: exception comments in parser-backed configs, Cargo `unsafe_code = "forbid"` and `"deny"`
  - Command: `g3rs validate --path repo --family code --inventory`

- Apparch config checks.
  - Tests: `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/*_tests/cases.rs`, `g3rs-apparch-ingestion/.../run/config_tests/pipeline.rs`
  - Old tests include dependency direction, same-layer cycles, purity, hidden patch aliases, target-specific dev violations
  - Fixture level: L70
  - Mutation: layered workspace with `types`, `logic`, `io/inbound`, `io/outbound`; forbidden runtime/dev edges; same-layer cycle; impure external deps
  - Command: `g3rs validate --path repo --family apparch --inventory`

- Apparch source public-surface checks.
  - Tests: `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/*_tests/cases.rs`
  - Old tests include public IO traits and public behavior in types crates
  - Fixture level: L70
  - Mutation: public traits in IO crates and public functions/methods in types crates, including reexport from private child module
  - Command: `g3rs validate --path repo --family apparch --inventory`

- Deps allowlist, canonical identity, caps, and path dependency policy.
  - Tests: `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/*/rule_tests/*.rs`, `g3rs-deps-ingestion/.../run_tests/deps.rs`
  - Old tests include external dep allowlist, renamed package identity, build/dev deps, dedupe, absolute path dep inside workspace non-member
  - Fixture level: L70 for policy findings, L40 for ingestion failures
  - Mutation: member crates with renamed external deps, target/build/dev deps, cap excess, internal path dep, separate invalid fixture for non-member path dep
  - Command: `g3rs validate --path repo --family deps --inventory`

- Deps lockfile and gitignore behavior.
  - Tests: `packages/rs/deps/g3rs-deps-filetree-checks/crates/runtime/src/*_tests/cases.rs`
  - Old tests: missing library lockfile info, ignored lockfile, gitignore unignore and last-match behavior
  - Fixture level: L70
  - Mutation: `.gitignore` ignore/unignore patterns for `Cargo.lock`; service and library missing-lock variants
  - Command: `g3rs validate --path repo --family deps --inventory`

- Active garde config and source validation.
  - Tests: `packages/rs/garde/g3rs-garde-config-checks`, `g3rs-garde-source-checks`, `g3rs-garde-ingestion`
  - Old tests include missing garde dependency, missing clippy bans, structs/enums missing validation, manual deserialize, missing dive, context mismatch, `sqlx::query_as!`, parse failure plus AST findings
  - Fixture level: L60 for clippy ban policy, L70 for source policy
  - Mutation: `checks.garde = true`, `garde` dependency, valid/invalid clippy bans, boundary structs/enums/manual deserialize, bad source
  - Command: `g3rs validate --path repo --family garde --inventory`

- Test config quality.
  - Tests: `packages/rs/test/g3rs-test-config-checks/crates/runtime/src/**/rule_tests/behavior.rs`
  - Old tests include missing nextest for async surface, missing mutants file/profile/hook step, exclude-everything, low timeout multiplier
  - Fixture level: L30 for missing required config, L60/L70 for weak policy
  - Mutation: async test surface with tokio dependency and mutation activation; omit or weaken `.config/nextest.toml` and `.cargo/mutants.toml`
  - Command: `g3rs validate --path repo --family test --inventory`

- Test source and file-tree quality.
  - Tests: `packages/rs/test/g3rs-test-source-checks`, `packages/rs/test/g3rs-test-file-tree-checks`
  - Old tests include inline test body, missing ignore reason, blank `should_panic(expected)`, literal-only assertions, weak `matches!`, missing shared proof, bad sidecar shape, runtime/assertions split, generic test support
  - Fixture level: L70
  - Mutation: compound malformed test source and file-tree shape
  - Command: `g3rs validate --path repo --family test --inventory`

- Release publishable crate policy.
  - Tests: `packages/rs/release/g3rs-release-config-checks`, `g3rs-release-ingestion`
  - Old tests include missing publish metadata, invalid semver, docs.rs/binstall metadata, local path dep to non-publishable crate, binary workflow mismatch
  - Fixture level: L70
  - Mutation: publishable lib and bin crates with invalid or missing release metadata
  - Command: `g3rs validate --path repo --family release --inventory`

- Release file-tree and README quality.
  - Tests: `packages/rs/release/g3rs-release-filetree-checks`, `g3rs-release-source-checks`
  - Old tests include missing LICENSE/release-plz/cliff/README and bad README
  - Fixture level: L30 for missing required files, L70 for bad README
  - Mutation: publishable crate missing release files or with low-quality README
  - Command: `g3rs validate --path repo --family release --inventory`

- Clean active baseline.
  - Tests: pipeline smoke tests across code, apparch, deps, garde, test, release
  - Fixture level: L80
  - Mutation: richer clean workspace with active clean source/architecture/dependency/test/release surfaces
  - Command: `g3rs validate --path repo --inventory`

### Agent C Fixture Additions

- Expand L30 when families are activated to expose missing nextest, mutants, release-plz, cliff.
- Expand L40 or add split L40 for malformed optional family inputs and malformed Rust source if CLI continues after parse failure.
- Expand L60 for active garde weak policy and dependency-tool policy weakening.
- Expand L70 into a compound project-policy fixture with code, apparch, deps, garde, test, and release violations.
- Replace or supplement L80 with a richer clean workspace.

### Agent C Risks

- Some release repo-root checks may not currently be reachable through public replay boundaries.
- Some unreadable-file tests need runner permission mutation support.
- Large L70 output is acceptable if it stays one unlock-layer fixture rather than one fixture per test.

## Agent D Plan: G3TS And TypeScript

### Scope Read

Agent D read:

```text
apps/guardrail3-ts
packages/ts
packages/parsers used by TypeScript checks
behavior/fixtures
```

Inventory:

```text
89 Rust test files / 688 Rust tests in TS-adjacent Rust packages
29 TypeScript test files / 101 TS tests
```

### Current Coverage Gap

- `behavior/fixtures/g3ts` does not exist.
- `behavior/migration/g3ts-test-ledger.toml` does not exist.
- Every CLI-visible G3TS behavior is missing from replay coverage.

### Missing Behaviors

- G3TS CLI family parsing, family order, inventory hiding, error routing.
  - Tests: `apps/guardrail3-ts/.../cli_tests/cases.rs`, `selection_tests/cases.rs`, `execute_tests/cases.rs`, `plain_text_tests/cases.rs`
  - Fixture level: L30/L80
  - Mutation: all-family and family-filter commands with and without `--inventory`
  - Commands: `g3ts validate --path repo --inventory`, `g3ts validate --path repo --family eslint --inventory`

- Guardrail config marker and opt-out behavior.
  - Tests: `apps/guardrail3-ts/.../family_opt_out_tests/cases.rs`, `packages/parsers/g3ts-toml-parser/.../parsing.rs`
  - Fixture level: L10/L20/L60
  - Mutation: missing, malformed, empty `guardrail3-ts.toml`, and `[checks] eslint=false style=false`
  - Command: `g3ts validate --path repo --inventory`
  - Conflict: current tests treat missing or malformed config as empty opt-out, but migration plan maps them to L10/L20. This must be resolved before accepting baselines.

- Missing required TS inputs.
  - Tests: `apps/guardrail3-ts/.../run_tests/cases.rs`, package run tests under `packages/ts/{eslint,tsconfig,jscpd,npmrc,package,fmt,spelling,typecov}`
  - Fixture level: L30
  - Mutation: valid `package.json` and `guardrail3-ts.toml`, omit `tsconfig`, ESLint config, `.jscpd.json`, `.npmrc`, prettier, cspell, syncpack, Astro config
  - Command: `g3ts validate --path repo --inventory`

- Malformed required TS inputs.
  - Tests: parser packages for `package-json`, `tsconfig-json`, `cspell-config`, `syncpack-config`, `jscpd-json`, `npmrc`, `eslint-config`, `astro-config`
  - Fixture level: L40
  - Mutation: malformed required input files, using family-filter commands so one malformed file does not hide unrelated families
  - Command: `g3ts validate --path repo --family <family> --inventory`

- Required npm packages and tools absent.
  - Tests: TS fmt, spelling, typecov, style, Astro setup/media/mdx/i18n/seo config tests, toolchain gate tests
  - Fixture level: L50
  - Mutation: valid configs, missing package deps and missing delegated tools
  - Commands: `g3ts validate --path repo --inventory`, `g3ts validate-repo`

- Delegated policy weakened.
  - Tests: TS eslint, style, package, spelling, typecov
  - Fixture level: L60
  - Mutation: wrong plugin identity, weak severities, unsafe scripts, weaker typecov threshold, missing syncpack pin groups, weakened npmrc
  - Commands: family-filtered G3TS validation

- Package script parser behavior.
  - Tests: `packages/parsers/package-script-command-parser`, package/fmt/spelling/typecov/style/Astro setup script tests
  - Fixture level: L60
  - Mutation: `package.json` scripts with fail-open `||`, fallback, pipe/semicolon blockers, package-manager shorthand, direct tool invocations
  - Commands: `g3ts validate --path repo --family package --inventory` and family-specific commands

- TS arch and apparch source violations.
  - Tests: `packages/ts/arch`, `packages/ts/apparch`
  - Fixture level: L70
  - Mutation: bad exports/facade, forbidden layer imports, framework imports in logic/types
  - Commands: `g3ts validate --path repo --family arch --inventory`, `g3ts validate --path repo --family apparch --inventory`

- Astro setup/content/mdx/i18n/media/seo/state behavior.
  - Tests: `packages/ts/astro/**/run_tests/cases.rs`, `packages/ts/g3ts-eslint-plugin-*`
  - Fixture level: L60/L70
  - Mutation: minimal Astro app with valid base config, then weakened plugin wiring for L60 and source/generated violations for L70
  - Commands: Astro subfamily validation commands

- Hooks, topology, marker pairs.
  - Tests: `packages/ts/hooks`, `packages/ts/topology`, `apps/guardrail3-ts/.../marker_pairs.rs`
  - Fixture level: L60/L70
  - Mutation: `.githooks/pre-commit`, adopted TS units, nested adopted unit, marker-pair incomplete variants
  - Command: `g3ts validate-repo`

- Parser edge cases visible through CLI.
  - Tests: Astro config parser, ESLint directive parser, ESLint config parser, hook shell parser
  - Fixture level: L40/L60/L70
  - Mutation: only include parser cases when they change CLI-visible output
  - Commands: family-filtered G3TS validation

- JS package/auditor behavior.
  - Tests: `packages/ts/astro/{llms,robots,sitemap,media}`, `packages/ts/g3ts-astro-nuasite-checks`
  - Fixture level: L70 if CLI-visible
  - Mutation: generated `dist` or public artifacts only if current G3TS CLI validates them
  - Risk: may be package API behavior, not CLI-visible guardrail behavior

### Agent D Fixture Additions

- Add `behavior/fixtures/g3ts/L00-workspace-root-not-found`.
- Add `behavior/fixtures/g3ts/L10-workspace-root-found-guardrail-config-missing`.
- Add `behavior/fixtures/g3ts/L20-workspace-root-found-guardrail-config-invalid`.
- Add `behavior/fixtures/g3ts/L30-guardrail-config-valid-required-inputs-missing`.
- Add `behavior/fixtures/g3ts/L40-required-inputs-present-invalid`.
- Add `behavior/fixtures/g3ts/L50-required-inputs-valid-delegated-tools-missing`.
- Add `behavior/fixtures/g3ts/L60-delegated-tools-present-policy-invalid`.
- Add `behavior/fixtures/g3ts/L70-delegated-policy-valid-project-policy-violated`.
- Add `behavior/fixtures/g3ts/L80-project-policy-valid-clean` with:
  - `apps/site` as a minimal clean Astro app
  - `packages/lib` as a minimal clean TS package

### Agent D Risks

- G3TS config missing/malformed behavior conflicts with the current migration plan.
- Existing TS packages may not have committed `guardrail3-ts.toml`; clean G3TS fixtures likely need to be synthesized.
- Parser package tests should enter fixtures only if behavior is CLI-visible.

## Cross-Agent Decisions

### Fixtures Must Stay Layered

Do not add one fixture per old test.

Add compound fixtures by unlock layer:

```text
L30: valid guardrail config, required inputs missing
L40: required inputs present but syntactically or semantically invalid
L50: required inputs valid, delegated tools missing
L60: delegated tools present, delegated policy weakened
L70: delegated policy valid, project source/tree/package policy violated
L80: project policy valid and clean
```

Split only when one behavior hides another.

### Fixture Runner Must Grow Before Test Deletion

Before deleting behavior tests, the replay runner must support:

```text
multiple commands per fixture
family-filtered commands
validate-repo commands
staged-file setup
runner PATH or delegated-tool mode
executable-bit preservation checks
repo-level Git config setup
baseline generation
baseline comparison
normalization of absolute paths
```

### Keep Non-Replay Tests Temporarily

Do not migrate these into static fixtures until runner support exists:

```text
unreadable file permission mutations
delete-after-crawl races
parser unit behavior that is not CLI-visible
public API compile contracts
fixture runner tests
normalizer tests
baseline metadata fail-closed tests
```

### Clean Baselines Need More Than One Package

The current L80 clean fixture is not enough.

It proves one clean deny config package.

It does not prove clean behavior for:

```text
hooks
topology
fmt/toolchain/cargo/clippy service profile
code
apparch
deps
garde
test
release
G3TS package
G3TS Astro app
```

Add clean baselines by meaning, not by old test count:

```text
G3RS clean library package
G3RS clean service package
G3RS clean layered apparch workspace
G3RS clean publishable crate
G3TS clean package
G3TS clean Astro content app
```

## Next Implementation Order

1. Extend fixture metadata and verifier for multiple commands per fixture.
2. Add `validate-repo`, `--family`, and `--staged` command support.
3. Add staged-file and runner-mode setup.
4. Add G3RS Agent A fixtures because they exercise command modes.
5. Add G3RS Agent B config/tooling compound fixtures.
6. Add G3RS Agent C source/architecture/policy compound fixtures.
7. Create G3TS L00-L80 fixture skeleton.
8. Resolve the G3TS missing/malformed config behavior conflict.
9. Add G3TS compound fixtures.
10. Generate baselines.
11. Build migration ledgers.
12. Delete behavior tests only after baseline replay covers them.
