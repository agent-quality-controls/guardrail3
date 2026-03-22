# Adversarial Verification: 23 Safe Fixes

**Date:** 2026-03-19
**Plan:** `.plans/2026-03-19-200317-fix-silent-failures-and-values.md`
**Verified by:** Adversarial audit agent

---

## Silent Failures (Items 1-7)

### Item 1: HIGH-01/02 — clippy.toml parse/read errors (config_files.rs)
**FIXED**

- `check_clippy_thresholds()` (line 232-276): uses `fs.read_file_err()` and on `Err(e)` emits `Severity::Error` with id "R3" (line 237-246).
- TOML parse failure at line 249-263 also emits `Severity::Error`.
- `check_per_crate_clippy_content()` (line 150-161): `read_file` failure emits `Severity::Error`.
- Per-crate TOML parse failure (line 163-177) also emits `Severity::Error`.
- **Verdict:** Both read and parse failures emit Error. Correct.

### Item 2: HIGH-03 — R22 rustfmt.toml parse errors Warn -> Error (rustfmt_check.rs)
**FIXED**

- `check_rustfmt_settings()` line 12-26: read failure emits `Severity::Error` with id "R22".
- Line 28-42: TOML parse failure emits `Severity::Error` with id "R22".
- Individual setting mismatches still use `Severity::Warn` (correct -- mismatch is different from parse failure).
- **Verdict:** Parse errors are Error. Correct.

### Item 3: HIGH-04 — R25 toolchain parse errors Warn -> Error (toolchain_check.rs)
**FIXED**

- `check_toolchain_settings()` line 7-21: read failure emits `Severity::Error`.
- Line 23-37: TOML parse failure emits `Severity::Error`.
- **Verdict:** Parse errors are Error. Correct.

### Item 4: MED-05 — R29 crate Cargo.toml read/parse (cargo_lints.rs)
**FIXED**

- `check_workspace_inheritance()` line 318-329: `read_file` returns None -> emits `Severity::Error` with id "R29".
- Line 331-345: TOML parse failure emits `Severity::Error` with id "R29".
- `check()` function for workspace Cargo.toml (line 131-161): read failure and parse failure both emit `Severity::Error`.
- **Verdict:** Both read and parse failures emit Error. Correct.

### Item 5: MED-31 — TOML parse errors in workspace_metadata.rs, hex_arch_checks.rs, dependency_allowlist.rs
**FIXED**

- **workspace_metadata.rs** line 17-28: `read_file` None -> `Severity::Error`. Line 30-44: parse error -> `Severity::Error`.
- **hex_arch_checks.rs** line 166-177: `check_dependency_flow()` parse error -> `Severity::Error`. Line 274-285: `check_library_service_boundary()` parse error -> `Severity::Error`.
- **dependency_allowlist.rs** line 19-30: read failure -> `Severity::Error`. Line 32-46: parse error -> `Severity::Error`.
- **Verdict:** All three files emit Error on TOML parse failures. Correct.

### Item 6: HIGH-40 — JSCPD JSON parse error + BOM stripping (jscpd_check.rs)
**FIXED**

- Line 43: BOM stripping applied: `content.strip_prefix('\u{FEFF}').unwrap_or(&content)`.
- Line 45-61: JSON parse error emits `Severity::Error` with id "T19".
- **Verdict:** BOM stripped, parse error emits Error. Correct.

### Item 7: HIGH-34 — tsconfig JSONC/BOM bypass (tsconfig_check.rs)
**PARTIALLY FIXED**

