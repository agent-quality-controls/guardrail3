Summary

- Hardened cargo allow-waiver rules so unreadable or malformed `guardrail3-rs.toml` no longer gets misreported as missing waivers.
- Added direct rule regressions and one config-pipeline regression to prove the stand-down behavior.

Decisions made

- Fixed the bug in the cargo config rules, not ingestion.
  - Reason: ingestion already preserved invalid rust-policy state correctly; the bug was that rules treated invalid state like an empty waiver set.
- Made `RS-CARGO-CONFIG-07`, `11`, and `12` stand down on unreadable and parse-error rust policy.
  - Rejected: synthesizing special placeholder waiver data or changing missing-policy semantics.
- Added one pipeline proof instead of duplicating end-to-end coverage for every rule.
  - Reason: rule-local tests now pin the per-rule behavior; the pipeline test only needs to prove the config lane does not pile on those findings.

Key files for context

- `.plans/2026-04-14-150955-cargo-rust-policy-invalid-allow-rules.md`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_07_approved_allow_inventory.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_11_unapproved_allow_entries.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_12_member_local_allows_forbidden.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

Next steps

- Move to the next family still carrying dead universal-config debt.
- Keep using `guardrail3-rs.toml` as the only live Rust policy file and reject any fallback to deleted app config.
