# Independent Verifier Scripts

## Status

This is the active verifier implementation plan.

Superseded plans:

- `.plans/2026-05-06-122051-guardrail-verifier-scripts.md`
- `.plans/2026-05-06-123032-split-guardrail-verifier-scripts.md`
- `.plans/2026-05-06-123748-independent-verifier-contracts.md`

## Goal

Add per-tool verifier scripts so agents can run the same required checks without attempting a commit.

Each guardrail tool is independent:

- `g3rs` owns Rust verification only.
- `g3ts` owns TypeScript verification only.
- neither tool knows the other exists.
- `.githooks/pre-commit` is app-owned shell composition.
- each tool only requires its own verifier line in the hook.

## Final Command Contract

Rust:

```sh
scripts/g3rs/verify --mode pre-commit --scope apps/guardrail3-rs
scripts/g3rs/verify --mode workspace --scope apps/guardrail3-rs
```

TypeScript:

```sh
scripts/g3ts/verify --mode pre-commit --scope apps/landing
scripts/g3ts/verify --mode workspace --scope apps/landing
```

Meaning:

- `--mode pre-commit` verifies the staged commit gate for one tool and one scope.
- `--mode workspace` verifies the current filesystem state for one tool and one scope.
- `--scope` is the app, package, or workspace root that the tool owns.

## Required Scripts

Rust script:

- `scripts/g3rs/verify`

TypeScript script:

- `scripts/g3ts/verify`

Both scripts must:

- use `#!/usr/bin/env bash`
- use `set -euo pipefail`
- resolve repo root with `git rev-parse --show-toplevel`
- require `--mode`
- require `--scope`
- accept only `pre-commit` and `workspace`
- reject unknown flags
- reject unknown modes
- reject missing scope
- normalize `--scope` to an absolute path before running tool commands
- return non-zero when any required command fails

## Pre-Commit Hook Shape

Rust-only hook:

```sh
#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

"$REPO_ROOT/scripts/g3rs/verify" --mode pre-commit --scope "$REPO_ROOT/apps/guardrail3-rs"
```

TypeScript-only hook:

```sh
#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

"$REPO_ROOT/scripts/g3ts/verify" --mode pre-commit --scope "$REPO_ROOT/apps/landing"
```

Mixed hook:

```sh
#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

"$REPO_ROOT/scripts/g3rs/verify" --mode pre-commit --scope "$REPO_ROOT/apps/guardrail3-rs"
"$REPO_ROOT/scripts/g3ts/verify" --mode pre-commit --scope "$REPO_ROOT/apps/landing"
```

Hook rules:

- Rust hook checks require only the `scripts/g3rs/verify` line.
- TypeScript hook checks require only the `scripts/g3ts/verify` line.
- no tool requires a shared verifier.
- no tool requires a unified dispatcher.

## Rust Verifier Behavior

File:

- `scripts/g3rs/verify`

### Rust `pre-commit` Mode

Input:

```sh
git diff --cached --name-only --diff-filter=ACM
```

Behavior:

- collect staged paths.
- exit 0 if no staged paths are relevant to Rust.
- treat these as Rust-relevant:
  - `*.rs`
  - `Cargo.toml`
  - `Cargo.lock`
  - Rust config files currently recognized by Rust hook checks
- when Rust-relevant staged paths exist, run the Rust workspace verifier commands below.
- when staged `*.rs` paths exist, run the Rust duplication command below.

Rust workspace verifier commands:

```sh
g3rs validate --path "$SCOPE"
cargo metadata --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo machete
cargo test --workspace
cargo mutants --check --in-place
```

Rust duplication command:

```sh
cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests
```

Environment:

```sh
export CARGO_TARGET_DIR="$REPO_ROOT/.cargo-target"
```

Working directory:

- run Cargo commands from `$SCOPE`.
- run `g3rs validate --path "$SCOPE"` from repo root or from any directory where it behaves the same.

