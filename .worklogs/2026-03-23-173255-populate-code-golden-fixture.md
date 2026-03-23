# Populate Code Golden Fixture

**Date:** 2026-03-23 17:32
**Scope:** `.plans/todo/check_review/test_hardening/02-code.md`, `.plans/todo/check_review/test_hardening/12-code-agent-brief.md`, `.plans/todo/check_review/test_hardening/12-code-execution-plan.md`, `apps/guardrail3/tests/fixtures/r_arch_01/golden/`

## Summary
Populated the shared `r_arch_01/golden` fixture with realistic Rust and TypeScript code across all remaining app and package slices, replacing the last placeholder-only modules. Updated the `rs/code` hardening docs so they describe the actual state of the fixture and the next phase of work accurately.

## Context & Problem
The `rs/code` hardening lane needs a believable mixed-monorepo source tree so attack-vector tests can mutate real code surfaces instead of tiny handwritten snippets. The reusable golden fixture already existed, but it was only structurally useful: many files were comments or single-line stubs, which meant it could not support the intended broad mutation model. The user explicitly wanted the fixture filled out enough that all apps and packages looked like plausible projects, not just a few top-level slices.

## Decisions Made

### Reuse the existing shared golden tree instead of inventing a second fixture
- **Chose:** Keep building on `apps/guardrail3/tests/fixtures/r_arch_01/golden/`.
- **Why:** The existing tree already matches the mixed-monorepo layout other tests know about, so extending it preserves reuse and avoids a second divergent fixture.
- **Alternatives considered:**
  - Create a separate `rs/code`-only fixture — rejected because it would duplicate structure and weaken reuse across lanes.
  - Copy a real external project wholesale — rejected because it would add a lot of irrelevant detail and make targeted mutations harder to reason about.

### Make each remaining app/package slice carry believable business logic
- **Chose:** Populate `worker`, `devctl`, `portal`, shared packages, and leftover adapter leaves with small but coherent flows.
- **Why:** The useful property for guardrail hardening is not production completeness, but realistic legal surfaces: ports, adapters, public APIs, nested modules, route handlers, DTOs, CLI entrypoints, and a mix of clean code shapes that later mutations can break.
- **Alternatives considered:**
  - Leave some slices as thin placeholders once one or two apps were realistic — rejected because empty leaves create obvious blind spots in broad mutation tests.
  - Add only bigger files without internal structure — rejected because many `rs/code` rules care about structure and boundaries, not just file existence.

### Verify Rust slices locally and keep the fixture source-only afterward
- **Chose:** Run targeted `cargo test`/`cargo check` against the newly populated Rust workspaces and then remove generated `target/` and fixture-local `Cargo.lock` artifacts.
- **Why:** The fixture should be trustworthy enough to mutate, but it should remain a source fixture in git rather than a build-output dump.
- **Alternatives considered:**
  - Skip verification and trust the fixture code by inspection — rejected because the new slices introduced real cross-crate wiring.
  - Keep build artifacts around — rejected because they add noise and are not part of the fixture contract.

## Architectural Notes
The golden fixture now contains several distinct but compatible code surfaces:
- Rust service-style apps:
  - `backend` planning service plus MCP transport slice
  - `worker` queue processor with retry/dead-letter behavior
  - `devctl` workspace-doctor CLI
- TypeScript web-style apps:
  - `landing` marketing/content app
  - `admin` validation dashboard
  - `portal` checkout/payment/support app
- Shared packages:
  - `packages/shared-types` for shared Rust types
  - `packages/ui-kit` for shared TS helpers

This gives `rs/code` later mutation passes a realistic spread of:
- crate manifests and workspace config
- library and binary entrypoints
- ports/adapters/app/domain layering
- public Rust API surfaces
- TS route/components/module boundaries
- helper utilities and shared package seams

## Information Sources
- `AGENTS.md`
- `.plans/todo/check_review/test_hardening/02-code.md`
- `.plans/todo/check_review/test_hardening/12-code-agent-brief.md`
- prior fixture state under `apps/guardrail3/tests/fixtures/r_arch_01/golden/`
- targeted verification commands run during this session:
  - `cargo test --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/backend/Cargo.toml -p backend-app-commands -p backend-adapters-inbound-rest -p backend-app-queries`
  - `cargo test --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/worker/Cargo.toml -p worker-app-processor -p worker-adapters-inbound-poller`
  - `cargo test --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/devctl/Cargo.toml -p devctl-app-core -p devctl-adapters-inbound-cli`
  - `cargo check --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound/transport/Cargo.toml`
  - `cargo check --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/packages/shared-types/Cargo.toml`

## Open Questions / Future Considerations
- The TS fixture apps still have not been typechecked. That is acceptable for this pass because the immediate consumer is `rs/code`, but a later fixture hardening pass should run TS verification too.
- The next `rs/code` work should stop expanding fixture realism and start consuming this fixture in broad mutation tests.
- Some early `rs/code` migrated tests still use “golden” in the narrow direct-snippet sense; those should be rewritten against this populated shared tree.

## Key Files for Context
- `.plans/todo/check_review/test_hardening/02-code.md` — current code-family hardening status and coverage matrix
- `.plans/todo/check_review/test_hardening/12-code-agent-brief.md` — current handoff brief for the `rs/code` lane
- `.plans/todo/check_review/test_hardening/12-code-execution-plan.md` — step-by-step execution order for the lane
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/Cargo.toml` — root fixture Rust metadata
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/guardrail3.toml` — root guardrail config for the shared fixture
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/backend/Cargo.toml` — richest Rust service fixture root
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/worker/Cargo.toml` — queue-processing Rust fixture root
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/devctl/Cargo.toml` — CLI Rust fixture root
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/admin/tsconfig.json` — admin TS app config
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/portal/tsconfig.json` — portal TS app config
- `.worklogs/2026-03-23-160918-add-family-hardening-agent-briefs.md` — prior hardening brief setup context

## Next Steps / Continuation Plan
1. Continue the `rs/code` hardening lane by rewriting migrated rule tests to mutate this populated golden tree instead of relying on isolated snippets.
2. Start with the next bypass-heavy cluster still not migrated or not yet using the shared fixture, beginning at `RS-CODE-07` and then `RS-CODE-17..24`.
3. For each migrated rule, add:
   - one clean golden-tree non-hit
   - one broad attack-vector mutation across all relevant owned files
   - exact owned hit/non-hit assertions
4. As those tests deepen, fix any parser/helper bugs exposed in `parse.rs`, especially around grouped/aliased attributes and whole-type `#[garde(skip)]`.
