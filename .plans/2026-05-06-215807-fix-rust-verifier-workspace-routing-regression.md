# Pre-Commit Routing And Repo-Level Validation: Full Scope

Last updated: 2026-05-07-172216.

## Goal

`.githooks/pre-commit` must route each staged file to the workspace that owns it, run that workspace's checks, and block the commit when routing or required checks regress. Symmetric for Rust and TypeScript. Source-check rules in guardrail3 must enforce the contract so future agent edits cannot silently relax it. Repo-level invariants (hook shape, tools, topology, marker-pair completeness) are validated by `validate-repo`. Per-workspace `validate --path <ws>` runs the static rule families plus the toolchain gates. With `--staged`, it filters by staged files. No bash verifier scripts.

## Surfaces

Two subcommands per binary, plus the hook. No bash scripts.

| Surface | Purpose |
|---|---|
| `.githooks/pre-commit` | Composition + shared inline checks + discovery loops. |
| `g3rs validate --path <ws>` | Per-workspace: rule families + cargo gates from inside `$path`. The cargo command set is sourced from each family's `hook_contract()` types (single source of truth). |
| `g3rs validate --path <ws> --staged` | Same, filtered by staged files (`git diff --cached --name-only --diff-filter=ACM`). Hook uses this. |
| `g3rs validate-repo` | Repo invariants: hook shape, tools installed, repo-wide topology, marker-pair completeness. |
| `g3ts validate --path <pkg>` / `g3ts validate --path <pkg> --staged` / `g3ts validate-repo` | Mirror. |

Optional convenience flag: `g3rs validate --path <ws> --rules-only` skips the cargo gates and runs only the static rule families (existing behavior). Used in tooling pipelines that want fast static checks separate from toolchain runs.

Deleted:
- `scripts/g3rs/verify` — folded into `g3rs validate --staged`.
- `scripts/g3ts/verify` — folded into `g3ts validate --staged`.

Not introduced:
- No separate `verify` subcommand. Folded into `validate`.
- No `scripts/repo-verify`. Shared inline checks live inline in the hook.

## Adopted unit definition

Adopted Rust unit: directory containing both `Cargo.toml` with `[workspace]` AND sibling `guardrail3-rs.toml`.
Adopted TS unit: directory containing both `package.json` AND sibling `guardrail3-ts.toml`.

Both files of the marker pair must coexist in the same directory. Half-adopted directories are rejected by `validate-repo`.

The unit's directory is the validation root. `g3rs validate --path <dir>` and `g3ts validate --path <dir>` cd into it.

Adopted units never nest. Enforced by topology rules (inward-only).

## Owning unit of a staged file

Walk upward from the staged file's directory. The first ancestor that is an adopted unit is the owning unit. Stop at repo root.

Files with no owning adopted unit are skipped silently. Validators only see inputs inside the validated unit; the hook mirrors this. Prototypes, standalone publishable crates, scripts, root configs are intentionally unmanaged.

Adopted units never nest, so the upward walk yields at most one owning unit per staged file.

## Topology rules

All inward-only. Run from `validate-repo` only (not from per-workspace `validate --path`).

| Rule | Trigger |
|---|---|
| `g3rs-topology/no-nested-workspaces` | descendant `Cargo.toml` with `[workspace]` inside an outer workspace |
| `g3rs-topology/no-nested-guardrail3-rs-toml` | descendant `guardrail3-rs.toml` inside an outer adopted unit |
| `g3ts-topology/no-nested-guardrail3-ts-toml` | descendant `guardrail3-ts.toml` inside an outer adopted unit |

## Pre-commit hook

`.githooks/pre-commit`:

1. Resolve `REPO_ROOT` and export `CARGO_TARGET_DIR`.
2. Collect staged files with `git diff --cached --name-only --diff-filter=ACM`. Exit 0 if empty.
3. Shared inline checks (one block per check):
   - Merge-conflict marker scan over staged files (regex form `<{7}` / `={7}` / `>{7}` plus literal seven-character forms).
   - `gitleaks protect --staged --no-banner` after verifying gitleaks is installed.
   - Staged file size cap (1 MB).
   - Migration consistency: forbid modifying existing `drizzle/*` files; require new `drizzle/*.sql` when `db/schema/*.ts` is staged.
   - Lockfile integrity: per staged `package.json`, run the appropriate lock check.
