# HOOK-SHARED — Language-agnostic hook structure checker (21 rules)

**Input:** effective pre-commit hook artifacts + modular hook files + hook-path/trust artifacts + executable-bit metadata
**Current code:** `crates/app/rs/checks/hooks/shared/**` (old `hook_checks.rs` / `hook_script_checks.rs` are legacy seed material only)

## Implementation mapping contract

- exactly one `HOOK-SHARED-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `facts.rs` and `inputs.rs` may contain shared discovery, typed inputs, and shell-command-context inputs only

Forbidden:

- grouped family test files such as `hook_shared_tests.rs`
- helper files that hide multiple unrelated rule predicates behind one API

## Discovery / ownership model

`HOOK-SHARED` is a validation-root family.

It owns the local hook artifacts that determine whether the validated pre-commit hook is the one that actually runs:
- primary hook path `.githooks/pre-commit`
- compatibility fallback `hooks/pre-commit`
- `.githooks/pre-commit.d/*`
- `.guardrail3/overrides/pre-commit.d/*`
- `git config core.hooksPath`
- `.git/hooks/pre-commit` when checking trust/shadowing risk
- competing hook-system artifacts such as Husky or Lefthook config files
- executable-bit facts for the dispatcher and modular scripts

It does not own language-specific command semantics inside the hook. Those belong to `HOOK-RS` and other language families.

Compatibility note:
- `.githooks/pre-commit` is the preferred contract
- `hooks/pre-commit` is a compatibility surface only
- modular discovery remains rooted under `.githooks/pre-commit.d`
- future verification must not treat `hooks/pre-commit` as equivalent to a full modular `.githooks` layout unless the rule explicitly says so

## Executable-command contract

Shell-step rules in this family must operate on executable command context, not raw substring matches.

That means:
- comments do not count
- `echo` text does not count
- inert mentions of required commands do not count
- dispatcher detection must use real dispatch syntax
- fail-open wrappers must be evaluated on executable command lines

This is the core contract behind:
- `HOOK-SHARED-18`
- `HOOK-SHARED-19`
- `HOOK-SHARED-20`
- `HOOK-SHARED-21`

## Input integrity / fail-closed expectations

The family depends on:
- readable cached hook script content for the dispatcher and modular scripts
- executable-bit metadata where permission checks apply
- readable `git config core.hooksPath` output for hook-path validation

If a required hook artifact cannot be read well enough to evaluate a rule, the family must not convert that into a false clean pass for command-context rules.

Even without a dedicated input-failure rule yet, the plan intent is fail-closed with respect to:
- command presence
- dispatcher presence
- fail-open wrapper detection
- trust/shadowing detection

## Cross-family dependency

`HOOK-SHARED` owns hook structure and trust.

`HOOK-RS` depends on it for:
- pre-commit existence
- actual validated hook location
- command-context parsing discipline
- fail-open and comment/text false-pass prevention

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| HOOK-SHARED-01 | H1 | Error | Effective pre-commit hook exists (`.githooks/pre-commit` preferred, `hooks/pre-commit` compatibility fallback) | Implemented |
| HOOK-SHARED-02 | H2 | Error | git config core.hooksPath = .githooks | Implemented |
| HOOK-SHARED-03 | H3 | Info | Hook layout inventory: modular `.githooks/pre-commit.d/` versus monolithic pre-commit mode | Implemented |
| HOOK-SHARED-04 | H4 | Error | Dispatcher pattern: modular hook sources scripts from pre-commit.d/ | Implemented |
| HOOK-SHARED-05 | H7 | Error | Effective pre-commit hook has executable permission. Unavailable permission metadata is an error. | Implemented |
| HOOK-SHARED-06 | H6 | Info | Effective pre-commit script stats inventory (line count + byte size) | Implemented |
| HOOK-SHARED-07 | H9 | Info | Modular script inventory: which scripts exist in `.githooks/pre-commit.d/`, or that none exist | Implemented |
| HOOK-SHARED-08 | H10 | Info | Pre-commit file size (bytes) | Implemented |
| HOOK-SHARED-09 | H11 | Info | Local override scripts in .guardrail3/overrides/pre-commit.d/ | Implemented |
| HOOK-SHARED-10 | H-SAFE-01 | Warn | set -e or set -euo pipefail for shell error handling | Implemented |
| HOOK-SHARED-11 | — | Warn | Valid shebang on pre-commit and each modular script (`#!/bin/bash`, `#!/usr/bin/env bash`, `#!/bin/sh`) | Implemented |
| HOOK-SHARED-12 | — | Warn | Every file in pre-commit.d/ has executable permission | Implemented |
| HOOK-SHARED-13 | — | Warn | No unconditional `exit 0` bypass that masks hook failures | Implemented |
| HOOK-SHARED-14 | — | Info | No `--no-verify` bypass instructions in comments | Implemented |
| HOOK-SHARED-15 | — | Warn | Merge-conflict marker check step present | Implemented |
| HOOK-SHARED-16 | — | Warn | File size check step present | Implemented |
| HOOK-SHARED-17 | — | Warn | Hook execution trust: detect conflicting hook systems or hook shadowing risk | Implemented |
| HOOK-SHARED-18 | — | Error | Required hook steps must be detected in executable command context, not comments or echo text | Implemented |
| HOOK-SHARED-19 | — | Warn | Dispatcher syntax must be real executable dispatch syntax, not weak token matches or inert text | Implemented |
| HOOK-SHARED-20 | — | Warn | Lockfile integrity step must be validated by concrete command shape, not generic text | Implemented |
| HOOK-SHARED-21 | — | Warn | No fail-open wrappers on guardrail-critical commands (`|| true`, `|| :`, `|| echo ...`) | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| HOOK-SHARED-11 | Warn | Shebang validation. The top-level pre-commit script and each modular script in `pre-commit.d/` must start with a valid shell shebang. Prevents shell mismatch bugs and silent execution failures on systems where `/bin/sh` and interactive shells differ. | Implemented |
| HOOK-SHARED-12 | Warn | Modular script permissions. Every file in `pre-commit.d/` must be executable, not just the dispatcher. Prevents silent skipping of scripts when modular hooks are invoked directly or via `run-parts`. | Implemented |
| HOOK-SHARED-13 | Warn | Proper exit-code handling. The hook must not end with unconditional `exit 0` or similar constructs that mask failures from earlier commands. | Implemented |
| HOOK-SHARED-14 | Info | No bypass instructions. The hook and modular scripts must not contain comments teaching `git commit --no-verify` or similar bypasses. | Implemented |
| HOOK-SHARED-15 | Warn | Merge-conflict marker check step present. This is language-agnostic and belongs in shared hooks, not TS-specific hooks. The hook should run a dedicated conflict-marker scan before other expensive steps. | Implemented |
| HOOK-SHARED-16 | Warn | File size check step present. Historical hook designs and CLAUDE.md treat large-file rejection as part of the standard hook guardrail set. This belongs in shared hooks because it is language-agnostic. | Implemented |
| HOOK-SHARED-17 | Warn | Hook execution trust. Detect conflicting hook systems or shadowing situations where the validated `.githooks/pre-commit` may not be the hook that actually runs (for example Husky, Lefthook, or an unexpected `.git/hooks/pre-commit`). | Implemented |
| HOOK-SHARED-18 | Error | Executable command context only. Required steps must be detected on executable command lines, not by raw substring presence in comments, echoed messages, or unrelated text. This is the core rule that prevents false passes from commented-out commands. | Implemented |
| HOOK-SHARED-19 | Warn | Real dispatcher syntax only. `HOOK-SHARED-04` owns whether modular mode actually dispatches. `HOOK-SHARED-19` owns the narrower syntax-hardening contract that the detected dispatcher must come from real executable dispatch syntax, not loose tokens like `. ` or `for ` in unrelated text. | Implemented |
| HOOK-SHARED-20 | Warn | Concrete lockfile integrity command. The lockfile check must validate a real command shape such as `pnpm install --frozen-lockfile`, not just the appearance of the word `lockfile`. | Implemented |
| HOOK-SHARED-21 | Warn | No fail-open wrappers on guardrail-critical commands. Commands such as gitleaks, guardrail3 validate, cargo clippy, cargo deny, cargo test, cargo machete, and cargo dupes must not be softened with `|| true`, `|| :`, or similar patterns that mask failure. | Implemented |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Maximum hook execution time | Not enforceable with a static checker. Would require runtime instrumentation or wrapper commands such as `timeout`. |
| Separate shared rule for script sourcing style variants | Too implementation-specific beyond the existing split: `HOOK-SHARED-04` owns dispatcher presence, `HOOK-SHARED-19` owns syntax hardening against false positives. |
