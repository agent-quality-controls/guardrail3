Summary
- Normalized `packages/parsers/cargo-toml-parser` onto the current package shape with sibling `crates/runtime`, `crates/assertions`, and `crates/types`.
- Fixed the parser test layout, moved final proof into the shared assertions crate, and added package policy files and schema waivers so the package validates cleanly.

Decisions made
- Kept the public root type names as explicit type aliases in `src/lib.rs` instead of forcing a repo-wide caller rewrite. This preserves the stable facade without the wildcard re-export that `g3rs-arch/lib-facade-only` rejects.
- Marked the facade and all internal crates `publish = false` so release checks stand down for this internal parser package.
- Added narrow `g3rs-code/ast-19-large-type-inventory` waivers for the schema-mirror structs in `crates/types/src/cargo_toml.rs`. The file intentionally mirrors Cargo's manifest surface and grouping those fields just to satisfy inventory limits would make the parser API less truthful.
- Kept the centralized `std::fs::read_to_string` and `toml::from_str` boundaries, but switched their `#[allow]` attributes to the inline `reason = ...` form that the code rule expects.

Key files for context
- `packages/parsers/cargo-toml-parser/Cargo.toml`
- `packages/parsers/cargo-toml-parser/guardrail3-rs.toml`
- `packages/parsers/cargo-toml-parser/src/lib.rs`
- `packages/parsers/cargo-toml-parser/src/types.rs`
- `packages/parsers/cargo-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/cargo-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/cargo-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/cargo-toml-parser/crates/types/src/cargo_toml.rs`
- `.plans/2026-04-16-235802-cargo-toml-parser-cleanup.md`

Next steps
- Move to the next package under `packages/parsers` and repeat the same package-local cleanup pass.
- Stop only if the next remaining issue is a real rule contradiction or another missing waiver capability.
