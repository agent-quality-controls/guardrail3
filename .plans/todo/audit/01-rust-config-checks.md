# Adversarial Audit: Rust Config File Validation (R1-R29)

**Date:** 2026-03-19
**Auditor:** Adversarial review agent
**Scope:** config_files.rs, clippy_coverage.rs, cargo_lints.rs, rustfmt_check.rs, toolchain_check.rs
**Baselines:** domain/modules/clippy.rs, domain/modules/canonical.rs

---

## Critical Findings

### CRIT-01: Canonical cargo lints include `missing_docs` and `missing_debug_implementations` but validation ignores them

**File:** `cargo_lints.rs` vs `canonical.rs`

The canonical `CARGO_LINTS` module (in `canonical.rs` lines 55-56) defines:
```toml
missing_docs = "deny"
missing_debug_implementations = "warn"
```

But `EXPECTED_RUST_LINTS` in `cargo_lints.rs` does NOT include either of these. A project could be missing `missing_docs = "deny"` and `missing_debug_implementations = "warn"` in `[workspace.lints.rust]` and R26 would not flag it.

**Impact:** Silent pass on incomplete lint configuration. The generate command produces these lints, but validate does not verify they exist.

**Fix:** Add both to `EXPECTED_RUST_LINTS`:
```rust
LintExpectation { name: "missing_docs", expected_level: "deny", priority: None },
LintExpectation { name: "missing_debug_implementations", expected_level: "warn", priority: None },
```

### CRIT-02: `_profile` parameter in clippy_coverage::check is completely ignored

**File:** `clippy_coverage.rs` line 61

The function signature accepts `_profile: Option<&str>` but the comment on line 111-112 says "All profiles (service, library) use the same expected bans. Unknown/missing profiles default to service."

While currently the service and library profiles have the same method bans, the **type bans differ**. The library profile includes global-state type bans (`LazyLock`, `OnceLock`, `once_cell::sync::Lazy`, `once_cell::sync::OnceCell`) that are NOT in `EXPECTED_TYPE_BANS`.

**Impact:** For library-profile workspaces, the validation will never check that global-state type bans are present in the workspace-level clippy.toml. A library project could omit `LazyLock`/`OnceLock` bans and R5 would not flag it.

**Specifically missing from `EXPECTED_TYPE_BANS`:**
- `std::sync::LazyLock`
- `std::sync::OnceLock`
- `once_cell::sync::Lazy`
- `once_cell::sync::OnceCell`

These are in `clippy.rs::TYPE_GLOBAL_STATE` and included in `library_profile_types()` but never validated.

### CRIT-03: R1 uses `clippy_path.exists()` directly instead of going through `FileSystem` trait

**File:** `config_files.rs` line 16

R1, R21, R24 all use `path.exists()` directly (std filesystem call) instead of the injected `FileSystem` trait. This:
1. Bypasses the centralized filesystem module
2. Makes these checks untestable with mock filesystems
3. Is inconsistent with the rest of the codebase which uses `fs.read_file()` / `fs.read_file_err()`

Same issue at lines 43, 71, and in `clippy_coverage.rs` line 66, `cargo_lints.rs` line 113, and `cargo_lints.rs` line 309.

**Impact:** Cannot unit test file-not-found paths without actually creating/deleting files on disk.

---

## High Findings

### HIGH-01: clippy.toml TOML parse error is silently swallowed in `check_per_crate_clippy_content`

**File:** `config_files.rs` line 156

```rust
let table: toml::Value = match content.parse() {
    Ok(v) => v,
    Err(_) => return,  // SILENT RETURN
};
```

If a per-crate `clippy.toml` is malformed TOML, the function silently returns with no diagnostic. The R2 result from line 113-124 has already been emitted as "Per-crate clippy.toml exists" (Info). The user sees a passing check for a broken file.

Compare to `check_clippy_thresholds` (line 229-243) which properly emits an R3 Error on parse failure.

**Fix:** Emit a Warn or Error result when per-crate clippy.toml fails to parse.

### HIGH-02: `read_file` failure is silently swallowed in `check_per_crate_clippy_content`

**File:** `config_files.rs` line 150

```rust
let Some(content) = fs.read_file(path) else {
    return;  // SILENT RETURN
};
```

Same issue as HIGH-01 but for read failures. The file was confirmed to exist (line 112) but cannot be read. No diagnostic emitted.

### HIGH-03: R22 rustfmt.toml parse/read failures are Warn, should be Error

**File:** `rustfmt_check.rs` lines 17 and 34

