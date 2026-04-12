# Goal

Prove whether the recent `fmt` package migration changed rule behavior, instead of assuming the remaining audit notes are only coverage gaps.

# Approach

- Add the highest-signal missing tests first in the `fmt` packages.
  - Config rule branches:
    - `RS-FMT-CONFIG-03` quiet when no nightly-only keys exist
    - `RS-FMT-CONFIG-04` `[package].edition` fallback
    - `RS-FMT-CONFIG-04` quiet when rustfmt `edition` is absent
    - `RS-FMT-CONFIG-02` `skip_macro_invocations` empty vs non-empty branch
  - Config pipeline:
    - only root `.rustfmt.toml` exists and config rules still run on it
    - malformed Cargo/toolchain does not suppress unrelated config findings
    - exact result-set assertions instead of `any(...)`
    - deleted-after-crawl root file behavior for rustfmt/Cargo/toolchain
  - Filetree:
    - `.claude/worktrees/**` exclusion
    - combined nested override + dual-conflict interaction, if not already pinned strongly enough
- Run only the `fmt` package test workspaces after the new tests land.
- Fix only the cases that now fail.
- Re-run the same suites plus one final adversarial review.

# Key decisions

- Treat the old app as rule inventory only.
  - We are comparing package behavior against old rule intent, not preserving app runtime wiring.
- Prefer exact result assertions for pipeline tests.
  - Presence-only checks can hide extra findings and suppressions.
- Fix boundary logic in ingestion, not in rules, if the break is about file selection or blocker-state propagation.

# Alternatives considered

- Fix likely weak spots immediately.
  - Rejected because the user explicitly asked for proof first.
- Add every suggested audit test before running anything.
  - Rejected because some gaps are low-signal. Start with the branches most likely to reveal real drift.

# Files to modify

- `packages/rs/fmt/g3rs-fmt-config-checks/.../rule_tests/*`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/.../run_tests/mod.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/ingest_tests/filetree.rs`
