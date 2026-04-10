# guardrail3 Workspace + Crate Split

## Problem

`apps/guardrail3` is laid out like a hex architecture in the filesystem, but Cargo still treats it as one package:

- `apps/guardrail3/Cargo.toml`
- `apps/guardrail3/crates/lib.rs`

That means:

- all lib-unit tests still link one monolithic `guardrail3` test binary
- collocated rule tests do not get compile or link isolation
- pseudo-crate boundaries under `crates/**` are architectural hints only, not real build boundaries

The goal is not “more hexes”.
The goal is to make the existing single hex real at the Cargo/workspace level, then complete the split until the heavy Rust validation surface is isolated into real family crates and the remaining shared crates have clean ownership.

The Rust test topology that must survive this split is defined in:

- `.plans/todo/checks/2026-03-25-rust-layered-test-architecture.md`

## Decision

Do **not** split guardrail3 into multiple top-level hex apps first.

There is still one product surface:

- one CLI
- one config model
- one report model
- one `ProjectTree`
- one Rust validation/generation engine

So the correct refactor is:

1. convert the existing pseudo-crates into real workspace members inside `apps/guardrail3`
2. split `app/rs` into smaller real crates, with Rust family crates as the end state for the heavy validation surface
3. normalize the remaining shared/generator/hook surfaces that are still too coupled for clean workspace boundaries

The plan should not pretend that every phase buys the same thing.

- Phases 1-3 are primarily correctness and ownership cleanup
- Phase 4 is the first phase that should materially remove the Rust test/link bottleneck
- root-test and fixture cleanup must advance in parallel, or earlier crate promotion will only relocate the bottleneck

## End-state shape

`apps/guardrail3` becomes a workspace root.

The root package should become thin:

- root workspace definition
- required thin facade package / binary package
- no giant library target containing the whole product

The root facade is required during the split.
It exists only to preserve the shipped product entry surface while internals are refactored cleanly.

It must not become an excuse to preserve bad internal ownership.
The target state is:

- shared types live in the right shared crates
- runtime/policy code depends on those crates directly
- tests depend on the narrowest real crate they need
- the facade is only a thin product-entry adapter

Root-facade imports in tests and internal code are transitional debt and must be removed as crates become real members.

The workspace should contain real members for the existing boundary lines:

- `crates/domain/validation-model`
- `crates/domain/config`
- `crates/domain/report`
- `crates/domain/modules`
- `crates/domain/project-tree`
- `crates/ports/outbound/traits`
- `crates/shared/fs` or equivalent extracted shared FS surface
- `crates/adapters/outbound/fs`
- `crates/adapters/outbound/report`
- `crates/adapters/outbound/tool-runner`
- `crates/app/commands`
- `crates/app/core`
- `crates/app/rs/runtime`
- `crates/app/rs/generate`
- `crates/app/rs/families/arch`
- `crates/app/rs/families/fmt`
- `crates/app/rs/families/toolchain`
- `crates/app/rs/families/clippy`
- `crates/app/rs/families/deny`
- `crates/app/rs/families/cargo`
- `crates/app/rs/families/code`
- `crates/app/rs/families/hexarch`
- `crates/app/rs/families/deps`
- `crates/app/rs/families/garde`
- `crates/app/rs/families/test`
- `crates/app/rs/families/release`
- `crates/app/rs/families/hooks-shared`
- `crates/app/rs/families/hooks-rs`
- `crates/app/hooks`
- `crates/adapters/inbound/cli`

Concrete owner files:

- pure Rust family identity:
  - crate: `crates/domain/validation-model`
  - file: `src/families.rs`
- Rust write set:
  - crate: `crates/app/rs/generate`
  - file: `src/owned_artifacts.rs`
- Rust namespace / command text:
  - crate: `crates/app/commands`
  - files:
    - `src/command_ids.rs`
    - `src/messages.rs`

`app/ts` should not drive this refactor.
It remains out of scope for the structural split unless a trivial workspace-member conversion is free after the Rust refactor is stable.

## Full sequence

## Phase 1 — Establish the real workspace and extract the mandatory shared substrate

Goal:

