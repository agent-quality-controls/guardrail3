Goal

- Make cargo allow-waiver rules stand down when `guardrail3-rs.toml` is unreadable or malformed, instead of misreporting missing waivers.
- Prove the behavior with direct rule tests and one config-pipeline regression.

Approach

- Add failing rule tests for `g3rs-cargo/approved-allow-inventory`, `11`, and `12` covering unreadable and parse-error rust policy state.
- Fix the rules in `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src` at the rule boundary using existing typed rust-policy validity, not by changing test fixtures or app behavior.
- Add one ingestion pipeline regression in `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` proving malformed `guardrail3-rs.toml` does not cascade into missing-waiver findings from those allow rules.
- Re-run cargo package tests and `git diff --check`.

Key decisions

- Fix in the cargo config rules, not ingestion.
  - Reason: the bug is semantic misattribution inside the allow-waiver rules after ingestion has already preserved invalid rust-policy state correctly.
- Treat unreadable and parse-error rust policy the same for these rules.
  - Reason: both states block trustworthy waiver resolution.

Files to modify

- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_07_approved_allow_inventory.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_11_unapproved_allow_entries.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_12_member_local_allows_forbidden.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_07_approved_allow_inventory_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_11_unapproved_allow_entries_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_12_member_local_allows_forbidden_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
