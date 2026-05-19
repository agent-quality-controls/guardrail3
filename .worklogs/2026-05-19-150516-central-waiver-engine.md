# Central Waiver Engine

## Summary

Centralized waiver parsing and application for G3RS and G3TS behind one shared `guardrail3-waivers` package. Rules and family crates now emit normal findings with stable waiver keys; runners apply waivers after family execution.

## Decisions Made

- Replaced the old shared guardrail TOML waiver package with `packages/shared/guardrail3-waivers` because waiver matching is not a parser concern.
- Kept the waiver contract to `rule`, `subject`, `selector`, and `reason`; removed `file` without compatibility aliases.
- Applied waivers centrally in the G3RS and G3TS validate-command runtimes instead of inside family rules or ingestion.
- Added `subject`, `selector`, and `waiver_reason` to `G3CheckResult` so every finding can be waived through the same key shape.
- Removed family-local waiver parsing and matching from Rust rule families that had bespoke support.
- Added a TypeScript fixture proving a G3TS rule can be waived through the same engine.

## Key Files

- `packages/shared/guardrail3-waivers/src/waiver.rs`
- `packages/shared/guardrail3-check-types/src/result.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `packages/parsers/g3rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/parsers/g3ts-toml-parser/crates/types/src/guardrail3_ts_toml.rs`
- `behavior/fixtures/g3ts-rule/fmt/fmt-R40-prettier-config-waived/repo/guardrail3-ts.toml`
- `scripts/verify-central-waiver-engine.py`

## Verification

- `python3 scripts/verify-central-waiver-engine.py`
- `cargo fmt --all --manifest-path apps/guardrail3-rs/Cargo.toml -- --check`
- `cargo fmt --all --manifest-path apps/guardrail3-ts/Cargo.toml -- --check`
- `cargo test --workspace --manifest-path apps/guardrail3-rs/Cargo.toml`
- `cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-rs/Cargo.toml -- -D warnings`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-ts/Cargo.toml -- -D warnings`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --bin g3rs --force`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `fixture3 check --all --json`
- `g3rs validate repo --path .`
- `g3ts validate repo --path .`

## Next Steps

- Use `subject` and `selector` explicitly in new rules when the derived defaults are not stable enough for waiver targeting.
- Keep waiver validation policy outside rule crates; rule crates should only emit findings.
