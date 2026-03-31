# RS-FMT — rustfmt.toml checker (8 rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/fmt.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/fmt.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** effective repository rustfmt policy file plus any nested override files
**Parser:** TOML
**Current code:** `apps/guardrail3/crates/app/rs/families/fmt/` (`crates/runtime`, `crates/assertions`, and `test_support`; old `checks/rs/fmt/**`, `config_files.rs`, and `rustfmt_check.rs` are legacy seed material only)

## Implementation mapping contract

- exactly one `RS-FMT-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `facts.rs` and `inputs.rs` may contain shared facts and typed inputs only

Forbidden:

- grouped family test files such as `fmt_tests.rs`
- regressions back to single-file sidecars; each rule already uses a rule-specific test module directory and should keep splitting by attack vector inside that directory as coverage expands
- helper files that hide multiple rule predicates behind one API

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-FMT-01 | R21 | Error | Root rustfmt config exists at repository root (`rustfmt.toml` or `.rustfmt.toml`). Success is intentionally quiet; missing root config is Error. | Implemented |
| RS-FMT-02 | R22 | Warn/Error | Baseline settings correctness. Owned keys are exactly: `edition`, `style_edition`, `max_width`, `tab_spaces`, `use_field_init_shorthand`, `use_try_shorthand`, `reorder_imports`, `reorder_modules`. Wrong or missing value is Warn; unreadable/unparseable or non-table root config for this rule is Error. | Implemented |
| RS-FMT-03 | R23 | Info | Extra settings beyond expected baseline (inventory) | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-FMT-04 | Warn | Nightly-only settings on stable toolchain. If rustfmt.toml contains nightly-only keys (group_imports, imports_granularity, format_code_in_doc_comments, format_strings, overflow_delimited_expr, normalize_comments, normalize_doc_attributes, wrap_comments, format_macro_matchers, format_macro_bodies, condense_wildcard_suffixes) AND rust-toolchain.toml has `channel = "stable"`, Warn. `cargo fmt` refuses to run. | Implemented |
| RS-FMT-05 | Error | Per-crate rustfmt.toml overrides are forbidden. Same bypass as RS-CLIPPY-13 — rustfmt uses closest config, no merging. A sub-crate rustfmt.toml completely replaces root settings. Placement is now enforced by `RS-TOPOLOGY-16`; `RS-FMT` no longer owns nested placement discovery. | Implemented (moved to `RS-TOPOLOGY-16`) |
| RS-FMT-06 | Warn | Edition mismatch: rustfmt.toml `edition` vs Cargo.toml `edition`. When they disagree, rustfmt formats for one edition while compiler parses another. Causes issues with edition-specific syntax (e.g., `gen` keyword in 2024). | Implemented |
| RS-FMT-07 | Warn | `ignore` setting escape hatch. The `ignore` key in rustfmt.toml silently excludes entire directories from formatting. Promote from generic RS-FMT-03 inventory to specific Warn — escape hatches deserve explicit visibility. | Implemented |
| RS-FMT-08 | Warn | Dual file conflict. Both `rustfmt.toml` and `.rustfmt.toml` exist at same level. rustfmt picks `rustfmt.toml`, but validator's `find_root_config` might pick a different one from sorted crawler results. Flag both-exist as Warn. | Implemented |

## Scope decision

`RS-FMT` is intentionally a repository-root family, not a per-workspace/per-package policy-root family.

It owns:
- the one effective root formatting contract
- detection of any nested configs that would shadow or replace that contract
- consistency checks against the root `Cargo.toml` edition and root toolchain channel

It does **not** currently model:
- separate allowed local formatting roots
- inherited formatting policy by workspace/package boundary

That is deliberate. A nested `rustfmt.toml` is treated as an override escape hatch, not as a second legitimate policy root.

## Discovery / ownership model

- the root config is:
  - `rustfmt.toml` if present at repo root
  - otherwise `.rustfmt.toml` if present at repo root
- any non-root `rustfmt.toml` or `.rustfmt.toml` is an `RS-FMT-05` override candidate
- if both root filenames exist in the same directory, `RS-FMT-08` reports the conflict

## Input integrity / fail-closed expectations

The family depends on:
- the root rustfmt config
- the root `Cargo.toml` for edition comparison
- the root `rust-toolchain.toml` for stable/nightly interaction

Malformed inputs that are required to evaluate a rule should not silently degrade the rule to “no finding”.
In particular:
- malformed root rustfmt config, including unsupported non-table top-level shapes, must surface explicitly through the rule that needs it
- malformed root `Cargo.toml` or `rust-toolchain.toml` must not silently disable `RS-FMT-04` or `RS-FMT-06`

For the current family shape:
- `RS-FMT-02` owns parse failures of the root rustfmt config
- `RS-FMT-04` and `RS-FMT-06` own parse/input failures of the secondary files they need
- there is no separate family-wide input-failure rule at the moment

## Cross-family dependencies

- `RS-FMT-04` depends on the root toolchain contract from `RS-TOOLCHAIN`
- `RS-FMT-06` depends on the root Cargo edition contract from `RS-CARGO`
- nested `rustfmt.toml` is treated as a local override bypass in the same spirit as local `clippy.toml` / `deny.toml`, but unlike those families it is not an allowed policy-root mechanism

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Recommend `group_imports`/`imports_granularity` | Opinion, not enforcement. Already in generated template as comments. |
| `normalize_comments`/`normalize_doc_attributes` | Opinion. Nightly-only and opinionated. |
| Harmful stable settings (fn_single_line, etc.) | Opinion-based. No universal "wrong" value for stable settings. |
| Nightly key list staleness | Maintenance burden. `cargo fmt` itself catches unknown nightly keys on stable. |
| Typo fuzzy matching for keys | Existing signals sufficient (RS-FMT-03 inventories extras, rustfmt warns on unknown keys). |
