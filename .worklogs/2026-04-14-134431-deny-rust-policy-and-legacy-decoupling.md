# Summary
Removed the deny family's dependency on deleted `apps/guardrail3` modules and replaced the dead `guardrail3.toml` policy path with Rust-only `guardrail3-rs.toml`. Added deny baseline data inside the package, updated typed inputs to carry Rust policy state, and tightened tests so profile-sensitive deny rules are proven against explicit fixtures instead of self-generated baselines.

# Decisions made
- Moved deny baseline data into `g3rs-deny-config-checks` as package-owned constants instead of preserving any runtime import from the archived app.
- Replaced `profile_name` plus `policy_context_valid` with typed `G3RsDenyRustPolicyState` so the boundary distinguishes `Missing`, `Unreadable`, `ParseError`, and `Parsed`.
- Kept missing Rust policy valid for deny and treated it as default service behavior, matching the previous package behavior where absent context did not suppress profile-sensitive checks.
- Routed deny filetree parse failures through `guardrail3-rs.toml` messages and removed all live `guardrail3.toml` handling except one explicit regression proving legacy config is ignored.
- Replaced self-fulfilling canonical deny fixtures with explicit hardcoded service/library fixtures so baseline drift can break tests.

# Key files for context
- `packages/rs/deny/g3rs-deny-types/src/lib.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/{select.rs,parse.rs,ingest.rs,run.rs}`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/{baseline.rs,support.rs,test_support.rs}`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/runtime/src/test_support.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/ingest_tests/{basic.rs,filetree.rs}`

# Next steps
- Fix `garde` the same way: remove remaining imports from deleted app crates and delete the dead universal config path.
- After `garde`, rerun `cargo test -q` in `apps/guardrail3-rs` to surface the next real package still coupled to archived app code.