If the file exists (R21 passes) but can't be read or parsed, the severity is `Warn`. But if we cannot parse the file, we have ZERO confidence in its settings. This should be `Error` — we confirmed the file exists so unreadability means something is actually wrong.

Compare to `config_files.rs` line 218 where clippy.toml unreadable is `Error`, and line 231 where parse error is `Error`.

### HIGH-04: R25 toolchain parse/read failures are Warn, should be Error

**File:** `toolchain_check.rs` lines 10 and 29

Same reasoning as HIGH-03. The file exists (R24 passed) but cannot be read/parsed. Should be Error for consistency and because this represents a real problem.

### HIGH-05: Toolchain channel "nightly" or pinned version strings accepted with only Warn

**File:** `toolchain_check.rs` line 61-69

A `channel = "nightly"` or `channel = "1.75.0"` only produces a Warn. For production services, using nightly is a stability risk — this should arguably be Error severity, or at minimum the message should be much stronger.

A pinned version like `"1.75.0"` is actually FINE (and arguably more reproducible than "stable"), but the check treats it the same as "nightly". These two cases should be distinguished.

### HIGH-06: `check_ban_list` does not verify ban entries have `reason` fields

**File:** `clippy_coverage.rs` lines 169-175

The function extracts `path` from ban entries but never checks that a `reason` field exists. A clippy.toml with:
```toml
disallowed-methods = [
    { path = "std::env::var" }
]
```
(no reason) would pass R4. Without a reason, clippy will still enforce the ban but the developer gets no guidance on what to use instead.

**Impact:** Bans without reasons pass validation silently. The canonical modules all include reasons, so `guardrail3 generate` produces them, but hand-edited files could omit reasons.

### HIGH-07: `check_rustfmt_str` emits no result on success (silent pass)

**File:** `rustfmt_check.rs` line 89

```rust
Some(toml::Value::String(v)) if v == expected_val => {}  // NOTHING EMITTED
```

When a rustfmt string setting is correct, no CheckResult is emitted at all. All other checks (clippy thresholds, lint levels, toolchain settings) emit an Info/inventory result on success. This means `--inventory` output will be inconsistent — you'll see confirmations for int/bool rustfmt settings but not string settings.

Compare to `check_rustfmt_setting` (line 128) which also emits nothing on success. Both functions are inconsistent with the pattern used everywhere else.

---

## Medium Findings

### MED-01: R3 threshold values are hardcoded, not sourced from canonical module

**File:** `config_files.rs` lines 245-251

```rust
let expected: &[ExpectedInt<'_>] = &[
    ("too-many-lines-threshold", 75),
    ("cognitive-complexity-threshold", 15),
    ("too-many-arguments-threshold", 7),
    ("type-complexity-threshold", 75),
    ("max-struct-bools", 3),
];
```

These values are duplicated from `clippy.rs::THRESHOLDS`. If the canonical thresholds change, this code must be updated manually. There's no compile-time link between the two.

**Fix:** Parse `clippy::THRESHOLDS` at compile time or test time and verify consistency, or extract the expected values into a shared const.

### MED-02: R22/R23 expected rustfmt settings are hardcoded, not sourced from canonical module

**File:** `rustfmt_check.rs` lines 54-61

The expected values (`edition = "2024"`, `max_width = 100`, etc.) are hardcoded. The canonical module (`canonical.rs::RUSTFMT`) is the source of truth. If someone updates the canonical RUSTFMT module to change `max_width` to 120, the validator would still expect 100.

### MED-03: R25 toolchain components check is incomplete

**File:** `toolchain_check.rs` line 96

Only checks for `["clippy", "rustfmt"]`. The canonical `RUST_TOOLCHAIN` module also only defines these two, so this is consistent — but there's no check for `rust-src` or `llvm-tools-preview` which are commonly needed for coverage (`cargo llvm-cov`). This is borderline acceptable since the canonical module also doesn't include them.

### MED-04: cargo_lints.rs priority check only fires when level is correct

**File:** `cargo_lints.rs` lines 441-484

The `emit_lint_correct` function checks priority, but it's only called when the lint level matches. If the lint level is wrong AND the priority is wrong, only the level issue is reported. The priority issue is silently masked.

This is arguably acceptable (fix the level first, then the priority), but it means the user might need two passes to fix all issues.

### MED-05: `check_lint_level` accepts "forbid" as valid when "deny" is expected, but NOT the reverse

**File:** `cargo_lints.rs` lines 409-422

```rust
Some("forbid") if expected_level == "deny" => { /* Info: stricter than expected */ }
```

