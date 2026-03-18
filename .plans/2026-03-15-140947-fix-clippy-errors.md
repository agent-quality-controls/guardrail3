# Fix all clippy errors in guardrail3

**Date:** 2026-03-15 14:09
**Task:** Fix ALL clippy errors (930 total) to achieve zero errors/warnings with `cargo clippy --all-targets`

## Goal
`cargo clippy --all-targets` produces zero errors and zero warnings. `cargo build` succeeds. `cargo run -- validate .` still works.

## Approach

Fix errors by category across all files:

1. **`str_to_string` (605)** — Replace `.to_string()` on `&str` with `.to_owned()`
2. **`print_stdout` (80) / `print_stderr` (25)** — Add `#[allow(clippy::print_stdout)]` or `#[allow(clippy::print_stderr)]` with `// reason: CLI output` at function level
3. **`manual_let_else` (43)** — Convert `match ... { Some(x) => x, None => { return } }` to `let Some(x) = ... else { return }`
4. **`disallowed_methods` for `std::process::exit` (22)** — Add `#[allow(clippy::disallowed_methods)] // reason: CLI entry point, exit code required` on specific calls
5. **`disallowed_methods` for `Command::new` (7)** — Add `#[allow(clippy::disallowed_methods)] // reason: CLI tool runs external commands`
6. **`redundant_closure` (18)** — Replace closures with method references
7. **`pub(crate) function inside private module` (18)** — These are fine, add allows or make modules pub(crate)
8. **`string_slice` (13)** — Replace indexing with safe alternatives
9. **`type_complexity` (11)** — Extract type aliases
10. **`too_many_lines` (multiple)** — Extract helper functions to reduce function size
11. **`needless_raw_string_hashes` (3)** — Remove `#` from raw strings
12. **`unnested_or_patterns` (1)** — Merge pattern arms
13. **`similar_names` (2)** — Rename variables
14. **All other categories** — Fix individually

## Files to Modify
All 50+ source files under src/
