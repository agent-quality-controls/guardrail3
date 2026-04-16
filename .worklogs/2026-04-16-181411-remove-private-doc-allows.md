Summary

Removed the blanket `clippy::missing_docs_in_private_items` escape hatches from the cleaned Rust packages. The files that actually had private items now carry short docs explaining what those internals do and why they exist.

Decisions made

- Removed the blanket allow instead of keeping a scaffold exception.
  - Why: these files are small and stable enough that one-line private docs are cheaper and clearer than a permanent escape hatch.
  - Rejected: keeping the allow in place just because the files are internal.
- Added docs only where the removed allow had been hiding real private items.
  - Why: most assertion `lib.rs` files had no private surface at all, so deleting the allow was enough.
  - Rejected: adding filler docs everywhere or replacing the blanket allow with another blanket allow.
- Kept verification scoped to this lint cleanup.
  - Why: full workspace clippy surfaced unrelated pre-existing lint and compile failures in other packages.
  - Rejected: dragging those unrelated issues into this commit.

Key files for context

- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/mod.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/ingest_tests/mod.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/src/lib.rs`

Next steps

- Audit the remaining non-doc escape hatches separately:
  - `clippy::disallowed_methods`
  - `dead_code`
  - structural waivers
- If you want private-item docs enforced more broadly, run the same cleanup package-by-package instead of turning on a repo-wide doc pass in one commit.
- The unrelated blockers seen during verification were:
  - `packages/rs/toolchain/g3rs-toolchain-ingestion` real workspace fixture test expecting `stable` while current data is `1.85.0`
  - `packages/rs/clippy/g3rs-clippy-ingestion` unresolved assertion helper imports in `crates/assertions/src/run.rs`
  - `packages/rs/code/g3rs-code-ingestion` unresolved imports from `g3rs_code_ingestion_types`
