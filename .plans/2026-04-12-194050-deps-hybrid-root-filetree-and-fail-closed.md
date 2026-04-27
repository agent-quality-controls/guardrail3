# Goal

Finish the next concrete `deps` package gaps proved by the review:

- fix hybrid-workspace-root membership normalization in config ingestion
- migrate `RS-DEPS-09` and `RS-DEPS-10` into a package filetree lane
- harden package fail-closed coverage around the old `RS-DEPS-11` matrix

# Approach

1. Add failing tests first in `g3rs-deps-ingestion` for:
   - hybrid root workspace members with local path edges back to the hybrid root
   - missing `Cargo.lock`
   - ignored `Cargo.lock`
   - root and member unreadable / malformed inputs that should stay fail-closed
   - unknown-key tolerance branches that should stay quiet
2. Fix the hybrid-root normalization bug in ingestion at the root-membership set builder.
3. Add `g3rs-deps-filetree-checks` with lane-scoped rule IDs:
   - `g3rs-deps/cargo-lock-present`
   - `g3rs-deps/gitignore-not-ignoring-cargo-lock`
4. Implement `ingest_for_file_tree_checks(...)` with workspace-local semantics:
   - one pointed workspace root
   - root `Cargo.lock`
   - root `.gitignore` masking for that root lockfile
   - optional root profile from `guardrail3-rs.toml`
5. Update deps package docs to reflect the new boundary instead of the stale app/package split.
6. Run deps package tests and a final adversarial review.

# Key decisions

- Keep the fix at the membership-set builder in `g3rs-deps-ingestion`.
  - Why: the bug is not in dependency resolution itself; the wrong root set is constructed before normalization.
- Treat `RS-DEPS-09/10` as package filetree rules.
  - Why: they inspect root lockfile and ignore-file placement, not config contents.
- Keep `RS-DEPS-11` as fail-closed boundary behavior expressed through ingestion errors and package coverage.
  - Why: there is not yet a separate public deps fail-closed checks lane, and the concrete gap found was missing package coverage.

# Files to modify

- `packages/rs/deps/g3rs-deps-types/src/input.rs`
- `packages/rs/deps/g3rs-deps-ingestion/README.md`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/select.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/ingest_tests/deps.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/deps/g3rs-deps-config-checks/README.md`
- new `packages/rs/deps/g3rs-deps-filetree-checks/**`
