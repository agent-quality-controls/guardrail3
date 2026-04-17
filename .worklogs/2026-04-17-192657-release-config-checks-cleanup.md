Summary
- Normalized `packages/rs/release/g3rs-release-config-checks` to the current internal package shape and finished the runtime/assertions sidecar migration so the package validates cleanly.
- Kept the work package-local. No rule changes were needed.

Decisions made
- Switched runtime from the local `crates/types` facade to `g3rs-release-types` directly to match the cleaned family boundary used elsewhere.
- Kept rule modules `01` through `11` in owned directory form with directory-scoped assertions modules, but kept flat production files `00`, `19`, and `20` paired with flat assertions files because those production files are still flat and the test rules key off that shape.
- Moved sidecar harness helpers into their owned production modules where needed so sidecars no longer reach through `lib_tests`.
- Put the smoke test back under `run.rs` with a matching `crates/assertions/src/run.rs` proof module because the behavior belongs to `run`, not `lib`.
- Added `semver` to the package allowlist rather than trying to hide the dependency behind a local helper or rule change.

Key files for context
- `packages/rs/release/g3rs-release-config-checks/Cargo.toml`
- `packages/rs/release/g3rs-release-config-checks/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_00_publish_must_be_explicit.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_10_release_plz_baseline/rule.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/assertions/src/common.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/assertions/src/run.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/assertions/src/rs_release_config_00_publish_must_be_explicit.rs`

Next steps
- Continue with `packages/rs/release/g3rs-release-filetree-checks` as the next dirty release package.
- If that package exposes a real contradiction, stop and handle it as a separate bug fix instead of widening this cleanup commit.