### Rust `workspace` Mode

Input:

- current filesystem under `$SCOPE`.
- no staged-path filtering.

Behavior:

- run all Rust workspace verifier commands.
- run the Rust duplication command.
- do not read staged paths to decide whether to run.
- do not claim the staged commit passes.

## TypeScript Verifier Behavior

File:

- `scripts/g3ts/verify`

### TypeScript `pre-commit` Mode

Input:

```sh
git diff --cached --name-only --diff-filter=ACM
```

Behavior:

- collect staged paths.
- exit 0 if no staged paths are relevant to TypeScript.
- treat these as TypeScript-relevant:
  - `*.ts`
  - `*.tsx`
  - `*.mts`
  - `*.cts`
  - `*.mjs`
  - `*.cjs`
  - `*.js`
  - `*.jsx`
  - `*.css`
  - `package.json`
  - lockfiles used by the repo package manager
  - TypeScript, ESLint, prettier, cspell, stylelint, syncpack, typecov, and g3ts config files
- route staged files to delegated tools using the existing TypeScript hook behavior.
- run app/package validation only for the configured `$SCOPE`.

Required TypeScript verifier categories:

- `g3ts validate --path "$SCOPE"` or a checked complete workspace validate route.
- typecheck.
- lint.
- format check.
- spelling check.
- stylelint when style family is enabled.
- package policy when package family is enabled.
- type coverage when typecov family is enabled.

### TypeScript `workspace` Mode

Input:

- current filesystem under `$SCOPE`.
- no staged-path filtering.

Behavior:

- run all TypeScript verifier categories enabled for `$SCOPE`.
- do not read staged paths to decide whether to run.
- do not claim the staged commit passes.

## Guardrail Rule Changes

### Rust Hook Rules

Rust hooks must enforce:

- `.githooks/pre-commit` calls `scripts/g3rs/verify --mode pre-commit --scope <scope>`.
- `scripts/g3rs/verify` exists.
- `scripts/g3rs/verify` supports `--mode pre-commit`.
- `scripts/g3rs/verify` supports `--mode workspace`.
- `scripts/g3rs/verify` rejects missing `--scope`.
- `scripts/g3rs/verify` rejects unknown modes.
- `scripts/g3rs/verify` runs `g3rs validate --path "$SCOPE"`.
- `scripts/g3rs/verify` runs each required Rust command listed in this plan.
- `scripts/g3rs/verify` exports `CARGO_TARGET_DIR="$REPO_ROOT/.cargo-target"`.
- `scripts/g3rs/verify` does not call `g3ts`.
- `scripts/g3rs/verify` does not call TypeScript package managers as part of Rust verification.

Rust hook ingestion must read:

- `.githooks/pre-commit`
- `scripts/g3rs/verify`

Rust hook ingestion must not read:

- `scripts/g3ts/verify`
- any `scripts/guardrails/*` verifier.

### TypeScript Hook Rules

TypeScript hooks must enforce:

- `.githooks/pre-commit` calls `scripts/g3ts/verify --mode pre-commit --scope <scope>`.
- `scripts/g3ts/verify` exists.
- `scripts/g3ts/verify` supports `--mode pre-commit`.
- `scripts/g3ts/verify` supports `--mode workspace`.
- `scripts/g3ts/verify` rejects missing `--scope`.
- `scripts/g3ts/verify` rejects unknown modes.
- `scripts/g3ts/verify` runs `g3ts validate --path "$SCOPE"` or a checked complete workspace validate route.
- `scripts/g3ts/verify` runs each enabled TypeScript verifier category listed in this plan.
- `scripts/g3ts/verify` does not call `g3rs`.
- `scripts/g3ts/verify` does not call Cargo as part of TypeScript verification.

TypeScript hook ingestion must read:

- `.githooks/pre-commit`
- `scripts/g3ts/verify`

TypeScript hook ingestion must not read:

- `scripts/g3rs/verify`
- any `scripts/guardrails/*` verifier.

## Tests To Add

### Rust Tests

Add unit tests for Rust hook rules:

- hook passes when it calls `scripts/g3rs/verify --mode pre-commit --scope <scope>`.
- hook fails when it does not call `scripts/g3rs/verify`.
- hook fails when the `scripts/g3rs/verify` line omits `--mode pre-commit`.
- hook fails when the `scripts/g3rs/verify` line omits `--scope`.
- verifier fails when missing `g3rs validate --path "$SCOPE"`.
- verifier fails when missing `cargo metadata --locked`.
- verifier fails when missing `cargo fmt --all -- --check`.
- verifier fails when clippy omits `-D warnings`.
- verifier fails when missing `cargo deny check`.
- verifier fails when missing `cargo machete`.
- verifier fails when missing `cargo test --workspace`.
- verifier fails when missing `cargo mutants --check --in-place`.
- verifier fails when cargo dupes omits thresholds.
- verifier fails when cargo dupes omits `--exclude-tests`.
- verifier fails when it calls `g3ts`.
- verifier fails when it calls `pnpm`, `npm`, `yarn`, or `bun`.
- Rust hook rules pass when `scripts/g3ts/verify` does not exist.

### TypeScript Tests

Add unit tests for TypeScript hook rules:

- hook passes when it calls `scripts/g3ts/verify --mode pre-commit --scope <scope>`.
- hook fails when it does not call `scripts/g3ts/verify`.
- hook fails when the `scripts/g3ts/verify` line omits `--mode pre-commit`.
- hook fails when the `scripts/g3ts/verify` line omits `--scope`.
- verifier fails when missing `g3ts validate --path "$SCOPE"` or the checked complete validate route.
- verifier fails when missing typecheck.
- verifier fails when missing lint.
- verifier fails when missing format check.
- verifier fails when missing spelling check.
- verifier fails when missing stylelint while style family is enabled.
- verifier fails when missing package policy while package family is enabled.
- verifier fails when missing type coverage while typecov family is enabled.
- verifier fails when it calls `g3rs`.
- verifier fails when it calls `cargo`.
- TypeScript hook rules pass when `scripts/g3rs/verify` does not exist.

### Script Behavior Tests

Add script tests for each verifier:

- missing `--mode` exits non-zero.
- missing `--scope` exits non-zero.
- unknown mode exits non-zero.
- `--mode worktree` exits non-zero.
- `--mode files` exits non-zero.
- `--mode pre-commit` exits zero when no relevant staged files exist.
- `--mode workspace` does not inspect staged paths before running checks.

## Implementation Steps

1. Mark this plan as active.
2. Do not implement `scripts/guardrails/*`.
3. Implement `scripts/g3rs/verify` for Rust.
4. Update Rust hook ingestion to parse `.githooks/pre-commit` and `scripts/g3rs/verify`.
5. Update Rust hook checks and tests.
6. Implement `scripts/g3ts/verify` for TypeScript when TypeScript hook work is in scope.
7. Update TypeScript hook ingestion to parse `.githooks/pre-commit` and `scripts/g3ts/verify`.
8. Update TypeScript hook checks and tests.
9. Run mechanical verification.
10. Send adversarial agents with this plan and the code.
11. Fix every adversarial finding.

## Parallel Delegation Plan

Two implementation agents should run in parallel:

- G3RS agent
- G3TS agent

They must receive this plan file as the source of truth.

They must not implement from a summary.

### G3RS Agent Workload

Goal:

- implement the Rust verifier contract without adding any TypeScript dependency or awareness.

Write scope:

- `scripts/g3rs/verify`
- `packages/rs/hooks/g3rs-hooks-types`
- `packages/rs/hooks/g3rs-hooks-ingestion`
- `packages/rs/hooks/g3rs-hooks-source-checks`
- Rust hook tests under the same package structure used by existing hook rules
- Rust worklog for this change

