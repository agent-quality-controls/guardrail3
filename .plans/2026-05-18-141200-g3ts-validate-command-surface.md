# G3TS Validate Command Surface

## Goal

Make the G3TS CLI expose the same validate command model as the current G3RS verifier architecture:

- `g3ts validate repo --path <repo>`
- `g3ts validate workspace --path <workspace>`

The current G3TS surface still exposes:

- `g3ts validate --path <workspace>`
- `g3ts validate-repo --path <repo>`

That is not an install-only problem. The old shape exists in the current source:

- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`

## Required End State

### CLI

- `g3ts validate repo --path <repo>` validates repo-level invariants.
- `g3ts validate workspace --path <workspace>` validates one adopted TypeScript workspace.
- `g3ts validate workspace` keeps the existing workspace flags:
  - `--family`
  - `--inventory`
  - `--staged`
  - `--rules-only`
- `g3ts validate repo` keeps repo root discovery when `--path` is omitted.
- `g3ts validate-repo` is removed.
- Bare `g3ts validate --path <path>` is removed.
- `g3ts --version` works and prints the package version.

### Behavior

- `g3ts validate workspace --path <workspace>` must run the same code path that `g3ts validate --path <workspace>` runs today.
- `g3ts validate repo --path <repo>` must run the same code path that `g3ts validate-repo --path <repo>` runs today.
- Repo validation remains repo-level validation. It must not silently become "validate every workspace".
- Workspace validation remains one-workspace validation. It must not silently validate the repo.
- `--rules-only` remains a workspace-only debug/fixture flag. It must not exist on `g3ts validate repo`.

### Hook Contract

- TS hook checks must require `g3ts validate repo`, not `g3ts validate-repo`.
- TS hook checks must require per-unit `g3ts validate workspace --path <unit> --staged`, not `g3ts validate --path <unit> --staged`.
- Error messages must say the exact command to put in `.githooks/pre-commit`.
- Hook checks must continue to require staged-file-to-owning-workspace routing through `package.json` plus `guardrail3-ts.toml`.

### Documentation

- G3TS CLI docs must document `g3ts validate repo`.
- G3TS CLI docs must document `g3ts validate workspace`.
- G3TS CLI docs must not document `g3ts validate-repo`.
- G3TS CLI docs must not document bare `g3ts validate --path`.
- G3TS setup docs must state that normal project validation must run both:
  - repo validation
  - workspace validation for each adopted TypeScript workspace that the project wants checked outside pre-commit

### Fixture Coverage

- G3TS fixtures must include at least one repo-level fixture that proves `g3ts validate repo` emits the hook/topology/tooling findings.
- G3TS fixtures must include at least one workspace-level fixture that proves `g3ts validate workspace` emits an existing workspace family finding.
- G3TS fixtures must include a hook-source fixture using the new command strings.
- Existing fixture expected outputs must be updated only for command spelling changes, not rule meaning changes.

### Install Debuggability

- `g3ts --version` must work so a target repo can prove which installed binary it is running.
- Any release/install guide must include one exact local install command for this repo.
- The guide must say that `g3ts --help` must show `validate repo` and `validate workspace` after install.

## Non-Goals

- Do not implement a G3TS hook generator in this plan.
- Do not change Slopless files in this plan.
- Do not decide the final Drizzle migration policy in this plan.
- Do not add a compatibility alias for `validate-repo`.
- Do not make repo validation recursively run every workspace.

## Key Decisions

- The fix belongs in G3TS, not in Slopless.
  Slopless exposed the issue, but current G3TS source still has the old command model.

- Repo validation and workspace validation stay separate.
  They validate different contracts and should be independently callable.

- No backwards compatibility alias is planned.
  Keeping both `validate-repo` and `validate repo` would keep docs, hooks, and agent instructions ambiguous.

- `--version` is required.
  Without it, "installed CLI is stale" cannot be separated from "source still has old behavior".

## Files To Modify

- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`
- G3TS hook fixture files under `behavior/fixtures/g3ts-rule/hooks`
- G3TS CLI fixture files under `behavior/fixtures/g3ts-cli-output`
- G3TS repo-validation fixture files under `behavior/fixtures/g3ts-validate-repo`
- G3TS docs that mention `validate`, `validate-repo`, or hook setup
- `scripts/behavior/verify-g3ts-family-rule-fixtures.py` only if command invocation is hardcoded there
- `scripts/behavior/fixture3-g3ts-fixture-replay.py` only if command invocation is hardcoded there

## Verification

Run these after implementation:

```bash
python3 scripts/verify-g3ts-validate-command-surface.py
cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml
fixture3 check --all
python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py
g3ts --help
g3ts --version
g3ts validate repo --path .
```

Expected CLI checks:

- `g3ts --help` shows nested `validate`.
- `g3ts validate --help` shows `repo` and `workspace`.
- `g3ts validate-repo --help` fails.
- `g3ts validate workspace --help` shows workspace flags.
- `g3ts validate repo --help` does not show workspace-only flags.
