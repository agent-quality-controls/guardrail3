# RS-FMT — rustfmt.toml checker (8 rules)

**Input:** rustfmt.toml / .rustfmt.toml (one per workspace, plus per-crate)
**Parser:** TOML
**Current code:** `config_files.rs` (existence), `rustfmt_check.rs` (settings)

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-FMT-01 | R21 | Error | rustfmt.toml exists at workspace root | Implemented |
| RS-FMT-02 | R22 | Warn | Settings correctness: edition, max_width, tab_spaces, use_field_init_shorthand, use_try_shorthand, reorder_imports, reorder_modules | Implemented |
| RS-FMT-03 | R23 | Info | Extra settings beyond expected baseline (inventory) | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-FMT-04 | Warn | Nightly-only settings on stable toolchain. If rustfmt.toml contains nightly-only keys (group_imports, imports_granularity, format_code_in_doc_comments, format_strings, overflow_delimited_expr, normalize_comments, normalize_doc_attributes, wrap_comments, format_macro_matchers, format_macro_bodies, condense_wildcard_suffixes) AND rust-toolchain.toml has `channel = "stable"`, Warn. `cargo fmt` refuses to run. | Planned |
| RS-FMT-05 | Warn | Per-crate rustfmt.toml overrides. Same bypass as RS-CLIPPY-13 — rustfmt uses closest config, no merging. A sub-crate rustfmt.toml completely replaces root settings. Crawler already discovers these. Flag any non-root rustfmt.toml or .rustfmt.toml. | Planned |
| RS-FMT-06 | Warn | Edition mismatch: rustfmt.toml `edition` vs Cargo.toml `edition`. When they disagree, rustfmt formats for one edition while compiler parses another. Causes issues with edition-specific syntax (e.g., `gen` keyword in 2024). | Planned |
| RS-FMT-07 | Warn | `ignore` setting escape hatch. The `ignore` key in rustfmt.toml silently excludes entire directories from formatting. Promote from generic RS-FMT-03 inventory to specific Warn — escape hatches deserve explicit visibility. | Planned |
| RS-FMT-08 | Warn | Dual file conflict. Both `rustfmt.toml` and `.rustfmt.toml` exist at same level. rustfmt picks `rustfmt.toml`, but validator's `find_root_config` might pick a different one from sorted crawler results. Flag both-exist as Warn. | Planned |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Recommend `group_imports`/`imports_granularity` | Opinion, not enforcement. Already in generated template as comments. |
| `normalize_comments`/`normalize_doc_attributes` | Opinion. Nightly-only and opinionated. |
| Harmful stable settings (fn_single_line, etc.) | Opinion-based. No universal "wrong" value for stable settings. |
| Nightly key list staleness | Maintenance burden. `cargo fmt` itself catches unknown nightly keys on stable. |
| Typo fuzzy matching for keys | Existing signals sufficient (RS-FMT-03 inventories extras, rustfmt warns on unknown keys). |
