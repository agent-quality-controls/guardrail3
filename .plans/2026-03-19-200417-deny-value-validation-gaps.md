# Fix value validation gaps in deny.toml checking

**Date:** 2026-03-19 20:04
**Task:** Fix 4 validation gaps across deny_inventory, deny_licenses, deny_bans, clippy_coverage

## Goal
Close value validation gaps: missing reason warnings, exact URL matching, crate key fallback, ban reason checks.

## Approach

### 1. deny_inventory.rs — advisory ignores + skip entries need reason (R20/R19)
- Advisory ignores: entries can be `{ id = "RUSTSEC-...", reason = "..." }` or plain strings. When table entry has no `reason`, emit Warn with existing ID (R20).
- Skip entries: already extract `reason` on line 34. When reason is empty, emit Warn with existing ID (R19).

### 2. deny_licenses.rs — MED-16: registry URL exact match
- Line 207: `!r.contains("crates.io")` → change to exact match against `"https://github.com/rust-lang/crates.io-index"`.

### 3. deny_bans.rs — MED-18: missing crate key support
- Line 106: `entry.get("name")` only. Add `.or_else(|| entry.get("crate"))` fallback for cargo-deny 0.19+ `{ crate = "name@version" }` format, extracting just the name part before `@`.

### 4. clippy_coverage.rs — HIGH-06: ban entries without reason
- In `check_ban_list`, after extracting paths from ban entries (line 169-176), check if each table entry has a `reason` field. If missing, emit Warn with the missing_id.

## Files to Modify
- `apps/guardrail3/src/app/rs/validate/deny_inventory.rs` — add reason warnings
- `apps/guardrail3/src/app/rs/validate/deny_licenses.rs` — exact URL match
- `apps/guardrail3/src/app/rs/validate/deny_bans.rs` — crate key fallback
- `apps/guardrail3/src/app/rs/validate/clippy_coverage.rs` — ban reason check