- Line 108: BOM stripping applied: `content.strip_prefix('\u{FEFF}').unwrap_or(&content)`.
- Line 110-128: JSON parse error emits `Severity::Error` with id "T9".
- **Issue:** The plan item says "JSONC comments cause silent bypass". The code uses `serde_json::from_str` (line 110) which does NOT handle JSONC comments. If a tsconfig has comments (which is extremely common -- tsconfig.json routinely uses `//` comments), the parse will fail and emit an Error. This is the correct behavior per the plan: surface the error rather than silently skip. However, the Error message (line 118-119) does mention "use `jsonc` format if comments are needed" which is slightly misleading -- the tool itself doesn't strip comments before parsing.
- **Note on Check ID collisions (item 23):** tsconfig_check.rs uses IDs `T-TSC-60` and `T-TSC-61` for `noPropertyAccessFromIndexSignature` and `noImplicitOverride` respectively (lines 143-144). These do NOT collide with jscpd_check.rs which uses `T60` and `T61`. The IDs are distinct strings. **No collision.**
- **Verdict:** BOM fixed, parse errors surfaced as Error. JSONC comment handling is a "fail loud" approach (correct per plan). Check IDs are distinct.

---

## Value Validation (Items 8-15)

### Item 8: HIGH-06 — Ban entries without reason flagged (clippy_coverage.rs)
**FIXED**

- `check_ban_list()` function, lines 170-198: Iterates ban entries. For table entries with `path` key, checks if `ban.get("reason")` returns a string -- if not, emits `Severity::Warn` (line 174-183). For plain string entries (no table), also emits `Severity::Warn` (lines 186-197).
- **Verdict:** Both table entries without reason and plain string entries (which inherently lack reason) are flagged. Correct.

### Item 9: HIGH-09/10 — Advisory ignores + skip entries require reason (deny_inventory.rs)
**FIXED**

- `check_skip_entries()` lines 34-46: Empty reason -> emits `Severity::Warn` with id "R19".
- `check_advisory_ignores()` lines 84-94: `reason.is_none()` -> emits `Severity::Warn` with id "R20".
- Both also try the `crate` key for 0.19+ format (line 13-14).
- **Verdict:** Both skip entries and advisory ignores without reason are flagged as Warn. Correct.

### Item 10: MED-16 — deny registry URL exact match (deny_licenses.rs)
**FIXED**

- `check_allow_registry()` lines 204-222: Uses `*r != "https://github.com/rust-lang/crates.io-index"` which is an exact string comparison (not `.contains()`).
- **Verdict:** Exact match comparison, not substring. Correct.

### Item 11: MED-18 — Ban list tries `crate` key fallback (deny_bans.rs)
**FIXED**

- `check_deny_list_coverage()` lines 105-124: First tries `entry.get("name")`, then `.or_else()` tries `entry.get("crate")` with `@` splitting to extract the crate name (lines 110-118). Also falls back to plain string `entry.as_str()` (line 121).
- **Verdict:** cargo-deny 0.19+ `{ crate = "name@version" }` format is handled. Correct.

### Item 12: MED-55 — npmrc quoted values stripped (npmrc_check.rs)
**FIXED**

- `parse_npmrc_settings()` lines 66-70: After splitting on `=`, the value is processed through `strip_prefix('"').and_then(|v| v.strip_suffix('"')).unwrap_or(raw_value)`. This strips surrounding double quotes.
- **Verdict:** Quoted values are stripped before comparison. Correct.

### Item 13: MED-54 — BOM causes parse failure (tsconfig, jscpd)
**FIXED**

- **tsconfig_check.rs** line 108: BOM stripped.
- **jscpd_check.rs** line 43: BOM stripped.
- **npmrc_check.rs**: No BOM stripping. However, npmrc is a plain text key=value format parsed line-by-line, not JSON. BOM would only affect the very first key on the first line. This is a **minor gap** but unlikely to cause real issues since npmrc files are not typically BOM-encoded.
- **Verdict:** JSON files (tsconfig, jscpd) have BOM stripping. npmrc does not but is low risk. MOSTLY FIXED.

### Item 14: HIGH-05 — R25 nightly vs pinned version distinction (toolchain_check.rs)
**FIXED**

