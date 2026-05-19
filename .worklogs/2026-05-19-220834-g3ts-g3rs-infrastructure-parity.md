# G3TS/G3RS Infrastructure Parity

## Summary

Implemented the shared infrastructure contract for G3TS and G3RS: matching command surfaces, matching front-loaded help structure, init commands that only write setup, scoped validation reports, validate-command ownership of repo validation, and inventory output that does not make clean runs fail.

## Decisions Made

- Moved G3TS repo adoption, marker-pair, required-tool, and exit-code policy into `validate-command`; the CLI runtime now only adapts clap args into request structs.
- Removed G3RS init self-validation so `init repo` and `init workspace` match G3TS and print the next validation command instead of running it.
- Treated satisfied hook inventory rows as `Info`, because inventory is visibility, not failure.
- Kept inventory out of exit-code decisions in both tools by computing exit status from non-inventory findings only.
- Updated the stale G3TS setup-flow verifier manifest after the family-enablement contract removed implicit Astro auto-skip.

## Key Files

- `.plans/2026-05-19-213005-g3ts-g3rs-infrastructure-parity.md`
- `.plans/2026-05-19-213005-g3ts-g3rs-infrastructure-parity.md.manifest.toml`
- `scripts/verify-g3ts-g3rs-infrastructure-parity.py`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/init.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/types/app-types/src/request.rs`
- `apps/guardrail3-ts/crates/types/app-types/src/report.rs`
- `behavior/golden/g3rs-cli-output/approved.normalized.json`
- `behavior/golden/g3ts-rule/approved.normalized.json`

## Verification

- `cargo fmt --manifest-path apps/guardrail3-ts/Cargo.toml --all -- --check`
- `cargo fmt --manifest-path apps/guardrail3-rs/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo clippy --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `python3 scripts/verify-g3ts-g3rs-infrastructure-parity.py`
- `python3 scripts/verify-family-enablement-contract.py`
- `python3 scripts/verify-g3ts-setup-flow-hardening.py`
- `fixture3 check --all --json`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --bin g3rs --force`
- `g3ts validate repo --path .`
- `g3rs validate repo --path .`
- `g3ts validate repo --path . --inventory`
- `g3rs validate repo --path . --inventory`
- `git diff --check`

## Next Steps

- None for this parity slice.
