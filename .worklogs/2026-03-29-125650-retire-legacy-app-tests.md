# Retire Legacy App Tests

**Date:** 2026-03-29 12:56
**Scope:** `apps/guardrail3/tests/**`, `apps/guardrail3/crates/adapters/inbound/cli/coverage/{clippy.rs,cspell.rs,deny.rs,engine.rs,eslint.rs,jscpd.rs,npmrc.rs,prettier.rs,rust_toolchain.rs,rustfmt.rs,stylelint.rs,tsconfig.rs}`

## Summary
Removed the old app-root integration/property/unit test tree under `apps/guardrail3/tests`, which no longer matches the current per-family Rust-check architecture, and added `Debug` derives to CLI coverage structs so the binary test target still compiles under the workspace lint baseline.

## Context & Problem
The repository direction has shifted to the new Rust family architecture under `apps/guardrail3/crates/app/rs/families/**`, with family-local sidecars/assertions/test-support carrying the durable coverage. The top-level `apps/guardrail3/tests` tree was legacy surface from the pre-family architecture and was only increasing maintenance cost and cognitive noise while the real work moved into family crates.

At the same time, the app-root workspace now enforces `missing_debug_implementations`, and the CLI coverage helper structs were simple exported types without `Debug`. That compile debt was small but needed to be fixed in the same cleanup lane so the binary test target stayed buildable after the legacy test surface was removed.

## Decisions Made

### Delete the obsolete app-root test tree instead of carrying it in parallel
- **Chose:** Remove `apps/guardrail3/tests/**` entirely.
- **Why:** The repo is now driven by family-local Rust guardrail tests. Keeping a second legacy app-root suite alongside them would preserve duplicate and stale coverage shapes from the removed architecture.
- **Alternatives considered:**
  - Leave the files in place while migrating family by family — rejected because they are already superseded and create misleading cold-start context.
  - Rewrite the whole legacy suite in place — rejected because the accepted target architecture is family-local, not a revitalized top-level test tree.

### Fix coverage helper compile fallout directly
- **Chose:** Add `#[derive(Debug)]` to the CLI coverage structs and output model types.
- **Why:** These are straightforward exported data/helper types. The workspace lint is intentional, and the lowest-friction honest fix is to derive `Debug`.
- **Alternatives considered:**
  - Add `allow(missing_debug_implementations)` — rejected because it would weaken the workspace lint contract.
  - Keep the types private just to avoid the lint — rejected because that would be a fake encapsulation change unrelated to the cleanup.

## Architectural Notes
This commit is a surface cleanup that aligns the repo with the already-adopted architecture:

- durable Rust check coverage lives with the families
- the app root is no longer treated as the place for a giant monolithic legacy test suite
- exported CLI coverage helpers comply with the workspace lint policy

It does not finish the active family migrations. It removes stale scaffolding so the remaining work is easier to reason about.

## Information Sources
- `AGENTS.md`
- `apps/guardrail3/crates/app/rs/families/**`
- `apps/guardrail3/tests/**`
- verification command:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --bin guardrail3 --no-run`

## Open Questions / Future Considerations
- Some of the deleted legacy tests may still contain useful adversarial scenarios not yet re-expressed in family-local suites. If a concrete gap is discovered later, it should be reintroduced in the owning family rather than by reviving `apps/guardrail3/tests`.
- This commit does not address the remaining live `RS-TEST` / `RS-CODE` family debt in the app root; it only removes obsolete top-level test surface.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/**` — current family-local test architecture that supersedes the deleted legacy suite
- `apps/guardrail3/crates/adapters/inbound/cli/coverage/engine.rs` — exported coverage output model now deriving `Debug`
- `apps/guardrail3/crates/adapters/inbound/cli/coverage/*.rs` — coverage tool structs now deriving `Debug`
- `.worklogs/2026-03-29-125544-harden-rs-test-proof-detection.md` — immediately preceding `RS-TEST` checker hardening checkpoint

## Next Steps / Continuation Plan
1. Commit this legacy-surface cleanup separately from the ongoing family migrations.
2. Resume the `release` family `RS-TEST` migration and commit it as its own checkpoint.
3. After `release`, continue the same structural/semantic migration on the next highest-count families: `garde`, `deps`, `code`, `hooks-shared`, and `deny`.
