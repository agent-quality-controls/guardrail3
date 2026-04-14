Summary

- Decoupled the `garde` family from deleted `apps/guardrail3` code and the dead `guardrail3.toml` contract.
- `garde` now uses typed `guardrail3-rs.toml` Rust policy input for activation and query-as waivers, and the dead `RS-GARDE-SOURCE-08` rule is removed.

Decisions made

- Deleted `RS-GARDE-SOURCE-08` instead of porting it.
  - It enforced `.validate()` on dead `GuardrailConfig` semantics from the old universal config model.
- Replaced old garde activation with Rust-only activation:
  - active when `garde` dependency is present
  - or `checks.garde = true` in `guardrail3-rs.toml`
- Treated missing `guardrail3-rs.toml` as "no waivers", not as an ingestion error.
  - Invalid or unreadable `guardrail3-rs.toml` still surfaces through `RS-GARDE-SOURCE-10` when the family is active.
- Replaced deleted test-only `build_clippy_toml(...)` imports with package-owned clippy baseline helpers.

Key files for context

- `.plans/2026-04-14-135016-garde-rust-policy-and-legacy-decoupling.md`
- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_ast_04_query_as_inventory/rule.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/test_support.rs`

Next steps

- Move to the next broken family after `garde`.
- Keep rejecting any new `guardrail3.toml` or deleted-app dependency that appears in active Rust packages.
