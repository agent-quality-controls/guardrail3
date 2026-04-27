# Goal

Finish the `cargo` family under the package model by fixing the existing hybrid-root bug, migrating the remaining live old-app rules into package-owned config and filetree lanes, and keeping the resulting lane boundaries pure.

## Approach

- Prove the current hybrid-root bug and the missing cargo filetree behavior with failing package tests first.
  - Add config tests that show hybrid workspace roots with `[package]` are treated like pure workspace roots today.
  - Add filetree pipeline tests for:
    - declared workspace member missing `Cargo.toml`
    - cargo-family input failures fail closed
- Fix hybrid manifest classification in existing cargo package config checks.
  - Make hybrid manifests expose package-local `edition` and package-local `lints`.
  - Keep pure workspace roots and pure package manifests unchanged.
- Expand cargo package inputs so remaining live old-app config rules can be expressed as lane-pure package checks.
  - Add workspace-policy config inputs for:
    - allow inventory and reason quality
    - unapproved allow entries and reason quality
    - rust-version policy
  - Add workspace/member pair config inputs for:
    - lint inheritance
    - no weakened overrides
    - member edition drift
    - member-local allows forbidden
- Build `g3rs-cargo-filetree-checks`.
  - Migrate:
    - `g3rs-cargo/missing-member-cargo` - declared workspace member missing `Cargo.toml`
    - `g3rs-cargo/input-failures` - cargo-family input failures fail closed
- Re-run cargo family tests and adversarial review until no concrete gap remains.
- Update cargo package README/TODO files to match the migrated family surface.

## Key decisions

- Keep tool/environment checks out of cargo.
  - Why: the live cargo family rules are about `Cargo.toml`, workspace membership, and guardrail config, not PATH tooling.
- Put `RS-CARGO-10` and `RS-CARGO-14` into filetree.
  - Why: they check declared member file presence and boundary input failures, not config semantics.
- Keep `RS-CARGO-03`, `04`, `06`, `09`, `12`, `13`, and `15` in config.
  - Why: they are derived from parsed `Cargo.toml` and parsed guardrail policy contents.
- Fix the hybrid-root bug at role classification, not inside individual rules.
  - Why: the misclassification is shared infrastructure and should not be patched rule-by-rule.

## Alternatives considered

- Leaving hybrid roots as `WorkspaceRoot` and adding special cases to individual rules.
  - Rejected: duplicates role logic and guarantees future drift.
- Treating `RS-CARGO-14` as config because it often involves malformed config files.
  - Rejected: it is a boundary failure rule about whether the filetree/input surface is analyzable at all.
- Reusing oversized existing config inputs for all new rules.
  - Rejected: the new rules need typed workspace-policy and workspace/member pair inputs, not one giant bag.

## Files to modify

- `packages/rs/cargo/g3rs-cargo-types/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/support.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/lib.rs`
- cargo config rule files and tests
- `packages/rs/cargo/g3rs-cargo-filetree-checks/*`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/select.rs`
- cargo ingestion tests
- cargo package README/TODO files