- convert the current internal manifests into real workspace members
- preserve the current single-product architecture
- remove the fake “one package pretending to be many crates” state
- extract only the shared substrate that is already justified as its own crate boundary

Required outputs:

- `apps/guardrail3/Cargo.toml` becomes a workspace root
- each promoted subtree has a real `src/lib.rs` or explicit `[lib] path = ...`
- root facade crate becomes thin but remains present
- shared dependencies move into the member manifests
- imports are updated from module paths to crate paths
- existing product entrypoints remain live through the root facade
- internal dependency edges are cleaned up rather than preserved behind facade forwarding
- root test targets stop acting as a broad mixed-language catch-all harness by default

Target members for Phase 1:

- `domain-validation-model`
- `domain-project-tree`
- normalized `ports-outbound-traits`
- `adapters-outbound-tool-runner` only after the trait surface is normalized
- `shared-fs`

The following members must wait until later phases:

- `domain-modules`
- `domain-config`
- `domain-report`
- inbound CLI promotion
- `app-rs`
- `app-hooks`
- `app-ts`
- `app-core`
- `adapters-outbound-report`
- `adapters-outbound-fs`

Phase 1 invariants:

- `domain-config` must not be promoted while it still depends on report-owned selection enums
- `domain-report` must not be promoted while it still owns validation/family selection types
- `domain-validation-model` owns the pure Rust family identity and other domain-only validation model types
- `domain-validation-model/src/families.rs` is the canonical owner of the Rust family identity
- CLI-specific parsing/display adapters for family names live outside `domain-validation-model`
- report-presentation helpers for family names live outside `domain-validation-model`
- `shared-fs` extraction is a hard prerequisite to promoting `app-core`, `adapters-outbound-fs`, or Rust family crates cleanly
- `shared-fs` extraction is not complete while Rust family code still imports root `crate::fs`
- inbound CLI is not an early optional crate; the current CLI/domain-modules cycle must be broken first
- `app-core` must not be promoted while it still owns mixed TS map/coverage residue or root-only FS usage
- `ports-outbound-traits` must not be promoted while it still hardens adapter-specific concrete OS types as the shared API
- normalized `ports-outbound-traits` owns the portable filesystem record types that replace `std::fs::DirEntry` and `std::fs::Metadata`
- the portable filesystem record types live in:
  - crate: `crates/ports/outbound/traits`
  - file: `src/fs_types.rs`

## Phase 2 — Normalize the blocking shared surfaces and break the known cycles

Goal:

- remove the remaining misplaced/shared code that would create cycles or force bad crate ownership
- make the Rust runtime, CLI, generators, and canonical modules depend on shared crates cleanly

Required outputs:

- shared validation-model types are extracted from report-owned runtime concerns
- `domain-project-tree` has clean standalone ownership
- shared `fs` ownership is extracted so `app-core` and FS adapters do not depend on root-only `crate::fs`
- `domain-modules` no longer mixes canonical/module baseline logic with CLI-generation behavior
- canonical module data/templates have one explicit owner crate
- the current `domain-modules <-> inbound CLI` cycle is removed
- hook-support substrate is extracted so `app-hooks` does not stay coupled back into legacy Rust validation or duplicate hook machinery
- the hook-support substrate has explicit ownership for:
  - hook script discovery/content loading
  - `core.hooksPath` lookup
  - shell parsing
  - executable/trust evaluation
- inbound CLI dependency edges no longer force shared/domain crates to reach back into adapters
- the root package is thin enough that promoting `app-rs` and `app-hooks` will not reintroduce a monolith through the facade
- the canonical module / generator / check surfaces have an explicit stable owner during the split
- the root test topology is decomposed enough that new member-crate isolation is not immediately erased by root-level aggregators
- binary-style integration tests that only shell out to `CARGO_BIN_EXE_guardrail3` stop importing the root facade library
- the root `tests/unit.rs` mixed aggregator is dismantled or sharply reduced before later crate promotion is claimed as a performance win

Target work items:

