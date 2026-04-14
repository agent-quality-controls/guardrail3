# Goal
Make the deny family self-contained and Rust-only: no imports from deleted `apps/guardrail3`, no dependence on dead `guardrail3.toml`, and preserve deny rule semantics through package-owned baseline data and `guardrail3-rs.toml` policy state.

# Approach
1. Read the exact deny runtime support, profile-sensitive rules, test helpers, and ingestion parse/select logic.
2. Add failing tests proving:
   - deny no longer needs legacy app modules to define its baseline behavior
   - `guardrail3-rs.toml` drives the Rust profile context
   - legacy `guardrail3.toml` is ignored
   - parse errors on `guardrail3-rs.toml` degrade through deny's typed policy state correctly
3. Replace legacy deny baseline imports with package-owned baseline data in `g3rs-deny-config-checks`.
4. Replace `guardrail3.toml` ingestion with `guardrail3-rs.toml` ingestion in `g3rs-deny-ingestion` and update public types if needed.
5. Run deny package tests, then adversarial review, then commit with a worklog.

# Key Decisions
- Keep the fix inside the deny package boundary, not in the app adapter.
- Use package-owned baseline constants/data instead of depending on legacy archived modules.
- Treat `guardrail3-rs.toml` as the only Rust policy file. `guardrail3.toml` is dead and must be ignored.

# Files To Modify
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/support.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/test_support.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_23_ban_baseline_complete.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_25_allow_override_channel.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_26_extra_deny_bans_inventory.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/{select.rs,parse.rs,ingest.rs,run.rs}`
- `packages/rs/deny/g3rs-deny-types/src/lib.rs`
- deny ingestion and config tests as required
