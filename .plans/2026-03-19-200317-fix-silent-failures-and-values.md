# Fix silent failures + value validation (architecture-safe)

**Date:** 2026-03-19 20:03
**Task:** Fix findings that survive the crawler wiring refactor

## Silent failures — emit Error on parse failure instead of silent return

1. HIGH-01/02: clippy.toml parse/read errors silently swallowed (config_files.rs)
2. HIGH-03: R22 rustfmt.toml parse errors Warn → Error (rustfmt_check.rs)
3. HIGH-04: R25 toolchain parse errors Warn → Error (toolchain_check.rs)
4. MED-05: R29 crate Cargo.toml read/parse silently skipped (cargo_lints.rs)
5. MED-31: F-04-10 TOML parse errors skip all dep/arch checks (workspace_metadata.rs, hex_arch_checks.rs)
6. HIGH-40: JSCPD-03 JSON parse error silently swallowed (jscpd_check.rs)
7. HIGH-34: TSC-01 JSONC comments cause silent bypass (tsconfig_check.rs)

## Value validation — check content not just existence

8. HIGH-06: Ban entries without reason field not flagged (clippy_coverage.rs)
9. HIGH-09/10: Advisory ignores + skip entries don't require reason (deny_inventory.rs)
10. MED-16: deny registry URL checked by substring not exact match (deny_licenses.rs)
11. MED-18: Ban list doesn't try crate key — cargo-deny 0.19+ format (deny_bans.rs)
12. MED-55: npmrc quoted values not stripped (npmrc_check.rs)
13. MED-54: BOM causes parse failure everywhere (tsconfig, npmrc, jscpd)
14. HIGH-05: R25 nightly vs pinned version not distinguished (toolchain_check.rs)
15. MED-30: R55-R57 workspace metadata all Info → should warn/error (workspace_metadata.rs)

## Misc safe fixes

16. MED-17: R58 fs.rs skip too broad — any fs.rs exempt (code_quality_checks.rs)
17. MED-18: R36 EXCEPTION case-insensitive (allow_checks.rs)
18. MED-19: R36 missing config files from check list (allow_checks.rs)
19. HIGH-32: T32 file length threshold contradictory (source_scan.rs)
20. MED-48: T35 missing v8 ignore pattern (ts_comment_checks.rs)
21. MED-49: T23 reason detection rejects --reason (ts_comment_checks.rs)
22. MED-51: is_ts_test_file incomplete patterns (source_scan.rs)
23. MED-41: Check ID collision T60/T61 (tsconfig_check.rs, jscpd_check.rs)
