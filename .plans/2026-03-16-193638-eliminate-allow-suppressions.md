# Eliminate 23 lazy #[allow] suppressions

**Date:** 2026-03-16 19:36
**Task:** Remove #[allow] attributes by fixing the underlying code

## Goal
Remove all lazy #[allow] suppressions and fix the code properly.

## Approach

### 1. case_sensitive_file_extension_comparisons (12 instances)
Replace `str.ends_with(".ext")` with `Path::new(s).extension().is_some_and(|e| e == "ext")`.

### 2. dead_code (4 instances)
All on config types deserialized by serde — keep fields but remove allows since fields are `pub` and accessed elsewhere.

### 3. format_push_string (4 instances)
Replace `out.push_str(&format!(...))` with `use std::fmt::Write; write!(out, ...).ok();`

### 4. type_complexity (8 instances)
Most are on simple types like `&[(&str, i64)]` or `Vec<(usize, String)>` — clippy wouldn't actually flag these. Remove allows. For `Option<BTreeMap<String, CrateConfig>>` and `Vec<(String, String)>` — also not complex enough for clippy to flag.

### 5. or_fun_call (4 instances)
Replace `.map_or("?".to_owned(), ...)` with `.map_or_else(|| "?".to_owned(), ...)`.

## Files to Modify
- apps/guardrail3/src/app/ts/validate/source_scan.rs
- apps/guardrail3/src/app/ts/validate/ts_arch_checks.rs
- apps/guardrail3/src/app/ts/validate/ts_comment_checks.rs
- apps/guardrail3/src/app/ts/validate/test_checks.rs
- apps/guardrail3/src/app/hooks/validate.rs
- apps/guardrail3/src/app/hooks/deploy_checks.rs
- apps/guardrail3/src/app/rs/validate/source_scan.rs
- apps/guardrail3/src/app/rs/validate/test_quality_checks.rs
- apps/guardrail3/src/domain/modules/deny.rs
- apps/guardrail3/src/domain/modules/clippy.rs
- apps/guardrail3/src/domain/config/types.rs
- apps/guardrail3/src/commands/generate.rs
- apps/guardrail3/src/app/ts/validate/npmrc_check.rs
- apps/guardrail3/src/app/ts/validate/tsconfig_check.rs
- apps/guardrail3/src/app/ts/validate/package_check.rs
- apps/guardrail3/src/app/rs/validate/cargo_lints.rs
- apps/guardrail3/src/app/rs/validate/rustfmt_check.rs
- apps/guardrail3/src/app/rs/validate/config_files.rs