- extract the shared validation-model crate
- extract `domain-project-tree`
- extract shared `fs` ownership
- normalize `domain-modules`
- assign canonical module data/templates to one explicit owner crate
- normalize generator ownership surfaces that currently cross into CLI code
- move CLI-only helper logic such as override deduplication out of inbound adapters and into the proper shared owner
- normalize inbound CLI dependencies so they consume app/domain crates rather than forcing reverse reachability
- extract the shared hook-support substrate before `app-hooks` promotion
- split Rust-only generator/check/diff surfaces off the all-stack generator path before inbound CLI promotion is claimed
- choose explicit ownership and namespace policy for module listing and module detail surfaces
- move help/guide/module/generated command text under one explicit owner instead of duplicating it across domain/adapters
- perform a full user-visible command-text audit, not only `--help`
- reduce or delete root-level test aggregators that keep the facade crate on the hot path
- reduce shared-fixture coupling where one root golden tree invalidates many unrelated family tests

Only after this phase may the following become real promoted members:

- `domain-config`
- `domain-report`
- `adapters-outbound-report`
- `adapters-outbound-fs`
- `app-core`
- `app/commands`
- `adapters/inbound/cli`

`app-hooks` must not be promoted in this phase.
Its promotion is blocked on the runtime hook-ownership decision in Phase 3.

Phase 2 is not complete while:

- `tests/unit.rs` still acts as the mixed root harness
- binary-only root tests still import the root facade library

## Phase 3 — Promote runtime crates and stabilize the Rust runtime boundary

Goal:

- make `app-rs` a real runtime member before the family split
- preserve one stable Rust runtime surface while later splits move behind it
- make the hooks/runtime boundary explicit before deeper crate extraction
- remove the remaining fake product surfaces that still force the root package to compile irrelevant stacks together

The Phase-3 runtime end state must be:

- one stable `app-rs-runtime` crate
- one stable `crates/app/rs/runtime` crate
- a stable product-facing Rust runtime API
- explicit hooks/runtime ownership
- a typed runtime-owned scope model instead of raw CLI-owned path strings
- a typed runtime-owned applicability model for per-root family enablement
- one explicit owner for Rust report assembly

The runtime crate must preserve the current public runtime contract during the split:

- one stable `run(...)` entrypoint for Rust validation
- one stable family-selection/report-section assembly point
- no family crate may assemble final reports directly

Phase 3 must also resolve the hooks/runtime ownership choice:

- either hooks stay above the Rust runtime and call it as a product API
- or Rust runtime formally depends on hook crates as part of its stable boundary

That ownership decision must be made before `app-hooks` becomes a real member boundary.

Phase 3 must also remove the remaining live dependencies on legacy `app/rs/validate/**` that block crate promotion:

- parser helpers used by new-family code
- coverage/map helpers
- hook validation routing

Deletion is gated mechanically, not rhetorically:

- no non-legacy file may import `crate::app::rs::validate::*`
- `pub mod validate` must be gone from the live Rust app surface
- hook validation and coverage paths must already be on the new/shared surfaces

Phase 3 must also make the shipped product surfaces coherent:

- help text only advertises commands that actually exist
- guide/remediation/module text only points at real commands
- Rust-only `check` / dry-run / diff surfaces stop expanding into TS-wide generation
- Rust-scoped module commands have coherent ownership and do not accidentally expose mixed-stack registry semantics
- one explicit owner defines the Rust write set used by:
  - `rs generate`
  - `rs check`
  - `rs diff`
  - `rs hooks-install`
- one explicit owner defines the Rust module namespace and the user-visible command text that refers to it

Concrete runtime owners:

- Rust write set:
  - crate: `crates/app/rs/generate`
  - file: `src/owned_artifacts.rs`
- Rust namespace / command text:
  - crate: `crates/app/commands`
  - files:
    - `src/command_ids.rs`
    - `src/messages.rs`

`rs hooks-install` belongs to the Rust write set.
It must not inspect TypeScript configuration or emit TypeScript-owned hook steps.
Any mixed-stack hook artifact remains a separate product surface outside the Rust write set.

Phase 3 must also migrate tests away from the root facade where possible:

