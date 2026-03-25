# Hexarch Layered Test Architecture Note

This is not an implementation brief.

It is a future-architecture note for any later agent touching `rs/hexarch`, the Rust family crate split, or Rust test topology.

Read this before restructuring `hexarch` tests.

## Why this exists

The current `hexarch` test corpus mixes:

- rule semantics
- collector behavior
- family ownership/fan-out behavior
- full golden-fixture behavior

under rule-sidecar test directories.

That makes trivial rules look deeply tested when the tests are often really proving:

- project walking
- structural discovery
- dependency collection
- source collection
- family-level ownership splits

So the long-term shape must separate those concerns.

## The intended `hexarch` test layers

`hexarch` is not a 3-layer family.

It needs 4 layers:

1. core rule tests
2. collector / facts tests
3. family-orchestrator tests
4. family integration tests

### 1. Core rule tests

Use these for:

- direct rule semantics over typed facts
- exact emitted rule id
- exact severity
- exact local attribution

Do not use these for:

- filesystem behavior
- project walking
- manifest parsing
- source collection
- cross-rule ownership splits

Examples:

- `RS-HEXARCH-04`
  - loose non-`.gitkeep` file fails
  - real `.gitkeep` exempts
  - symlinked `.gitkeep` does not exempt
- `RS-HEXARCH-21`
  - allowlist applies
  - dev-deps are excluded from rule ownership

### 2. Collector / facts tests

Use these for raw collector output only.

For `hexarch`, that means:

- `facts.rs`
  - root discovery
  - child discovery
  - leaf classification
  - workspace member resolution
- `dependency_facts.rs`
  - edge collection
  - alias resolution
  - inherited dependency resolution
  - config parse capture
  - cycle collection
- `source_facts.rs`
  - reachable module graph
  - source item extraction
  - trait / impl counting
  - source parse capture

These tests should usually stop before final rule findings.

### 3. Family-orchestrator tests

This is the missing layer that matters most.

Use it for:

- shared-input fan-out
- cross-rule ownership splits
- collector-backed rule application on small synthetic trees
- exact non-hit ownership boundaries

This is where `hexarch` proves things like:

- `02` owns missing top-level dirs, not `03`
- `04` owns loose-file hits, not `05`
- `07`, `09`, and `10` split one shared workspace/member input correctly
- `13`, `20`, `24`, and `25` split one collected edge set correctly
- `17` and `18` split inherited-renamed dependency ownership correctly

If a test currently goes through the family entrypoint and proves which rule should or should not fire, but does not need the full golden fixture, it probably belongs here.

### 4. Family integration tests

Use these for:

- the real golden fixture
- broad attack vectors
- nested-root parity
- exact owned hit and non-hit sets in realistic repo shapes

This is where the “one test = one attack vector” model still applies most strongly.

But not every current `hexarch` fixture-backed test belongs here.

## Cargo/test-shape constraints

When `hexarch` becomes a real family crate, the tests must fit Cargo’s actual test model.

So:

- do not plan on bare `tests/facts/` or `tests/integration/` directories being auto-run
- use top-level harnesses such as:
  - `tests/facts.rs`
  - `tests/orchestrator.rs`
  - `tests/integration.rs`
- those harnesses can `mod` nested files under:
  - `tests/facts/**`
  - `tests/orchestrator/**`
  - `tests/integration/**`

## Privacy constraint

Do not widen the family crate’s public API just so tests can call private rule helpers.

The intended split is:

- crate-internal test helpers:
  - stay crate-private or `pub(crate)`
- external integration harnesses:
  - assert through public family surfaces

So the family crate should not grow a fake public test API.

## Fixture/test-support constraint

`hexarch` will need family-owned test support.

That means:

- primary `hexarch` fixtures should eventually live under the family crate
  - or under an explicit shared fixture crate if we later choose that
- test support should import real crate dependencies directly through `dev-dependencies`
- do not keep long-term dependence on root-facade namespaces like `crate::adapters::*`

## What a later agent should remember

If you are changing `hexarch` tests later, assume:

- many current rule-sidecar tests are misfiled
- ownership-boundary tests are often orchestrator tests, not collector tests
- parse-error ownership often belongs to the rule, not the collector
- `RS-HEXARCH-14` does not need a huge integration attack matrix
- `RS-HEXARCH-22/23` need both:
  - small typed core rule tests
  - filesystem-backed source-collector/orchestrator coverage

## Canonical follow-up docs

For more detail, use:

- `.plans/todo/checks/2026-03-25-rust-layered-test-architecture.md`
- `.plans/todo/check_review/test_hardening/31-hexarch-layered-test-map.md`
- `.plans/todo/check_review/test_hardening/32-hexarch-01-06-layered-migration-checklist.md`

This note is the short version to keep the architecture in view before any later implementation pass.
