# G3RS Workspace Scope Error

## Goal

Make `g3rs validate --path <dir>` fail fast when `<dir>` is not a Rust workspace/package root, instead of running partial checks against a repository root that only contains nested Rust workspaces.

## Approach

- Find the `g3rs` validate command entry point.
- Add a unit test for validating a directory without `Cargo.toml` but with nested Rust workspaces.
- Return one explicit error explaining that `g3rs` is workspace scoped and must be run on a directory containing the target Rust `Cargo.toml`.
- Keep existing behavior for valid Rust workspace/package roots unchanged.

## Key Decisions

- Do not try to auto-discover and validate every nested workspace from a non-workspace root. That would mix independent package policies and produce noisy false failures.
- The command should stop before family runners execute, so users get one actionable error instead of hundreds of unrelated findings.

## Files To Modify

- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/*`
- Associated sidecar tests under the same crate.
