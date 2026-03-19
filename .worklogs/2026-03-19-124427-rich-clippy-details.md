# Rich clippy coverage details + toml_edit dep

**Date:** 2026-03-19 12:44
**Scope:** coverage/clippy.rs, clippy_coverage.rs, Cargo.toml, lib.rs

## Summary
Clippy coverage details now diff against required baseline: methods, types, thresholds each show required_present/required_missing/user_extra. Added toml_edit dependency for future merge support. Made EXPECTED_METHOD_BANS and EXPECTED_TYPE_BANS pub for reuse.
