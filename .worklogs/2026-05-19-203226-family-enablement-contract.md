# Family Enablement Contract

## Summary

Implemented the shared G3TS/G3RS family enablement model: workspace families are default-on, `[checks].family = false` is the only opt-out, and runners no longer infer framework applicability from workspace files. Added behavior fixtures and a verifier so the contract is checked mechanically.

## Decisions Made

- Removed the G3TS Astro auto-skip because framework detection in the runner duplicated family rule ownership.
- Made G3TS workspace config parsing match G3RS: missing or invalid `guardrail3-ts.toml` fails before family execution.
- Passed enabled family sets through the family runner trait so hooks and toolchain gates can use the same selection as normal rules.
- Added section-level family disable hints in both plain-text renderers instead of duplicating opt-out text inside every rule crate.
- Added adoption-marker `guardrail3-ts.toml` files to old G3TS fixtures that were meant to test specific families, not missing workspace adoption.

## Key Files

- `.plans/2026-05-19-194309-family-enablement-contract.md`
- `.plans/2026-05-19-194309-family-enablement-contract.md.manifest.toml`
- `scripts/verify-family-enablement-contract.py`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/crates/runtime/src/toolchain_gates.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/io/outbound/report/crates/runtime/src/plain_text.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text.rs`
- `behavior/fixtures/g3ts-rule/family-enablement`
- `behavior/fixtures/g3rs-rule/family-enablement`

## Verification

- `python3 scripts/verify-family-enablement-contract.py`
- `cargo fmt --manifest-path apps/guardrail3-ts/Cargo.toml --all -- --check`
- `cargo fmt --manifest-path apps/guardrail3-rs/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo clippy --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `fixture3 check --all --json`
- `g3ts validate repo --path .`
- `g3rs validate repo --path .`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --bin g3rs --force`

## Next Steps

- None for this contract.
