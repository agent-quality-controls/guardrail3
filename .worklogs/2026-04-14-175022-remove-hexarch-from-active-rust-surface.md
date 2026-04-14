Summary

- Removed `hexarch` from the active Rust surface and archived the package tree under ignored `legacy/`.
- Proved the remaining live boundary bug first: `guardrail3-rs.toml` still treated `checks.hexarch` as a typed live field.
- Cleaned the active parser, app surface, and docs so `hexarch` no longer exists outside archive and historical plans.

Decisions made

- Removed the typed `checks.hexarch` field instead of keeping a dead compatibility knob.
  - Reason: a deleted family must not remain in the active schema.
- Kept `hexarch` only in ignored `legacy/packages/rs/hexarch`.
  - Reason: consultable archive is fine; live build input is not.
- Updated only live docs and handoff files.
  - Reason: historical `.plans/todo/...` material is record-keeping, not the current contract.

Key files for context

- `.plans/2026-04-14-174812-remove-hexarch-from-active-rust-surface.md`
- `packages/parsers/guardrail3-rs-toml-parser/crates/parser/types/src/guardrail3_rs_toml.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs`
- `apps/guardrail3-rs/crates/types/app-types/src/lib.rs`
- `AGENTS.md`
- `README.md`
- `GUARDRAIL3_GUIDE.md`

Next steps

- If we want zero stale naming in tests, rename the parser and app test names that still mention `hexarch` as a removed key/family.
- Historical plans under `.plans/todo/...` still mention `hexarch`; leave them unless we explicitly decide to rewrite historical planning records.