- family-unit tests should compile against their owning family crate
- shared/support tests should compile against the smallest shared crate they actually need
- root-level tests should be reserved for product-entry integration only
- binary-only root integration tests must stop importing the root library
- legacy-validator root tests must either migrate or be explicitly quarantined as remaining blocker debt

Target shape:

- `crates/app/rs/runtime`
- `app-hooks` only after the hook ownership decision and hook-support substrate are real

## Phase 4 — Split `app/rs` into family crates

Goal:

- reduce compile/link/test blast radius for Rust validation families
- keep family tests collocated while making them real Cargo-isolated tests
- avoid simply moving the monolith into one oversized runtime crate

Do **not** introduce artificial coarse crates such as:

- `app-rs-config-policy`
- `app-rs-source-analysis`
- `app-rs-architecture`

The current families already mix root policy, scoped source analysis, and architecture facts internally.
Those coarse seams would recreate the monolith behind a different crate boundary.

After `app-rs-runtime` is stable and the shared substrate is clean, split directly into family crates.

Final target family shape:

- `crates/app/rs/runtime`
- `crates/app/rs/generate`
- `crates/app/rs/families/arch`
- `crates/app/rs/families/fmt`
- `crates/app/rs/families/toolchain`
- `crates/app/rs/families/clippy`
- `crates/app/rs/families/deny`
- `crates/app/rs/families/cargo`
- `crates/app/rs/families/code`
- `crates/app/rs/families/hexarch`
- `crates/app/rs/families/deps`
- `crates/app/rs/families/garde`
- `crates/app/rs/families/test`
- `crates/app/rs/families/release`
- `crates/app/rs/families/hooks-shared`
- `crates/app/rs/families/hooks-rs`

`hooks` should stay outside this split only after the hook-support substrate is real and the ownership boundary is explicit.
The target hook shape is:

- `app-hooks`
- with a later explicit split into `app-hooks-shared` and `app-hooks-rs` if `app-hooks` still remains materially heavy after workspace promotion

## Phase 5 — Complete the remaining heavy shared surfaces

Goal:

- finish the split so the remaining known heavy/shared surfaces do not quietly become the next monolith

Required outputs:

- `domain-modules` has a clean permanent ownership model
- `app-hooks` is promoted cleanly and split further if test/build cost still justifies it
- generator surfaces are either left intentionally centralized with clean ownership or split alongside checker families where parity requires it
- the product binary still exposes one coherent CLI/help/config/report contract across the whole refactor
- root-level mixed integration/unit aggregators are retired or reduced to true product-entry integration only
- root integration tests that are only exercising the binary no longer pull the root library into the build
- any remaining shared fixture trees are intentionally owned rather than acting as an accidental invalidation hotspot

Phase 4 is part of the plan.
It is not optional.
What is conditional is only the exact depth of the final sub-split, based on measured bottlenecks after Phase 3.

## Shared crates that must exist before Phase 4

These surfaces should be shared crates, not copied into family crates:

- config types
- report types / family enums / section types
- canonical module baselines
- `ProjectTree`
- filesystem and tool-runner traits
- core walker/discovery substrate that is family-agnostic

Minimum shared set:

- `domain-validation-model`
- `domain-project-tree`
- `shared-fs`
- reduced `domain-report`
- reduced `domain-config`
- explicit architecture/root-placement shared facts substrate if `arch` and `hexarch` would otherwise duplicate it
- `ports-outbound-traits`
- `app/commands`
- `app-core`

`domain-modules` is not in the minimum shared set.
It currently mixes canonical/module baseline logic with generator-facing behavior and should be normalized before promotion.

## Things that should stay centralized

Do not duplicate these into every family crate:

- `ProjectTree` construction
- `guardrail3.toml` parse model
- report aggregation and family selection types
- CLI argument parsing
- outbound adapter implementations

Phase-4 family crates should depend on shared crates and expose:

- `check(...)`
- family facts/inputs/rules

The runtime crate should own:

- family dispatch
- typed scope handling
- typed applicability handling
- family selection resolution
- report section assembly

## Things that should not drive the Rust split

- TypeScript validator structure
- deploy planning
- frontend/content planning
- generator-family splitting

Those should not block the Rust-side workspace split.

But these live product surfaces must remain explicitly accounted for during the split:

- `rs validate`
- `rs generate`
- `rs check`
- `rs diff`
- `rs hooks-install`
- TS-facing commands that remain shipped in the binary
- `list-modules`
- `show-module`
- `guide`
- coverage/map/help output

TS-facing root integration tests and commands remain part of the shipped binary during the split.
They must be explicitly either:

- kept as legacy product-entry tests with bounded scope
- or isolated from the Rust crate-splitting success criteria

but not silently left as broad root-package compile anchors.

## Test-topology requirements

The split must explicitly change the test topology, not only the crate topology.

Required changes:

- retire or decompose the root `tests/unit.rs` mixed aggregator
- move family-specific root tests to the owning crate where possible
- keep root integration tests only for end-to-end product-entry behavior
- prevent new tests from importing through the root facade when a narrower crate exists
- make shared fixture helpers live in the narrowest shared test-support surface possible
- treat the root `[[test]]` target set, not just `tests/unit.rs`, as part of the bottleneck to shrink
- define one explicit measurable gate for root-test shrink before build-isolation progress is claimed

Root-test shrink gate:

- `apps/guardrail3/tests/unit.rs` is deleted
- no file under `apps/guardrail3/tests/` imports `guardrail3::`
- root `[[test]]` target count for `apps/guardrail3` is `<= 8`

The split is not successful if:

- family crates exist
- but root-level test harnesses still force the same broad compile/link wall

## Fixture strategy

The crate split must include fixture-surface normalization.

Required direction:

- keep shared golden fixture trees only where true cross-family reuse is unavoidable
- move family-local fixture mutation helpers into the owning family crate
- avoid one giant shared fixture harness becoming the next bottleneck after crate splitting
- treat shared fixture data itself as the invalidation hotspot, not only helper location

Cross-family fixture reuse is allowed, but it must be explicit and measured.

Phase 4 is not successful while one shared fixture tree still dominates invalidation for unrelated family crates.

Fixture-hotspot reduction gate:

- `apps/guardrail3/tests/unit/test_support/fixture.rs` is deleted
- no single fixture root under `apps/guardrail3/tests/fixtures/` is referenced by more than `2` family crates
- each heavy Rust family crate owns its own primary fixture root:
  - `code`
  - `garde`
  - `hexarch`
  - `clippy`
  - `deny`
  - `release`

## Risks

## Risk 1 — Circular dependencies

Main risk:

- `app-rs` and CLI/runtime code currently depend directly on `domain/config`, `domain/report`, `ProjectTree`, ports, and adapters through module paths

Mitigation:

- extract shared crates first
- keep adapters outward only
- keep family crates depending on shared/domain/ports crates, not on CLI

## Risk 2 — Over-splitting too early

If Phase 1 and Phase 2 are attempted together, the refactor surface becomes too wide.

Mitigation:

- finish Phase 1 first
- get the workspace compiling
- then split `app-rs`

## Risk 3 — TS and legacy runtime noise

TS and legacy validator code still exist and may create false pressure to solve everything in one pass.

Mitigation:

- keep this refactor Rust-first
- only carry TS crates along if they are trivial and non-blocking

## Ordered implementation plan

1. convert `apps/guardrail3` root into a real workspace root
2. extract the shared validation-model crate from report-owned runtime enums and selection types
3. split domain validation data from CLI/report presentation concerns so the shared validation-model crate stays domain-only
4. extract `domain-project-tree` as a mandatory shared crate
5. normalize `ports-outbound-traits` so it stops hardening concrete OS-facing types into the shared boundary
6. extract shared `fs` ownership so `app-core`, FS-related crates, and Rust family code no longer depend on root-only `crate::fs`
7. promote the shared domain/ports/outbound crates that are clean after those extractions
8. make the root package thin
9. keep the root facade exporting the current product/runtime surfaces while the new members come online
10. get the workspace compiling with those shared members first
11. normalize `domain-modules` and the inbound CLI dependency edges that still point back into monolithic code
12. split Rust-only generator/check/diff surfaces off the all-stack generation path
13. choose one explicit owner for the Rust write set and make every Rust-facing generation command consume it
14. choose ownership for module listing/detail/help/guide/generated command text and make the user-visible command surface coherent
15. decompose root test aggregators and shrink root test targets so crate isolation can actually pay off
16. freeze the stable runtime/API contracts that must survive the split:
   - typed applicability model
   - typed scope model
   - single Rust report owner