4. Call `g3rs validate-repo`. Exit non-zero if it fails.
5. Call `g3ts validate-repo`. Exit non-zero if it fails.
6. Rust discovery loop:
   - Filter staged files to Rust-relevant.
   - For each, walk upward to nearest adopted Rust unit (marker pair). Skip silently when no owning unit.
   - Dedup.
   - For each unique unit: `g3rs validate --path "$unit" --staged`.
7. TypeScript discovery loop: mirror of step 6 with TS marker pair and `g3ts validate --path "$unit" --staged`.
8. Final success line.

The hook does not invoke `cargo`, `pnpm`, `npm`, `tsc`, `eslint`, `prettier`, `cspell`, `stylelint` directly. All toolchain work happens inside `g3rs validate --path <ws> --staged` and `g3ts validate --path <pkg> --staged`.

The hook does not contain hardcoded scope literals, env-override default substitutions, command-substitution defaults, default-fallback assignments, ambient-variable scopes, or upward-walk-from-discovered-units patterns.

## g3rs validate (per-workspace)

`g3rs validate --path <path> [--staged] [--rules-only]`. Existing subcommand, extended.

Behavior:

- Resolve `REPO_ROOT` from `git rev-parse --show-toplevel` and canonicalize.
- Normalize `--path` to absolute path under `REPO_ROOT` or accept absolute paths directly.
- Set `CARGO_TARGET_DIR="$REPO_ROOT/.cargo-target"` if not already set.
- Always: run static rule families (apparch, arch, cargo, clippy, code, deps, deny, fmt, garde, release, test, toolchain, per-workspace topology). This is the existing behavior and must not regress.
- Unless `--rules-only`: also run cargo gates derived from each family's `hook_contract()` requirements:
  - `cargo metadata --locked`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo deny check`
  - `cargo machete`
  - `cargo test --workspace`
  - `cargo mutants --check --in-place`
  - `cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests`
- With `--staged`: filter cargo invocations by staged files (`git diff --cached --name-only --diff-filter=ACM`). If no Rust-relevant staged paths inside `--path`, exit 0 without running cargo gates. The static rule families still run.
- The cargo command set is sourced from family `hook_contract()` types. Adding a family adds its commands. Removing a family removes its commands. No drift between source-check rules and the executor.
- Does not call `g3ts`, `pnpm`, `npm`, `yarn`, `bun`.
- Does not discover sibling workspaces. Validates exactly the passed path.
- Does not reject any specific path value.

Per-workspace family opt-out: a workspace's `guardrail3-rs.toml` may disable specific families (e.g. `[mutants] enabled = false`). `g3rs validate` reads that config and skips both the static rule family AND its cargo gates for disabled families.

## g3ts validate (per-package)

Mirror of `g3rs validate` for TypeScript.

`g3ts validate --path <path> [--staged] [--rules-only]`. Existing subcommand, extended.

- Always: run static rule families (eslint, astro, tsconfig, package, npmrc, jscpd, style, fmt, spelling, typecov, per-package topology, etc.).
- Unless `--rules-only`: also run TS toolchain gates derived from family `hook_contract()` types:
  - typecheck (package's `typecheck` script if present, else `tsc -p tsconfig.json --noEmit`)
  - lint (package's `lint` script if present, else `eslint --max-warnings 0`)
  - format check (package's `format:check` / `check:format` script if present, else `prettier --check`)
  - spelling (package's `spellcheck` script if present, else `cspell --no-progress --no-summary`)
  - stylelint when style family enabled
  - syncpack when package family enabled
  - type-coverage when typecov family enabled
- With `--staged`: filter by staged files. Exit 0 if no TS-relevant staged paths inside `--path`. Static rules still run.
- Does not call `g3rs`, `cargo`.
- Does not discover sibling packages.
- Does not reject any specific path value.

Per-package family opt-out via `guardrail3-ts.toml`.

## g3rs validate-repo

New subcommand. Runs against repo root (no `--path`). Checks:

- `.githooks/pre-commit` exists, is executable, satisfies hook source-check rules.
- Required Rust tools installed: gitleaks, cargo-deny, cargo-machete, cargo-mutants, cargo-dupes.
- Topology across all adopted Rust workspaces in the repo: no nesting (`g3rs-topology/no-nested-guardrail3-rs-toml`, `g3rs-topology/no-nested-workspaces`).
- All adopted Rust workspaces marker-pair-complete.

Severity: every required-shape violation is `Severity::Error`. The command exits non-zero on any violation.

## g3ts validate-repo

Mirror for TypeScript:

- Hook satisfies TS hook source-check rules.
- Required TS tools installed: pnpm (or detected package manager), gitleaks (independently from RS).
- Topology across all adopted TS packages: no nesting (`g3ts-topology/no-nested-guardrail3-ts-toml`). Wire into `g3ts` CLI.
- All adopted TS packages marker-pair-complete.

## g3rs validate --path <workspace>

Existing subcommand. Workspace-internal concerns only.

After this slice:
- Removed: `hooks` family.
- Removed: tool-presence rules (`required-tools-installed`, `contract-required-tools-installed`). Moved to `validate-repo`.
- Topology family at this scope checks per-workspace topology only (descendants inside the validated workspace). Repo-wide nesting is checked by `validate-repo`.
- Other rule families (apparch, arch, cargo, clippy, code, deps, deny, fmt, garde, release, test, toolchain) unchanged.

No hook ingestion. No verifier ingestion. No upward walk.

## g3ts validate --path <package>

Mirror.

## Source-check rule contract

### Sovereignty

`g3rs-hooks-source-checks` and `g3ts-hooks-source-checks` are independent. No shared crate, no shared types. Each family checks the same `.githooks/pre-commit` against its own copy of the rule.

### Hook routing rules (each language, against the hook)

All emit `Severity::Error` on violation. The rules apply to the hook's language-specific routing block (the rule for RS family checks the Rust loop and the calls to `g3rs validate-repo` and `g3rs validate`; same for TS).

- Hook calls `g3rs validate-repo` (or `g3ts validate-repo` for TS family) before any per-workspace dispatch.
- Hook contains the shared inline checks (the language family checks for the inline-shape of merge-conflict scan, gitleaks invocation, file-size cap; TS family additionally checks for migration consistency and lockfile integrity).
- Hook reads staged files with `git diff --cached --name-only --diff-filter=ACM`.
- Hook walks upward to find owning adopted unit (marker pair literals present).
- Hook dedups owning units before invoking the verifier.
- Hook skips silently when no owning unit (lenient policy; reject any fail-closed).
- Hook half-adopted detection: rule fires when only one of marker pair is referenced.
- Hook scope is a discovery-loop variable. Disallowed: hardcoded path literals, ambient variables (`$REPO_ROOT`, `$PWD`, `$HOME`, `$SCOPE`), env-override default substitutions (`${VAR:-default}`), command-substitution defaults (`$(... || echo <literal>)`), default-fallback assignments to scope-feeder variables (`if [ -z "$X" ]; then X=<literal>; fi`, including variable-prefixed literals like `$REPO_ROOT/apps/...`).
- Hook does not call the verifier on ancestor adopted units (no upward-walk-from-discovered-units).
- Hook does not invoke any language toolchain directly (rule fires if `cargo`, `pnpm`, `npm`, `tsc`, `eslint`, `prettier`, `cspell`, `stylelint` is invoked from the hook).

### Verifier-body rules

Removed entirely. The verifier is binary code, not a shell script. The cargo / TS toolchain command set is owned by the binary's source code, validated by:
- The binary's own unit tests proving each required command is invoked.
- An integration test that runs `g3rs validate --mode workspace --scope <test-fixture>` and asserts the expected commands ran.

No shell-text-scraping rules looking at a verifier script. No `verifier-runs-cargo-deny-check`-style rules.

### Tool-presence rules

Live inside `validate-repo` only. Removed from per-workspace `validate --path`.

RS family checks: gitleaks, cargo-deny, cargo-machete, cargo-mutants, cargo-dupes installed.
TS family checks: pnpm (or detected manager), gitleaks installed.

### Topology rules

Live inside `validate-repo` only.

### Marker-pair completeness rules

Live inside `validate-repo` only. Each `validate-repo` walks the repo and rejects any directory with one half of a marker pair but not the other.

### Test pinning

Every failure-mode test pins `Severity::Error` via `assert_error_finding`. Synthesized fixtures plus real-artifact `include_str!` tests against the actual `.githooks/pre-commit`. Each artifact has at least one test that injects a known regression and asserts the expected rule fires `Severity::Error`.

## Implementation order

Six waves. Inside each wave, agents run in parallel where files do not overlap.

### Wave R1: extend g3rs validate + delete bash script

- Extend `g3rs validate --path <ws>` to also run the cargo gates derived from family `hook_contract()` types, in addition to the existing static rule families.
- Add `--staged` flag: filters cargo gates by staged files (`git diff --cached --name-only --diff-filter=ACM`); exit 0 if no Rust-relevant staged paths inside `--path`. Static rules still run.
- Add `--rules-only` flag: skip cargo gates, run static rules only (preserves the existing behavior for tooling pipelines that want fast static checks).
- Per-workspace family opt-out from `guardrail3-rs.toml` (skips both the static rule family and its cargo gates for disabled families).
- Delete `scripts/g3rs/verify`.

### Wave R2: g3rs validate-repo subcommand + family split

- Add `validate-repo` subcommand to `g3rs` CLI.
- Move hooks ingestion / source-checks dispatch from per-workspace `validate --path` into `validate-repo`.
- Move tool-presence rules into `validate-repo`.
- Move repo-wide topology dispatch into `validate-repo`.
- Add marker-pair completeness rules into `validate-repo`.
- Per-workspace `validate --path` no longer dispatches the hooks family or tool-presence rules. Per-workspace topology stays per-workspace; repo-wide topology moves up.
- Remove or gate the upward-walk path in `g3rs-hooks-ingestion` so it is only used by `validate-repo`.

### Wave R3: g3rs hook source-check rules updated for the new contract

- Update `g3rs-hooks-source-checks` rules:
  - Require call to `g3rs validate-repo` in the hook.
  - Require shared inline checks present in the hook (rule looks at the hook for the shapes; was previously expected in `repo-verify` in the discarded plan version).
  - Require call to `g3rs validate --path <loop var> --staged` per discovered unit.
  - Keep all routing rules (hardcoded scope, ambient vars, env-override patterns, half-adopted, dedup, skip-silent, upward-walk-from-discovered-units, --diff-filter=ACM).
  - Remove all `verifier-runs-*` rules and `verifier-precommit-reads-staged-files` (the verifier is now in-binary, not a script).
  - Add `dedups-owning-units` and `skips-when-no-owning-unit` rules to mirror TS family.
- Add real-artifact tests:
  - Real `.githooks/pre-commit` passes all hook rules.
  - Each regression scenario (10 RS) asserts `Severity::Error` via `assert_error_finding`.
- Bump severity of every failure case to Error per the existing pattern.

### Wave T1: extend g3ts validate + delete bash script

- Extend `g3ts validate --path <pkg>` to also run TS toolchain gates derived from family `hook_contract()` types, in addition to the existing static rule families.
- Add `--staged` and `--rules-only` flags (mirror RS).
- Per-package family opt-out from `guardrail3-ts.toml`.
- Delete `scripts/g3ts/verify` (this also removes the `discover_scopes` and repo-root-rejection violations).

### Wave T2: g3ts validate-repo subcommand + family split

- Add `validate-repo` subcommand to `g3ts` CLI. Add `topology` to family enum.
- Move hooks, tool-presence, repo-wide topology, marker-pair completeness into `validate-repo`.
- Per-package `validate --path` strips hooks family and tool-presence.
- Wire `g3ts-topology/no-nested-guardrail3-ts-toml` into the topology family dispatch in `validate-repo`.

### Wave T3: g3ts hook source-check rules updated + parity with RS

- Update `g3ts-hooks-source-checks` rules to match the new hook contract:
  - Require call to `g3ts validate-repo` and `g3ts validate`.
  - Require shared inline checks in the hook.
  - All routing rules at `Severity::Error`.
- Bring detectors to parity with RS:
  - Env-override detection covers `${VAR:-default}`, command-substitution defaults, default-fallback assignments (literal and variable-prefixed literal).
  - Half-adopted detection on routing-discovers-marker-pair.
  - Merge-conflict regex form recognition (`<{7}` etc., not just literal seven-character runs).
  - Disallowed ambient scope variables (`$REPO_ROOT`, `$PWD`, `$HOME`, `$SCOPE`).
- Add `assert_error_finding` test pinning for every failure-mode test.
- Add real-artifact tests including 10 TS injection scenarios.
- Remove TS-side `verifier-*` script-content rules (verifier is in-binary).

### Wave S: Hygiene

- Fix `g3ts-hooks-source-checks-runtime` clippy debt to clippy-clean.
- Sweep for any remaining dead `#[cfg(test)]`-only modules in any source-check crate. Delete or wire.
- Verify all RS Cargo workspaces created earlier have adoption packs (already done; double-check).