Read scope:

- this plan file
- `.githooks/pre-commit`
- `.plans/todo/checks/hooks/rs.md`
- `.plans/todo/checks/hooks/shared.md` only to understand existing hook concepts, not to add a shared verifier
- current Rust hook packages under `packages/rs/hooks`
- current shell parser package used by Rust hook ingestion

Required implementation:

- create `scripts/g3rs/verify`
- support `--mode pre-commit`
- support `--mode workspace`
- require `--scope`
- reject missing `--mode`
- reject missing `--scope`
- reject unknown modes
- reject unknown flags
- normalize `--scope` to an absolute path
- set `REPO_ROOT` from `git rev-parse --show-toplevel`
- set `CARGO_TARGET_DIR="$REPO_ROOT/.cargo-target"`
- in `pre-commit` mode:
  - read staged files with `git diff --cached --name-only --diff-filter=ACM`
  - exit 0 when no Rust-relevant staged paths exist
  - treat staged `*.rs`, `Cargo.toml`, `Cargo.lock`, and existing Rust config patterns as Rust-relevant
  - run Rust workspace verifier commands when Rust-relevant staged paths exist
  - run cargo dupes when staged `*.rs` paths exist
- in `workspace` mode:
  - do not read staged paths to decide whether to run
  - run all Rust workspace verifier commands
  - run cargo dupes

Required Rust workspace verifier commands:

```sh
g3rs validate --path "$SCOPE"
cargo metadata --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo machete
cargo test --workspace
cargo mutants --check --in-place
```

Required Rust duplication command:

```sh
cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests
```

Required Rust hook ingestion changes:

- ingest `.githooks/pre-commit`
- ingest `scripts/g3rs/verify`
- parse both through the existing shell parser
- expose typed facts needed by Rust hook rules
- do not ingest `scripts/g3ts/verify`
- do not ingest any `scripts/guardrails/*` verifier

Required Rust hook rules:

- `.githooks/pre-commit` calls `scripts/g3rs/verify --mode pre-commit --scope <scope>`
- `scripts/g3rs/verify` exists
- `scripts/g3rs/verify` supports `--mode pre-commit`
- `scripts/g3rs/verify` supports `--mode workspace`
- `scripts/g3rs/verify` rejects missing `--scope`
- `scripts/g3rs/verify` rejects unknown modes
- `scripts/g3rs/verify` runs `g3rs validate --path "$SCOPE"`
- `scripts/g3rs/verify` runs every required Rust workspace verifier command
- `scripts/g3rs/verify` runs the Rust duplication command with thresholds and `--exclude-tests`
- `scripts/g3rs/verify` exports `CARGO_TARGET_DIR="$REPO_ROOT/.cargo-target"`
- `scripts/g3rs/verify` does not call `g3ts`
- `scripts/g3rs/verify` does not call `pnpm`, `npm`, `yarn`, or `bun`

Required Rust tests:

- hook passes when it calls `scripts/g3rs/verify --mode pre-commit --scope <scope>`
- hook fails when it does not call `scripts/g3rs/verify`
- hook fails when the Rust verifier line omits `--mode pre-commit`
- hook fails when the Rust verifier line omits `--scope`
- verifier facts fail when missing `g3rs validate --path "$SCOPE"`
- verifier facts fail when missing `cargo metadata --locked`
- verifier facts fail when missing `cargo fmt --all -- --check`
- verifier facts fail when clippy omits `-D warnings`
- verifier facts fail when missing `cargo deny check`
- verifier facts fail when missing `cargo machete`
- verifier facts fail when missing `cargo test --workspace`
- verifier facts fail when missing `cargo mutants --check --in-place`
- verifier facts fail when cargo dupes omits thresholds
- verifier facts fail when cargo dupes omits `--exclude-tests`
- verifier facts fail when it calls `g3ts`
- verifier facts fail when it calls `pnpm`, `npm`, `yarn`, or `bun`
- Rust hook rules pass when `scripts/g3ts/verify` does not exist

