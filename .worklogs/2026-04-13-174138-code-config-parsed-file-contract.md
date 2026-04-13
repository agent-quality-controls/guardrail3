Summary

Replaced the `code` family config public boundary with parsed config-file variants and typed exception-comment facts. Removed raw config file content from the public `code` config input while preserving `RS-CODE-CONFIG-07` and `RS-CODE-CONFIG-12`.

Decisions made

- Kept parser work in ingestion. Rules no longer receive raw config file strings.
- Split the `code` config surface into:
  - parsed config files for semantic config rules
  - typed exception-comment facts for `RS-CODE-CONFIG-07`
- Rejected keeping a `Text` fallback variant. That would keep the public boundary weak.
- Kept legacy bare `rust-toolchain` out of the parsed config file set because there is no parser-backed representation for it in the repo. It still contributes exception-comment facts during ingestion.
- Tightened tests to use valid TOML comments (`#`) for TOML-backed files instead of relying on the old raw-text loophole for `//`.

Key files for context

- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_files.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_07_exception_comment_inventory/rule.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

Next steps

- Audit the remaining family public types for the same weakness: raw file content or mixed parsed/unparsed variants.
- Decide whether legacy bare `rust-toolchain` needs its own parser-backed representation anywhere else, or whether it should stay outside parsed config-family boundaries.
