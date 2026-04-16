Summary
- Cleaned the package-local debt in `packages/parsers/cargo-config-toml-parser` and reduced validation to one remaining warning.
- The package now uses the current sibling-crate shape, shared test proof, explicit package policy files, and unpublished internal crate intent.

Decisions made
- Flattened `crates/parser/{runtime,assertions,types}` to `crates/{runtime,assertions,types}`.
  Why: the old nested layout triggered the same runtime/assertions split failures already fixed across the Rust package families.
  Rejected: keeping the nested `crates/parser/...` shape and patching around each rule signal.
- Moved the facade API away from broad flat type re-exports and into a `types` module.
  Why: this removes the broad re-export and excessive import-count signals without inventing fake helper layers.
  Rejected: keeping a flat top-level export list and fighting both `broad re-export` and `too many imports` at once.
- Marked the whole package unpublished.
  Why: the facade depends on internal sibling crates, so the package is not crates.io-ready without publishing internal implementation crates too.
  Rejected: pretending the package is publishable while `cargo publish --dry-run` fails on internal path dependencies.
- Stopped before changing `RS-CODE-SOURCE-19`.
  Why: the only remaining finding is a warning on `CargoConfigToml` having 22 fields while it intentionally mirrors Cargo's top-level config schema. That is no longer clear package debt; it needs a rule or policy decision.

Key files for context
- `packages/parsers/cargo-config-toml-parser/Cargo.toml`
- `packages/parsers/cargo-config-toml-parser/guardrail3-rs.toml`
- `packages/parsers/cargo-config-toml-parser/src/lib.rs`
- `packages/parsers/cargo-config-toml-parser/src/types.rs`
- `packages/parsers/cargo-config-toml-parser/crates/runtime/src/lib.rs`
- `packages/parsers/cargo-config-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/cargo-config-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/cargo-config-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/cargo-config-toml-parser/crates/types/src/cargo_config_toml.rs`

Next steps
- Decide whether parser schema-mirror types should be exempt from `RS-CODE-SOURCE-19 large type inventory`.
- If yes, add a narrow mechanism for that case and then re-run this package.
- If no, redesign the Cargo config model so the top-level schema is split across smaller flattened structs and update parser callers to the new shape.
