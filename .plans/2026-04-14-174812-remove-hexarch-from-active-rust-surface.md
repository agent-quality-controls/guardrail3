Goal

- Remove `hexarch` from the active Rust system completely.
- Keep the archived package tree only under ignored `legacy/`.
- Ensure no active package, app, parser, or doc surface still exposes `hexarch` as a live family.

Approach

- Add tests first at the live boundaries that still expose `hexarch`:
  - `guardrail3-rs` parser tests must prove `checks.hexarch` is no longer a typed key and instead lands in `extra`
  - `apps/guardrail3-rs` app-types tests must prove `SupportedFamily::parse_cli("hexarch")` returns `None`
- Remove `hexarch` from the active Rust policy schema:
  - delete the typed `hexarch` field from `RustChecksConfig`
  - update parser assertions, fixtures, and parsing tests accordingly
- Remove `hexarch` from active docs and handoff files that describe the live system:
  - `AGENTS.md`
  - `README.md`
  - `GUARDRAIL3_GUIDE.md`
- Move `packages/rs/hexarch` to ignored `legacy/packages/rs/hexarch`
  - the move is archive-only and must not be staged
  - stage only the tracked deletions from `packages/rs/hexarch`
- Run grep and targeted tests to prove:
  - no active app/package/parser code still references `hexarch`
  - the new app still builds

Key decisions

- Do not preserve a dead `hexarch` config toggle in `guardrail3-rs.toml`.
  - Reason: a family that no longer exists must not remain in the typed active schema.
- Treat archived `hexarch` code the same way as the old app:
  - consultable by humans
  - nonexistent to the live repo and build graph
- Update only active docs, not historical planning material under `.plans/todo/...`.
  - Reason: historical plans are records, not the live contract.

Files to modify

- `packages/parsers/guardrail3-rs-toml-parser/crates/parser/types/src/guardrail3_rs_toml.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/parser/assertions/src/parser.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/parser/runtime/src/parser_tests/fixtures/workspace_service.toml`
- `apps/guardrail3-rs/crates/types/app-types/src/lib.rs`
- `AGENTS.md`
- `README.md`
- `GUARDRAIL3_GUIDE.md`
- move `packages/rs/hexarch` to `legacy/packages/rs/hexarch`
