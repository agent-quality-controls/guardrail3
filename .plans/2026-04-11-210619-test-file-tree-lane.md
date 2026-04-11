Goal

Implement the remaining `test` family structural lane in packages so the old app-only `RS-TEST-02`, `RS-TEST-03`, and `RS-TEST-18` rules run through `g3rs-test-file-tree-checks` and `g3rs-test-ingestion::ingest_for_file_tree_checks(...)`.

Approach

1. Extend `g3rs-test-types` with a real `G3RsTestFileTreeChecksInput` and the minimal supporting types for:
   - owned test root and component facts
   - runtime/assertions dependency facts
   - owned Rust files relevant to structural test checks
   - file-tree input failures
2. Extend `g3rs-test-ingestion` to build that input from the existing root/component discovery:
   - reuse root selection and component discovery already used by config/source ingestion
   - classify and load the owned Rust files needed by the structural rules
   - parse component Cargo facts needed for runtime/assertions dependency boundaries
   - add ingestion-unit tests and end-to-end pipeline tests
3. Add `packages/rs/test/g3rs-test-file-tree-checks` following the established package pattern:
   - parse owned Rust files once in runtime support
   - implement `RS-TEST-02`, `RS-TEST-03`, and `RS-TEST-18`
   - add rule-local sidecar tests that cover the old app attack surface
4. Verify mechanically, then run adversarial review and close any gaps before reporting done.

Key decisions

- Keep this lane under `file-tree`, even though two rules parse Rust source.
  Why: these are structural ownership and boundary rules, not ordinary source semantics.
  Alternative rejected: folding them into `g3rs-test-source-checks`, because that would blur the clean split between test-quality source rules and workspace/component structural rules.

- Reuse the existing source parser logic inside the new file-tree runtime.
  Why: `RS-TEST-03` and `RS-TEST-18` need the same import/call/function facts already extracted by `g3rs-test-source-checks`.
  Alternative rejected: inventing a second parser surface or shoving pre-parsed AST facts into ingestion.

- Keep ingestion workspace-local and typed.
  Why: the current package model validates one pointed workspace, not the old repo-global surface.

Files to modify

- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-ingestion/**`
- `packages/rs/test/g3rs-test-file-tree-checks/**` (new package)
- package manifests and re-export `src/lib.rs` files under `packages/rs/test`
- relevant README/TODO files for `g3rs-test-ingestion` and the new file-tree package