17. promote `crates/app/rs/runtime` as a real member and stabilize the Rust runtime boundary before family extraction
18. decide and implement the hooks/runtime ownership boundary
19. move inbound CLI onto the runtime crate instead of direct monolith internals
20. promote `crates/app/rs/generate` and split `crates/app/rs` directly into family crates
21. move the heaviest Rust families first:
   - `code`
   - `garde`
   - `hexarch`
   - `cargo`
   - `clippy`
   - `deny`
22. move the lighter Rust families
23. measure the remaining build/test hotspots
24. finish `domain-modules` permanent ownership cleanup
25. promote/split `adapters-outbound-fs` only once shared FS ownership is real
26. promote `app-hooks` only after the hook-support substrate and runtime ownership decision are both complete
27. split `app-hooks` further only if measurement still shows it as a meaningful bottleneck
28. decide generator-side crate splitting based on real parity/build pressure instead of guessing early
29. split overweight family crates further if measurement shows a single family crate remains a bottleneck
30. remove temporary facade forwarding only when the binary is fully rewired to member crates

## Delegation lanes

Lane 1:

- Phase-1 workspace root and member wiring
- Phase-1 workspace root and member wiring, without freezing bad trait or validation-model edges

Lane 2:

- shared crate extraction (validation-model, `ProjectTree`, config, report, ports, shared `fs`)

Lane 3:

- `app/rs` runtime + generate + family-crate dependency map

Lane 4:

- runtime/CLI rewiring once the shared crates exist

Lane 5:

- Phase-4 completion work (`domain-modules`, hooks depth, generator-side follow-through)

Lane 6:

- root test-topology decomposition and fixture normalization

## Success criteria

Phase 1 is successful when:

- `apps/guardrail3` is a real workspace
- the Phase-1 substrate boundaries are real Cargo members
- the root facade package is thin
- the CLI still works
- shared validation-model, `ProjectTree`, and shared `fs` ownership are no longer trapped in the root monolith
- root-test shrink has an explicit measured gate for later phases, even if the shrink itself is not complete yet

Phase 2 is successful when:

- `domain-config` and `domain-report` no longer have a bad ownership knot around runtime enums/selections
- `app-core` and FS-related crates do not depend on root-only `crate::fs`
- `domain-modules` no longer has hidden generator/CLI ownership leakage
- inbound CLI and shared/domain crates have one-way dependency flow
- the workspace is ready for `app-rs` promotion without crystallizing bad cycles
- canonical module / generator ownership is explicit and stable
- root mixed test aggregators are no longer masking the benefit of member-crate isolation
- binary-only root tests no longer pull the root facade in just to shell out to the binary

Phase 3 is successful when:

- one stable Rust runtime crate owns selection, scope, and report assembly
- that runtime uses typed applicability data, not only raw path filtering
- inbound CLI uses that runtime boundary instead of monolith internals
- hooks/runtime ownership is explicit and stable
- one explicit owner defines the Rust write set for Rust-facing generation commands
- one explicit owner defines Rust command ids and shared user-facing messages
- the Rust runtime API remains stable while the next split happens behind it

Phase 4 is successful when:

- Rust family tests no longer link one monolithic `guardrail3` lib-test binary
- heavy families compile as separate crates
- runtime dispatch is centralized in one runtime crate, not duplicated
- family tests primarily compile against family/shared crates, not the root facade
- root test shrink is measured through an explicit gate, not assumed
- the shared fixture hotspot is reduced enough that crate splitting is not being masked by one dominant fixture corpus

Phase 5 is successful when:

- no remaining shared crate is obviously the next monolith
- `domain-modules`, `app-hooks`, and generator surfaces each have an explicit stable ownership story
- CLI/config/report/help surfaces remain coherent throughout the split, not only at the end
- root integration tests are true product-entry tests, not a broad mixed harness
