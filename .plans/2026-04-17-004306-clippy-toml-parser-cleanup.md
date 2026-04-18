Goal
- Normalize `packages/parsers/clippy-toml-parser` to the current parser-package shape and get `guardrail3-rs validate --path packages/parsers/clippy-toml-parser` to `No findings.`

Approach
- Flatten the nested `crates/parser/{runtime,assertions,types}` layout into sibling `crates/{runtime,assertions,types}` crates and update all manifest paths.
- Add the missing package policy files and `guardrail3-rs.toml`, mark the facade and member crates unpublished, and move the facade to the clean `types` module shape instead of root type re-exports.
- Fix the parser sidecar layout to the owned `parser_tests` shape, move final result proof into the shared assertions crate, and keep sidecar helpers limited to parser entrypoints plus external type paths.
- Remove the old comment-style `#[allow(clippy::disallowed_methods)]` uses in favor of the inline `reason = ...` form already used by the clean parser packages.
- Use narrow package-local waivers only for intentional schema-mirror surfaces that still trigger large-type inventory after the structural cleanup.

Key decisions
- No backward-compat root aliases. Schema types belong under `clippy_toml_parser::types::...`.
- Treat the parser types as internal schema mirrors, not publishable public crates, so release checks should stand down through explicit `publish = false`.
- If `ClippyToml` still trips size inventory after cleanup, waive the exact schema-mirror structs rather than distorting the model just to satisfy the threshold.

Files to modify
- `packages/parsers/clippy-toml-parser/Cargo.toml`
- `packages/parsers/clippy-toml-parser/src/*`
- `packages/parsers/clippy-toml-parser/crates/**`
- `packages/parsers/clippy-toml-parser/*.toml`
- `packages/parsers/clippy-toml-parser/*.md`
