# G3TS Help Workspace Adoption

## Summary

Updated top-level `g3ts --help` to explain pnpm, repo init, workspace init, and workspace path selection before users run commands. Added a manifest verifier so the help contract is checked mechanically.

## Decisions Made

- Put the guidance in top-level help instead of only `init workspace --help` because new users start from `g3ts --help`.
- Kept this slice help-only; init scaffolding and repo adoption detection remain separate implementation work.
- Added a dedicated verifier instead of relying on prose review.

## Key Files

- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `.plans/2026-05-19-182155-g3ts-help-workspace-adoption.md`
- `.plans/2026-05-19-182155-g3ts-help-workspace-adoption.md.manifest.toml`
- `scripts/verify-g3ts-help-workspace-adoption.py`

## Verification

- `python3 scripts/verify-g3ts-help-workspace-adoption.py`
- `cargo fmt --manifest-path apps/guardrail3-ts/Cargo.toml --all -- --check`
- `cargo build --quiet --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts --bin g3ts`
- `g3ts validate repo --path .`
- `g3rs validate repo --path .`
- `python3 scripts/verify-g3ts-validate-command-surface.py`

## Next Steps

- Implement repo-level detection for unadopted TypeScript package roots.
- Harden `g3ts init workspace` so it scaffolds the required pnpm workspace contract.
- Add structured toolchain inventory output.
