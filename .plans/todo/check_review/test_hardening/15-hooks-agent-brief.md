# Hooks Agent Brief

You own the hook migration and hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/05-hooks.md`
6. `.plans/todo/checks/hooks/shared.md`
7. `.plans/todo/checks/hooks/rs.md`
8. `.plans/todo/check_review/01-hooks-and-cli.md`

## Primary code

- migrated hook family work now lives under `apps/guardrail3/crates/app/rs/checks/hooks/`
- legacy hook validation remains under `apps/guardrail3/crates/app/hooks/` as migration-source code
- current hook generation paths remain under the CLI/generate code

This lane is expected to create or define the new-family shape for hooks rather than only tweaking old code.

## Current migrated state

Already created:
- `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/hooks/shared/`
- `apps/guardrail3/crates/app/rs/checks/hooks/rs/`
- `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs`
- migrated Rust hook checks are wired into `apps/guardrail3/crates/app/rs/validate/mod.rs`
- `rs hooks-validate` now uses the migrated Rust hook report path
- legacy `apps/guardrail3/crates/app/hooks/validate.rs` now delegates to the migrated Rust hook report when `has_rust` is true
- `ts hooks-validate` is pinned to the legacy TS/non-Rust hook path so mixed repos do not accidentally run Rust hook semantics through the TS command
- hook generation now shares one workspace-root-aware content builder across generate/install/diff paths

Executable-command parsing already exists and supports:
- shebang capture
- inert-text vs executable-command separation
- `if ! ...; then` command extraction
- `cd ... && cargo ...` extraction
- dispatcher syntax recognition
- fail-open wrapper recognition
- `exit 0`
- command substitutions in assignments like `file_size=$(git cat-file -s ...)`

Shared rules already migrated:
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
- `HOOK-SHARED-11`
- `HOOK-SHARED-12`
- `HOOK-SHARED-13`
- `HOOK-SHARED-14`
- `HOOK-SHARED-15`
- `HOOK-SHARED-16`
- `HOOK-SHARED-17`
- `HOOK-SHARED-18`
- `HOOK-SHARED-19`
- `HOOK-SHARED-20`
- `HOOK-SHARED-21`

Rust rules already migrated:
- `HOOK-RS-01`
- `HOOK-RS-02`
- `HOOK-RS-03`
- `HOOK-RS-04`
- `HOOK-RS-05`
- `HOOK-RS-06`
- `HOOK-RS-07`
- `HOOK-RS-08`
- `HOOK-RS-09`
- `HOOK-RS-10`
- `HOOK-RS-11`
- `HOOK-RS-12`
- `HOOK-RS-13`
- `HOOK-RS-14`
- `HOOK-RS-15`
- `HOOK-RS-16`

Still missing:
- remaining TS/non-Rust routing cleanup
- generator parity migration
- any additional `RS-TEST-08` hardening beyond parser reuse
- shared inventory/metadata sidecar coverage for the remaining uncovered `HOOK-SHARED-*` rules

## Old adversarial sources to mine

- existing hook-related validation code in `apps/guardrail3/crates/app/rs/validate/`
- generated hook templates and generator code paths in the CLI/adapters
- any current `.claude/` or hook fixture handling in the repo

## What you are trying to prove

Hook validation should operate on real executable-command semantics, not broad substring matching.

One test = one attack vector.

That vector should be applied across all relevant hook surfaces:
- root hooks
- modular scripts
- Rust steps
- prerequisite checks
- config-change trigger logic

## Known gaps already identified

- `HOOK-SHARED` / `HOOK-RS` rule inventory is migrated, but legacy code still exists alongside it
- Rust hook routing is largely fixed; the remaining stale routing is isolated to the TS/non-Rust legacy path
- major `workspace_root` generation drift is fixed; remaining parity work is broader legacy/modern behavior alignment
- `RS-TEST-08` should eventually reuse the same executable-command parsing model
- `RS-TEST-08` now reuses the shared executable-command parser for mutation-hook detection
- first `test-attack` pass found and fixed several false-pass bugs:
  - split-line `guardrail3 ... validate` + `--staged` no longer satisfies `HOOK-RS-08`
  - `echo`/banner text no longer satisfies `HOOK-SHARED-15`
  - bare `MAX_FILE_SIZE` references no longer satisfy `HOOK-SHARED-16`
  - comment-only config filename mentions no longer satisfy `HOOK-RS-16`
  - echoed tool strings no longer satisfy Rust/shared step-presence checks such as clippy, gitleaks, workspace test, lockfile, and fail-open detection
- second hardening pass added missing sidecar tests for `HOOK-RS-01..07`, `HOOK-RS-14`, and `HOOK-RS-15`
- compile verification still passes with `cargo check --lib`, but full `cargo test` remains blocked by unrelated existing lib-test failures elsewhere on the branch

## Important constraint for the next agent

Assume the current golden fixture is empty except for folder structure and config/hook files.

That means:
- safe to keep implementing:
  - parser-driven hook content rules
  - cached pre-commit file checks
  - `ToolChecker`-based tool availability checks
- do not silently assume:
  - populated Rust workspaces
  - real source trees
  - execute-bit metadata in fixtures
  - end-to-end staged-file behavior from a real project

If the next step needs more than structure plus config/hook files, say so explicitly before proceeding.

## Required attack classes

- comments or prose masquerading as commands
- executable `echo` / banner lines masquerading as real tool execution
- shebang and permission failures
- `exit 0` bypass and fail-open wrappers
- missing Rust guardrail steps
- wrong `workspace_root`
- missing prerequisite tools
- missing config-change triggers

## Structural requirement

Build the new families in the same architecture as the Rust check families:
- one rule per production file
- one rule-specific `*_tests/` directory per rule
- orchestrator + facts + inputs

## Done means

- migrated hook family shape exists or is brought to near-complete form
- executable-command parsing model exists
- broad bypass attacks are tested against golden hook fixtures
- raw substring command detection is no longer the semantic core

## Next concrete continuation point

Resume with:
1. expand adversarial sidecar coverage across the remaining uncovered shared inventory/metadata rules:
   - `HOOK-SHARED-01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `12`, `17`
   - keep adding explicit false-pass cases, not only happy-path tests
2. move to routing/generator parity:
   - legacy/modern hook semantic split
   - remaining TS/non-Rust hook path cleanup
3. `RS-TEST-08` is already aligned to the parser; deepen its hardening only if needed

## Do not

- keep hook validation as broad `contains()` checks and call it done
- couple hook routing to stale coarse CLI domains
- leave the hook lane without a concrete new-family structure