Mechanical verification required before handoff:

- Rust hook package tests pass
- `g3rs validate --path apps/guardrail3-rs` passes, or every failure is reported with exact rule and reason
- `scripts/g3rs/verify --mode pre-commit --scope apps/guardrail3-rs` runs without shell syntax errors
- `scripts/g3rs/verify --mode workspace --scope apps/guardrail3-rs` runs or reports missing external tools precisely

Forbidden for G3RS agent:

- do not edit `scripts/g3ts/verify`
- do not edit TypeScript hook packages
- do not create `scripts/guardrails/*`
- do not add a shared verifier
- do not add `worktree` mode
- do not add `files` mode

### G3TS Agent Workload

Goal:

- implement the TypeScript verifier contract without adding any Rust dependency or awareness.

Write scope:

- `scripts/g3ts/verify`
- `packages/ts/hooks/g3ts-hooks-types`
- `packages/ts/hooks/g3ts-hooks-ingestion`
- `packages/ts/hooks/g3ts-hooks-source-checks`
- TypeScript hook tests under the same package structure used by existing hook rules
- TypeScript worklog for this change

Read scope:

- this plan file
- `.githooks/pre-commit`
- current TypeScript hook packages under `packages/ts/hooks`
- current shell parser package used by TypeScript hook ingestion
- current TypeScript family configs only where needed to detect enabled verifier categories

Required implementation:

- create `scripts/g3ts/verify`
- support `--mode pre-commit`
- support `--mode workspace`
- require `--scope`
- reject missing `--mode`
- reject missing `--scope`
- reject unknown modes
- reject unknown flags
- normalize `--scope` to an absolute path
- set `REPO_ROOT` from `git rev-parse --show-toplevel`
- in `pre-commit` mode:
  - read staged files with `git diff --cached --name-only --diff-filter=ACM`
  - exit 0 when no TypeScript-relevant staged paths exist
  - treat staged TypeScript, JavaScript, CSS, package, lockfile, and TypeScript-tooling config files as TypeScript-relevant
  - route staged files to delegated tools using the existing TypeScript hook behavior
  - run app/package validation only for the configured `$SCOPE`
- in `workspace` mode:
  - do not read staged paths to decide whether to run
  - run every enabled TypeScript verifier category for `$SCOPE`

Required TypeScript verifier categories:

- `g3ts validate --path "$SCOPE"` or a checked complete workspace validate route
- typecheck
- lint
- format check
- spelling check
- stylelint when style family is enabled
- package policy when package family is enabled
- type coverage when typecov family is enabled

Required TypeScript hook ingestion changes:

- ingest `.githooks/pre-commit`
- ingest `scripts/g3ts/verify`
- parse both through the existing shell parser
- expose typed facts needed by TypeScript hook rules
- do not ingest `scripts/g3rs/verify`
- do not ingest any `scripts/guardrails/*` verifier

Required TypeScript hook rules:

- `.githooks/pre-commit` calls `scripts/g3ts/verify --mode pre-commit --scope <scope>`
- `scripts/g3ts/verify` exists
- `scripts/g3ts/verify` supports `--mode pre-commit`
- `scripts/g3ts/verify` supports `--mode workspace`
- `scripts/g3ts/verify` rejects missing `--scope`
- `scripts/g3ts/verify` rejects unknown modes
- `scripts/g3ts/verify` runs `g3ts validate --path "$SCOPE"` or a checked complete workspace validate route
- `scripts/g3ts/verify` runs every enabled TypeScript verifier category
- `scripts/g3ts/verify` does not call `g3rs`
- `scripts/g3ts/verify` does not call `cargo`

Required TypeScript tests:

