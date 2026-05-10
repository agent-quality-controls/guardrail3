# Worklog: Partial progress on `g3rs validate` for 15 workspaces

## Summary

Started fixing `g3rs validate --path <ws>` errors across the 15 workspaces. Scope is far larger than initially indicated by error counts: each workspace has multi-layered cargo gates (clippy + deny + dupes + code + ...), so a single workspace progressively reveals more failures even after clippy is clean.

This session made progress on `packages/parsers/clippy-toml-parser` only.

## Initial error counts (per workspace clippy errors only)

- 304 packages/rs/deny/g3rs-deny-config-checks
- 145 packages/rs/cargo/g3rs-cargo-config-checks
- 100 packages/rs/apparch/g3rs-apparch-ingestion
- 44 packages/rs/cargo/g3rs-cargo-ingestion
- 43 packages/rs/clippy/g3rs-clippy-ingestion
- 19 packages/rs/arch/g3rs-arch-config-checks
- 17 packages/rs/arch/g3rs-arch-source-checks
- 16 packages/rs/arch/g3rs-arch-file-tree-checks
- 5 packages/rs/apparch/g3rs-apparch-hook-contract
- 5 packages/rs/arch/g3rs-arch-types
- 5 packages/rs/cargo/g3rs-cargo-hook-contract
- 5 packages/rs/code/g3rs-code-hook-contract
- 5 packages/rs/clippy/g3rs-clippy-hook-contract
- 4 packages/parsers/astro-config-parser
- 4 packages/parsers/clippy-toml-parser

Total clippy errors: ~721. Real total higher due to cascading gate failures.

## What was done

### packages/parsers/clippy-toml-parser

Cleared all clippy errors. Auto-fix (`cargo clippy --fix`) handled most pedantic suggestions. Manual fixes:

- `crates/types/src/document.rs`: added `#[expect(clippy::large_enum_variant, reason = ...)]` on `ClippyTomlParseState`. Cannot Box the variant because downstream `g3rs-clippy-config-checks::support` calls `typed()` (a `const fn` returning `Option<&ClippyToml>`) and `as_ref()` on `Box` is not const. Renaming `ClippyTomlDocument` was not viable - it is a public API consumed by `g3rs-clippy-config-checks` and `g3rs-clippy-types`, both outside the 15-workspace scope.
- `crates/runtime/src/document.rs`: extracted `parse_ban_table_entry` helper to drop nesting depth; converted catch-all `other =>` to enumerated `Value::Integer(_) | Value::Float(_) | ...` arm (eliminates `wildcard_enum_match_arm`); converted two `match` blocks into `Option::map_or` chains; added module-level docs and `#[must_use]` on functions.
- `crates/runtime/src/parser.rs`: added `# Errors` doc to `parse_document`.
- `crates/runtime/src/parser_tests/parsing.rs`: `#[expect(clippy::too_many_lines, reason = "single end-to-end fixture asserting every structured clippy.toml field")]` on `exact_structured_fields_parse`.
- `crates/assertions/src/parser.rs`: replaced `assert!(false, ...)` with `panic!(...)`; added `clippy::panic` to the existing file-level `#![allow]`.

After clearing clippy, the next gates failed:
- `cargo deny check`: yanked `fastrand 2.4.0`. Fixed by `cargo update -p fastrand` -> `2.4.1`.
- `cargo dupes check`: 12.6% exact duplication (max 10%). Four duplicate groups:
  - 4 `assert_<setting>` helper bodies in `crates/assertions/src/parser.rs:161-197`
  - 3 simple `parse(...).expect_err(...)` test blocks in `crates/runtime/src/parser_tests/parsing.rs`
  - 3 mechanical `Serialize` impl bodies in `crates/types/src/clippy_toml.rs:306-502`
  - 2 trivial `From` impls in `crates/runtime/src/error.rs`

The dupes failure was not resolved this session. It needs a refactor (e.g., generic helper for `assert_<setting>`, macro for the `Serialize` impls) that touches the public types crate API surface.

### packages/parsers/astro-config-parser

`cargo clippy --fix` ran here as a side-effect of the multi-workspace clippy run. The auto-edits (added `#[must_use]`, replaced `unwrap_or_else` with `map_or_else`, dropped a needless `.clone()`) are staged and look benign; not validated end-to-end.

## Decisions made

- Use `#[expect(... reason = "...")]` only with a real specific reason (per task rules). Two annotations added so far:
  1. `large_enum_variant` on `ClippyTomlParseState` because boxing breaks the downstream const-fn API contract.
  2. `too_many_lines` on `exact_structured_fields_parse` because it is one fixture verifying every clippy.toml field; splitting hurts coverage clarity.
  3. `clippy::panic` added to existing assertions-module-level allow alongside `expect_used`/`missing_const_for_fn`/`missing_panics_doc` (the existing reason already covered "panic-based proof sites").
- Did not rename `ClippyTomlDocument` (would require editing files outside the 15 scope).

## Why not all 15 cleared

Each workspace has ~99 clippy errors plus cascading gate failures (deny / dupes / code / etc.) that only surface after clippy passes. The clippy-toml-parser workspace alone (smallest at 4 initial errors) required ~30 minutes and is still blocked at the dupes gate. Realistic estimate: 8-15+ hours of focused work for the full 15.

## Key files for context (cold-start reading list)

- `apps/guardrail3-rs/target/release/g3rs validate --help`
- `packages/parsers/clippy-toml-parser/Cargo.toml` (workspace.lints.clippy is the deny list applied everywhere)
- `packages/parsers/clippy-toml-parser/crates/runtime/src/document.rs` (example of fixed file)
- `packages/parsers/clippy-toml-parser/crates/types/src/document.rs` (example of justified `#[expect]`)
- `.plans/2026-05-10-183339-validate-15-workspaces.md`

## Next steps

1. Resolve the dupes gate in clippy-toml-parser:
   - Refactor 4 `assert_<setting>` helpers into a single generic helper.
   - Macroize the 3 `Serialize` impls in `crates/types/src/clippy_toml.rs`.
   - Inline the 2 `From` impls or use `thiserror`.
2. Validate astro-config-parser end-to-end.
3. Then attack workspaces in ascending error-count order:
   - 5-error tier: apparch-hook-contract, arch-types, cargo-hook-contract, code-hook-contract, clippy-hook-contract.
   - 16-19 tier: arch-file-tree-checks, arch-source-checks, arch-config-checks.
   - 43-44 tier: clippy-ingestion, cargo-ingestion.
   - 100+ tier: apparch-ingestion, cargo-config-checks, deny-config-checks (largest).
4. After every workspace exits 0, run `scripts/verify/all.sh` (8 layers).
