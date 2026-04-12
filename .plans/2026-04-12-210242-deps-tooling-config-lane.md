# Goal

Move old deps tool-presence rules into package-owned deps config checks so the package model enforces required dependency tools on PATH without relying on hooks.

## Approach

- Add failing deps config tests first.
  - Add runtime tests for missing/present tool results and severity parity.
  - Add ingestion pipeline tests that prove one workspace-scoped config input emits tool results.
- Extend deps config input shape with an explicit scope discriminator and installed-tool facts.
  - Keep crate policy inputs for allowlist/cap rules.
  - Add one synthetic workspace-tooling input from deps ingestion.
- Implement package config rules for:
  - cargo-deny on PATH
  - cargo-machete on PATH
  - cargo-dupes on PATH
  - gitleaks on PATH
- Reuse the existing PATH discovery pattern already used by hooks ingestion.
- Update deps package docs and TODOs to reflect the migrated tool slice.

## Key decisions

- Use deps config lane, not hooks, for standalone tool presence.
  - Why: hooks only cover selected hook execution, not deps policy availability itself.
- Add a workspace-scoped config input instead of duplicating tool checks per crate.
  - Why: tool presence is one workspace-level fact and should emit one finding per tool.
- Keep crate policy rules unchanged in meaning.
  - Why: the tool migration should not disturb existing allowlist and cap behavior.

## Alternatives considered

- Duplicating tool facts into every crate input.
  - Rejected: would emit repeated findings and blur workspace-vs-crate ownership.
- Creating a new deps env/tooling lane.
  - Rejected: the user explicitly accepted config as the home.

## Files to modify

- `packages/rs/deps/g3rs-deps-types/src/input.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/support.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run.rs`
- deps config rule test files
- deps ingestion pipeline tests
- deps package README/TODO files