This is correct — forbid is stricter than deny. But there's no equivalent for the case where expected is "forbid" and actual is "deny". In `EXPECTED_RUST_LINTS`, `unsafe_code` expects "forbid". If someone sets it to "deny" instead, the `emit_lint_wrong` function is called and `is_weakened` correctly evaluates to `true` (line 497: `("forbid", "deny")`), producing an Error. This is correct.

**No bug here**, but documenting the analysis for completeness.

### MED-06: `check_workspace_inheritance` silently continues on read/parse failure

**File:** `cargo_lints.rs` lines 313-319

```rust
let Some(content) = fs.read_file(&crate_cargo) else {
    continue;  // SILENT
};
let table: toml::Value = match content.parse() {
    Ok(v) => v,
    Err(_) => continue,  // SILENT
};
```

If a crate's Cargo.toml exists but cannot be read or parsed, no R29 result is emitted. The crate is silently skipped, giving the impression that all crates were checked.

### MED-07: R23 extra settings check only works for top-level keys

**File:** `rustfmt_check.rs` lines 200-218

`check_rustfmt_extra_settings` only iterates over top-level table keys. If someone nests settings under a section (which rustfmt doesn't actually support, but TOML allows), they'd be reported as a single extra key. This is fine for rustfmt since it uses flat keys, but the check doesn't validate that no unexpected sections exist.

### MED-08: R2 per-crate clippy.toml global-state check uses `contains()` for string matching

**File:** `config_files.rs` lines 173-178

```rust
for gs_type in &global_state_types {
    for tp in &type_paths {
        if tp.contains(gs_type) {
            found_global_bans.push(tp.clone());
        }
    }
}
```

Using `contains()` for substring matching is fragile. A type path like `my_crate::NotLazyLock` would match because it contains "LazyLock". A path like `std::sync::LazyLockGuard` would also match incorrectly.

**Fix:** Match against the full canonical paths (`std::sync::LazyLock`, `std::sync::OnceLock`, `once_cell::sync::Lazy`, `once_cell::sync::OnceCell`) or at least check that the match occurs at a segment boundary.

---

## Low Findings

### LOW-01: R6/R7 extra bans are reported as Info (inventory), never flagged

**File:** `clippy_coverage.rs` lines 206-218

Extra bans beyond the baseline are always Info/inventory. This is by design (project-specific bans are fine), but there's no way to detect if someone added a ban that conflicts with or duplicates a baseline ban with different casing or path.

For example, adding `std::collections::hashmap::HashMap` (wrong path) would show as an "extra ban" instead of being flagged as a likely error.

### LOW-02: R28 allows missing expected "allow" entries without any severity

**File:** `cargo_lints.rs` lines 285-295

When an expected allow lint is missing, the result is Info. The message says "consider adding" but the severity is Info. This means if a project is using `deny` groups (all, pedantic, nursery) but forgot to add the approved `allow` exceptions, they'll get noisy false positives from clippy — but guardrail3 only whispers about it.

This might be intentional (allows are suggestions, not requirements), but it means projects can have broken builds and guardrail3 doesn't flag it as a problem.

### LOW-03: R22 rustfmt "wrong value" message includes TOML display formatting

**File:** `rustfmt_check.rs` line 95

```rust
message: format!("{key} = {v} but should be \"{expected_val}\"."),
```

The `{v}` here uses the TOML `Display` impl which wraps strings in quotes. So the message would read: `edition = "2021" but should be "2024"`. The double quoting is visually awkward but functionally fine.

### LOW-04: R3 uses `read_file_err` but R2 per-crate check uses `read_file`

**File:** `config_files.rs` lines 150 vs 213

Inconsistent error handling between workspace clippy.toml (R3 uses `read_file_err`, gets error message) and per-crate clippy.toml (R2 uses `read_file`, loses error detail). Should consistently use `read_file_err`.

### LOW-05: No check for duplicate entries in disallowed-methods/disallowed-types

**File:** `clippy_coverage.rs` lines 168-176

The `found_paths` is a `BTreeSet`, so duplicates in the clippy.toml are silently deduplicated. If someone has the same path listed twice (perhaps with different reasons), this won't be flagged.

### LOW-06: `check_rustfmt_str` has a different code path from `check_rustfmt_int`/`check_rustfmt_bool`

**File:** `rustfmt_check.rs`

`check_rustfmt_str` (line 81) is a standalone function with its own match arms. `check_rustfmt_int` and `check_rustfmt_bool` both delegate to `check_rustfmt_setting` (line 119). There's also dead code potential — `check_rustfmt_setting` is general enough to handle strings too, but `check_rustfmt_str` doesn't use it.

### LOW-07: No validation that rustfmt.toml nightly-only settings are NOT uncommented on stable

**File:** `rustfmt_check.rs`

The canonical rustfmt.toml includes commented-out nightly-only settings (`imports_granularity`, `group_imports`, etc.). If someone uncomments these and the project uses stable rustfmt, `cargo fmt` will error. R22/R23 don't check for this — an uncommented nightly setting would show up as R23 "extra setting" (Info) instead of a warning about incompatibility.

---

## Bypass Vectors

### BYPASS-01: clippy.toml ban can be neutered by changing the `reason` without changing the `path`

If someone changes the reason to misleading text (e.g., `reason = "allowed"`), the R4/R5 checks still pass because they only verify path presence, not reason content.

### BYPASS-02: Workspace lint inheritance can be circumvented by per-crate overrides

**File:** `cargo_lints.rs`

R29 checks that `[lints] workspace = true` exists, but a crate can have BOTH `workspace = true` AND per-crate lint overrides that weaken specific lints:
```toml
[lints]
workspace = true

[lints.clippy]
unwrap_used = "allow"  # Overrides workspace deny
```

R29 would report "Workspace lints inherited" (pass) while the crate has silently weakened critical lints. No check exists for per-crate lint overrides.

### BYPASS-03: TOML comments can hide intent

None of the checks validate TOML comments. A file could have:
```toml
# TODO: re-enable this
# unsafe_code = "forbid"
```

And the uncommented version might be `unsafe_code = "deny"` or missing entirely. This is standard TOML behavior and not really bypassable via the tool, but worth noting.

### BYPASS-04: `clippy.toml` at a nested path takes precedence over workspace root

Clippy uses the closest `clippy.toml` to the file being linted. If a `clippy.toml` exists in `src/` (not in a crate root, just inside the source tree), it would override the workspace one. R2 only checks `member_dirs`, not arbitrary subdirectories.

---

## Summary Table

| ID | Severity | Check | Issue |
|---|---|---|---|
| CRIT-01 | Critical | R26 | `missing_docs` and `missing_debug_implementations` not validated |
| CRIT-02 | Critical | R4-R5 | Profile parameter ignored; library global-state type bans never validated |
| CRIT-03 | Critical | R1/R21/R24 | Direct `path.exists()` bypasses FileSystem trait |
| HIGH-01 | High | R2 | Per-crate clippy.toml parse error silently swallowed |
| HIGH-02 | High | R2 | Per-crate clippy.toml read failure silently swallowed |
| HIGH-03 | High | R22 | Parse/read errors should be Error, not Warn |
| HIGH-04 | High | R25 | Parse/read errors should be Error, not Warn |
| HIGH-05 | High | R25 | Nightly vs pinned version not distinguished |
| HIGH-06 | High | R4-R5 | Ban entries without `reason` field not flagged |
| HIGH-07 | High | R22 | String setting success emits no result (inconsistent) |
| MED-01 | Medium | R3 | Threshold values duplicated from canonical, not linked |
| MED-02 | Medium | R22 | Rustfmt expected values duplicated from canonical |
| MED-03 | Medium | R25 | Only checks clippy/rustfmt components |
| MED-04 | Medium | R27 | Priority error masked when level is also wrong |
| MED-05 | Medium | — | (Analysis only, no bug) |
| MED-06 | Medium | R29 | Silent skip on read/parse failure |
| MED-07 | Medium | R23 | Only checks top-level keys |
| MED-08 | Medium | R2 | Substring matching for global-state types is fragile |
| LOW-01 | Low | R6/R7 | No detection of typo'd extra bans |
| LOW-02 | Low | R28 | Missing allows not flagged as problem |
| LOW-03 | Low | R22 | Awkward double-quoting in error message |
| LOW-04 | Low | R2/R3 | Inconsistent read_file vs read_file_err |
| LOW-05 | Low | R4-R5 | Duplicate ban entries silently deduplicated |
| LOW-06 | Low | R22 | Dead code / inconsistent code paths |
| LOW-07 | Low | R22/R23 | Nightly-only settings not flagged on stable |
| BYPASS-01 | — | R4-R5 | Reason field not validated |
| BYPASS-02 | — | R29 | Per-crate lint overrides can weaken workspace lints |
| BYPASS-03 | — | — | TOML comments hide intent (inherent limitation) |
| BYPASS-04 | — | R2 | Nested clippy.toml in non-crate dirs not detected |
