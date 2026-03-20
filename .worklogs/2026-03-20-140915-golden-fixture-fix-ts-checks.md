# Fix golden fixture + rewrite T-ARCH-01 to parity with R-ARCH-01

**Date:** 2026-03-20 14:09

## Summary
Fixed admin fixture loose files (ports had .ts files directly in container dirs). Rewrote T-ARCH-01 to enforce same structural rules as R-ARCH-01: exact dir contents, no loose files, inbound/outbound, container validation, hex-in-hex recursion. Added ports to required layers.

## Fixture changes
- `ports/inbound/use-cases.ts` → `ports/inbound/use-cases/index.ts`
- `ports/outbound/validator-client.ts` → `ports/outbound/validator/index.ts`

## T-ARCH-01 rewrite
- `check_single_app_structure` now delegates to `check_ts_modules_dir` (reusable for hex-in-hex)
- `check_ts_inbound_outbound` enforces exactly `{inbound, outbound}` in adapters/ and ports/
- `validate_ts_container` checks subdirs have .ts files, .gitkeep, or modules/ (hex-in-hex)
- `check_ts_loose_files` flags any file except .gitkeep in structural/container dirs
- Leaf subdir conflict: having both .ts files AND modules/ dir → ERROR
- .gitkeep alongside real content is fine (harmless leftover)
- Uses `fs.metadata()` for dir existence (not `list_dir` which can't distinguish empty from nonexistent)

## Key files
- `crates/app/ts/validate/ts_arch_checks.rs`
- `tests/fixtures/r_arch_01/golden/apps/admin/`

## Next steps
- TDD: write failing tests for each check, then implement
- Review check code structure — split into clear per-check functions
