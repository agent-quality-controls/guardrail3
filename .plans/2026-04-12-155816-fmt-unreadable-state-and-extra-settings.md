# Goal

Correct the `fmt` package boundary so it distinguishes unreadable root inputs from parse errors, and restore inventory semantics for explicit non-baseline `skip_macro_invocations` usage.

# Approach

- Add failing tests first for:
  - `RS-FMT-CONFIG-01` reporting unreadable rustfmt distinctly from parse error
  - `RS-FMT-CONFIG-03` reporting unreadable toolchain distinctly from parse error
  - `RS-FMT-CONFIG-04` reporting unreadable Cargo distinctly from parse error
  - ingestion preserving unreadable states instead of collapsing them into `ParseError`
  - `RS-FMT-CONFIG-02` inventorying `skip_macro_invocations = []`
- Then change the typed package boundary:
  - add `Unreadable` variants to `G3RsFmtRustfmtConfigState`, `G3RsFmtCargoState`, and `G3RsFmtToolchainState`
  - map unreadable and stale-read failures into `Unreadable`, not `ParseError`
- Update config rules to emit distinct unreadable findings.
- Remove the empty `skip_macro_invocations` suppression from the extra-settings rule.

# Key decisions

- Do not preserve old app snapshot semantics for stale reads.
  - The package model does not have cached file content in `G3RsWorkspaceCrawl`, so the robust behavior is fail-closed with an explicit unreadable state.
- Keep unreadable and parse-error separate.
  - They have different causes and should produce different findings.
- Treat explicit presence of a non-baseline rustfmt key as inventory-worthy even if the value is currently empty.
  - The rule is about policy surface, not only effective runtime behavior.

# Files to modify

- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/inputs.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_01_settings/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_02_extra_settings/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_03_nightly_keys_on_stable/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_04_edition_mismatch/rule.rs`
- relevant fmt rule tests
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- relevant fmt ingestion tests
