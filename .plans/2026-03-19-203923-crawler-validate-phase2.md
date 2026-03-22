# Phase 2: Wire crawler data into Rust validation checks

**Date:** 2026-03-19 20:39
**Task:** Replace hardcoded path construction in RS validate checks with crawler data

## Goal
Each check module currently does `workspace_root.join("filename")` to find configs. Change to use relevant fields from `CrawlResult` instead. The `_crawl` parameter added in Phase 1 gets renamed to `crawl` and used.

## Approach

### mod.rs (orchestrator)
- Rename `_crawl` to `crawl`
- Filter crawler vecs by workspace_root prefix
- Pass filtered slices to each check module

### config_files.rs
- `check()`: Accept `&[PathBuf]` for clippy_tomls, rustfmt_tomls, rust_toolchains instead of constructing paths
- Find root config by filtering for `parent == workspace_root`
- `check_per_crate_clippy()`: Accept `&[PathBuf]` clippy_tomls, filter by workspace membership

### clippy_coverage.rs
- `check()`: Accept `&[PathBuf]` clippy_tomls, find root one at workspace_root

### deny_audit.rs
- `check()`: Accept `&[PathBuf]` deny_tomls, iterate each one within workspace

### deny_bans.rs, deny_licenses.rs, deny_inventory.rs
- These are called FROM deny_audit — no signature change needed, they already take parsed table + path

### cargo_lints.rs
- `check()`: Accept `&[PathBuf]` cargo_tomls, find workspace root one

## Files to Modify
- `app/rs/validate/mod.rs` — orchestrator wiring
- `app/rs/validate/config_files.rs` — accept crawler paths
- `app/rs/validate/clippy_coverage.rs` — accept crawler paths
- `app/rs/validate/deny_audit.rs` — iterate deny_tomls from crawler
- `app/rs/validate/cargo_lints.rs` — accept crawler cargo_tomls
