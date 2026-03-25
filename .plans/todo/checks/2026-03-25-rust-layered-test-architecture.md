# Rust Layered Test Architecture

## Purpose

The current Rust test corpus mixes several different things under rule-sidecar test directories:

- pure rule semantics
- facts / discovery / parsing behavior
- family-orchestrator ownership and fan-out behavior
- full golden-fixture pipeline behavior

That is the wrong boundary.

It makes small rules look “well tested” when the tests are mostly exercising:

- `ProjectTree` discovery
- family fact builders
- workspace / manifest parsing
- runner orchestration

The goal of this plan is to realign Rust tests with the planned crate architecture so:

- rule tests prove rule semantics only
- collector tests prove discovery / parsing / classification
- family-orchestrator tests prove prebinding, fan-out, and cross-rule ownership
- integration tests prove real fixture behavior through the public family pipeline
- family crates keep collocated fast tests
- golden fixtures remain available without pretending every fixture-backed test is a unit test

This plan is an execution contract for the Rust side only.
It complements:

- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md`
- `.plans/todo/checks/2026-03-24-rust-validation-cutover.md`
- `.plans/todo/check_review/test_hardening/00-shared-test-story.md`

Family-specific concrete mapping starts with:

- `.plans/todo/check_review/test_hardening/31-hexarch-layered-test-map.md`

## Core decision

Every Rust family uses four family-local test layers:

1. core rule tests
2. collector / facts tests
3. family-orchestrator tests
4. family integration tests

Those layers must not be conflated.

### Layer 1 — Core rule tests

Purpose:

- prove the rule contract over typed inputs / facts
- prove exact findings, exact targets, and exact severity

Allowed:

- inline constructed inputs
- tiny builders / fixtures in code
- direct calls to the rule function

Forbidden:

- filesystem IO
- project walking
- manifest parsing
- golden fixture loading
- family orchestration

A rule like “Cargo.toml exists” should have very few core rule tests:

- pass when the fact says cargo exists
- fail when the fact says cargo does not exist
- malformed-edge case only if malformed input is part of the rule contract

If a test mostly proves that the walker discovered `Cargo.toml`, it is not a rule test.

### Layer 2 — Collector / facts tests

Purpose:

- prove the discovery and fact-building layer
- prove path classification
- prove workspace/member parsing
- prove manifest-kind classification
- prove scope narrowing inputs where applicable

Allowed:

- fixture-backed tree loading
- parser helpers
- manifest parsing
- project walker
- fact builders

Forbidden:

- asserting final rule ownership or cross-rule ownership splits
- asserting final family finding sets unless the collector contract itself includes them

These tests own questions like:

- did we discover every relevant `Cargo.toml`?
- did we classify this root as `apps/*`, `packages/*`, or `other`?
- did we parse workspace members correctly?
- did we build the leaf facts correctly?

### Layer 3 — Family-orchestrator tests

Purpose:

- prove family-level prebinding and fan-out
- prove rule ownership splits on shared collectors
- prove collector-backed rule application that is smaller than a golden attack

This is the layer that owns behavior like:

- one shared input fanning out to multiple rules
- one collected edge set split across rules by edge kind
- one source/dependency collector feeding a rule while preserving rule ownership
- cross-rule non-hit contracts such as “rule 17 owns this, rule 18 does not”

Allowed:

- tiny synthetic `ProjectTree` or tempdir-backed collector runs
- fact builders
- family input builders
- family entrypoints that are still internal to the family crate

Forbidden:

- root CLI/product surface tests
- broad golden-fixture attack matrices unless the point is truly full integration

### Layer 4 — Family integration tests

Purpose:

- prove the real fixture passes or fails under the full family pipeline
- prove attack vectors against the real golden fixture
- prove exact owned hit and non-hit sets in realistic project shapes

Allowed:

- full golden fixture loading
- mutation helpers
- parser / facts / rule composition
- family runner entrypoints

Required:

- one test = one attack vector
- exact owned hit set
- exact owned non-hit set
- exact rule id and severity

## One shared rule assertion seam

Each rule should expose a small shared assertion seam for crate-internal tests.

The pattern is:

- core assertion helper takes typed rule input / facts
- inline unit tests build minimal facts inline and call it
- collector/orchestrator tests may reuse that helper when they remain crate-internal
- external integration tests should assert through the public family surface, not through a widened test API

That means the shared seam is about rule behavior, not fixture loading.

Good:

- `assert_passes(input)`
- `assert_hits(input, expected_findings)`
- `assert_exact_findings(input, expected_findings)`

Bad:

- helper that loads a fixture internally
- helper that hides the rule intent behind large opaque setup
- helper that mixes discovery assertions and rule assertions together
- making test-only helpers public just so external `tests/` targets can call them

## Ownership by planned crate architecture

The layered test model follows the planned Rust crate split.

### Shared crates

- `crates/domain/project-tree`
  - owns project-tree data-model tests only
- `crates/domain/validation-model`
  - owns family-identity selection tests
- `crates/ports/outbound/traits`
  - owns portable filesystem type and trait-shape tests
- `crates/app/core`
  - owns project walker tests
  - owns tree-discovery tests unless and until the walker itself is moved

These crates do not own family rule tests.

### Rust family crates

Each family crate under:

- `crates/app/rs/families/<family>/`

owns:

- its core rule tests
- its collector / facts tests
- its family-orchestrator tests
- its family integration tests

That keeps rule and family ownership local while still separating the layers inside the crate.

### Rust runtime crate

- `crates/app/rs/runtime`

owns only:

- runner/orchestration tests
- family selection / applicability / scope behavior
- report assembly behavior

It does not own family rule semantics.

### Rust generation crate

- `crates/app/rs/generate`

owns:

- `owned_artifacts` tests
- `check` / `diff` / generation write-surface tests
- hook-install write-surface tests

It does not own validator rule tests.

## Standard family layout

Inside each Rust family crate:

```text
crates/app/rs/families/<family>/
  src/
    lib.rs
    facts.rs
    inputs.rs                  # when needed
    rs_<family>_01_*.rs
    rs_<family>_02_*.rs
    tests/
      mod.rs
      testkit/
        mod.rs
        rs_<family>_01.rs
        rs_<family>_02.rs
      collectors/
        ...
      orchestrator/
        ...
  tests/
    support/
      mod.rs
      fixtures.rs
      assert.rs
    facts.rs
    orchestrator.rs
    integration.rs
    facts/
      ...
    orchestrator/
      ...
    integration/
      ...
```

Rules:

- rule production files stay one-file-per-rule
- shared rule assertion helpers live in crate-internal `src/tests/testkit/`
- crate-internal test helpers remain `pub(crate)` or private
- external `tests/*.rs` harnesses call public family surfaces only
- Cargo harness files are top-level:
  - `tests/facts.rs`
  - `tests/orchestrator.rs`
  - `tests/integration.rs`
- submodules may live under:
  - `tests/facts/**`
  - `tests/orchestrator/**`
  - `tests/integration/**`
- full golden tests live under the integration harness
- each heavy family owns its primary fixture roots under the family crate, or uses an explicit shared fixture crate if we choose one later

Do not assume Cargo auto-discovers nested `tests/facts/` or `tests/integration/` by itself.

This is the target shape after the family crates become real.

## Transitional shape before full crate split

Before the full workspace split lands, the same layering must still be applied within the current monolith.

That means:

- do not keep pretending fixture-backed tests are unit tests
- move collector/facts tests under explicit collector test modules
- move family fan-out and ownership-split tests under orchestrator test modules
- move full golden tests under family integration modules
- keep only true rule-semantics tests next to rule code

Transitional target inside the current tree:

```text
apps/guardrail3/crates/app/rs/checks/rs/<family>/
  rs_<family>_01_*.rs
  tests/
    mod.rs
    testkit/
    collectors/
    orchestrator/
  tests/
    facts.rs
    orchestrator.rs
    integration.rs
    facts/
    orchestrator/
    integration/
```

This transitional layout is acceptable only until the family crates are real.

## Reclassification rules for the current corpus

When migrating existing tests, do not port by filename.
Reclassify every existing test into exactly one bucket:

1. rule
2. collectors / facts
3. family-orchestrator
4. family integration
5. runtime / product

Use this litmus test:

- if the test would still be meaningful after replacing the walker/fact builder with stubbed typed input, it is probably a rule test
- if the test is proving discovery/parsing/classification, it is a collector/facts test
- if the test is proving ownership split, fan-out, or collector-backed rule application, it is a family-orchestrator test
- if the test needs the whole golden tree and multiple rules/facts in play, it is a family integration test
- if the test shells out through CLI/report/config surfaces, it is a runtime / product test

## Hard rules

Do not:

- count discovery tests as rule coverage
- count orchestrator tests as collector coverage
- count integration tests as unit coverage
- duplicate the same behavior assertion in three layers unless the layer boundary itself is being checked
- keep 50 fixture-backed tests under a rule because the rule happens to consume a discovered file-presence fact

Do:

- keep core rule tests minimal and semantic
- give discovery/parsing their own explicit home
- give family fan-out and ownership-split behavior its own explicit home
- keep golden fixture attacks at the family integration layer
- keep rule assertion helpers crate-internal
- keep external integration helpers in `tests/support/`

## Coverage standard per rule

Every rule still owes the shared attack model from:

- `.plans/todo/check_review/test_hardening/00-shared-test-story.md`

But the layer matters.

Expected split:

- core rule tests:
  - minimal pass/fail contract
  - exact severity
  - local false-positive edge if the rule itself has one
- collector/facts tests:
  - discovery / parsing / classification edges
  - raw collector fail-closed capture where the collector owns it
- family-orchestrator tests:
  - rule ownership splits
  - shared-input fan-out
  - collector-backed rule application
  - family-level non-hit ownership boundaries
- integration tests:
  - golden
  - attack vectors
  - owned hit set
  - owned non-hit set
  - multi-root / nested-root where the family model needs it

Not every attack class belongs in the rule-unit layer.
Many belong in collectors, orchestrator, or integration.

## Success criteria

This plan is implemented correctly only when:

1. no family claims fixture-backed discovery tests as rule-unit coverage
2. every rule has a small core semantic test seam over typed inputs
3. every family has explicit collector/facts tests
4. every family has explicit orchestrator tests where the family has shared fan-out or ownership splits
5. every family has explicit golden integration tests
6. moving a fixture file through the walker is no longer the default way to test a trivial rule
7. root-level mixed test aggregators stop being the primary home of Rust rule behavior tests
8. the crate split can preserve this layering without inventing a new testing model later

## Ordered implementation

1. freeze the test taxonomy
- rule
- collectors / facts
- family-orchestrator
- family integration
- runtime / product

2. audit one Rust family at a time
- reclassify every current test
- mark misfiled tests

3. extract rule-local assertion helpers
- one helper per rule where useful
- helper consumes typed inputs / facts only

4. move discovery/parsing tests out of rule-sidecar ownership
- create family collector test homes

5. move ownership-split and fan-out tests into family orchestrator homes

6. move golden fixture tests into family integration homes
- keep the same attack-vector standard

7. make the new family crates preserve the same four-layer structure
- do not redesign tests again during the crate split

8. shrink the root harness
- delete or sharply reduce root aggregators that still mix rule/facts/integration behavior together

## First migration targets

Start with the worst offenders:

- `hexarch`
- `cargo`
- `garde`
- `code`

Why:

- these families currently blur discovery/facts/rules the most
- they also lean hardest on large fixture trees
- they are the most likely to fake “rule coverage” with integration-heavy tests

Then apply the same model to:

- `clippy`
- `deny`
- `release`
- `deps`
- `test`
- hooks families

## Non-goals

This plan does not:

- reduce the shared attack standard
- replace golden tests with only tiny inline cases
- move runtime/product CLI tests into family crates
- redefine rule ids or family ownership

It only corrects the layer boundaries so the planned crate architecture and the test corpus stop fighting each other.
