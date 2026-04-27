Goal

Replace the `code` config public input contract with parsed config-file variants only. Remove raw config-file content from the public boundary while preserving `g3rs-code/exception-comment-inventory` exception-comment inventory and `g3rs-code/unsafe-code-lint` Cargo lint checks.

Approach

1. Read the current `code` config type, ingestion, and rule code to identify every place that still depends on raw `content: String`.
2. Add a parsed `G3RsCodeConfigFileKind` enum with one variant per supported parsed config file type:
   - `Guardrail3Toml`
   - `ClippyToml`
   - `DenyToml`
   - `CargoToml`
   - `RustfmtToml`
   - `RustToolchainToml`
3. Add a typed `G3RsCodeExceptionComment` input surface so `g3rs-code/exception-comment-inventory` can consume extracted comment facts instead of raw file strings.
4. Update `g3rs-code-ingestion` to parse supported config files once and to extract exception comments during ingestion.
5. Decide how to handle legacy bare `rust-toolchain`:
   - do not keep it as raw text in the public contract
   - either exclude it from `code` config inputs or introduce a typed legacy representation if one already exists locally
6. Update `g3rs-code/exception-comment-inventory` and `g3rs-code/unsafe-code-lint` to consume the new typed boundary.
7. Add tests that fail under the old contract:
   - config input no longer carries raw file content
   - exception comment inventory still works across supported parsed config files
   - Cargo workspace unsafe-code lint checks still work
8. Verify:
   - `cargo test -q` in `packages/rs/code/g3rs-code-types`
   - `cargo test --workspace -q` in `packages/rs/code/g3rs-code-config-checks`
   - `cargo test --workspace -q` in `packages/rs/code/g3rs-code-ingestion`
   - `git diff --check`

Key decisions

- Do not keep `Text` or raw `content: String` in the public config file enum.
- Do not push parser work down into rules.
- Keep exception-comment discovery in ingestion as typed comment facts because the parser crates do not preserve comments.
- Prefer removing legacy bare `rust-toolchain` from this family boundary over keeping an untyped escape hatch.

Files to modify

- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_files.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_07_exception_comment_inventory/rule.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_07_exception_comment_inventory/rule_tests/*`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule_tests/*`
