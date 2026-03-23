# Hexarch Agent Brief

You own the `rs/hexarch` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/01-hexarch.md`
6. `.plans/todo/checks/rs/hexarch.md`

## Primary code

- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/`

## Current status

Already done in this lane:
- coverage matrix and old-to-new rule mapping live in `01-hexarch.md`
- exhaustive execution order lives in `16-hexarch-execution-plan.md`
- `RS-HEXARCH-15` no longer fails open on malformed `guardrail3.toml`
- `RS-HEXARCH-22/23` no longer fail open on unreadable/unparsable Rust source
- source collection for `RS-HEXARCH-22/23` now descends into inline `mod { ... }` blocks, so hidden inline traits/impls no longer evade source-based checks
- `RS-HEXARCH-01..25` now use rule-specific `*_tests/` directories
- `RS-HEXARCH-01..25` now have at least one stronger exact-hit or graph-shaped attack test instead of only loose “some result exists” checks
- `RS-HEXARCH-21` now proves dev-deps stay out of scope, build-deps stay in scope, inherited workspace externals are checked, and pure domain/ports path deps stay clean
- `RS-HEXARCH-22` now proves balanced counts, DTO-only ports crates, non-ports crates, private-trait ports crates, and multi-file aggregation
- `RS-HEXARCH-23` now proves pristine golden non-hit, non-adapter non-hit, `pub(crate)` non-hit, nested-file hit, inline-module hit, and fail-closed parse errors
- `RS-HEXARCH-15` now proves golden non-hit, single-app omission, all-app omission, non-app non-hit, and parse-error fail-closed ownership
- `RS-HEXARCH-19` now proves one-hit same-layer cycle exactness, mixed-layer non-hit, exact result shape, and the collector no longer misreports cycles containing an unlayered member as same-layer
- `RS-HEXARCH-24` now proves cross-app leaks across normal/dev/build/target dependency sections, plus golden and `packages/` non-hits
- `RS-HEXARCH-25` now proves forbidden target edges across `target.dependencies`, `target.dev-dependencies`, and `target.build-dependencies`, plus golden non-hit
- `RS-HEXARCH-01..03` now cover old replacement-shaped attacks, not just directory removal
- `RS-HEXARCH-02` now sees top-level files under owned `crates/` roots, so stray root files can no longer evade the exact-contents rule
- `RS-HEXARCH-04..06` now have much deeper old-corpus coverage around file replacement, `.gitkeep` boundaries, nested ownership, and valid-vs-invalid placeholder variants

Current active tranche:
- breadth backfill across `RS-HEXARCH-01..25`

Next tranche after that:
- severity-exactness and remaining false-positive/fail-closed backfill where still missing

## Old adversarial sources to mine

- `apps/guardrail3/tests/unit/test_hex_arch_checks.rs`
- `apps/guardrail3/tests/unit/rs_arch_01/`
- `apps/guardrail3/tests/fixtures/r_arch_01/`

## What you are trying to prove

The family should survive broad structural attacks against:
- all Rust app hex roots
- nested hex roots
- workspace-member coverage
- dependency-direction rules
- boundary config bypasses

One test = one attack vector.

That test should mutate the golden fixture everywhere the vector should matter:
- all matching top-level hex roots
- all matching nested hex roots
- all matching workspace members or dependency edges

## Known gaps already identified

- many rules are still `*_tests.rs` instead of rule-specific `*_tests/` directories
- migrated test depth is weaker than the old deliberate corpus
- `RS-HEXARCH-01..03` improved exact hit-set coverage but still do not match old-corpus breadth
- `RS-HEXARCH-13..25` still need deeper family-level breadth even after the first graph-shaped rewrite
- need direct proof that nested and top-level roots are attacked together
- `crates/macros/` is optional and must be allowed without weakening the rest of the structure
- the golden fixture should be treated as structure/config baseline; source-rule tests must explicitly call out when they depend on real Rust source content
- `dependency_facts` still classifies internal layers from path segments, so out-of-tree paths with names like `domain`, `ports`, `app`, or `adapters` remain a false-positive risk until that classification is hardened
- targeted Cargo verification is still blocked by crate-wide `-D dead-code` noise, including existing unused helpers in hexarch/release test support
- structural `RS-HEXARCH-01..06` still lag the old corpus on some mixed-combination attacks and severity-exactness, but the main replacement-shaped gaps are now covered

## Required attack classes

### Structural roots
- golden
- missing required dirs across all owned hex roots
- illegal extra sibling across all owned hex roots
- nested root parity
- optional `macros/`
- false positives against non-owned or non-Rust roots

### Workspace coverage
- missing members everywhere
- extra members everywhere
- out-of-boundary members
- malformed Cargo.toml fail-closed

### Dependency / boundary
- illegal direction edges across all matching members
- renamed dependency bypass
- inherited workspace dependency bypass
- target/dev edge variants
- cross-app leaks
- malformed boundary config fail-closed

## Structural requirement

Every rule must end with a rule-specific `*_tests/` directory.

Do not leave `*_tests.rs` rule files in place.

## Done means

- every `RS-HEXARCH-*` rule has a `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact owned hit/non-hit assertions are used
- semantic bugs found during hardening are fixed or written into the lane file

## Resume here

If starting fresh in a new session:
1. read `01-hexarch.md` for the current coverage matrix
2. read `16-hexarch-execution-plan.md` for execution order
3. continue with breadth backfill, starting from the weakest rows in `01-hexarch.md`; immediate next tranche is source/dependency depth, starting with `RS-HEXARCH-22/23` and then policy breadth in `RS-HEXARCH-20/21`
4. update both `01-hexarch.md` and this brief after each completed rule group

## Do not

- port old tests mechanically
- keep grouped or loose “some result exists” assertions
- change rule policy silently just to satisfy a test
