Summary

- Fixed the `rs/arch` ingestion compile break caused by one file importing the wrong crawl crate and type.
- This unblocked `g3rs validate` for `packages/rs/arch/g3rs-arch-ingestion`.

Decisions made

- Fixed the import at the broken source file instead of adding aliases or compatibility glue.
  - Why: every other file in this package already uses `g3rs_workspace_crawl::G3RsWorkspaceCrawl`. The broken file was a plain typo, not a design gap.
  - Rejected: introducing compatibility exports. That would preserve the wrong name and add noise to the package boundary.

Key files for context

- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/view.rs`

Next steps

- Continue the package-boundary cleanup with the next concrete validation failures in `packages/parsers/hook-shell-parser`.
- Split the oversized parser command-query engine and fix the sidecar boundary on the new API tests.
