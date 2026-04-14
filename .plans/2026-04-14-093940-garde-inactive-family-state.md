Goal

Make the `garde` family package self-determine inactivity so the thin app can invoke it unconditionally. When the pointed workspace has neither a `garde` dependency nor a `guardrail3.toml`, the family should be `Inactive` and return no results instead of surfacing `GuardrailTomlNotFound`.

Approach

- Add explicit `Inactive` applicability to the public `garde` input types.
  - File: `packages/rs/garde/g3rs-garde-types/src/lib.rs`
  - Why: the family package needs to express "selected but inactive" without pushing that decision into the app.
- Add package-level regression tests first.
  - Files:
    - `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
    - `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run_tests/mod.rs`
    - `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run.rs` or its local test support
  - Why: prove the current bug and pin the expected inactive behavior.
- Update `garde` ingestion to return inactive inputs when:
  - `Cargo.toml` is parseable
  - no `garde` dependency is present
  - no `guardrail3.toml` exists
  - Files:
    - `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
  - Why: applicability belongs in the family package boundary, not in the app.
- Update config and source checks to return `Vec::new()` for inactive inputs.
  - Files:
    - `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run.rs`
    - `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run.rs`

Key decisions

- Keep `Cargo.toml` required for applicability.
  - Rejected: silently treating missing or unreadable root `Cargo.toml` as inactive.
  - Reason: the family cannot honestly decide applicability without readable Cargo metadata.
- Use `Inactive`, not `Cold`.
  - Reason: matches the current session terminology decision.
- Do not change the app.
  - Reason: the app must remain thin and only invoke selected families.

Files to modify

- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run.rs`
