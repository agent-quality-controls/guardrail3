Summary

- Removed the cargo family's dependency on dead `guardrail3.toml` semantics and moved it onto typed `guardrail3-rs.toml` Rust policy.
- Kept the app unchanged and fixed the package boundary where cargo still carried legacy waiver/profile state.

Decisions made

- Replaced cargo's legacy root policy fields with typed `G3RsCargoRustPolicyState`.
  - Rejected: keeping `profile_name`, `escape_hatches`, and `guardrail_parse_error` under new semantics.
- Reused `guardrail3-rs.toml` `[[waivers]]` for cargo allow-rule documentation.
  - Rejected: inventing a cargo-only policy file or preserving dead `[[escape_hatches]]`.
- Made unreadable Rust policy respect the crawl snapshot before file reads.
  - Reason: package ingestion should honor the discovered workspace state, not silently recover by re-reading after permissions change in tests.
- Kept one explicit legacy-negative regression.
  - Reason: proving `guardrail3.toml` is ignored is still worth carrying in tests even though the live package no longer uses it.

Key files for context

- `.plans/2026-04-14-144105-cargo-rust-policy-and-legacy-decoupling.md`
- `packages/rs/cargo/g3rs-cargo-types/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/{select.rs,parse.rs,ingest.rs,run.rs}`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/{support.rs,test_support.rs,rs_cargo_config_07_approved_allow_inventory.rs,rs_cargo_config_11_unapproved_allow_entries.rs,rs_cargo_config_12_member_local_allows_forbidden.rs,rs_cargo_config_13_rust_version_policy.rs}`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/{run.rs,rs_cargo_filetree_14_input_failures.rs}`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/{basic.rs,pipeline.rs}`

Next steps

- Move to the next family still carrying dead `guardrail3.toml` debt, which is `fmt`.
- Keep rejecting any active Rust package that depends on deleted `apps/guardrail3` or revives universal-config vocabulary in public types.
