# G3TS Setup Flow Hardening

## Goal

Make the G3TS adoption path work from a fresh TypeScript package without requiring the user to infer hidden setup steps from later failures.

## Scope

This slice fixes setup and reporting behavior only. It does not change TypeScript family policy.

## Requirements

- `g3ts validate repo --path <repo> --inventory` must be accepted.
- `g3ts validate repo --path <repo>` must report a TypeScript package root with `package.json` but no `guardrail3-ts.toml` as an unadopted workspace.
- `g3ts validate repo --path <repo> --inventory` must include package-root adoption inventory when package roots are found.
- `g3ts init workspace --path <path>` must scaffold the package-manager contract that current G3TS package and npmrc rules require.
- `g3ts init workspace --path <path>` must not overwrite project-owned files unless `--force` is passed.
- `g3ts init workspace --path <path>` must update generated or previously G3TS-managed files with `--force`.
- Workspace toolchain gates must render command inventory when `--inventory` is passed.
- Workspace toolchain gate failures must render as structured G3TS-style findings, not only raw external stderr.
- If static rules emit `No findings.` and toolchain gates emit findings, the final output must not keep the misleading `No findings.` line.
- Default workspace validation must not run Astro families in a non-Astro workspace.

## Files To Modify

- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/Cargo.toml`
- `scripts/verify-g3ts-setup-flow-hardening.py`

## Verification

- The manifest verifier must build the local `g3ts` binary and exercise real temporary repos.
- The verifier must prove repo inventory exists.
- The verifier must prove an unadopted package root is reported.
- The verifier must prove `init workspace` creates or updates the expected setup files.
- The verifier must prove toolchain inventory shows command lines.
- The verifier must prove non-Astro workspaces do not print Astro family inventory by default.
