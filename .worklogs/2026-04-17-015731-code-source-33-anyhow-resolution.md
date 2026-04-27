# Summary
Fixed `g3rs-code/ast-33-public-weak-error-forms` so it only flags `anyhow::Error` when the return type actually resolves to `anyhow`. Bare typed errors like a local `Error` enum or `crate::error::Error` no longer false-positive.

# Decisions made
- Added import-aware `anyhow` binding collection in the code-source parser instead of dropping bare `Error` detection entirely.
- Kept the rule strict for real `anyhow` aliases: `use anyhow::Error;`, `use anyhow::Error as AppError;`, and `use anyhow as ah; ah::Error` still trigger.
- Left `String`, `&str`, and `Box<dyn Error>` behavior unchanged.

# Key files for context
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/analysis_helpers.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/types.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_33_public_weak_error_forms/rule_tests/direct.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_33_public_weak_error_forms/rule_tests/false_positives.rs`

# Next steps
- Commit the `guardrail3-rs-toml-parser` package cleanup separately.
- Continue sweeping `packages/parsers` and `packages/shared` package roots until the next issue is not clearly package debt or a clearly valid rule.
