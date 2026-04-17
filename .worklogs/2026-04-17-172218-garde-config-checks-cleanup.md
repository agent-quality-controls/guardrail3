Summary

Cleaned `packages/rs/garde/g3rs-garde-config-checks` to the current package shape and brought its runtime tests onto the owned-sidecar and shared-proof patterns. Validation and package tests now pass with no findings.

Decisions made

- Replaced the old flat assertions files with per-rule assertion modules plus a shared `run` assertions module so runtime tests and any external tests use the same proof surface.
- Kept the package-local `crates/types` crate as a thin re-export boundary over `g3rs-garde-types` instead of inventing new local types.
- Removed the old runtime `test_support.rs` and moved the remaining fixture ownership into rule-local helpers and `run_tests/cases.rs` to keep `mod.rs` facade-only.
- Updated rule-side test imports to target each owned assertions `rule` module directly because the current test rules reject imports through the broader sibling module.
- Fixed the shared run assertions helper to reflect the actual missing-clippy messages, including the extractor rule's `disallowed-types` wording.

Key files for context

- `packages/rs/garde/g3rs-garde-config-checks/Cargo.toml`
- `packages/rs/garde/g3rs-garde-config-checks/guardrail3-rs.toml`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/assertions/src/run.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/rs_garde_config_05_additional_method_bans/rule_tests/helpers.rs`

Next steps

- Continue with `packages/rs/garde/g3rs-garde-ingestion`.
- Do not change rules unless that package exposes a real contradiction.
