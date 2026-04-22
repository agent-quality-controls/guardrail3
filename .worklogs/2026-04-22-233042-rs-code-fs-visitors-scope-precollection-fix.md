Summary
- Fixed `rs/code` `fs_visitors` so alias resolution is scope-based instead of source-order based.
- Forward alias order now works inside a scope, and top-level `#[cfg(test)]` aliases no longer leak into production scans.

Decisions made
- Fixed the bug in the visitor/support boundary because the issue was incorrect scope modeling, not rule matching.
- Precollected aliases per file/module/block/function-like scope instead of adding exceptions for forward aliases.
- Skipped test-only items during production-scope precollection so `#[cfg(test)]` aliases stay isolated.

Key files for context
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/support.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/inline_std_fs.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_import.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_glob_import.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/false_positives.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/false_positives.rs
- .plans/2026-04-22-233042-rs-code-fs-visitors-scope-precollection-fix.md

Next steps
- Fix `RS-TEST-SOURCE-17` for qualified calls to re-aliased owned assertions names such as `self::again()`.
- Fix `RS-TEST-FILETREE-18` for module-alias helper calls like `h::fixture_path()` and `h::any_rule()`.
- Fix the new hooks/parser attack findings around helper-definition order and escaped-space-before-hash handling.