### Wave I: Integration verification

After all waves above land:

- Build `g3rs` and `g3ts` binaries from source.
- `g3rs validate-repo` against the repo: exit 0, zero errors.
- `g3ts validate-repo` against the repo: exit 0, zero errors.
- `g3rs validate --path apps/guardrail3-rs`: exit 0, no `hooks` section in output.
- `g3ts validate --path apps/guardrail3-ts`: exit 0, no `hooks` section in output.
- `g3rs validate --mode workspace --scope apps/guardrail3-rs`: exit 0.
- `g3ts validate --mode workspace --scope apps/guardrail3-ts`: exit 0 (or fails cleanly with specific tool-missing errors that point at fixable causes).
- 20 empirical regression injection scenarios (10 RS, 10 TS) against `.githooks/pre-commit`:
  - Hardcoded scope literal.
  - $REPO_ROOT-prefixed hardcoded scope.
  - Ambient $REPO_ROOT scope after discovery loop.
  - Env-override `${VAR:-default}`.
  - Command-substitution default `$(... || echo <literal>)`.
  - Default-fallback assignment with literal.
  - Default-fallback assignment with variable-prefixed literal.
  - Hook directly invokes language toolchain (regression toward inline cargo/pnpm).
  - Required shared inline check removed from hook.
  - Half-adopted reference (hook tests one of marker pair only).
