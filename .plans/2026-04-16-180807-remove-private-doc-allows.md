Goal
- Remove the blanket `clippy::missing_docs_in_private_items` escape hatches from the cleaned Rust packages and replace them with short private-item docs where needed.

Approach
- Remove the crate-level `missing_docs_in_private_items` allows from the current cleaned package files.
- Run clippy on the affected workspaces to surface the real private items that need docs.
- Add short docs to the specific private modules and helpers clippy reports.
- Re-run clippy and grep to prove the blanket allows are gone.

Key decisions
- Keep the `disallowed_methods` fs-boundary allows. They are a real boundary exception, not blanket scaffold slack.
- Keep the `dead_code` allows out of scope for this slice. This task is only about the private-doc blanket allows.
- Fix the docs where the lint fires instead of replacing one blanket allow with another narrower blanket allow.

Files to modify
- The current `packages/rs/**/lib.rs` and `ingest_tests/mod.rs` files that still use `clippy::missing_docs_in_private_items`.
- Any private module declaration or helper file clippy reports once those allows are removed.
- One worklog for the cleanup commit.
