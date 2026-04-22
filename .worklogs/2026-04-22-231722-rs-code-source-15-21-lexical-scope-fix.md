Summary
- Fixed the `rs/code` std/fs visitor scope bug at the parser boundary.
- Alias state no longer leaks across sibling functions or nested blocks, and the `fs_visitors` split now follows the module-directory facade rules.

Decisions made
- Fixed the bug in the shared visitor layer instead of adding rule-local exceptions.
- Widened the fix from sibling functions to lexical block scope because `use` items are block-scoped in Rust.
- Split `fs_visitors` into `api.rs`, `support.rs`, and per-visitor files so `mod.rs` stays facade-only and the package stays green under arch checks.

Key files for context
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/mod.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/api.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/support.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/inline_std_fs.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_import.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_glob_import.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/false_positives.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs

Next steps
- Fix the `rs/test` `RS-TEST-SOURCE-17` alias-chain false positive: owned assertions alias -> local alias -> harness call.
- Fix the hook parser function-tail bug caused by `rsplit_once('}')` misreading braces in strings/comments.
- Fix the hook parser `\#` handling so escaped hash does not truncate executable text.
- Tighten `RS-HOOKS-SOURCE-15` so discarded trigger-like comparisons do not count as guarded coverage.