- Each scenario must produce exit 1 with `Severity::Error` finding citing the expected rule, after `g3rs validate-repo` (or `g3ts validate-repo`) is run.
- Real `git commit` simulation: stage a file, inject one regression, attempt commit, assert blocked. Restore.
- md5 verification of `.githooks/pre-commit` against pre-attack original after every injection.

### Wave A: Adversarial review to convergence

Independent reviewer reads the plan and the diff. Uses the contract below verbatim. Findings become tasks. Loop affected waves -> Wave I -> Wave A until reviewer reports nothing.

## Adversarial review contract

Reviewer prompt:

```text
Read .plans/2026-05-06-215807-fix-rust-verifier-workspace-routing-regression.md and verify that the implementation:

1. Validates that staged files are routed to the unit that owns them, separately for Rust and TypeScript. Both languages must be empirically gated end-to-end against 10 regression scenarios each.
2. Rejects any implementation that hardcodes one Rust workspace or one TS package as the verifier scope.
3. Rejects environment-override routing in any form: `${VAR:-default}` brace syntax, `$(... || echo <literal>)` command substitution, and `if [ -z "$X" ]; then X=<literal-or-variable-prefixed-literal>; fi` default-fallback assignments.
4. Rejects ambient-variable scopes ($REPO_ROOT, $PWD, $HOME, $SCOPE) used directly or via the patterns above.
5. Rejects upward-walk-from-discovered-units in the hook.
6. Rejects shared package or coupling between g3rs-hooks-source-checks and g3ts-hooks-source-checks. Each family must enforce its own copy of the shared inline check rules independently.
7. Verifies topology nesting rules are inward-only and run from validate-repo, not from per-workspace validate.
8. Verifies half-adopted directories (marker-pair incomplete) are rejected by validate-repo.
9. Verifies g3rs validate and g3ts validate (the binary subcommands) do NOT discover sibling units, do NOT reject specific scopes, do NOT call the other language's toolchain. The bash scripts scripts/g3rs/verify and scripts/g3ts/verify are deleted.
10. Verifies the hook skips silently when a language-relevant staged file has no owning adopted unit (lenient policy; reject any fail-closed).
11. Verifies validate --path <workspace> does NOT include the hooks family or tool-presence checks.
12. Verifies validate-repo includes hook content checks, tools, repo-wide topology, marker-pair completeness across the repo.
13. Verifies every failure-mode test pins Severity::Error via assert_error_finding.
14. Verifies real-artifact tests exist via include_str! against .githooks/pre-commit. Synthesized-fixture-only coverage is insufficient for any production-gating rule.
15. Verifies sovereignty: zero imports between g3rs-hooks-source-checks and g3ts-hooks-source-checks; zero shared types.
16. Verifies the hook does not invoke any language toolchain directly. All toolchain work is inside `g3rs validate` and `g3ts validate` binary subcommands.

Find every gap. Reproduce empirically. Do not categorize anything as not-introduced-by-this-slice or out-of-scope unless this plan explicitly says so. The plan does not defer any item. Every visible defect is in scope.
```

