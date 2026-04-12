# Summary

Finished the current deps package gaps after proving them with failing tests. The deps family now has a package filetree lane for `Cargo.lock` policy, config ingestion handles hybrid-root and absolute local-path normalization correctly, and the fail-closed package coverage was expanded around guardrail inputs and empty allowlist entries.

## Decisions made

- Fixed hybrid-root membership at the workspace-member-set builder.
  - Why: the bug was introduced before dependency normalization, so the correct fix was to include the hybrid root in the member directory set instead of adding a special case in dependency resolution.
- Added `g3rs-deps-filetree-checks` for `RS-DEPS-FILETREE-09` and `RS-DEPS-FILETREE-10`.
  - Why: these checks belong to a separate filetree lane, not to config checks or ingestion glue.
- Kept deps fail-closed ownership in package ingestion.
  - Why: unreadable, malformed, and normalization-blocking inputs are boundary failures that should stop bad deps inputs before they reach the pure checks.
- Treated absolute local paths under the pointed workspace root the same as normalized relative local paths.
  - Why: local path identity should not depend on whether the manifest used an absolute or relative spelling.
- Validated `allowed_deps` entries after parsing `guardrail3-rs.toml`.
  - Why: empty dependency names are semantic normalization failures, not TOML syntax failures.

## Key files for context

- `.plans/2026-04-12-194050-deps-hybrid-root-filetree-and-fail-closed.md`
- `packages/rs/deps/g3rs-deps-types/src/input.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/Cargo.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/runtime/src/rs_deps_filetree_09_cargo_lock_present.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/runtime/src/rs_deps_filetree_10_gitignore_not_ignoring_cargo_lock.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/select.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/ingest_tests/deps.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/ingest_tests/filetree.rs`
- `apps/guardrail3/crates/app/rs/families/deps/README.md`

## Next steps

- The current deps package boundary is clean for config plus filetree.
- Remaining unmigrated deps scope is still the tool-presence slice `RS-DEPS-01..04`.
- If the next pass revisits deps test quality, the remaining optional improvement is converting more config rule tests from `any()` assertions to exact result vectors.
