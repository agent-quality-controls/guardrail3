# Test Family Rewrite Agent Brief

This is the droppable handoff file for rewriting the live `rs/test` family to match the accepted target-state contract.

The contract is the family README:

- `apps/guardrail3/crates/app/rs/families/test/README.md`

Treat that README as the source of truth for:

- the target rule inventory
- the target numbering
- the activation model
- the target detection logic
- the target test architecture

Do not preserve the old 19-rule shape just because it already exists in code.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `apps/guardrail3/crates/app/rs/families/test/README.md`
6. `.plans/todo/checks/rs/test.md`
7. `.plans/todo/test_architecture.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/families/test/`

Important family files:

- `src/lib.rs`
- `src/discover.rs`
- `src/facts.rs`
- `src/inputs.rs`
- `src/parse.rs`
- `src/test_support.rs`

## Current State You Are Replacing

The live family is partially modernized but still wrong relative to the accepted contract.

What is already true:

- `rs/test` is a real family crate
- it already has shared family support files
- it already uses one production file per current rule id

What is still wrong:

- it still implements the old 19-rule inventory
- it still contains rules that the accepted README drops or moves out:
  - tests-exist
  - coverage inventory
  - integration-tests-exist
  - test-function-naming
  - cfg-test-module-naming
  - test-file-length
- it still uses flat `*_tests.rs` sidecars instead of rule-specific `*_tests/` directories
- its live detection logic still contains heuristics the README explicitly rejects

## Mission

Rewrite the live `rs/test` family to match the README exactly.

That means:

1. replace the old 19-rule family with the accepted 15-rule family
2. renumber and rename the production files accordingly
3. rewrite `src/lib.rs` orchestration to the new activation model
4. convert every rule to a rule-specific `*_tests/` directory
5. rewrite tests so they prove the accepted detection logic, not the old behavior
6. remove rules that are now owned elsewhere or dropped entirely

## Accepted Rule Inventory

The accepted live family is:

- `RS-TEST-01` inline test bodies in `src/` forbidden
- `RS-TEST-02` no ad hoc test-module sprawl; owned sidecar shape only
- `RS-TEST-03` `runtime/assertions` split and import boundaries enforced
- `RS-TEST-04` `#[ignore]` requires reason
- `RS-TEST-05` `#[should_panic]` requires `expected = "..."`
- `RS-TEST-06` no tautological literal-vs-literal assertions
- `RS-TEST-07` real proof site required
- `RS-TEST-08` weak wildcard `matches!` assertions forbidden
- `RS-TEST-09` nextest timeouts required for async-test crates
- `RS-TEST-10` fail closed on required test inputs
- `RS-TEST-11` mutation adoption requires `cargo-mutants` on `PATH`
- `RS-TEST-12` mutation adoption requires `.cargo/mutants.toml`
- `RS-TEST-13` mutation adoption requires `[profile.mutants]`
- `RS-TEST-14` mutation adoption requires mutation step in active hooks
- `RS-TEST-15` mutation config must be sane

## Explicitly Removed From This Family

These are not part of the accepted `rs/test` family anymore:

- tests-exist requirement
- coverage inventory / test-count ratio
- integration-tests-exist requirement
- test-function-naming descriptiveness
- cfg-test-module naming orthodoxy
- test-file-length rule

The test-file structural-pressure decision is:

- file length
- top-level `use` count

belong to `RS-CODE`, including for test files.

Do not keep duplicates in `RS-TEST`.

## Activation Contract

Judgment is per owned Rust crate/root.

### Test activation

If an owned crate has no tests, `RS-TEST` is inactive for that crate.

If an owned crate has tests, non-mutation `RS-TEST` rules activate for that crate.

Detectable test markers:

- test functions in Rust source
- sidecar harnesses under `src/*_tests/`
- external harnesses under `tests/*.rs`

### Mutation activation

Mutation rules activate only if any mutation-adoption marker exists:

- `.cargo/mutants.toml`
- `[profile.mutants]` in `Cargo.toml`
- active hook surfaces invoke `cargo mutants`

If none exist, mutation rules do nothing.