Every adversarial finding becomes a fix task. Loop until convergence.

## Files to modify or create

Hook:
- `.githooks/pre-commit` (rewrite to 8-step composition with shared inline checks inline)

Bash scripts (delete):
- `scripts/g3rs/verify` (delete)
- `scripts/g3ts/verify` (delete)

Rust binary changes:
- `apps/guardrail3-rs/crates/io/inbound/cli/...` (add `validate-repo` subcommand; extend `validate` with `--staged` and `--rules-only` flags)
- `apps/guardrail3-rs/crates/logic/validate-command/...` (split repo-level vs per-workspace dispatch; remove `hooks` family and tool-presence from per-workspace validate)
- `apps/guardrail3-rs/crates/logic/family-runner-process/...` (split runners between repo and workspace contexts as needed; add a workspace-toolchain runner that invokes cargo)
- `packages/rs/hooks/g3rs-hooks-ingestion/...` (gate upward-walk to repo-level mode only)
- `packages/rs/hooks/g3rs-hooks-source-checks/...` (update rules per Wave R3; add `dedups-owning-units` and `skips-when-no-owning-unit` rules; remove `verifier-runs-*` rules; add real-artifact tests; pin Severity::Error via assert_error_finding)
- `packages/rs/hooks/g3rs-hooks-config-checks/...` (verify tool-presence rules fire only at repo level)

