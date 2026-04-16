Goal
- Normalize `packages/parsers/cliff-toml-parser` to the current parser-package shape and get `guardrail3-rs validate --path packages/parsers/cliff-toml-parser` to `No findings.`

Approach
- Flatten the nested `crates/parser/{runtime,assertions,types}` layout into sibling `crates/{runtime,assertions,types}` crates and update all manifest paths.
- Add the missing package policy files and `guardrail3-rs.toml`, mark the package and member crates unpublished, and mirror the same parser-package release metadata used in the cleaned parser packages.
- Convert the root and member facades to the current `types`-module pattern so import surfaces stay explicit without broad wildcard re-exports.
- Fix the parser sidecar layout to the owned `parser_tests` shape, move final result checks into the shared assertions crate, and keep sidecar helpers limited to the owned parser module.
- Re-run package tests and validation. If only schema-mirror inventory warnings remain, add narrow `RS-CODE-SOURCE-19` waivers only for the exact large schema structs.

Key decisions
- Preserve the existing public root names through explicit facade aliases only if the package is already consumed that way. Prefer `types::...` as the real schema home.
- Do not change any rules here unless the remaining issue is a real contradiction after the package is structurally clean.

Files to modify
- `packages/parsers/cliff-toml-parser/Cargo.toml`
- `packages/parsers/cliff-toml-parser/src/*`
- `packages/parsers/cliff-toml-parser/crates/**`
- `packages/parsers/cliff-toml-parser/*.toml`
- `packages/parsers/cliff-toml-parser/*.md`