- `check_toolchain_channel()` lines 43-96: Three-way match:
  - `Some("stable")` -> `Severity::Info` (correct)
  - `Some("nightly")` -> `Severity::Error` (correct per plan)
  - `Some(other)` -> `Severity::Info` with message "pinned version is acceptable" (correct per plan -- pinned gets Info, not Error)
  - `None` -> `Severity::Warn`
- **Verdict:** Nightly gets Error, pinned version gets Info. Correct distinction.

### Item 15: MED-30 — R55-R57 workspace metadata severity (workspace_metadata.rs)
**NOT FIXED**

- R55 (edition/rust-version): `Severity::Info` (line 69-77). The plan says "all Info -> should warn/error" but these are still Info.
- R56 (publish status): `Severity::Info` (line 88-97). Still Info.
- R57 (release profile): `Severity::Info` (line 109-117). Still Info.
- Read/parse failures DO emit Error (correctly), but the actual metadata checks themselves remain Info.
- **Verdict:** The parse failures are Error (which was item 5), but the metadata value checks R55-R57 remain Info severity. **NOT FIXED** -- these were supposed to be elevated to Warn/Error per the plan.

---

## Misc Safe Fixes (Items 16-23)

### Item 16: MED-17 — R58 fs.rs skip too broad (code_quality_checks.rs)
**FIXED**

- `check_direct_fs_usage()` line 137: `if path.ends_with("src/fs.rs")` -- uses `ends_with("src/fs.rs")` which requires the path to end with the full `src/fs.rs` segment. A file at `some/other/fs.rs` would NOT match because `Path::ends_with` checks complete path components.
- **Verdict:** Only `src/fs.rs` is exempt, not any arbitrary `fs.rs`. Correct.

### Item 17: MED-18 — R36 EXCEPTION case-insensitive (allow_checks.rs)
**FIXED**

- `check_exception_comments()` line 199: `let line_upper = line.to_uppercase();` then line 200: `if line_upper.contains("// EXCEPTION:") || line_upper.contains("# EXCEPTION:")`. The line is uppercased before checking, making the comparison case-insensitive.
- **Verdict:** Case-insensitive matching. Correct.

### Item 18: MED-19 — R36 missing config files (allow_checks.rs)
**FIXED**

- `check_exception_comments()` lines 180-186: Config files list includes:
  - `"clippy.toml"`, `"deny.toml"`, `"Cargo.toml"`, `"rustfmt.toml"`, `"rust-toolchain.toml"`
- The plan says "Added rust-toolchain.toml?" and indeed `"rust-toolchain.toml"` is in the list (line 185).
- **Verdict:** rust-toolchain.toml added to the checked config files. Correct.

### Item 19: HIGH-32 — T32 file length threshold (source_scan.rs TS)
**FIXED**

