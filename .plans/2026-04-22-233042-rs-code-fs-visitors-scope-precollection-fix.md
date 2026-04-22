Goal
- Make `fs_visitors` respect Rust scope semantics for aliasing so forward alias order works within a scope and `#[cfg(test)]` aliases do not leak into production scans.

Approach
- Add red tests for forward alias import/call/glob cases and for top-level `#[cfg(test)]` alias leakage.
- Move alias collection from sequential `visit_item_use` mutation to scope precollection in the shared visitor support layer.
- Apply the scope precollection consistently at file, module, block, and function-like scope boundaries in all three visitors.
- Verify with full package tests and `g3rs validate`.

Key decisions
- Fix in the visitor/support boundary because the bug is about scope modeling, not rule matching.
- Precollect aliases per scope instead of inventing source-order exceptions.
- Skip test-only items during production-scope precollection so `#[cfg(test)]` aliases stay isolated.

Files to modify
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/support.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/inline_std_fs.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_import.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_glob_import.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/false_positives.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/false_positives.rs
