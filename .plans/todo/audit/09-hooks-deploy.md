# Adversarial Audit: Hook & Deployment Validation (H1-H12, H-TOOL-*, D1-D5)

## Files Audited

- `apps/guardrail3/src/app/hooks/validate.rs`
- `apps/guardrail3/src/app/hooks/hook_checks.rs`
- `apps/guardrail3/src/app/hooks/hook_script_checks.rs`
- `apps/guardrail3/src/app/hooks/tool_checks.rs`
- `apps/guardrail3/src/app/hooks/deploy_checks.rs`
- `websmasher/.githooks/pre-commit` (actual hook)
- `ts-rust-railway/.githooks/pre-commit` (actual hook)

## Critical Findings

### FINDING-H-01: No `--no-verify` bypass detection

**Severity: CRITICAL**

guardrail3 validates that a pre-commit hook exists and is configured, but it has ZERO detection of `--no-verify` usage. An agent (or developer) can run `git commit --no-verify` and bypass every single hook check. guardrail3 does not:
- Check git log for commits that skipped hooks (no hook output artifacts)
- Warn about `--no-verify` as a bypass vector
- Suggest any mitigation (e.g., server-side hooks, CI re-validation)

This is the single biggest hole in the entire hook validation system. The hooks are meaningless if they can be trivially bypassed.

### FINDING-H-02: `set -e` missing from actual hooks — guardrail3 does not check for it

**Severity: CRITICAL**

Both actual pre-commit hooks use `set -uo pipefail` but NOT `set -e`. Without `set -e`, if a command fails and its exit code is not explicitly checked with `if`, the script continues. While both hooks do use `if ! command; then exit 1; fi` patterns consistently, guardrail3 should validate that the hook uses `set -e` or equivalent fail-fast behavior. A future edit could add a bare command without `if` wrapping and it would silently pass.

guardrail3's `check_monolithic_patterns` does NOT check for `set -e` or `set -uo pipefail` or any shell safety settings.

### FINDING-H-03: H5 pattern matching is trivially bypassable with comments

**Severity: HIGH**

`check_monolithic_patterns` uses `content.contains("gitleaks")`, `content.contains("cargo fmt")`, etc. This means:
- A commented-out line like `# gitleaks protect --staged` passes H5
- A script that mentions gitleaks in an echo message but doesn't actually run it passes H5
- `echo "TODO: add gitleaks"` passes H5

The check does not verify that the tool is actually *executed* (e.g., preceded by `if !` or as a bare command with `set -e`). It only confirms the string appears somewhere in the file.

### FINDING-H-04: H-TOOL-02 conflict marker check has absurdly broad pattern matching

**Severity: HIGH**

`check_conflict_marker_hook` checks for `content.contains("<<<")`. The string `<<<` appears in bash heredoc syntax (`<<< "$STAGED_FILES"`), which BOTH actual pre-commit hooks use extensively. This means H-TOOL-02 would report "Conflict marker check in hook" as passing for ANY hook that uses a bash heredoc — even one that has zero conflict-marker detection logic.

Looking at both actual hooks: NEITHER has explicit conflict marker detection. But both would pass H-TOOL-02 due to their `<<<` heredoc usage. This is a false pass.

### FINDING-H-05: H-TOOL-03 lockfile check has overly broad pattern matching

**Severity: MEDIUM**

`check_lockfile_hook` checks for `content.contains("lockfile")` or `content.contains("frozen-lockfile")`. The first pattern matches ANY occurrence of the word "lockfile" — including comments, echo messages, variable names. Neither actual pre-commit hook has lockfile integrity checking, but if one had a comment like `# TODO: add lockfile check`, it would pass.

### FINDING-H-06: H8 tool check list is incomplete — missing tools from actual hooks

**Severity: HIGH**

`check_required_tools` in tool_checks.rs only checks 3 tools:
- gitleaks
- cargo-deny
- cargo-machete

But the actual pre-commit hooks also require:
- `pnpm` (TypeScript builds, ESLint, jscpd)
- `cargo` (fmt, clippy, test)
- `tsc` (via `pnpm exec tsc`)
- `eslint` (via `pnpm exec eslint`)
- `jscpd` (via `pnpm exec jscpd`)

These are assumed to exist but never verified. If `cargo` is not on PATH, the hook fails with a confusing error instead of a clear "tool not installed" message. guardrail3 should check for the core toolchain (cargo, pnpm/npm) not just the auxiliary tools.

### FINDING-H-07: H8 does not check tool versions

**Severity: MEDIUM**

`check_required_tools` only checks `tc.is_installed(tool)` — presence on PATH. It does not verify minimum versions. A stale `cargo-deny` from 2 years ago might not support current deny.toml format. Same for gitleaks, cargo-machete.

### FINDING-H-08: H7 permission check only covers `pre-commit`, not modular scripts

