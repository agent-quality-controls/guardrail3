# Adversarial Audit: hooks/ and deploy/ Rule Plans

Audit date: 2026-03-21

---

## 1. hooks/shared.md (10 rules) — Missing Rules

### HOOK-SHARED-11: Shebang validation (Warn)
**What:** Pre-commit script (and every script in pre-commit.d/) must start with a valid shebang (`#!/bin/bash`, `#!/usr/bin/env bash`, `#!/bin/sh`). A script without a shebang may execute in an unexpected shell or fail silently on some systems.
**Why missing matters:** macOS ships with zsh as default; a script without `#!/bin/bash` that uses bash-isms will break. In modular mode, individual scripts in pre-commit.d/ sourced via `. script` inherit the parent shell, but scripts invoked directly (via `run-parts` or explicit execution) need their own shebang.

### HOOK-SHARED-12: Modular script permissions (Warn)
**What:** Every file in pre-commit.d/ must have executable permission. Currently HOOK-SHARED-05 only checks the top-level pre-commit script.
**Why missing matters:** If using `run-parts` or direct invocation (`./pre-commit.d/01-fmt.sh`), non-executable scripts are silently skipped. This is a common failure mode on fresh clones where `git` doesn't preserve execute bits by default.

### HOOK-SHARED-13: Proper exit code handling (Warn)
**What:** The pre-commit script must not contain `exit 0` at the end (which masks failures from earlier steps). In monolithic mode, the last command's exit code should propagate. With `set -e`, an explicit `exit 0` at the end masks any late failures that might slip through traps.
**Why missing matters:** An agent adding a debugging `exit 0` at the bottom effectively disables the entire hook. This is a real bypass vector.

### HOOK-SHARED-14: No `--no-verify` bypass instructions (Info)
**What:** The hook script and any script in pre-commit.d/ must not contain comments like `# git commit --no-verify` or `# use --no-verify to skip`. These comments teach users (and agents) how to bypass the hook.
**Why missing matters:** Agents that read hook scripts may learn to use `--no-verify` from these comments. This is a social-engineering vector against the guardrail system.

### Considered and rejected

- **Maximum hook execution time:** Not enforceable from a static check. Would require runtime instrumentation or wrapping commands with `timeout`. Out of scope for a static validator.
- **Modular pre-commit.d/ scanning for patterns:** Already handled by `check_modular_scripts` which concatenates all script contents and runs the same pattern checks. The plan accurately reflects this via HOOK-SHARED-07 (inventory) and the per-language pattern checks in HOOK-RS / HOOK-TS.

---

## 2. hooks/rs.md (7 rules) — Missing Rules

### HOOK-RS-08: guardrail3 validate step present (Warn)
**What:** The pre-commit hook should contain `guardrail3 rs validate` or `guardrail3 validate`. This is the AST-based tamper detection step (catches `#[allow]` without reason, garde skip, etc.) that replaces grep-based checks.
**Why missing matters:** The CLAUDE.md explicitly states "The hook runs `guardrail3 rs validate --staged`". The generated hook includes this step. But no rule verifies its presence. An agent could remove the guardrail3 step and all other Rust checks would still pass — but source-level tamper detection would be gone.
**Note:** The implementation's `HOOK_PATTERN_CHECKS` does NOT include a `guardrail3` pattern. This is a real gap in both the plan and the code.

### HOOK-RS-09: cargo clippy uses `-D warnings` (Warn)
**What:** Not just "clippy step present" but "clippy step includes `-D warnings` or `--deny warnings`". Running clippy without `-D warnings` means warnings don't fail the commit.
**Why missing matters:** `cargo clippy` without `-D warnings` exits 0 even with warnings. An agent could change `cargo clippy -- -D warnings` to just `cargo clippy` and the HOOK-RS-02 check would still pass (it only checks for the pattern "cargo clippy" or "clippy").

### HOOK-RS-10: cargo test uses `--workspace` (Info)
**What:** For workspace projects, `cargo test` should include `--workspace` to test all crates, not just the root.
**Why missing matters:** `cargo test` without `--workspace` only tests the root package. In a workspace with multiple crates, this silently skips tests for library crates.

### Considered and rejected

- **cargo fmt scope (--all vs staged):** Both `cargo fmt --check` and `cargo fmt --check --all` are valid. Staged-only formatting is handled by lint-staged patterns in TS, but Rust doesn't have an equivalent. The current "cargo fmt present" check is sufficient; scope is a style choice.

---

## 3. hooks/ts.md (7 rules) — Missing Rules

### Plan/code mismatch: ESLint and tsc ARE checked in code but NOT in the plan

The `HOOK_PATTERN_CHECKS` array already contains:
- `["tsc", "--noEmit"]` with `severity_if_missing: Warn, requires_ts: true`
- `["eslint"]` with `severity_if_missing: Warn, requires_ts: true`

These are real checks in the running code but are completely absent from hooks/ts.md. The plan has 7 rules and none of them are ESLint or tsc. These need to be added to the plan to match reality:

### HOOK-TS-08: ESLint step present (Warn) — EXISTS IN CODE, MISSING FROM PLAN
**What:** Pre-commit hook contains an `eslint` invocation.
**Status:** Implemented in `HOOK_PATTERN_CHECKS` but not documented in the plan file.

### HOOK-TS-09: TypeScript type-check step present (Warn) — EXISTS IN CODE, MISSING FROM PLAN
**What:** Pre-commit hook contains `tsc` or `--noEmit` invocation.
**Status:** Implemented in `HOOK_PATTERN_CHECKS` but not documented in the plan file.

