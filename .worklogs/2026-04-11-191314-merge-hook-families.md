Summary
- Merged the public `hooks-rs` and `hooks-shared` package split into one public `g3rs-hooks` family under `packages/rs/hooks`.
- Kept the combined work source-only for now and removed the old public package roots.

Decisions made
- Merged the public boundary into one `g3rs-hooks` family.
  - Rejected keeping `hooks-rs` and `hooks-shared` as separate public checker families because the Rust library must own the full hook contract itself.
- Kept rule IDs unchanged.
  - Rejected renumbering or renaming `HOOK-RS-*` / `HOOK-SHARED-*` during the package merge to avoid coupling a boundary migration to a ledger migration.
- Kept hook parsing inside the existing `hook-shell-parser` crate.
  - Rejected folding parser code into the new family because it is still a real reusable parser boundary.
- Merged only the source lane in this pass.
  - Rejected inventing config or file-tree hook lanes here because the live extracted hook work is still source/content plus ingestion.
- Switched hook command matching from `guardrail3` to `g3rs`.
  - Kept `guardrail3-rs.toml` file triggers unchanged because only the binary-name correction was established in-session.

Key files for context
- `.plans/2026-04-11-185934-merge-hook-families.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/selection.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `.plans/todo/checks/hooks/rs.md`
- `.plans/todo/checks/hooks/shared.md`

Next steps
- Decide whether to rename the hook rule ledgers from `HOOK-RS` / `HOOK-SHARED` into one `HOOK-*` family or keep the split only at rule-ID level.
- Extract the remaining hook structural and tool-availability work into the merged `g3rs-hooks` family instead of reviving the old public split.
