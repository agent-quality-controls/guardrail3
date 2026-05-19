# G3TS Help Workspace Adoption

## Goal

Make `g3ts --help` front-load the adoption workflow so a new user knows which commands to run and where to run them before touching a repo.

## Required Help Contract

- Top-level help must say G3TS requires pnpm-managed TypeScript workspaces.
- Top-level help must say the first command is `g3ts init repo` at the Git repo root.
- Top-level help must say every top-level package root with `package.json` should be managed by G3TS.
- Top-level help must explain workspace placement choices:
  - root package: use `g3ts init workspace --path .`
  - app with I/O: use `apps/<name>` for CLIs, APIs, servers, UI apps, and other executable surfaces
  - library without I/O: use `packages/<name>` for reusable packages consumed by other software
- Top-level help must say to run `g3ts validate workspace --path <path>` for each adopted workspace.
- Top-level help must say to run `g3ts validate repo` after workspace adoption.

## Approach

- Update `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`.
- Add a verifier script that builds the local G3TS binary and checks the top-level help output for the exact adoption contract.
- Keep this change help-only. Do not change init behavior or validation behavior in this slice.

## Files To Modify

- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `scripts/verify-g3ts-help-workspace-adoption.py`