**Severity: MEDIUM**

`check_permissions` in hook_checks.rs is only called for `pre_commit_path`. If the project uses modular hooks (`pre-commit.d/`), the individual scripts in that directory are NOT checked for executable permissions. The dispatcher might find them but fail to execute them.

### FINDING-H-09: H2 uses `Command::new("git")` but does not validate git is installed

**Severity: LOW**

`check_hooks_path` runs `git config core.hooksPath` via `Command::new("git")`. If `git` is not on PATH, the match arm falls to `_ =>` and reports "core.hooksPath not configured" — a misleading error message. The actual problem is git not being available.

### FINDING-H-10: Actual hooks have steps guardrail3 doesn't validate

**Severity: HIGH**

Both actual pre-commit hooks have these steps that guardrail3 does NOT check for:

1. **File size check** (1MB limit) — No H-check validates that the hook enforces file size limits
2. **Migration consistency** (drizzle/ append-only) — No check validates this step exists
3. **Schema-without-migration detection** — No check validates this step exists
4. **Guardrail tamper detection** (#[allow] without reason, eslint-disable without reason, config relaxation) — No check validates this critical self-defense mechanism exists in the hook
5. **Rust structural health** (500 lines, 20 uses, crate-wide allow scan) — No check validates these exist in the hook
6. **`set -uo pipefail`** — No check validates shell safety settings

The CLAUDE.md lists 12 hook steps but H5 only pattern-matches for about 10 tool names. The semantic/structural checks (tamper detection, file size, migration, structural health) are completely invisible to guardrail3.

### FINDING-H-11: `has_apps_dir` bypass in validate.rs skips deployment checks

**Severity: MEDIUM**

In `validate.rs` line 31: `let has_apps_dir = path.join("apps").is_dir();` — deployment checks (D1-D5) only run if there are railpack configs OR an `apps/` directory. A project that deploys a single service without an `apps/` directory and without railpack configs will skip all deployment validation entirely. This means standalone services with Dockerfiles or other deployment methods get zero coverage.

### FINDING-H-12: D2 provider heuristic is filename-based, not content-based

**Severity: MEDIUM**

`check_railpack_provider` uses `filename.contains("web") || filename.contains("landing")` to decide if a missing provider field is an error vs warning. This is fragile:
- `railpack-api.json` for a Node.js API service would only get a Warn, not Error
- `railpack-webworker.json` for a Rust service would incorrectly get Error severity
- The heuristic doesn't inspect the actual Railpack config content to detect Node.js indicators

### FINDING-H-13: D3 standalone check is a bare string match

**Severity: MEDIUM**

`check_nextjs_configs` checks `content.contains("standalone")` for D3. This matches:
- A comment: `// standalone output is required`
- A variable: `const standalone = false;`
- The word in any context

It does NOT parse the config to verify `output: "standalone"` is actually set as a configuration value. A Next.js config with `// TODO: add standalone` would pass D3.

### FINDING-H-14: D4 outputFileTracingRoot is not validated for correct value

**Severity: MEDIUM**

D4 checks `content.contains("outputFileTracingRoot")` but does not validate that the value points to the monorepo root. It could be `outputFileTracingRoot: "."` (wrong for a monorepo) or even `// outputFileTracingRoot is needed` (a comment) and D4 would pass.

### FINDING-H-15: D5 Tailwind check only covers `apps/*/` — misses root-level apps

**Severity: LOW**

`check_tailwind_deps` only scans `apps/*/package.json`. A Next.js app at the repo root or in a non-standard location would not be checked.

### FINDING-H-16: No deployment checks for Dockerfile-based services

**Severity: MEDIUM**

D1-D5 only cover Railpack and Next.js configs. There are no checks for:
- Dockerfile existence and best practices
- `RAILWAY_DOCKERFILE_PATH` env var configuration
- Docker-compose files (which CLAUDE.md explicitly bans)
- `.dockerignore` existence

### FINDING-H-17: No check that the hook is actually sourced via `core.hooksPath`

**Severity: LOW**

H1 checks `.githooks/pre-commit` exists. H2 checks `core.hooksPath` equals `.githooks`. But there's no check that verifies the hook is actually being USED. If someone also has `.git/hooks/pre-commit` that shadows the `.githooks/` version, or if another tool (husky, pre-commit framework) overrides `core.hooksPath`, the validated hook might not be the one that runs.

### FINDING-H-18: H-TOOL-01 through H-TOOL-05 only check monolithic script content

**Severity: MEDIUM**

In `hook_checks.rs` line 164-175, the H-TOOL checks (`check_cspell_hook`, `check_conflict_marker_hook`, `check_lockfile_hook`, `check_prettier_hook`, `check_audit_hook`) all receive `ctx.pre_commit_content` — only the main pre-commit file. If the project uses modular hooks (`pre-commit.d/`), these checks do NOT scan the modular scripts. Only H5's `check_modular_scripts` concatenates and scans modular content.

So a modular hook setup with cspell in `pre-commit.d/50-cspell.sh` would fail H-TOOL-01 even though cspell IS configured.

### FINDING-H-19: `check_dispatcher_pattern` has a false-positive prone pattern

**Severity: LOW**

The dispatcher check looks for `pre-commit_content.contains(". ")` as one of the dispatch patterns (dot-source in bash). The string `. ` appears in virtually every English sentence and comment. A hook with `# This script checks pre-commit.d scripts.` would match `. ` (the period-space before "This") AND `pre-commit.d`, satisfying the dispatcher check without actually having dispatcher logic.

### FINDING-H-20: No validation of shebang line

**Severity: LOW**

guardrail3 does not validate that the pre-commit hook starts with `#!/usr/bin/env bash` or any valid shebang. A hook that starts with `#!/bin/sh` might not support bash-specific syntax used in the templates. A hook with no shebang at all would be executed by the default shell, which may not be bash.

### FINDING-H-21: ts-rust-railway hook is missing websmasher hook features

**Severity: INFO** (observation, not a guardrail3 bug)

The ts-rust-railway hook differs from the websmasher hook:
- websmasher has `--no-warn-ignored` in ESLint (line 148), ts-rust-railway does not
- websmasher excludes `legacy/` from TS staged files, ts-rust-railway does not
- websmasher has multi-workspace Rust support (root + backend), ts-rust-railway only has backend
- websmasher has LINE_COUNT_EXCEPTIONS, ts-rust-railway does not
- websmasher has `grep -c '^[[:space:]]*$'` for blank lines (returns 0 on no match via `|| true`), ts-rust-railway uses `|| echo 0` (different error handling)

guardrail3 would report both as passing the same H5 checks despite having different feature sets.

### FINDING-H-22: D1 detection logic is duplicated

**Severity: LOW** (code quality)

`has_railpack_files` in `validate.rs` and `find_railpack_configs` in `deploy_checks.rs` contain identical logic for finding `railpack-*.json` files. The former returns bool, the latter returns `Vec<PathBuf>`. This duplication means a bug fix in one might not reach the other.

### FINDING-H-23: No check for `cargo test` being conditional on changed files

**Severity: MEDIUM**

Both actual hooks only run `cargo test` when Rust files change. guardrail3's H5 just checks that the string `cargo test` appears in the hook. It does not verify that tests are actually run unconditionally or verify the conditional logic is correct. A hook that has `# cargo test -- disabled for speed` would pass H5.

## Summary

| ID | Severity | Finding |
|---|---|---|
| FINDING-H-01 | CRITICAL | No `--no-verify` bypass detection |
| FINDING-H-02 | CRITICAL | No `set -e` / shell safety validation |
| FINDING-H-03 | HIGH | H5 pattern matching fooled by comments |
| FINDING-H-04 | HIGH | H-TOOL-02 conflict marker false pass via heredoc `<<<` |
| FINDING-H-06 | HIGH | H8 missing core toolchain checks (cargo, pnpm) |
| FINDING-H-10 | HIGH | 6+ actual hook steps have no corresponding guardrail3 check |
| FINDING-H-18 | MEDIUM | H-TOOL-01..05 ignore modular hook scripts |
| FINDING-H-05 | MEDIUM | H-TOOL-03 lockfile check overly broad |
| FINDING-H-07 | MEDIUM | H8 no version checking for tools |
| FINDING-H-08 | MEDIUM | H7 permissions not checked on modular scripts |
| FINDING-H-11 | MEDIUM | Deployment checks skipped for non-standard project layouts |
| FINDING-H-12 | MEDIUM | D2 provider heuristic is filename-based, not content-based |
| FINDING-H-13 | MEDIUM | D3 standalone check is a bare string match |
| FINDING-H-14 | MEDIUM | D4 tracing root value not validated |
| FINDING-H-16 | MEDIUM | No Dockerfile deployment checks |
| FINDING-H-23 | MEDIUM | No validation of test conditionality logic |
| FINDING-H-09 | LOW | H2 misleading error when git not installed |
| FINDING-H-15 | LOW | D5 only covers apps/*/ |
| FINDING-H-17 | LOW | No check for hook shadowing |
| FINDING-H-19 | LOW | Dispatcher pattern false positive via `. ` |
| FINDING-H-20 | LOW | No shebang validation |
| FINDING-H-22 | LOW | Duplicated railpack detection logic |
| FINDING-H-21 | INFO | Hook feature divergence between projects not detected |

**Total: 23 findings (2 CRITICAL, 4 HIGH, 10 MEDIUM, 6 LOW, 1 INFO)**
