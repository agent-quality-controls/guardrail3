# Summary

Moved the old deps PATH tool checks into package-owned deps config checks after proving the missing behavior with failing package pipeline tests. The deps config lane now owns standalone `cargo-deny`, `cargo-machete`, `cargo-dupes`, and `gitleaks` presence, while deps ingestion provides one workspace-scoped tooling input plus the existing per-crate policy inputs.

## Decisions made

- Kept tool presence in deps config.
  - Why: the user explicitly chose config, and standalone deps tool availability is not equivalent to hook availability.
- Added one synthetic workspace-scoped config input instead of duplicating tool facts into every crate input.
  - Why: tool presence is a workspace-level fact and should emit one result per tool, not one result per crate.
- Added an explicit config input scope discriminator.
  - Why: crate policy rules and workspace tooling rules are different subjects and need a typed boundary.
- Reused the hooks ingestion PATH-discovery pattern locally in deps ingestion.
  - Why: it is already the repo's package-model precedent for PATH-backed tool facts.

## Key files for context

- `.plans/2026-04-12-210242-deps-tooling-config-lane.md`
- `packages/rs/deps/g3rs-deps-types/src/input.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/support.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/rs_deps_config_06_cargo_deny_installed/rule.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/rs_deps_config_07_cargo_machete_installed/rule.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/rs_deps_config_08_cargo_dupes_installed/rule.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/rs_deps_config_09_gitleaks_installed/rule.rs`
- `packages/rs/deps/g3rs-deps-config-checks/README.md`

## Next steps

- `deps` is now package-complete for meaningful config and filetree checks.
- The next family audit should move to another remaining partial family rather than revisiting deps, unless a later test-attack finds a concrete bug.
