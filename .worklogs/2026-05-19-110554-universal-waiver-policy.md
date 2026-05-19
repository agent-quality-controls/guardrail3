Summary:
- Added a shared guardrail TOML waiver package used by both RS and TS parser schemas.
- Removed RS family-local waiver structs and matcher logic from the families that already consume waivers.
- Added visible waiver support for `g3rs-deps/direct-dependency-cap` with a fixture proving unwaived error and waived warning behavior.

Decisions made:
- Kept `guardrail3-rs.toml` and `guardrail3-ts.toml` as separate root schemas because their policy sections are language-specific.
- Shared only the common TOML waiver type and exact matcher in `g3-guardrail-toml-types`.
- Did not implement runner-level suppression because current check results do not carry a selector and silent suppression would hide escape hatches.
- Removed the manual `Eq` implementation from the shared waiver type. Structs that carry TOML `Value` extras should not claim `Eq`.
- Kept TS waiver behavior parser-only for now. TS rules can opt into the shared matcher when a concrete rule needs waivers.
- Updated parser workspace guardrail policies to allow the shared TOML package explicitly.
- Moved `g3rs-toml-parser` to Cargo resolver 3 so it satisfies the rust-version-aware resolver contract.
- Updated every staged Rust workspace touched by the shared waiver dependency to pass resolver and dependency allowlist gates.
- Removed an obsolete `clippy::excessive_nesting` expectation in garde ingestion after the waiver migration lowered the nesting enough for the expectation to become unfulfilled.

Key files for context:
- `.plans/2026-05-19-103450-universal-waiver-policy.md`
- `.plans/2026-05-19-103450-universal-waiver-policy.md.manifest.toml`
- `scripts/verify-universal-waiver-policy.py`
- `packages/parsers/g3-guardrail-toml-types/src/lib.rs`
- `packages/parsers/g3rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/parsers/g3ts-toml-parser/crates/types/src/guardrail3_ts_toml.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/direct_dependency_cap/rule.rs`
- `behavior/fixtures/g3rs-rule/deps/deps-R22-direct-dependency-cap-waived`
- `behavior/golden/g3rs-rule/approved.normalized.json`

Verification:
- `python3 scripts/verify-universal-waiver-policy.py`
- `cargo fmt --all --manifest-path packages/parsers/g3-guardrail-toml-types/Cargo.toml -- --check`
- `cargo test --workspace --manifest-path packages/parsers/g3-guardrail-toml-types/Cargo.toml`
- `cargo clippy --workspace --all-targets --all-features --manifest-path packages/parsers/g3-guardrail-toml-types/Cargo.toml -- -D warnings`
- `g3rs validate workspace --path packages/parsers/g3-guardrail-toml-types --inventory`
- `cargo test --workspace --manifest-path packages/parsers/g3rs-toml-parser/Cargo.toml`
- `cargo test --workspace --manifest-path packages/parsers/g3ts-toml-parser/Cargo.toml`
- `g3rs validate workspace --path packages/parsers/g3rs-toml-parser --inventory`
- `g3rs validate workspace --path packages/parsers/g3ts-toml-parser --inventory`
- `cargo fmt --all --manifest-path apps/guardrail3-rs/Cargo.toml -- --check`
- `cargo fmt --all --manifest-path apps/guardrail3-ts/Cargo.toml -- --check`
- `cargo test --workspace --manifest-path apps/guardrail3-rs/Cargo.toml`
- `cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-rs/Cargo.toml -- -D warnings`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-ts/Cargo.toml -- -D warnings`
- `g3rs validate workspace --path <each staged Rust workspace> --inventory`
- `fixture3 check --all --json`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --force --bin g3rs`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --force --bin g3ts`
- `g3rs validate repo --path .`
- `g3ts validate repo --path .`

Next steps:
- None for this plan.
