# Hooks Rust Agent Brief

This is the droppable handoff file for the `hooks/rs` hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/05-hooks.md`
6. `.plans/todo/checks/hooks/rs.md`
7. `.plans/todo/checks/hooks/shared.md`
8. `.plans/todo/check_review/test_hardening/17-hooks-execution-plan.md`
9. `.plans/todo/check_review/test_hardening/18-hooks-coverage-matrix.md`
10. `.plans/todo/check_review/01-hooks-and-cli.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/checks/hooks/rs/`

Important family files:
- `mod.rs`
- `facts.rs`
- `inputs.rs`
- `test_support.rs`

Rust rules:
- `hook_rs_01_fmt_step_present.rs`
- `hook_rs_02_clippy_step_present.rs`
- `hook_rs_03_cargo_deny_step_present.rs`
- `hook_rs_04_test_step_present.rs`
- `hook_rs_05_cargo_machete_step_present.rs`
- `hook_rs_06_required_tools_installed.rs`
- `hook_rs_07_duplication_tool_is_cargo_dupes.rs`
- `hook_rs_08_guardrail_validate_staged_present.rs`
- `hook_rs_09_clippy_denies_warnings.rs`
- `hook_rs_10_test_uses_workspace.rs`
- `hook_rs_11_gitleaks_step_present.rs`
- `hook_rs_12_cargo_dupes_step_present.rs`
- `hook_rs_13_cargo_dupes_excludes_tests.rs`
- `hook_rs_14_guardrail_binary_available.rs`
- `hook_rs_15_cargo_dupes_installed.rs`
- `hook_rs_16_config_changes_trigger_validation.rs`

Shared parser/utilities:
- `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs`
- `apps/guardrail3/crates/app/rs/checks/hooks/shell_tests.rs`

## Legacy Seed Material

Use these as seed material only:

- `apps/guardrail3/crates/app/hooks/validate.rs`
- `apps/guardrail3/crates/app/hooks/hook_script_checks.rs`
- `apps/guardrail3/crates/app/hooks/tool_checks.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/generate.rs`
- `apps/guardrail3/crates/domain/modules/pre_commit.rs`

Do not port legacy string matching or broad substring semantics.

## Lane Context You Must Keep

Even though this is the split Rust-family brief, the hook lane still has live cross-family debt:
- generator/checker parity follow-up
- remaining TS/non-Rust hook-path cleanup
- `workspace_root` legacy/modern parity context
- `RS-TEST-08` parser-sharing continuation

Do not treat this brief as if the only remaining work is local Rust-family test migration.

Current migrated routing/generation state you must assume:
- Rust validate/report routing already uses migrated hook checks
- legacy `app/hooks/validate.rs` delegates to migrated Rust hook reporting when Rust is present
- `ts hooks-validate` is pinned to the legacy TS/non-Rust path
- generation already shares one workspace-root-aware builder across generate/install/diff paths

## Family Contract

`hooks/rs` owns Rust-specific hook step semantics on top of the shared executable-command model.

It owns:
- required Rust hook steps
- required Rust prerequisite tools
- `guardrail3 validate --staged`
- `cargo clippy -D warnings`
- `cargo test --workspace`
- config-change trigger coverage for Rust policy files
- duplication-tool choice semantics

It does not own:
- generic hook existence
- dispatcher structure
- execute-bit / shebang / generic shell safety

Those belong to `hooks/shared`.

Compatibility contract:
- `.githooks/pre-commit` is preferred
- `hooks/pre-commit` is compatibility fallback
- if neither exists, missing-hook failure belongs to `hooks/shared`; Rust rules may stay silent

Presence normalization is part of the contract:
- env wrappers count
- path-qualified binaries count
- `cargo +toolchain` prefixes count
- called shell functions count if they execute the relevant command
- reachable command substitutions / subshell-style segments count only when actually executed

## Fail-Closed Contract

Required inputs include:
- active shared/Rust hook surfaces
- executable-command context from the shared parser
- Rust tool availability facts when a rule needs them

Malformed or unreadable required inputs must not silently turn missing Rust enforcement into a clean pass.

Current fixture constraint:
- assume the current golden fixture is effectively empty except for folder structure and config/hook files
- do not silently assume populated Rust workspaces, real source trees, execute-bit metadata in fixtures, or end-to-end staged-file behavior

## Known Live Gaps

The family implementation exists, but it still needs hardening.

Highest-signal remaining gaps:
- the family still uses only `*_tests.rs`, not rule-specific `*_tests/` directories
- there is also a grouped family test artifact in `mod_tests.rs`
- step-presence semantics must stay command-level, not text-level
- `HOOK-RS-06` / `14` / `15` tool-availability semantics need exact owned hit sets
- `HOOK-RS-07` must stay distinct from plain duplication-step presence:
  - cargo-dupes required when Rust duplication step exists
  - jscpd may coexist but cannot substitute
- `HOOK-RS-16` must keep matching real trigger logic, not comments or banners
- generator/checker parity and TS/non-Rust routing debt still exist at the lane level even if this pass stays Rust-family-local

## Current Test Shape

The family still uses single sidecar files:
- `hook_rs_*_tests.rs`

This pass should move it to the standard:
- one rule-specific `*_tests/` directory per rule
- one test file per attack vector
- no grouped family test artifact left behind

## Required Attack Classes

Every rule should move toward:

1. golden pass
2. attack-vector test
3. exact owned hit set
4. exact owned non-hit set
5. false-positive control
6. fail-closed coverage where applicable
7. exact severity assertions

Highest-signal attacks for this family:
- comments or prose masquerading as Rust steps
- executable `echo` / banner lines masquerading as real Rust steps
- path-qualified or wrapped commands that should still count
- split tokens that should not count as one real step
- missing `-D warnings`
- non-workspace test command variants
- missing prerequisite tools
- config trigger comments/banners that should not count

## Mission

Harden `hooks/rs` only.

Required outcomes:
- verify Rust-family structure and ownership against `hooks/rs.md`
- convert every rule to a rule-specific `*_tests/` directory
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- use exact owned hit/non-hit assertions
- fix real semantic bugs you find
- update `.plans/todo/checks/hooks/rs.md` with:
  - gaps closed
  - gaps remaining
  - policy questions, if any

## Do Not

- re-own shared hook structure inside Rust rules
- use raw text presence as the semantic core
- leave the family on `*_tests.rs`
- silently relax Rust step semantics to make tests pass
- mix TS/non-Rust cleanup into this family except where it blocks Rust ownership

## Done Means

The pass is not done until:

- every Rust hook rule has a rule-specific `*_tests/` directory
- every Rust hook rule has golden coverage
- every Rust hook rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- semantic bugs are fixed or written down explicitly
- `hooks/rs.md` reflects what changed and what remains

## Suggested Start Order

1. read `hooks/rs.md` and map all 16 rules to current files
2. audit `mod.rs` / `facts.rs` / `inputs.rs` plus the shared `shell.rs` contract
3. convert all `hook_rs_*_tests.rs` files to `*_tests/`
4. harden the highest-risk semantic rules first:
   - `HOOK-RS-02`
   - `HOOK-RS-06`
   - `HOOK-RS-07`
   - `HOOK-RS-08`
   - `HOOK-RS-09`
   - `HOOK-RS-16`
5. finish exact tool-availability and config-trigger coverage before stopping
