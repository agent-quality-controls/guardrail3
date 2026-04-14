Goal

- Remove the fmt family's remaining dependency on dead `guardrail3.toml`.
- Replace fmt escape-hatch routing with Rust-only `guardrail3-rs.toml` waiver state.
- Keep the app unchanged and fix the package boundary only.

Approach

- Add failing tests that prove:
  - `guardrail3-rs.toml` drives fmt ignore waivers.
  - legacy `guardrail3.toml` is ignored.
- Replace `G3RsFmtEscapeHatch` with typed Rust-policy waiver state in `g3rs-fmt-types`.
- Update `g3rs-fmt-ingestion` to read `guardrail3-rs.toml` via the parser package and pass waivers through typed state.
- Update `RS-FMT-CONFIG-07` and its tests to use waivers and Rust-only wording.
- Update package READMEs to stop advertising dead universal-config vocabulary.

Key decisions

- Use `guardrail3-rs.toml` `[[waivers]]` instead of preserving `[[escape_hatches]]`.
  - The existing fmt rule only needs `rule`, `file`, `selector`, and `reason`, which already matches the Rust-only waiver contract.
- Keep missing Rust policy equivalent to "no waivers".
  - Rejected: making `guardrail3-rs.toml` required for fmt generally.
- Do not touch the app.
  - The bug is inside the fmt package boundary.

Files to modify

- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/{select.rs,run.rs}`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_07_ignore_escape_hatch/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_07_ignore_escape_hatch/rule_tests/helpers.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/README.md`
- `packages/rs/fmt/g3rs-fmt-ingestion/README.md`