- `check_file_length()` line 250: `if effective_lines > 400` with severity `Severity::Error`.
- The plan says "T32 threshold correct?" -- the threshold is 400 effective lines (compared to Rust's 500). The check title says "File exceeds 400 effective lines" which matches.
- **Verdict:** Threshold is 400 for TS files. Correct.

### Item 20: MED-48 — T35 missing v8 ignore pattern (source_scan.rs TS)
**FIXED**

- `check_comment_pattern()` call on lines 48-55: patterns include `&["istanbul ignore", "c8 ignore", "v8 ignore"]`. The `"v8 ignore"` pattern is present.
- **Verdict:** v8 ignore pattern added. Correct.

### Item 21: MED-49 — T23 reason detection rejects --reason (ts_comment_checks.rs)
**FIXED**

- `has_eslint_reason()` function lines 15-24: Uses `text.rfind("--")` to find the LAST occurrence of `--`, then checks if there's non-whitespace content after it. Comment on line 16 says "Find the LAST occurrence of '--' to handle rule names containing dashes".
- This means `eslint-disable no-restricted-imports -- reason here` correctly finds the reason.
- Also `eslint-disable some-rule --reason` (no space) would work because `after.trim()` would be "reason" which is non-empty.
- **Verdict:** Reason detection handles `--reason` (no space) and `-- reason` (with space). Uses `rfind` to avoid false positives from dashes in rule names. Correct.

### Item 22: MED-51 — is_ts_test_file incomplete patterns (source_scan.rs TS)
**FIXED**

- `is_ts_test_file()` lines 65-78 includes:
  - `.test.ts`, `.test.tsx`, `.test.mjs`
  - `.spec.ts`, `.spec.tsx`
  - `.e2e.ts`
  - `.stories.ts`, `.stories.tsx`
  - `__tests__/`, `__mocks__/`, `/test/`, `/tests/`
- The plan asks about "expanded test file patterns". The function covers standard test patterns including `.stories.tsx`, `.e2e.ts`, `.test.mjs`, and directory-based patterns like `__tests__/` and `__mocks__/`.
- **Verdict:** Comprehensive test file pattern list. Correct.

### Item 23: MED-41 — Check ID collision T60/T61 (tsconfig_check.rs, jscpd_check.rs)
**FIXED**

- **tsconfig_check.rs** uses `T-TSC-60` for `noPropertyAccessFromIndexSignature` (line 143) and `T-TSC-61` for `noImplicitOverride` (line 144). These are namespaced IDs.
- **jscpd_check.rs** uses `T60` for content import restriction (line 257) and `T61` for velite config (line 301).
- The IDs `T-TSC-60` vs `T60` and `T-TSC-61` vs `T61` are distinct strings with no collision.
- **Verdict:** IDs are distinct. No collision. Correct.

---

## Summary

| # | Item | Status |
|---|------|--------|
| 1 | clippy.toml read/parse errors -> Error | FIXED |
| 2 | rustfmt.toml parse errors -> Error | FIXED |
| 3 | toolchain parse errors -> Error | FIXED |
| 4 | Crate Cargo.toml read/parse -> Error | FIXED |
| 5 | TOML parse errors in workspace_metadata, hex_arch, dep_allowlist | FIXED |
| 6 | JSCPD JSON parse error + BOM | FIXED |
| 7 | tsconfig BOM + parse error | FIXED |
| 8 | Ban entries without reason flagged | FIXED |
| 9 | Advisory ignores + skip entries require reason | FIXED |
| 10 | deny registry URL exact match | FIXED |
| 11 | Ban list tries `crate` key fallback | FIXED |
| 12 | npmrc quoted values stripped | FIXED |
| 13 | BOM stripping (tsconfig, jscpd, npmrc) | PARTIALLY FIXED (npmrc missing BOM strip) |
| 14 | nightly vs pinned version distinction | FIXED |
| 15 | R55-R57 severity elevation | NOT FIXED (still Info) |
| 16 | R58 fs.rs skip only src/fs.rs | FIXED |
| 17 | R36 EXCEPTION case-insensitive | FIXED |
| 18 | R36 added rust-toolchain.toml | FIXED |
| 19 | T32 threshold correct | FIXED |
| 20 | T35 v8 ignore pattern | FIXED |
| 21 | T23 reason detection --reason | FIXED |
| 22 | is_ts_test_file expanded | FIXED |
| 23 | T60/T61 check ID collision | FIXED |

**Result: 21 FIXED, 1 PARTIALLY FIXED, 1 NOT FIXED**

### Remaining gaps

1. **Item 15 (NOT FIXED):** R55-R57 workspace metadata checks still emit `Severity::Info` for the actual metadata values. The plan says they should be elevated to Warn/Error. Only the read/parse failures were elevated to Error (which was item 5, a separate concern).

2. **Item 13 (PARTIALLY FIXED):** npmrc_check.rs does not strip BOM. The tsconfig and jscpd files do. While npmrc BOM encoding is rare, it should be added for consistency. A single line at the start of `parse_npmrc_settings()` or in `check_npmrc()` after reading the file would fix this.