- hook passes when it calls `scripts/g3ts/verify --mode pre-commit --scope <scope>`
- hook fails when it does not call `scripts/g3ts/verify`
- hook fails when the TypeScript verifier line omits `--mode pre-commit`
- hook fails when the TypeScript verifier line omits `--scope`
- verifier facts fail when missing `g3ts validate --path "$SCOPE"` or checked complete validate route
- verifier facts fail when missing typecheck
- verifier facts fail when missing lint
- verifier facts fail when missing format check
- verifier facts fail when missing spelling check
- verifier facts fail when missing stylelint while style family is enabled
- verifier facts fail when missing package policy while package family is enabled
- verifier facts fail when missing type coverage while typecov family is enabled
- verifier facts fail when it calls `g3rs`
- verifier facts fail when it calls `cargo`
- TypeScript hook rules pass when `scripts/g3rs/verify` does not exist

Mechanical verification required before handoff:

- TypeScript hook package tests pass
- relevant `g3ts validate` command passes, or every failure is reported with exact rule and reason
- `scripts/g3ts/verify --mode pre-commit --scope <known-ts-scope>` runs without shell syntax errors
- `scripts/g3ts/verify --mode workspace --scope <known-ts-scope>` runs or reports missing external tools precisely

Forbidden for G3TS agent:

- do not edit `scripts/g3rs/verify`
- do not edit Rust hook packages
- do not create `scripts/guardrails/*`
- do not add a shared verifier
- do not add `worktree` mode
- do not add `files` mode

### Parent Agent Integration Workload

Goal:

- review both implementation branches against this plan and keep cross-tool boundaries clean.

Parent write scope:

- `.githooks/pre-commit`
- plan updates if implementation reveals a real design gap
- integration worklog
- small conflict fixes between independent patches

Parent responsibilities:

- ensure `.githooks/pre-commit` has one line per enabled tool
- ensure Rust and TypeScript verifier lines are independent
- ensure no `scripts/guardrails/*` verifier exists
- ensure Rust code does not inspect TypeScript verifier files
- ensure TypeScript code does not inspect Rust verifier files
- run mechanical verification for both toolchains
- send adversarial review agents with this plan and the final code
- convert every adversarial finding into a fix task
- do not report done until adversarial review converges

## Do Not Do

- Do not create a unified verifier.
- Do not create `verify-shared`.
- Do not create `scripts/guardrails/lib`.
- Do not make `g3rs` inspect `scripts/g3ts/verify`.
- Do not make `g3ts` inspect `scripts/g3rs/verify`.
- Do not add `worktree` mode.
- Do not add `files` mode.
- Do not call the current-filesystem mode `current`.
- Do not call the target flag `--workspace`.
- Do not imply `workspace` mode proves the staged commit passes.
- Do not imply `pre-commit` mode proves the current filesystem passes.

## Error Messages

Rust missing hook line:

```text
g3rs-hooks/precommit-calls-g3rs-verifier: .githooks/pre-commit does not run the Rust verifier script for pre-commit mode. Add this line after REPO_ROOT is set: "$REPO_ROOT/scripts/g3rs/verify" --mode pre-commit --scope "$REPO_ROOT/apps/guardrail3-rs".
```

Rust unsupported mode:

```text
g3rs-hooks/g3rs-verifier-supported-modes: scripts/g3rs/verify must support exactly these modes: pre-commit, workspace. Remove unsupported mode <mode>.
```

TypeScript missing hook line:

```text
g3ts-hooks/precommit-calls-g3ts-verifier: .githooks/pre-commit does not run the TypeScript verifier script for pre-commit mode. Add this line after REPO_ROOT is set: "$REPO_ROOT/scripts/g3ts/verify" --mode pre-commit --scope "$REPO_ROOT/apps/landing".
```

TypeScript unsupported mode:

```text
g3ts-hooks/g3ts-verifier-supported-modes: scripts/g3ts/verify must support exactly these modes: pre-commit, workspace. Remove unsupported mode <mode>.
```
