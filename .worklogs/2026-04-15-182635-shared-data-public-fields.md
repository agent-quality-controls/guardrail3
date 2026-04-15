Summary

Fixed `RS-CODE-SOURCE-31` so it no longer punishes plain shared transport structs for using public fields. Finished `packages/rs/clippy/g3rs-clippy-types` after that rule fix by removing its last inherent method and re-validating the package.

Decisions made

- Kept the rule for normal crates. Public field bags still signal there.
- Added one explicit exception path instead of a naming hack:
  - crate must say `shared = true`
  - struct must be a brace-body public record
  - all named fields must be `pub`
  - no inherent `impl StructName { ... }`
- Improved the rule messages so they say what is wrong, what to do, and why:
  - normal crate public field bag
  - shared crate with methods
  - shared crate with mixed field visibility
- Proved the new metadata path in ingestion with direct tests for:
  - `shared = true`
  - normal crate without shared metadata
- Removed the unused `from_typed` inherent constructor from `g3rs-clippy-types` so the shared input struct is a plain data record.

Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/support.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/classify.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/clippy/g3rs-clippy-types/src/types.rs`

Next steps

- Continue package-by-package validation.
- The whole clippy family is now clean:
  - `g3rs-clippy-config-checks`
  - `g3rs-clippy-filetree-checks`
  - `g3rs-clippy-ingestion`
  - `g3rs-clippy-types`
- Move to the next package outside clippy and keep the same loop:
  - fix obvious package issues
  - stop only on a real rule contradiction
