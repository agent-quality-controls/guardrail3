# G3TS init and G3RS version wiring

## Goal

- `g3ts init repo` bootstraps repo-level TypeScript guardrails in the same modular hook style as `g3rs init repo`.
- `g3ts init workspace --path <path>` bootstraps one adopted TypeScript workspace marker without overwriting project files unless explicitly forced.
- `g3rs --version` works through Clap's built-in version flag.
- `g3ts validate repo --path .` does not treat fixture, reducer, or behavior-test trees as live repo adoption markers.
- Generic TypeScript hook checks do not require Drizzle migration policy in every TypeScript repo.

## Current failures

- `g3rs --version` exits with Clap argument error because the G3RS parser lacks the `version` command attribute.
- `g3ts validate repo --path .` validates `.githooks/pre-commit` directly, so the existing modular `.githooks/pre-commit.d/g3rs` chain cannot host a separate generated G3TS hook without source-check failures.
- `g3ts validate repo --path .` scans `.fixture3/**` and `behavior/fixtures/**` as live repo content, which creates false marker-pair and topology errors from intentionally broken fixtures.
- `g3ts-hooks/migration-consistency` is a generic hooks rule, but Drizzle is an app/database policy. A TypeScript repo without `drizzle/` and `db/schema/` should not need to fake migration checks.

## Approach

### G3RS version

- Add `version` to the G3RS Clap parser in `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`.
- Add a G3RS CLI fixture command for `--version`.
- Extend verifier coverage so the expected output contains `g3rs `.

### G3TS init command

- Add app request types in `apps/guardrail3-ts/crates/types/app-types/src/request.rs`:
  - `AppCommand::Init`
  - `InitCommand::Repo`
  - `InitCommand::Workspace`
  - `InitRepoRequest`
  - `InitWorkspaceRequest`
- Add Clap parsing in `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`:
  - `g3ts init repo [--path <path>] [--force]`
  - `g3ts init workspace --path <path> [--force]`
- Dispatch init from `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`.
- Implement init writing in a new runtime module under `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init.rs`.
- Repo init writes:
  - `.githooks/pre-commit` dispatcher with a managed G3TS block when needed.
  - `.githooks/pre-commit.d/g3ts` managed hook.
  - local Git `core.hooksPath=.githooks`.
- Workspace init writes `guardrail3-ts.toml` beside an existing `package.json`.
- Existing project-owned hook files are not modified without `--force`.
- Generated files are executable where the platform exposes executable bits.

### Modular G3TS hook validation

- Teach `g3ts-hooks-ingestion` to select `.githooks/pre-commit.d/g3ts` as the effective G3TS source when the dispatcher runs it.
- Keep `.githooks/pre-commit` as the file-tree dispatcher target.
- Source checks validate the effective G3TS hook body, not the G3RS-only dispatcher body.
- File-tree checks still validate hook path, dispatcher presence, modular directory, executability, and local override risks.

### Repo walker exclusions

- Add one shared CLI runtime predicate for repo validation walks.
- Exclude generated and test-fixture roots from live repo adoption scanning:
  - `.fixture3`
  - `behavior`
  - `node_modules`
  - `target`
  - `.git`
  - `.cargo-target`
  - `dist`
  - `build`
- Use the predicate in both marker-pair scanning and topology adopted-unit discovery.

### Remove Drizzle from generic hook checks

- Remove `g3ts-hooks/migration-consistency` from generic hook source checks.
- Remove the `pnpm exec drizzle-kit generate` exception from the generic direct-toolchain rule.
- If Drizzle policy is needed later, it belongs in a database or migration family, not in generic TypeScript hooks.

### Generated G3TS hook content

- The managed `.githooks/pre-commit.d/g3ts` hook must:
  - collect staged files using `git diff --cached --name-only --diff-filter=ACM`
  - scan staged files for merge-conflict markers
  - run `gitleaks protect --staged`
  - enforce staged-file size cap with `git cat-file -s`
  - run `g3ts validate repo --path "$repo_root"`
  - discover owning TS units by walking upward to a `package.json` plus `guardrail3-ts.toml` marker pair
  - dedup owning units
  - skip TS-relevant files with no owning adopted unit
  - run `g3ts validate workspace --path "$unit" --staged`
  - run lockfile integrity checks when package manifests are staged

## Files to modify

- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/types/app-types/src/request.rs`
- `apps/guardrail3-ts/crates/types/app-types/src/lib.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/marker_pairs.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/topology.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/lib.rs`
- `packages/ts/hooks/g3ts-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`
- `behavior/fixtures/g3rs-cli-output/**`
- `behavior/fixtures/g3ts-cli-output/**`
- `behavior/golden/**`
- `scripts/verify-g3ts-validate-command-surface.py`
- new verifier script for this plan

## Verification

- `python3 scripts/verify-g3ts-init-and-g3rs-version.py`
- `cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml`
- `cargo test --workspace --manifest-path apps/guardrail3-rs/Cargo.toml`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-ts/Cargo.toml -- -D warnings`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-rs/Cargo.toml -- -D warnings`
- `fixture3 check --all`
- `g3ts init repo --path <temp repo>`
- `g3ts init workspace --path <temp repo>/<workspace>`
- `g3ts validate repo --path .`
- `g3rs --version`
