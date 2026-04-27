Summary

Fixed the remaining non-hexarch gaps from the repo-wide family audit. Clippy no longer advertises a fake source lane, release now rejects non-workspace roots at the public ingestion boundary, and the remaining code/hooks/release doc drift now matches the real package surface.

Decisions made

- Kept the clippy typed-invalid behavior unchanged after proving it was not a real bug.
  - Added a regression showing raw-parseable but typed-invalid `clippy.toml` still reaches `g3rs-clippy/config-parseable`.
  - Rejected: changing clippy config behavior based on a false audit claim.
- Removed the clippy source-lane stub instead of preserving it for symmetry.
  - The package only implements config and filetree lanes, so the public contract now says that directly.
- Fixed release root validation at the public ingestion boundary.
  - Rejected: letting a non-workspace root flow into lane inputs and trying to express that misuse as ordinary input failures.
- Treated `g3rs-code/ast-24-path-attr-with-reason` as an intentional divergence.
  - The session decision was to keep it in code, so the package README now documents that explicitly instead of pretending the old app comment still governs package ownership.

Key files for context

- `.plans/2026-04-13-201635-post-audit-family-fixes.md`
- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/types/src/error.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/release/g3rs-release-config-checks/README.md`
- `packages/rs/code/g3rs-code-source-checks/README.md`
- `packages/rs/hooks/g3rs-hooks-config-checks/README.md`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/README.md`

Next steps

- Hexarch is still intentionally untouched and remains the only clearly unfinished family.
- If another repo-wide audit is run, start from clippy/release to confirm these contract changes hold under the external family orchestrators too.
