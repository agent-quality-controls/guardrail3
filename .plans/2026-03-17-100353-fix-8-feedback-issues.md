# Fix 8 user feedback issues

**Date:** 2026-03-17 10:03
**Task:** Fix 8 issues from user feedback on guardrail3 validation output

## Goal
Address all 8 feedback items: message fixes, per-struct reporting, help text, guide schema, overwrite warnings.

## Approach

### 1. R-TEST-09 message fix (test_checks.rs)
Replace the message to remove compile time claim, add structural separation policy explanation.

### 2. R-GARDE-05 per-struct reporting (garde_checks.rs + ast_helpers.rs + ast_visitors.rs)
- Add `name: Option<String>` to `DeriveInfo` struct
- Populate it in `DeriveVisitor` from struct/enum ident
- Refactor `check_derive_inventory` to produce one `CheckResult` per missing struct
- Gate R-GARDE-05 on R-GARDE-01 (skip if garde not in deps)

### 3. R50 cargo tree suggestion (dependency_scan.rs)
Append `cargo tree -i {crate_name}` hint to banned crate message.

### 4. JSON schema in guide (guide.rs)
Add JSON output schema section to GUIDE_CONTENT.

### 5. --inventory vs --verbose help text (cli.rs)
Update help strings for both flags.

### 6. R-TEST-09 severity explanation
Already handled in #1 — the new message explains WHY it's an error.

### 7. Generate overwrite warning (generate.rs)
Before writing each file, check if existing content differs; print warning about local/ overrides.

### 8. Double-check R-TEST-09 message
Covered by #1.

## Files to Modify
- `apps/guardrail3/src/app/rs/validate/ast_helpers.rs` — add name to DeriveInfo
- `apps/guardrail3/src/app/rs/validate/ast_visitors.rs` — populate name in DeriveVisitor
- `apps/guardrail3/src/app/rs/validate/test_checks.rs` — R-TEST-09 message
- `apps/guardrail3/src/app/rs/validate/garde_checks.rs` — per-struct + gate on R-GARDE-01
- `apps/guardrail3/src/app/rs/validate/dependency_scan.rs` — R50 cargo tree hint
- `apps/guardrail3/src/domain/modules/guide.rs` — JSON schema section
- `apps/guardrail3/src/cli.rs` — help text
- `apps/guardrail3/src/commands/generate.rs` — overwrite warning