TypeScript binary changes:
- `apps/guardrail3-ts/crates/io/inbound/cli/...` (add `validate-repo` subcommand; extend `validate` with `--staged` and `--rules-only` flags; add `topology` to family enum)
- `apps/guardrail3-ts/crates/logic/validate-command/...` (split repo-level vs per-workspace; wire `g3ts-topology` family)
- `packages/ts/hooks/g3ts-hooks-ingestion/...` (mirror RS upward-walk gating)
- `packages/ts/hooks/g3ts-hooks-source-checks/...` (Wave T3 rule updates; parity detectors with RS; remove `verifier-*` script-content rules; assert_error_finding pinning; real-artifact tests)
- `packages/ts/hooks/g3ts-hooks-config-checks/...` (TS tool-presence at repo level)

Per-workspace `guardrail3-rs.toml` and `guardrail3-ts.toml`:
- Extend schema with family-disable settings if not already present (e.g. `[mutants] enabled = false`). `g3rs validate` and `g3ts validate` read these.

Plan and worklog:
- This plan file (already updated).
- New worklog at commit time covering the full slice.

## Out of scope

Nothing.

## Commit rule

One commit covering the full slice after:
- All implementation waves R1, R2, R3, T1, T2, T3, S complete.
- Wave I integration verification passes for all 20 scenarios with empirical evidence.
- Wave A adversarial review converges (reviewer reports no findings).
- A worklog covering the complete scope is staged with the code.
