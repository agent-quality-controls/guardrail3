# Fix silent failure patterns in Rust config validation

**Date:** 2026-03-19 20:04
**Task:** Fix silent failure patterns where parse/read errors are swallowed or use wrong severity

## Goal
Ensure all read/parse failures emit Error-severity CheckResults instead of silently continuing or using Warn.

## Approach

### 1. config_files.rs (HIGH-01/02) — lines 150-157
`check_per_crate_clippy_content`: read_file returns None → silent return; TOML parse fails → silent return.
Add Error CheckResult emissions before each return.

### 2. rustfmt_check.rs (HIGH-03) — lines 17, 35
`check_rustfmt_settings`: read error and parse error both use Severity::Warn. Change to Severity::Error.

### 3. toolchain_check.rs (HIGH-04/05) — lines 12, 28, 61-71
- read/parse errors use Severity::Warn → change to Severity::Error
- Channel check: "nightly" → Error; pinned version (not "stable", not "nightly") → Info

### 4. cargo_lints.rs (MED-05) — lines 318-325
`check_workspace_inheritance`: read_file returns None → silent continue; parse fails → silent continue.
Add Error emissions.

### 5. workspace_metadata.rs (MED-31) — lines 17-24
`check_workspace_metadata`: read_file returns None → silent return; parse fails → silent return.
Add Error emissions.

### 6. dependency_allowlist.rs — lines 19-26
`check_dependency_allowlist`: read_file returns None → silent return; parse fails → silent return.
Add Error emissions.

### 7. hex_arch_checks.rs — lines 163-168
`check_dependency_flow` and `check_library_service_boundary`: parse failures → silent continue.
These are less critical since they're per-dep-crate parsing, but should still emit errors.

## Files to Modify
- `config_files.rs` — add error emissions for per-crate clippy read/parse
- `rustfmt_check.rs` — Warn→Error for read/parse
- `toolchain_check.rs` — Warn→Error for read/parse, nightly→Error, pinned→Info
- `cargo_lints.rs` — add error emissions for per-crate Cargo.toml read/parse
- `workspace_metadata.rs` — add error emissions for Cargo.toml read/parse
- `dependency_allowlist.rs` — add error emissions for read/parse
- `hex_arch_checks.rs` — add error emissions for parse failures
