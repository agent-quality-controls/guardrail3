# Hooks Shared Agent Brief

This is the droppable handoff file for the `hooks/shared` hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/05-hooks.md`
6. `.plans/todo/checks/hooks/shared.md`
7. `.plans/todo/check_review/test_hardening/17-hooks-execution-plan.md`
8. `.plans/todo/check_review/test_hardening/18-hooks-coverage-matrix.md`
9. `.plans/todo/check_review/01-hooks-and-cli.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/checks/hooks/shared/`

Important family files:
- `mod.rs`
- `facts.rs`
- `inputs.rs`

Shared rules:
- `hook_shared_01_pre_commit_exists.rs`
- `hook_shared_02_hooks_path_configured.rs`
- `hook_shared_03_modular_directory_inventory.rs`
- `hook_shared_04_dispatcher_pattern.rs`
- `hook_shared_05_pre_commit_executable.rs`
- `hook_shared_06_script_stats_inventory.rs`
- `hook_shared_07_modular_scripts_inventory.rs`
- `hook_shared_08_pre_commit_file_size_inventory.rs`
- `hook_shared_09_local_override_inventory.rs`
- `hook_shared_10_shell_error_handling.rs`
- `hook_shared_11_valid_shebang.rs`
- `hook_shared_12_modular_scripts_executable.rs`
- `hook_shared_13_no_unconditional_exit_zero.rs`
- `hook_shared_14_no_bypass_instructions.rs`
- `hook_shared_15_merge_conflict_step_present.rs`
- `hook_shared_16_file_size_step_present.rs`
- `hook_shared_17_execution_trust.rs`
- `hook_shared_18_executable_command_context_only.rs`
- `hook_shared_19_real_dispatcher_syntax_only.rs`
- `hook_shared_20_concrete_lockfile_command.rs`
- `hook_shared_21_no_fail_open_wrappers.rs`

Shared parser/utilities:
- `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs`
- `apps/guardrail3/crates/app/rs/checks/hooks/shell_tests.rs`

## Legacy Seed Material

Use these as seed material only:

- `apps/guardrail3/crates/app/hooks/validate.rs`
- `apps/guardrail3/crates/app/hooks/hook_script_checks.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`

Do not port legacy tests or old string-matching behavior mechanically.

## Lane Context You Must Keep

Even though this is the split shared-family brief, the hook lane still has live cross-family debt:
- generator/checker parity follow-up
- remaining TS/non-Rust hook-path cleanup
- `workspace_root` legacy/modern parity context
- `RS-TEST-08` parser-sharing continuation

Do not treat this brief as if the only remaining work is local shared-family test migration.

## Family Contract

`hooks/shared` owns hook structure and generic executable-command safety.

It owns:
- effective pre-commit hook existence
- hooks path configuration
- modular vs monolithic inventory
- dispatcher syntax
- execute-bit / shebang / shell-safety checks
- merge-conflict / file-size / lockfile concrete step presence
- fail-open wrapper detection
- execution trust

Concrete trust/shadowing surfaces for `HOOK-SHARED-17` include:
- `.git/hooks/pre-commit`
- competing hook systems such as Husky or Lefthook
- `git config core.hooksPath`

It does not own Rust-step semantics such as:
- `cargo fmt`
- `cargo clippy`
- `cargo deny`
- `cargo test --workspace`
- Rust prerequisite tools

Those belong to `hooks/rs`.

Compatibility contract:
- `.githooks/pre-commit` is preferred
- `hooks/pre-commit` is compatibility fallback only
- modular discovery remains rooted under `.githooks/pre-commit.d`

## Fail-Closed Contract

Required inputs include:
- effective pre-commit hook artifacts
- modular hook files
- hook-path/trust artifacts
- executable-bit metadata where available

Malformed or unreadable required hook inputs must not silently turn structural failures into clean passes.

Important ownership split:
- missing hook files themselves are shared-family failures
- Rust-step absence is not shared-family-owned unless it is the generic structural shell command itself

Current fixture constraint:
- assume the current golden fixture is effectively empty except for folder structure and config/hook files
- do not silently assume populated Rust workspaces, real source trees, execute-bit metadata in fixtures, or end-to-end staged-file behavior

## Known Live Gaps

The family implementation exists, but it is not fully hardened yet.

Highest-signal remaining gaps:
- inventory/metadata sidecar coverage is still thin for:
  - `HOOK-SHARED-01`
  - `HOOK-SHARED-02`
  - `HOOK-SHARED-03`
  - `HOOK-SHARED-04`
  - `HOOK-SHARED-05`
  - `HOOK-SHARED-06`
  - `HOOK-SHARED-07`
  - `HOOK-SHARED-08`
  - `HOOK-SHARED-09`
  - `HOOK-SHARED-10`
  - `HOOK-SHARED-12`
  - `HOOK-SHARED-17`
- the family still mixes old single-file sidecars with some newer `*_tests/` directories
- permission-metadata-unavailable behavior must stay explicit, not implicit
- `HOOK-SHARED-04` and `HOOK-SHARED-19` must stay distinct:
  - dispatcher layout/wiring
  - dispatcher syntax authenticity
- generator/checker parity and TS/non-Rust routing debt still exist at the lane level even if this pass stays shared-family-local

## Current Test Shape

The family is mixed:
- some rules still use `*_tests.rs`
- some rules already use rule-specific `*_tests/` directories

This pass should move the family to the standard:
- one rule-specific `*_tests/` directory per rule
- one test file per attack vector

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
- comments/prose masquerading as commands
- executable `echo` / banner lines masquerading as real tool steps
- shebang and execute-bit failures
- `exit 0` and fail-open wrappers
- fake dispatcher paths such as `pre-commit.dummy`
- lookalike lockfile/file-size/merge-conflict commands
- trust/path metadata drift

## Mission

Harden `hooks/shared` only.

Required outcomes:
- verify shared-family structure and ownership against `hooks/shared.md`
- convert every rule to a rule-specific `*_tests/` directory
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- use exact owned hit/non-hit assertions
- fix real semantic bugs you find
- update `.plans/todo/checks/hooks/shared.md` with:
  - gaps closed
  - gaps remaining
  - policy questions, if any

## Do Not

- re-merge shared and Rust semantics into one family
- use raw `contains()` as the semantic core
- leave rules on mixed ad hoc test shapes
- treat `echo "cargo fmt"` or comment text as executed commands
- quietly relax compatibility or trust semantics to make tests pass

## Done Means

The pass is not done until:

- every shared rule has a rule-specific `*_tests/` directory
- every shared rule has golden coverage
- every shared rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- semantic bugs are fixed or written down explicitly
- `hooks/shared.md` reflects what changed and what remains

## Suggested Start Order

1. read `hooks/shared.md` and map all 21 rules to current files
2. audit `mod.rs` / `facts.rs` / `inputs.rs` plus `shell.rs` for the real ownership split
3. convert the remaining `*_tests.rs` files to `*_tests/`
4. harden the highest-risk structural rules first:
   - `HOOK-SHARED-04`
   - `HOOK-SHARED-05`
   - `HOOK-SHARED-10`
   - `HOOK-SHARED-18`
   - `HOOK-SHARED-19`
   - `HOOK-SHARED-21`
5. finish the thin inventory/metadata coverage set before stopping
