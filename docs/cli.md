# Guardrail3 CLI

## G3RS Command Tree

```text
g3rs init repo [--path <path>] [--force]
g3rs init workspace --path <path> [--force]
g3rs validate repo [--path <path>] [--inventory]
g3rs validate workspace --path <path> [--family <family>] [--inventory] [--staged] [--rules-only]
```

## Init Vs Validate

- `init` writes setup files or Git config.
- `validate` only reports findings.
- `init repo` writes repo hook setup.
- `init workspace` writes one Rust workspace setup.
- `validate repo` checks that Git will run G3RS and checks repo-level shape.
- `validate workspace` checks one adopted Rust unit.

## Repo Vs Workspace

- `repo` means the Git repository control surface containing `--path`.
- `workspace` means one adopted Rust unit with `Cargo.toml` and `guardrail3-rs.toml`.
- `--path` is the only location flag.

## Hook Setup

`g3rs init repo` creates the managed hook file:

```text
.githooks/pre-commit.d/g3rs
```

Git runs `.githooks/pre-commit`. That file must run `.githooks/pre-commit.d/g3rs`.

## Output Examples

```text
initialized repo
created .githooks/pre-commit
created .githooks/pre-commit.d/g3rs
configured core.hooksPath=.githooks
validated with: g3rs validate repo --path /repo
```

```text
scope: workspace
root: /repo/packages/rs/fmt/g3rs-fmt-types

No findings.
```

## Deleted Command Forms

These deleted command forms are invalid:

```text
g3rs validate-repo
g3rs validate --path <path>
```

## G3TS Command Tree

```text
g3ts --version
g3ts validate repo [--path <path>]
g3ts validate workspace --path <path> [--family <family>] [--inventory] [--staged] [--rules-only]
```

## G3TS Local Install

From this repository root:

```text
cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --force
g3ts --version
g3ts --help
```

After install, `g3ts --help` must show `validate`, and `g3ts validate --help` must show `repo` and `workspace`.

## G3TS Repo Vs Workspace

- `repo` means the Git repository control surface containing `--path`.
- `workspace` means one adopted TypeScript unit with `package.json` and `guardrail3-ts.toml`.
- `validate repo` checks hooks, required tools, repo topology, and marker-pair completeness.
- `validate workspace` checks one adopted TypeScript unit.
- `--rules-only` is a workspace-only debug and fixture flag.

## G3TS Normal Validation

Run both repo and workspace validation in a project verify path:

```text
g3ts validate repo --path <repo>
g3ts validate workspace --path <workspace>
```

For a monorepo, run `g3ts validate workspace --path <workspace>` once for each adopted TypeScript workspace that should be checked outside pre-commit.