If any one exists, the full mutation setup must be present and sane.

## Exact Detection Contract

The README already defines the target detection logic.

You must implement that logic literally.

Do not keep or introduce heuristics the README rejects.

Especially:

- `RS-TEST-07` must not accept:
  - “function name contains assert/verify/expect”
  - “returns Result so it counts”
- `RS-TEST-02` and `RS-TEST-03` must be enforced by actual filesystem, manifest, and import-boundary checks
- `RS-TEST-14` must use executable-line matching on active hook surfaces, not raw substring checks

## Required Family Shape

Inside the family crate:

- one production file per `RS-TEST-*` rule
- one rule-specific `*_tests/` directory per rule
- `src/lib.rs` orchestrates only
- `discover.rs`, `facts.rs`, `inputs.rs`, `parse.rs`, and `test_support.rs` remain support files only

Forbidden:

- `*_tests.rs` flat sidecars
- grouped family test files
- grouped production files
- helper APIs that hide multiple rule predicates behind one interface

## Required Test Strategy

The rewrite is not just a renumbering pass.

Every surviving rule must be tested against the accepted contract:

1. golden pass
2. attack-vector test
3. exact owned hit set
4. exact owned non-hit set
5. false-positive control
6. fail-closed coverage where applicable
7. exact severity assertions

Highest-signal rule risks in this family:

- internal sidecar ownership drift
- assertions-boundary leakage
- external tests importing private glue
- `#[ignore]` and `#[should_panic]` attribute parsing edge cases
- weak proof-site detection in `RS-TEST-07`
- wildcard payload matching in `RS-TEST-08`
- timeout activation for async test crates only
- mutation partial-adoption states
- malformed Rust/TOML inputs failing open

## Exact Rewrite Tasks

1. Audit the current 19-rule implementation against the accepted 15-rule contract.
2. Delete or migrate the removed rules.
3. Renumber surviving rules to the accepted numbering.
4. Update `src/lib.rs` so orchestration follows the new activation model.
5. Rewrite facts and inputs only as needed to support the accepted detection contract.
6. Convert every rule test from `*_tests.rs` to `*_tests/`.
7. Replace heuristic tests with exact detection tests.
8. Update `.plans/todo/checks/rs/test.md` to reflect the accepted contract after the code rewrite lands.

## Legacy Seed Material

Use these only as seed material:

- `apps/guardrail3/crates/app/rs/families/test/src/*.rs`
- `apps/guardrail3/tests/unit/rs_test_checks_test.rs`
- `apps/guardrail3/tests/unit/rs_test_quality_checks_test.rs`
- older legacy validator files if needed for historical parsing edge cases

Do not preserve old semantics just because they already exist.

## Do Not

- do not keep the old 19-rule inventory
- do not keep inventory/reporting rules in this family
- do not keep test-file-length in this family
- do not rely on naming heuristics for proof-site detection
- do not use raw substring matching for mutation-hook detection
- do not leave tests as flat `*_tests.rs`
- do not silently broaden ownership to repo-root-only behavior

## Done Means

This pass is not done until:

- the live code matches the accepted README rule inventory exactly
- every surviving rule has its own production file
- every surviving rule has a rule-specific `*_tests/` directory
- the old removed rules are gone from live family code
- exact-result assertions replace loose presence checks
- the activation model is per-owned-crate and conditional
- mutation rules activate only from detectable mutation-adoption markers
- `.plans/todo/checks/rs/test.md` is updated to the new contract

## Suggested Start Order

1. diff the accepted README rule inventory against the current 19 live rules
2. rewrite the inventory and numbering on paper first
3. update `src/lib.rs` orchestration and shared facts/inputs for the new activation model
4. remove dropped rules and move test-length ownership out mentally to `RS-CODE`
5. implement the structural rules first:
   - `RS-TEST-01`
   - `RS-TEST-02`
   - `RS-TEST-03`
6. implement the proof/quality rules:
   - `RS-TEST-04` through `RS-TEST-08`
7. implement safety and mutation:
   - `RS-TEST-09` through `RS-TEST-15`
8. convert all tests to `*_tests/` directories before finishing
