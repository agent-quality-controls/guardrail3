Summary
- Fixed shared `rs/code` fs-visitor semantics so mixed `cfg(any(test, ...))` production paths still fire and grouped non-`fs` `std::{...}` imports stay quiet.
- Fixed attributed `use` diagnostics to report the actual import line instead of the preceding `#[cfg(...)]` line.

Decisions made
- Kept the fix in shared parse/fs-visitor helpers because both `RS-CODE-SOURCE-15` and `RS-CODE-SOURCE-21` rely on the same cfg and grouped-import semantics.
- Replaced the old "mentions test anywhere" gating with a `cfg` evaluator that asks whether the predicate can be true when `test = false`.
- Tightened grouped `std::{...}` matching so only `fs` subtrees count. `self as ...` and bare grouped globs no longer masquerade as `std::fs`.

Key files for context
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/helpers.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/support.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_import.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/std_fs_glob_import.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/false_positives.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/false_positives.rs`
- `.plans/2026-04-23-095221-rs-code-fs-cfg-and-group-matching-fix.md`

Next steps
- Land the in-flight `rs/test` worker fixes for owned-assertions root aliases and runtime import-alias chains.
- Review the hook worker fix for shared command-query helper chaining and redefinition.
- Land the `rs/apparch` ingestion fix for private child modules leaking into public surface facts.
