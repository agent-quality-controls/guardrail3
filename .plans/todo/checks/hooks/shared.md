# HOOK-SHARED — Language-agnostic hook structure checker (21 rules)

**Input:** .githooks/pre-commit script content
**Current code:** `hook_checks.rs`, `hook_script_checks.rs`

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| HOOK-SHARED-01 | H1 | Error | .githooks/pre-commit file exists | Implemented |
| HOOK-SHARED-02 | H2 | Error | git config core.hooksPath = .githooks | Implemented |
| HOOK-SHARED-03 | H3 | Info | pre-commit.d/ directory exists (modular vs monolithic) | Implemented |
| HOOK-SHARED-04 | H4 | Error | Dispatcher pattern: modular hook sources scripts from pre-commit.d/ | Implemented |
| HOOK-SHARED-05 | H7 | Error | Pre-commit script has executable permission (Unix) | Implemented |
| HOOK-SHARED-06 | H6 | Info | Script stats inventory (line count, size, mtime) | Implemented |
| HOOK-SHARED-07 | H9 | Info | Extra scripts in pre-commit.d/ (modular mode inventory) | Implemented |
| HOOK-SHARED-08 | H10 | Info | Pre-commit file size (bytes) | Implemented |
| HOOK-SHARED-09 | H11 | Info | Local override scripts in .guardrail3/overrides/pre-commit.d/ | Implemented |
| HOOK-SHARED-10 | H-SAFE-01 | Warn | set -e or set -euo pipefail for shell error handling | Implemented |
| HOOK-SHARED-11 | — | Warn | Valid shebang on pre-commit and each modular script (`#!/bin/bash`, `#!/usr/bin/env bash`, `#!/bin/sh`) | Planned |
| HOOK-SHARED-12 | — | Warn | Every file in pre-commit.d/ has executable permission | Planned |
| HOOK-SHARED-13 | — | Warn | No unconditional `exit 0` bypass that masks hook failures | Planned |
| HOOK-SHARED-14 | — | Info | No `--no-verify` bypass instructions in comments | Planned |
| HOOK-SHARED-15 | — | Warn | Merge-conflict marker check step present | Planned |
| HOOK-SHARED-16 | — | Warn | File size check step present | Planned |
| HOOK-SHARED-17 | — | Warn | Hook execution trust: detect conflicting hook systems or hook shadowing risk | Planned |
| HOOK-SHARED-18 | — | Error | Required hook steps must be detected in executable command context, not comments or echo text | Planned |
| HOOK-SHARED-19 | — | Warn | Dispatcher detection must require real dispatch syntax, not weak token matches | Planned |
| HOOK-SHARED-20 | — | Warn | Lockfile integrity step must be validated by concrete command shape, not generic text | Planned |
| HOOK-SHARED-21 | — | Warn | No fail-open wrappers on guardrail-critical commands (`|| true`, `|| :`, `|| echo ...`) | Planned |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| HOOK-SHARED-11 | Warn | Shebang validation. The top-level pre-commit script and each modular script in `pre-commit.d/` must start with a valid shell shebang. Prevents shell mismatch bugs and silent execution failures on systems where `/bin/sh` and interactive shells differ. | Planned |
| HOOK-SHARED-12 | Warn | Modular script permissions. Every file in `pre-commit.d/` must be executable, not just the dispatcher. Prevents silent skipping of scripts when modular hooks are invoked directly or via `run-parts`. | Planned |
| HOOK-SHARED-13 | Warn | Proper exit-code handling. The hook must not end with unconditional `exit 0` or similar constructs that mask failures from earlier commands. | Planned |
| HOOK-SHARED-14 | Info | No bypass instructions. The hook and modular scripts must not contain comments teaching `git commit --no-verify` or similar bypasses. | Planned |
| HOOK-SHARED-15 | Warn | Merge-conflict marker check step present. This is language-agnostic and belongs in shared hooks, not TS-specific hooks. The hook should run a dedicated conflict-marker scan before other expensive steps. | Planned |
| HOOK-SHARED-16 | Warn | File size check step present. Historical hook designs and CLAUDE.md treat large-file rejection as part of the standard hook guardrail set. This belongs in shared hooks because it is language-agnostic. | Planned |
| HOOK-SHARED-17 | Warn | Hook execution trust. Detect conflicting hook systems or shadowing situations where the validated `.githooks/pre-commit` may not be the hook that actually runs (for example Husky, Lefthook, or an unexpected `.git/hooks/pre-commit`). | Planned |
| HOOK-SHARED-18 | Error | Executable command context only. Required steps must be detected on executable command lines, not by raw substring presence in comments, echoed messages, or unrelated text. This is the core rule that prevents false passes from commented-out commands. | Planned |
| HOOK-SHARED-19 | Warn | Real dispatcher syntax only. Modular hook dispatcher checks must require actual sourcing or dispatch constructs, not loose tokens like `. ` or `for ` that appear in ordinary script text. | Planned |
| HOOK-SHARED-20 | Warn | Concrete lockfile integrity command. The lockfile check must validate a real command shape such as `pnpm install --frozen-lockfile`, not just the appearance of the word `lockfile`. | Planned |
| HOOK-SHARED-21 | Warn | No fail-open wrappers on guardrail-critical commands. Commands such as gitleaks, guardrail3 validate, cargo clippy, cargo deny, cargo test, cargo machete, and cargo dupes must not be softened with `|| true`, `|| :`, or similar patterns that mask failure. | Planned |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Maximum hook execution time | Not enforceable with a static checker. Would require runtime instrumentation or wrapper commands such as `timeout`. |
| Separate shared rule for script sourcing style variants | Too implementation-specific. HOOK-SHARED-04 already checks the dispatcher pattern at the right abstraction level. |