### HOOK-TS-10: knip unused exports step present (Info) — NEW
**What:** Pre-commit hook contains a `knip` invocation for detecting unused exports, types, and dependencies.
**Why missing matters:** knip is the successor to ts-prune and detects dead code at the project level (unused exports, unused dependencies, unused files). For TS projects this is the equivalent of cargo-machete. Without it, dead exports accumulate.

### HOOK-TS-11: next build / next lint step present (Info) — NEW
**What:** For Next.js projects (detected by presence of next.config.*), check that the hook contains `next build` or `next lint`.
**Why missing matters:** ESLint alone doesn't catch Next.js-specific issues (invalid page exports, bad getServerSideProps signatures, etc.). `next lint` runs ESLint with Next.js-specific rules. `next build` catches type errors that `tsc --noEmit` misses due to Next.js compiler transforms.
**Scope:** Info severity only — running `next build` in pre-commit is slow, so this is advisory.

### Considered and rejected

- **Separate knip as Error severity:** knip is not universally adopted yet. Info is appropriate.

---

## 4. deploy/ts.md (5 rules) — Missing Rules

### DEPLOY-TS-06: Health check endpoint configured (Warn)
**What:** For Railway deployments (railpack config present), check that `railpack-*.json` contains a `healthcheckPath` field, or that `package.json` has a health check script.
**Why missing matters:** Railway restarts services that fail health checks. Without a configured health check path, Railway uses TCP connect which doesn't verify the app is actually serving. A deployed service that starts but crashes on first request will appear healthy.

### DEPLOY-TS-07: Railway.toml or railway.json configuration (Info)
**What:** Check for `railway.toml` or `railway.json` in the project root for Railway-specific settings (build command, start command, environment variables, region).
**Why missing matters:** Without explicit Railway config, Railway uses heuristics to detect the build system. For monorepos, this often picks the wrong app or wrong build command. Explicit config prevents deployment surprises.

### DEPLOY-TS-08: Dockerfile present when no railpack config (Info)
**What:** If no `railpack-*.json` exists AND no `Dockerfile` exists, emit Info. If `Dockerfile` exists, validate it has a multi-stage build pattern and doesn't run as root (no `USER` directive = warn).
**Why missing matters:** Projects without railpack AND without a Dockerfile rely entirely on Railway's auto-detection, which is fragile. This is an "are you sure?" check.

### DEPLOY-TS-09: Environment variable manifest (.env.example or .env.template) (Warn)
**What:** Check that `.env.example`, `.env.template`, or a documented list of required environment variables exists in the project root.
**Why missing matters:** Without a manifest, deployments fail because someone forgot to set `DATABASE_URL` or `NEXT_PUBLIC_API_URL`. This is the #1 cause of "works locally, breaks in prod." The manifest doesn't need to contain values — just the variable names.

### DEPLOY-TS-10: next.config.* has no `experimental` overrides (Info)
**What:** If Next.js is detected, check that `next.config.*` does not contain `experimental:` settings that may not be supported in production or may change behavior between Next.js versions.
**Why missing matters:** Experimental flags are unstable by definition. An agent that enables `experimental.serverActions` in Next.js 14 will find it moved to a stable flag in Next.js 15, causing build failures on upgrade.

### Considered and rejected

- **SSL/TLS configuration:** Railway handles TLS termination automatically. Checking for TLS config would be misleading — users should NOT configure TLS themselves on Railway.
- **Vercel/Netlify config:** guardrail3 is opinionated toward Railway. Adding Vercel/Netlify support would dilute focus. If needed later, it should be a separate deploy profile, not bolted onto DEPLOY-TS.
- **docker-compose validation:** docker-compose is a development tool, not a deployment artifact. Railway doesn't use it. Checking it would be a dev-environment concern, not a deployment concern.
- **Full Dockerfile linting:** Tools like hadolint already do this. guardrail3's philosophy is "enforce that the right tools are configured, don't re-implement them." A future rule could check that hadolint is configured, rather than linting Dockerfiles directly.

---

## Summary

| Section | Existing | Missing (new) | Missing (plan/code mismatch) | Total proposed |
|---------|----------|---------------|------------------------------|----------------|
| hooks/shared | 10 | 4 | 0 | 14 |
| hooks/rs | 7 | 3 | 0 | 10 |
| hooks/ts | 7 | 2 | 2 (ESLint + tsc in code but not plan) | 11 |
| deploy/ts | 5 | 5 | 0 | 10 |
| **Total** | **29** | **14** | **2** | **45** |

### Priority ranking (most impactful first)

1. **HOOK-RS-08** (guardrail3 validate step) — This is the meta-guardrail. Without it, all source-level tamper detection is bypassable by removing one line from the hook.
2. **HOOK-TS-08/09** (plan/code mismatch) — The plan is wrong, not the code. Fix the plan to match reality.
3. **HOOK-RS-09** (clippy -D warnings) — Without this, clippy warnings don't fail commits.
4. **HOOK-SHARED-13** (no exit 0 bypass) — Direct bypass vector.
5. **DEPLOY-TS-09** (env var manifest) — #1 deployment failure cause.
6. **HOOK-SHARED-11** (shebang) — Cross-platform correctness.
7. **HOOK-SHARED-12** (modular permissions) — Silent failure mode.
8. **DEPLOY-TS-06** (health check) — Silent production failure.
9. **HOOK-SHARED-14** (no-verify comments) — Anti-social-engineering.
10. **DEPLOY-TS-07** (Railway config) — Deployment reliability.
11. **HOOK-RS-10** (cargo test --workspace) — Test coverage gap.
12. **HOOK-TS-10** (knip) — Dead code detection.
13. **HOOK-TS-11** (next build/lint) — Next.js-specific.
14. **DEPLOY-TS-08** (Dockerfile fallback) — Advisory only.
15. **DEPLOY-TS-10** (no experimental) — Advisory only.
