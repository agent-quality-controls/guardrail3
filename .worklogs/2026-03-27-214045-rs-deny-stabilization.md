# Stabilize RS-DENY Family

**Date:** 2026-03-27 21:40
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/deny/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/deny/crates/runtime`, `apps/guardrail3/crates/app/rs/families/deny/crates/assertions`, `apps/guardrail3/crates/app/rs/families/deny/test_support`, `apps/guardrail3/crates/app/rs/families/deny/README.md`, `apps/guardrail3/crates/app/rs/families/deny/rustfmt.toml`, `apps/guardrail3/crates/app/rs/families/deny/rust-toolchain.toml`, `apps/guardrail3/crates/app/rs/families/deny/deny.toml`

## Summary
Migrated `RS-DENY` from the old single-crate layout into the self-hosted family workspace shape used by stabilized families, then rewired its tests so the family passes `RS-TEST` and quiet self-validation at the source level. The family root now carries its own runtime, assertions, generic test support, and local config assets, while `RS-DENY-01` was tightened to suppress inventory noise for compliant self-hosted family roots.

## Context & Problem
`RS-DENY` still had the pre-stabilization layout under `src/`, including flat family-wide test sidecars and in-crate support modules that violated the project’s required `RS-TEST` sidecar ownership pattern. A direct `rs validate ... --family test` against the family root reported many `RS-TEST-02` and `RS-TEST-03` failures, so the family was not at parity with `RS-FMT`.

The target state was the same practical end state already established for `RS-FMT`: a nested family workspace with `crates/runtime`, sibling `crates/assertions`, external `test_support`, local self-hosting config files, and clean `arch` / `test` / family self-validation. That also required preventing the family from reporting its own root as noisy inventory once it was structurally compliant.

## Decisions Made

### Split RS-DENY into the stabilized family workspace shape
- **Chose:** Move the family from `apps/guardrail3/crates/app/rs/families/deny/src` into a nested workspace rooted at `apps/guardrail3/crates/app/rs/families/deny`, with production logic under `crates/runtime`, assertions helpers under `crates/assertions`, and shared test fixture helpers under `test_support`.
- **Why:** This is the established self-hosted shape for migrated Rust families and is required to satisfy the current architecture and `RS-TEST` ownership rules.
- **Alternatives considered:**
  - Keep the single-crate family and only rename sidecar files — rejected because it would still violate the intended family packaging pattern and leave `test_support` ownership ambiguous.
  - Move only tests into a helper crate while leaving runtime in place — rejected because that would preserve the old structure instead of actually stabilizing the family.

### Rework deny tests to satisfy RS-TEST without widening production visibility
- **Chose:** Convert the runtime to rule-local sidecar test module directories in the new workspace, route reusable expectations through the sibling assertions crate, and add direct local proof sites where `RS-TEST-07` requires them.
- **Why:** The family needed to satisfy the test family using the same sidecar discipline as other stabilized families, without backsliding into flat `*_tests.rs` files or exposing internals only for test access.
- **Alternatives considered:**
  - Consolidate tests into external integration suites — rejected because the project handoff explicitly wants rule-local sidecar tests.
  - Leave helper assertions in runtime modules — rejected because it weakens ownership boundaries and recreates the pre-migration coupling.

### Quiet RS-DENY-01 for compliant self-hosted family roots
- **Chose:** Add `quiet_if_self_hosted` on `CoveredRustUnitFacts` and compute it from a narrow self-hosted-family-root detector in `facts.rs`, then early-return from `check_covered` in `rs_deny_01_coverage.rs` when that flag is set.
- **Why:** Once `deny` became self-hosted, the family root itself should validate cleanly instead of emitting informational coverage inventory for its own compliant layout.
- **Alternatives considered:**
  - Hardcode an exception for the `deny` family path — rejected because that would be brittle and family-specific.
  - Suppress all `RS-DENY-01` inventory globally — rejected because the inventory is still useful for ordinary covered roots and only self-hosted family roots should go quiet.

### Materialize a canonical self-hosted deny baseline
- **Chose:** Add family-local `README.md`, `rustfmt.toml`, `rust-toolchain.toml`, and `deny.toml`, with `deny.toml` generated from the current service baseline rather than handwritten drift-prone content.
- **Why:** The family needs to self-host the same configuration discipline it enforces, and generating the deny baseline from the domain helper keeps the root config aligned with the current canonical policy.
- **Alternatives considered:**
  - Copy an existing project `deny.toml` by hand — rejected because it risks silent drift from the canonical generator.
  - Skip a root `deny.toml` and rely on upstream workspace config — rejected because the family root itself is one of the validation targets.

## Architectural Notes
The resulting family now follows the project’s stabilized Rust-family pattern:
- discovery and orchestration stay in `crates/runtime`
- reusable assertion helpers live in the sibling `crates/assertions`
- shared fixture and filesystem helpers move to generic `test_support`
- the family root contains the self-hosting config that its own checks consume

`RS-DENY-01` remains the coverage and parse-error ownership rule, but it now distinguishes ordinary covered roots from compliant self-hosted family roots. This matches the behavior already required for `fmt`: the family should still enforce coverage semantics, but it should not produce inventory noise against itself once the root is in the intended shape.

One practical limitation remains outside the source changes: the repo’s existing `apps/guardrail3/target/debug/guardrail3` binary predates this migration, and a fresh top-level rebuild is currently blocked by nested workspace-root interactions after introducing the family-local workspace. That means source-level validation is authoritative for the final `deny` clean state, while the stale binary can still report the old `RS-DENY-01` infos.

## Information Sources
- `AGENTS.md` — worklog rules, current handoff, and Rust-family migration constraints
- `apps/guardrail3/crates/app/rs/families/fmt` — specimen for the stabilized family shape and self-hosted quiet validation behavior
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts.rs` — family fact collection and self-hosted root detection
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_01_coverage.rs` — coverage rule quieting and family test runner hook
- `apps/guardrail3/crates/domain/modules/deny` — canonical deny baseline generator used to derive the family root `deny.toml`
- `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/deny/Cargo.toml --workspace`
- `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/deny --family test --inventory --format json`
- `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/deny --family arch --inventory --format json`
- temporary source-level runner against current library code to confirm `--family deny` returns `errors=0 warnings=0 info=0`

## Open Questions / Future Considerations
- The top-level build currently rejects the nested family workspace as a second workspace root. If the repo intends to keep family-local workspaces, the top-level workspace wiring needs a deliberate compatibility decision rather than ad hoc local exceptions.
- The checked-in binary should be rebuilt once the workspace-root conflict is resolved so CLI validation matches the current source behavior for `RS-DENY-01`.

## Key Files for Context
- `AGENTS.md` — current project handoff, migration rules, and worklog requirements
- `apps/guardrail3/crates/app/rs/families/deny/Cargo.toml` — family workspace root and member wiring
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/lib.rs` — deny family orchestrator and rule fan-out
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts.rs` — normalized deny facts plus self-hosted-root quieting signal
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_01_coverage.rs` — coverage rule behavior and self-validation suppression
- `apps/guardrail3/crates/app/rs/families/deny/crates/assertions/src/rs_deny_01_coverage.rs` — assertion-side proof helpers for the migrated sidecar pattern
- `apps/guardrail3/crates/app/rs/families/deny/test_support/src/lib.rs` — generic fixture helpers used by deny runtime tests
- `apps/guardrail3/crates/app/rs/families/deny/deny.toml` — self-hosted deny baseline derived from the canonical generator

## Next Steps / Continuation Plan
1. Resolve the top-level nested-workspace conflict so `cargo build --manifest-path apps/guardrail3/Cargo.toml -p guardrail3` can rebuild the CLI after family-local workspace migrations.
2. Rebuild `apps/guardrail3/target/debug/guardrail3`, then rerun `rs validate ... --family deny --inventory --format json` on the `deny` family root to confirm the binary matches the source-level zero state.
3. Apply the same stabilization pass to the next Rust config family that still uses the legacy single-crate shape and flat sidecar tests.
