# G3TS Setup Flow Hardening

## Summary

Fixed the G3TS adoption path exposed by Slopless: repo validation now reports unadopted package roots, workspace init writes the package-manager contract G3TS requires, toolchain failures render as G3TS findings, and non-Astro workspaces no longer show Astro inventory by default.

## Decisions

- `validate repo --inventory` is supported so agents can inspect repo-level adoption state without guessing which repo checks ran.
- `init workspace` now updates `package.json` and writes `.npmrc`, `.syncpackrc`, `pnpm-workspace.yaml`, and `guardrail3-ts.toml` because the validator already requires those contracts.
- Toolchain gates now emit structured `g3ts-toolchain/*` inventory and errors, including the command that ran.
- Astro families are skipped only for default workspace validation when the workspace does not declare Astro through config, package metadata, or `[astro]` policy. Explicit `--family astro-*` still runs.
- Package metadata detection uses the existing `package-json-parser` instead of ad hoc JSON parsing.
- `init.rs` was split at the package.json mutation boundary after G3RS correctly rejected the oversized file.

## Key Files

- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init_package_json.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/guardrail3-rs.toml`
- `.plans/2026-05-19-183426-g3ts-setup-flow-hardening.md`
- `scripts/verify-g3ts-setup-flow-hardening.py`

## Verification

- `python3 scripts/verify-g3ts-setup-flow-hardening.py`
- `python3 scripts/verify-g3ts-help-workspace-adoption.py`
- `python3 scripts/verify-g3ts-validate-command-surface.py`
- `cargo fmt --manifest-path apps/guardrail3-ts/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `fixture3 check --all --json`
- `g3ts validate repo --path .`
- `g3rs validate repo --path .`
- `apps/guardrail3-ts/target/debug/g3ts validate workspace --path /Users/tartakovsky/Projects/agent-quality-controls/slopless --rules-only --inventory`

## Next Steps

- Slopless still has real G3TS package setup findings: missing `private: true`, incomplete engines, missing `.npmrc`, incomplete pnpm policy, and incomplete Syncpack banned dependency policy.
