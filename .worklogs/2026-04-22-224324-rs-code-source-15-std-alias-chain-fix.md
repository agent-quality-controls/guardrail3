## Summary

Fixed `g3rs-code/ast-15-direct-fs-usage` so chained std alias rebinding is detected at the parser visitor boundary. `use std as s; use s as t; t::fs::read_to_string(...)` now resolves through the visitor alias set, and the same fix covers the equivalent `extern crate std as s; use s as t; ...` form.

## Decisions made

- Extended `fs_visitors.rs` instead of adding rule-local alias logic.
  - Why: both direct std-fs import detection and inline std-fs call detection consume the same visitor-derived alias state.
- Added direct regressions for both `use`-chain and `extern crate`-chain forms.
  - Why: both are valid Rust and share the same alias propagation root cause.
- Kept the change limited to parser alias propagation.
  - Why: the rule already depends on the visitor output; the bug was in alias collection, not in the rule match itself.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`
- `.plans/2026-04-22-224230-rs-code-source-15-std-alias-chain-fix.md`

## Next steps

- None for this fix.
