# Deps Follow-Up Hardening

**Date:** 2026-04-07 19:16
**Scope:** `packages/rs/deps/g3rs-deps-config-checks`, `packages/rs/deps/g3rs-deps-config-ingestion`, `packages/rs/deps/g3rs-deps-types`

## Summary
Hardened the new deps package rewrite after an adversarial review. Fixed the main fail-open cases in deps ingestion, cleaned up two package-shape inconsistencies, refreshed the stale `g3rs-deps-types` lockfile, and added tests for the cases that were previously unproven.

## Context & Problem
The first deps rewrite got the package boundary mostly right, but it still had a few bad gaps:

- hybrid root workspaces were skipped
- missing declared workspace members were silently ignored
- in-workspace non-member path deps were silently dropped while file-tree ingestion was still stubbed
- absolute path deps were misclassified as internal
- `workspace.exclude` was ignored
- tests were weaker than the new normalization demanded

The user explicitly asked for an attack pass and then asked to fix the issues that came out of it, so this follow-up focused on correctness and test honesty rather than adding new surface area.

## Decisions Made

### Fail closed on in-workspace non-member path deps
- **Chose:** Treat any in-workspace path dep that is not a declared workspace member as an ingestion failure.
- **Why:** The earlier rewrite silently returned `None` and depended on a future file-tree lane that does not exist yet. That was a real bypass.
- **Alternatives considered:**
  - Keep skipping these deps until file-tree ingestion exists — rejected because that is fail-open.
  - Push them back into config checks — rejected because that would reintroduce raw file-tree ownership into config input.

### Include hybrid workspace roots in config ingestion
- **Chose:** Include the root `Cargo.toml` as a member input when the workspace root is also a package.
- **Why:** Cargo treats that root package as part of the workspace, and skipping it meant the root crate got no deps config checks at all.
- **Alternatives considered:**
  - Keep root crates out of the deps lane — rejected because it creates a blind spot with no equivalent replacement.

### Validate workspace member selection instead of trusting the crawl
- **Chose:** Make member selection check that every `[workspace].members` pattern resolves to at least one manifest and respect `workspace.exclude`.
- **Why:** The old selection only returned whatever existed in the crawl and silently tolerated typos or deleted members.
- **Alternatives considered:**
  - Validate only literal member paths and ignore empty globs — rejected because it still leaves the selection fail-open for a broad class of mistakes.

### Narrow config-checks exports back to the standard shape
- **Chose:** Make `g3rs-deps-config-checks` export only `check` and `G3RsDepsConfigChecksInput`.
- **Why:** The prior commit leaked shared/future-lane family types through the config-checks package, which does not match the rest of the package architecture.
- **Alternatives considered:**
  - Keep re-exporting everything for convenience — rejected because it blurs the ownership boundary between family types and family checks.

### Make tests prove the risky branches directly
- **Chose:** Add ingestion tests for allowlist presence, hybrid root workspaces, `exclude`, missing members, build/dev normalization, absolute/workspace path handling, and fail-closed undefined workspace deps. Also renamed two pure-check tests so their names match what they now cover.
- **Why:** The first rewrite moved a lot of complexity into ingestion, but the tests were still mostly checking pre-normalized fixtures.
- **Alternatives considered:**
  - Leave the pure-check tests alone and rely on manual reasoning — rejected because the review already showed that the risky branches were effectively untested.

## Architectural Notes
The resulting shape is:

- `g3rs-deps-types` still owns the shared normalized deps types
- `g3rs-deps-config-checks` stays a narrow config-check package
- `g3rs-deps-config-ingestion` owns the normalization rules that are safe to do before file-tree ingestion exists

Important current boundary:

- real workspace members are skipped as internal deps
- in-workspace non-members now fail ingestion
- absolute path deps are treated as external and stay in the normalized config facts
- `workspace.exclude` is honored during member selection

This is still not the final file-tree story. It is the minimum hardening needed so the current config lane does not silently stand down on owned cases.

## Information Sources
- `.worklogs/2026-04-07-185100-deps-config-ingestion.md` — prior deps rewrite
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/select.rs`
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/ingest_tests`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/*`
- `packages/parsers/cargo-toml-parser/crates/parser/types/src/cargo_toml.rs`
- adversarial findings produced in this session against commit `570671370`

## Open Questions / Future Considerations
- The current “every workspace member pattern must match at least one manifest” rule is intentionally strict. If the repo later wants looser glob semantics, that should be an explicit policy choice, not an accidental fail-open behavior.
- Outside-workspace path deps still normalize by declared package identity (`package` or alias). If the real package name matters when `package` is omitted, that will need the future file-tree lane.
- The parser package still does not preserve presence for `allowed_deps` directly; ingestion is still recovering that from raw TOML.

## Key Files for Context
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/select.rs` — workspace member selection, root-package inclusion, and `exclude` handling
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/ingest.rs` — path normalization and fail-closed handling
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/ingest_tests/basic.rs` — fail-closed and allowlist-presence tests
- `packages/rs/deps/g3rs-deps-config-ingestion/crates/runtime/src/ingest_tests/deps.rs` — normalization coverage for root/member/build/dev/target/workspace path cases
- `packages/rs/deps/g3rs-deps-config-checks/src/lib.rs` — narrowed package export surface
- `packages/rs/deps/g3rs-deps-config-checks/crates/types/src/lib.rs` — narrowed config-checks types export
- `packages/rs/deps/g3rs-deps-types/Cargo.lock` — refreshed package-local dependency graph
- `.worklogs/2026-04-07-185100-deps-config-ingestion.md` — prior rewrite context

## Next Steps / Continuation Plan
1. Build the deps file-tree lane so local target manifest existence, malformed local target `Cargo.toml`, and missing `[package].name` become first-class owned findings instead of ingestion errors.
2. Decide whether external path deps without explicit `package = "..."` need file-tree-backed package-name recovery, or whether alias-based normalization is sufficient for policy.
3. Once the file-tree lane exists, revisit whether any current ingestion failures should move out of config ingestion and become explicit file-tree findings instead.
