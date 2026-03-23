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

- current legacy hook validation under `apps/guardrail3/crates/app/`
- current hook generation paths under the CLI/generate code

This lane is expected to create or define the new-family shape for hooks rather than only tweaking old code.

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

- `HOOK-SHARED` / `HOOK-RS` do not yet exist as migrated families
- current hook validation is legacy and too text-based
- shebang / execute-bit / `exit 0` / fail-open-wrapper checks are missing or weak
- Rust hook command checks are incomplete
- `guardrail3` fail-closed behavior is missing
- config-change-triggered Rust validation is missing
- hook generation still drifts on `workspace_root`
- `RS-TEST-08` should eventually reuse the same executable-command parsing model
- CLI/reporting routing is stale around hooks

## Required attack classes

- comments or prose masquerading as commands
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

## Do not

- keep hook validation as broad `contains()` checks and call it done
- couple hook routing to stale coarse CLI domains
- leave the hook lane without a concrete new-family structure
