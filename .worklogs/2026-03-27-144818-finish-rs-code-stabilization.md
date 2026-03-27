# Finish RS-CODE Stabilization

**Date:** 2026-03-27 14:48
**Scope:** `apps/guardrail3/crates/app/rs/families/code`, `apps/guardrail3/crates/shared/fs/src/lib.rs`, `apps/guardrail3/Cargo.lock`

## Summary
Finished the `rs/code` family migration into the same self-hosted shape used by the stabilized Rust families. The family now passes its own unit tests and validates clean under `RS-CODE`, `RS-TEST`, and `RS-ARCH`.

## Context & Problem
`rs/code` had already been moved into a workspace shape, but it was still mid-migration. The runtime tree still carried family-local test support, many sidecars still owned semantic assertions directly, and a number of self-hosting `RS-TEST` findings were being suppressed only because the migration was incomplete. By the start of this session, the validator passes were already green, but the family test suite still had expectation drift and the workspace changes had not been committed as a coherent checkpoint.

## Decisions Made

### Move test support into a sibling crate for real
- **Chose:** Remove `crates/runtime/src/test_support.rs` and move the reusable filesystem and fixture helpers into `families/code/test_support`.
- **Why:** `RS-TEST` now enforces the sibling `runtime/assertions/test_support` split. Keeping test support in runtime would preserve the old architecture and keep boundary leakage alive.
- **Alternatives considered:**
  - Keep runtime-local `test_support.rs` and special-case `rs/code` — rejected because it would undermine the self-hosting contract.
  - Push all helpers into assertions — rejected because filesystem and fixture helpers are generic support, not semantic proof.

### Keep canonical `#[path = ".../mod.rs"]` sidecar wiring and exempt only that exact pattern
- **Chose:** Restore the standard sidecar wiring in the runtime rule files and tighten `RS-CODE-24` so it only exempts the canonical `#[cfg(test)]` + `#[path = "..._tests/mod.rs"]` + `mod ...;` pattern.
- **Why:** Replacing `#[path]` with plain `mod` did not actually work from these file locations, and relaxing the rule wholesale would weaken the code family against the exact pattern it is supposed to police.
- **Alternatives considered:**
  - Remove all `#[path]` attrs and rely on default module resolution — rejected because the tree shape does not support it.
  - Blanket-ignore `#[path]` in the `code` family — rejected because it would make the family self-pass artificially.

### Prefer direct self-hosting fixes over rule relaxations
- **Chose:** Remove unnecessary `#[allow(dead_code)]` usage, split parse helpers, move assertions into the sibling crate, and tighten test expectations instead of weakening the `RS-CODE` rules for self-hosting.
- **Why:** The goal was to make `rs/code` genuinely compliant under the same rules it enforces, not to create another carveout.
- **Alternatives considered:**
  - Add self-exemptions for `dead_code`, direct fs helpers, or path attrs — rejected because the previous four stabilized families already established the stricter standard.

### Normalize inventory/bypass tests around touched-file scoping
- **Chose:** Update many sidecar tests to filter to the intended touched files before asserting exact findings.
- **Why:** Once the family started running against the real owned file tree in its new layout, several tests were failing because they were asserting against full-family baseline inventory instead of the attack surface they modified.
- **Alternatives considered:**
  - Broaden expectations to include ambient baseline hits — rejected because those tests are supposed to prove specific attack vectors, not all incidental results in the fixture tree.
  - Narrow the family runtime just for tests — rejected because that would hide real ownership behavior.

## Architectural Notes
`rs/code` now matches the same architectural baseline as `rs/test`, `rs/arch`, `rs/cargo`, and `rs/hexarch`:
- family workspace root
- `crates/runtime` for production logic and sidecar test modules
- `crates/assertions` for proof-bearing rule assertions
- sibling `test_support` for generic fixture/fs helpers

The rule-specific sidecars still live under runtime, but they now prove behavior through the owned assertions crate rather than carrying semantic proof inline. The `shared/fs` crate was also extended slightly so the code-family `test_support` crate could stay generic without falling back to direct `std::fs` usage in runtime tests.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/README.md`
- `.plans/todo/checks/rs/code.md`
- `.plans/todo/checks/rs/code-family-stabilization.md`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs`
- Prior worklogs:
  - `.worklogs/2026-03-27-132300-start-rs-code-stabilization.md`
  - `.worklogs/2026-03-27-140627-rs-code-rs-test16-slice.md`
  - `.worklogs/2026-03-27-142821-rs-code-parse-shape-cleanup.md`

## Open Questions / Future Considerations
- `rs/code` is now structurally self-hosting, but it has not gone through the same adversarial attack loop that `rs/test` and `rs/hexarch` did. The next quality step is not more migration work; it is an attack review of the family’s own rule completeness and false-green resistance.
- `shared/fs` now exposes a couple of extra helpers for test support. If more families start to use it, that crate should stay intentionally tiny and generic rather than becoming a dumping ground for fixture logic.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/README.md` — current contract for the family layout and responsibilities
- `apps/guardrail3/crates/app/rs/families/code/Cargo.toml` — workspace root for the family
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs` — family runtime entrypoint and module inventory
- `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/lib.rs` — exported assertions modules for proof-bearing sidecar use
- `apps/guardrail3/crates/app/rs/families/code/test_support/src/lib.rs` — generic code-family test support helpers
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr.rs` — exact `#[path]` exemption logic for canonical sidecar wiring
- `apps/guardrail3/crates/shared/fs/src/lib.rs` — shared filesystem helpers now used by code-family test support
- `.worklogs/2026-03-27-132300-start-rs-code-stabilization.md` — initial workspace split and migration plan
- `.worklogs/2026-03-27-140627-rs-code-rs-test16-slice.md` — first assertions/test-support extraction slice
- `.worklogs/2026-03-27-142821-rs-code-parse-shape-cleanup.md` — parse subtree cleanup context

## Next Steps / Continuation Plan
1. Run an adversarial review of `rs/code` against its README and live rule inventory, the same way `rs/test`, `rs/arch`, and `rs/hexarch` were attacked.
2. Decide the next Rust family stabilization target after `code`; `release`, `garde`, and `deps` are the obvious remaining candidates.
3. Keep the broader Rust pipeline cleanup moving by continuing to eliminate any remaining family-local scope discovery outside the shared placement/mapper architecture.
