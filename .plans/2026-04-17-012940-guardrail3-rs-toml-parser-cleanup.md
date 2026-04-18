# Goal
Make `packages/parsers/guardrail3-rs-toml-parser` clean under validation with the current parser package shape: sibling `crates/runtime`, `crates/assertions`, and `crates/types`, clean root facade, shared proof in assertions, and no backward-compat root type aliases.

# Approach
- Normalize the package layout from `crates/parser/{runtime,assertions,types}` to `crates/{runtime,assertions,types}`.
- Rewrite the root facade so parse API stays at the root and schema types live under `guardrail3_rs_toml_parser::types`.
- Add the missing root policy files and package metadata.
- Fix runtime/parser test sidecars to the owned `parser_tests` shape with shared proof in the assertions crate.
- Replace stale root-type imports in repo callers with `guardrail3_rs_toml_parser::types::...`.
- Rerun package tests and validator. Stop only if the next remaining issue is not clearly package debt or a clearly valid rule.

# Key decisions
- No backward-compat root type aliases.
- No nested `crates/parser/...` layout.
- Keep parser assertions as the proof surface instead of local test result assertions.

# Files to modify
- `packages/parsers/guardrail3-rs-toml-parser/**`
- repo callers that still import root types from `guardrail3_rs_toml_parser`
