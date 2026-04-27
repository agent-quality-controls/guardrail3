# Goal
Fix `g3rs-code/ast-33-public-weak-error-forms` so it flags public `Result<_, anyhow::Error>` only when the error type actually resolves to `anyhow`, and does not false-positive on local typed errors named `Error`.

# Approach
- Add rule tests that prove the current bug and cover the broader resolution cases.
- Read the current public-result-error parser and import helpers.
- Thread simple file-local import resolution into the weak-error detector.
- Keep existing `String`, `&str`, and `Box<dyn Error>` behavior unchanged.
- Re-run the code rule tests and then re-run validation on `packages/parsers/guardrail3-rs-toml-parser`.

# Key decisions
- Do not weaken the rule by dropping bare `Error` detection entirely.
- Resolve only file-local imports and direct aliases; no deep name resolution.
- Treat bare `Error` as weak only when the file explicitly imports it from `anyhow`.

# Files to modify
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/analysis_helpers.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_33_public_weak_error_forms/rule_tests/*.rs`
