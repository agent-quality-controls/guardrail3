## Summary

Built the first `test` family package set under `packages/rs/test`. This adds the family types crate, a real config checks lane, a real family ingestion package with `ingest_for_config_checks`, and a stub AST checks package so the family shape matches the current architecture.

## Decisions made

- Kept parsed config files at the package boundary instead of normalizing them too early.
  - `G3RsTestConfigChecksInput` carries typed `CargoToml`, `NextestToml`, and `MutantsToml`.
  - Rejected custom `nextest` / `mutants` summary structs because they would duplicate parser schemas and lose file semantics too early.

- Made `test` config ingestion root-scoped.
  - Each owned test root becomes one `G3RsTestConfigChecksInput`.
  - Rejected per-file config inputs because the live legacy rules reason about root-level mutation tooling and root-local config files.

- Kept mutation activation as a derived orchestration fact.
  - The runtime gates `RS-TEST-11..15` behind real mutation activity, matching the legacy family.
  - Rejected unconditional execution because it would fire on roots that do not participate in mutation testing.

- Stubbed AST and file-tree ingestion methods instead of guessing contracts.
  - `ingest_for_ast_checks` and `ingest_for_file_tree_checks` return typed not-implemented errors.
  - Rejected placeholder fake data because it would hide the remaining architecture work.

## Key files for context

- `.plans/2026-04-09-204323-test-config-and-ast-packages.md`
- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/roots.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/activation.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hooks.rs`

## Next steps

- Build `g3rs-test-ast-checks` for the real root-scoped AST rules.
- Implement `g3rs-test-ingestion::ingest_for_ast_checks`.
- Add adversarial parity coverage against the legacy `test` family once the AST lane exists.
