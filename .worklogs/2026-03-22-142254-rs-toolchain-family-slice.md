# RS-TOOLCHAIN Family Slice In New Checks Architecture

**Date:** 2026-03-22 14:22
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/toolchain/**`

## Summary
Implemented the `rs/toolchain` family under the new checks architecture. The family now validates `rust-toolchain.toml` existence, channel/components policy, MSRV consistency against Cargo `rust-version`, and legacy `rust-toolchain` migration warnings using a single root input and collocated sidecar tests.

## Context & Problem
After `rs/fmt` and `rs/cargo`, the next useful migration target was a simple single-file config family. `rs/toolchain` is a good fit because it confirms that the new architecture works not only for parent/child families but also for small root-only config families with a couple of cross-file facts, specifically Cargo `rust-version`.

The old implementation split toolchain logic across `config_files.rs` and `toolchain_check.rs`. That older layout still mixed discovery and rule execution and did not capture the new “family-local facts and inputs” pattern directly.

## Decisions Made

### Keep `rs/toolchain` as a root-only family
- **Chose:** Model the family around a single `ToolchainRootInput`.
- **Why:** All current toolchain checks judge one workspace-root relationship. There is no benefit in widening the input or inventing per-member logic.
- **Alternatives considered:**
  - Put Cargo `rust-version` comparison in a different family — rejected because the MSRV consistency rule is part of the toolchain contract and belongs here.
  - Skip a family module and leave toolchain logic in a shared config file checker — rejected because it breaks the family-by-family migration model.

### Require `rust-toolchain.toml` specifically, not the legacy file
- **Chose:** `RS-TOOLCHAIN-01` still errors when `rust-toolchain.toml` is missing, while `RS-TOOLCHAIN-04` handles the legacy `rust-toolchain` file as a migration warning.
- **Why:** This preserves the existing product direction: the TOML form is the intended canonical file because it can declare components explicitly.
- **Alternatives considered:**
  - Treat legacy `rust-toolchain` as satisfying existence — rejected because it would weaken the migration goal and collapse two distinct invariants.

### Pull Cargo `rust-version` in as a fact, not as a separate traversal
- **Chose:** `discover.rs` reads root Cargo metadata once and exposes `cargo_rust_version` in family facts.
- **Why:** The `RS-TOOLCHAIN-03` rule needs that context, but the rule should not parse Cargo.toml on its own.
- **Alternatives considered:**
  - Let the rule parse Cargo.toml directly — rejected because it violates the orchestrator/facts separation.

## Architectural Notes
The family layout follows the same pattern established by `fmt` and `cargo`:
- `discover.rs` collects root toolchain facts
- `facts.rs` stores normalized toolchain state
- `inputs.rs` binds one atomic root input
- each rule lives in its own file
- tests live in a sidecar `toolchain_tests.rs`

This family currently implements:
- `RS-TOOLCHAIN-01` rust-toolchain.toml existence
- `RS-TOOLCHAIN-02` channel and required components
- `RS-TOOLCHAIN-03` pinned toolchain vs Cargo MSRV consistency
- `RS-TOOLCHAIN-04` legacy/duplicate toolchain file warnings

The rules remain side-by-side with the old validator for now and are not yet wired into the top-level validate pipeline.

## Information Sources
- `.plans/todo/checks/rs/toolchain.md` — canonical toolchain rule inventory
- `apps/guardrail3/crates/app/rs/validate/config_files.rs` — old existence wiring
- `apps/guardrail3/crates/app/rs/validate/toolchain_check.rs` — old toolchain semantics
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/*` — simple single-file family specimen
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/*` — more advanced family specimen
- `cargo test --lib checks::rs::toolchain`
- `cargo test --lib checks::rs::cargo`
- `cargo test --lib checks::rs::fmt`

## Open Questions / Future Considerations
- `RS-TOOLCHAIN-03` currently only compares numeric pinned channels like `1.84.0`; nonstandard strings are ignored.
- The family does not yet model a more nuanced library/service distinction for MSRV messaging.
- As with the other migrated families, this code is not yet wired into the old top-level validation path.

## Key Files for Context
- `AGENTS.md` — current project scope and architecture
- `.plans/todo/checks/rs/toolchain.md` — toolchain rule contract
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/mod.rs` — family orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/discover.rs` — root discovery and Cargo/toolchain fact extraction
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/facts.rs` — normalized toolchain facts
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/inputs.rs` — single root input shape
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/toolchain_tests.rs` — sidecar tests
- `.worklogs/2026-03-22-141917-rs-cargo-family-slice.md` — prior family checkpoint for the new checks tree

## Next Steps / Continuation Plan
1. Move to `rs/clippy` or `rs/deny` next to continue the family-by-family migration through the simpler config families.
2. Keep the same structure: facts first, typed inputs second, rule files third, sidecar tests last.
3. Do not wire `rs/toolchain` into the top-level validator until a few adjacent families have validated the architecture consistently.
