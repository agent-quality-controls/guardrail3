Goal

- Remove the cargo family's remaining dependency on dead `guardrail3.toml`.
- Move cargo policy routing onto Rust-only `guardrail3-rs.toml`.
- Keep the app untouched and fix the package boundary where the debt lives.

Approach

- Add failing ingestion tests that prove:
  - `guardrail3-rs.toml` drives cargo profile and waiver state.
  - legacy `guardrail3.toml` is ignored.
  - malformed or unreadable `guardrail3-rs.toml` degrades to typed Rust-policy state and filetree failures.
- Replace cargo's root policy fields with typed Rust-policy state in:
  - `packages/rs/cargo/g3rs-cargo-types/src/lib.rs`
  - `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/{select.rs,run.rs,ingest.rs}`
- Update config rules to consume Rust-policy waivers and profile state instead of `escape_hatches`, `profile_name`, and `guardrail_parse_error`.
- Update filetree routing to point at `guardrail3-rs.toml` and keep inventory behavior correct.
- Update tests, README text, and fixture helpers so no active cargo code or tests still reference `guardrail3.toml`.

Key decisions

- Reuse `guardrail3-rs.toml` waivers instead of inventing cargo-specific policy schema.
  - Cargo's current "escape hatch" data is rule/file/selector/reason shaped already, which fits the Rust-only waiver contract.
- Keep the fix inside the cargo package.
  - Rejected: app-layer shims or adapters. The bug is package-local coupling to dead config.
- Use a typed Rust-policy enum rather than preserve dead field names with new semantics.
  - This matches the clippy and deny decoupling shape and removes stale universal-config vocabulary from the public contract.

Files to modify

- `packages/rs/cargo/g3rs-cargo-types/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/select.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/{basic.rs,pipeline.rs}`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/{support.rs,test_support.rs,rs_cargo_config_07_approved_allow_inventory.rs,rs_cargo_config_11_unapproved_allow_entries.rs,rs_cargo_config_12_member_local_allows_forbidden.rs,rs_cargo_config_13_rust_version_policy.rs}`
- `packages/rs/cargo/g3rs-cargo-config-checks/README.md`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/{run.rs,run_tests/mod.rs,rs_cargo_filetree_14_input_failures.rs}`
