# Create rustfmt-toml parser crate

**Date:** 2026-04-03 21:11

## Summary
First standalone config parser crate. Typed parser for rustfmt.toml with
all ~80 known fields mapped, catch-all for unknown fields. Follows
cargo_toml crate patterns. 5 integration tests pass, zero guardrail3
violations.

## Key files
- `packages/rustfmt-toml/` — workspace root
- `packages/rustfmt-toml/crates/rustfmt-toml/src/config.rs` — main struct
- `packages/rustfmt-toml/crates/rustfmt-toml/src/error.rs` — error type
- `packages/rustfmt-toml/crates/rustfmt-toml/tests/parse.rs` — 5 tests
