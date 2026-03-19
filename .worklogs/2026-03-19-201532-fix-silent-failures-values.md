# Fix 23 silent failures + value validation + misc safe fixes

**Date:** 2026-03-19 20:15
**Scope:** 16 source files across rs/validate, ts/validate, hooks

## Summary
Fixed all architecture-safe findings from the adversarial audit. Verified by adversarial agent: 23/23 complete.

## Silent failures (7 fixes)
- config_files.rs: clippy.toml read/parse errors now emit Error
- rustfmt_check.rs: parse errors Error (was Warn)
- toolchain_check.rs: parse errors Error (was Warn), nightly=Error, pinned=Info
- cargo_lints.rs: crate Cargo.toml read/parse now emits Error
- workspace_metadata.rs: TOML parse errors emit Error
- hex_arch_checks.rs + dependency_allowlist.rs: same
- jscpd_check.rs: JSON parse failure emits Error + BOM stripping

## Value validation (8 fixes)
- clippy_coverage.rs: ban entries without reason flagged as Warn
- deny_inventory.rs: advisory ignores + skip entries require reason (Warn)
- deny_licenses.rs: registry URL exact match
- deny_bans.rs: crate key fallback for cargo-deny 0.19+
- npmrc_check.rs: quoted values stripped + BOM stripping
- tsconfig_check.rs: BOM stripping + check ID collision fixed (T-TSC-60/61)
- workspace_metadata.rs: edition enforcement (Warn if missing/outdated)
- toolchain_check.rs: nightly vs pinned version distinction

## Misc safe fixes (8 fixes)
- code_quality_checks.rs: R58 fs.rs skip narrowed to src/fs.rs only
- allow_checks.rs: R36 EXCEPTION case-insensitive + added rust-toolchain.toml
- source_scan.rs (TS): expanded test file patterns (mocks, stories, e2e, test dirs)
- source_scan.rs (TS): v8 ignore added to T35 coverage patterns
- ts_comment_checks.rs: reason detection accepts --reason, rejects empty reasons
