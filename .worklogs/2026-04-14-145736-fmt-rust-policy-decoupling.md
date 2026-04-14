Summary

- Removed the fmt family's remaining dependency on dead `guardrail3.toml`.
- Replaced fmt ignore escape-hatch routing with Rust-only waiver state from `guardrail3-rs.toml`.

Decisions made

- Replaced `G3RsFmtEscapeHatch` with typed `G3RsFmtRustPolicyState` plus `G3RsFmtWaiver`.
  - Rejected: preserving dead universal-config vocabulary in the public types crate.
- Kept missing Rust policy equivalent to "no waivers".
  - Reason: fmt itself is still valid without a root Rust-policy file unless a waiver is actually needed.
- Left one explicit legacy-negative regression in ingestion tests.
  - Reason: active code should ignore `guardrail3.toml`, and that deserves a direct proof.

Key files for context

- `.plans/2026-04-14-145226-fmt-rust-policy-decoupling.md`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/{select.rs,run.rs}`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_07_ignore_escape_hatch/{rule.rs,rule_tests/helpers.rs}`
- `packages/rs/fmt/g3rs-fmt-config-checks/README.md`
- `packages/rs/fmt/g3rs-fmt-ingestion/README.md`

Next steps

- Move to the next family still carrying dead `guardrail3.toml` debt.
- Keep active Rust packages free of deleted old-app imports and dead universal-config types.
