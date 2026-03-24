# Test Agent Brief

This is the droppable handoff file for the `rs/test` hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/checks/rs/test.md`
6. `.plans/todo/checks/hooks/shared.md`
7. `.plans/todo/checks/hooks/rs.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/checks/rs/test/`

Important family files:
- `mod.rs`
- `discover.rs`
- `facts.rs`
- `inputs.rs`
- `parse.rs`
- `test_support.rs`

Rules:
- `rs_test_01_cargo_mutants_installed.rs`
- `rs_test_02_mutants_toml_exists.rs`
- `rs_test_03_mutants_profile_present.rs`
- `rs_test_04_tests_exist.rs`
- `rs_test_05_test_coverage_inventory.rs`
- `rs_test_06_integration_tests_exist.rs`
- `rs_test_07_ignore_without_reason.rs`
- `rs_test_08_mutation_hook_present.rs`
- `rs_test_09_no_inline_tests_in_src.rs`
- `rs_test_10_test_function_naming.rs`
- `rs_test_11_cfg_test_module_naming.rs`
- `rs_test_12_nextest_timeouts_present.rs`
- `rs_test_13_should_panic_expected.rs`
- `rs_test_14_tautological_assertions.rs`
- `rs_test_15_test_without_assertions.rs`
- `rs_test_16_test_file_length.rs`
- `rs_test_17_weak_matches_assert.rs`
- `rs_test_18_mutants_config_content.rs`
- `rs_test_19_input_failures.rs`

## Legacy Seed Material

Use these as seed material only:

- `apps/guardrail3/tests/unit/rs_test_checks_test.rs`
- `apps/guardrail3/tests/unit/rs_test_quality_checks_test.rs`
- `apps/guardrail3/crates/app/rs/validate/test_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/test_quality_checks.rs`

Do not port old line-based heuristics mechanically.

## Family Contract

`rs/test` is a multi-root family.

Owned roots:
- workspace roots
- standalone package roots that are not members of a workspace

Per owned root, the family determines:
- Cargo test-related config facts
- root-local `.cargo/mutants.toml`
- root-local `.config/nextest.toml`
- Rust source/test files that belong to that root

Validation-root artifacts also matter:
- validation-root `guardrail3.toml`
- active shared/Rust hook surfaces for mutation-hook presence

Important split:
- `RS-TEST-08` owns whether the active hook surfaces contain the mutation-testing step
- generic hook structure is owned by the hook families

## Fail-Closed Contract

Malformed required inputs must surface through `RS-TEST-19`.

That includes:
- owned-root `Cargo.toml`
- owned-root `.cargo/mutants.toml`
- owned-root `.config/nextest.toml`
- relevant Rust source files
- validation-root `guardrail3.toml` when policy resolution needs it

Malformed inputs must not silently suppress test-policy findings.

## Known Live Gaps

The family is implemented but still structurally behind the test standard.

Highest-signal remaining gaps:
- every rule still uses `*_tests.rs`, not `*_tests/`
- some rules still rely on token-text heuristics:
  - `RS-TEST-13`
  - `RS-TEST-15`
- `RS-TEST-08` already reuses the shared parser, but still needs broader adversarial coverage
- multi-root ownership and validation-root policy split need harder exact-result testing

## Current Test Shape

The family still uses:
- `rs_test_*_tests.rs`

This pass should move it to:
- one rule-specific `*_tests/` directory per rule
- one test file per attack vector

## Required Attack Classes

Every rule should move toward:

1. golden pass
2. attack-vector test
3. exact owned hit set
4. exact owned non-hit set
5. multi-root coverage where applicable
6. inheritance / policy-root coverage where applicable
7. false-positive control
8. fail-closed coverage where applicable
9. exact severity assertions

Highest-signal attacks for this family:
- owned-root discovery and non-owned non-hit control
- hook-presence semantics via active shared/Rust hook surfaces
- malformed Cargo / mutants / nextest / Rust source inputs
- heuristic false positives in test naming and assertion rules
- weak `matches!` payload assertions
- inline `#[cfg(test)] mod tests { ... }` in `src/`

## Mission

Harden `rs/test` only.

Required outcomes:
- verify multi-root ownership against `test.md`
- convert every rule to a rule-specific `*_tests/` directory
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- use exact owned hit/non-hit assertions
- fix real semantic bugs you find
- update `.plans/todo/checks/rs/test.md` with:
  - gaps closed
  - gaps remaining
  - policy questions, if any

## Do Not

- re-own generic hook structure inside `RS-TEST-08`
- leave tests as `*_tests.rs`
- keep loose “some finding exists” assertions
- widen token-text heuristics without documenting them
- silently relax ownership to repo-root-only behavior

## Done Means

The pass is not done until:

- every test-family rule has a rule-specific `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- semantic bugs are fixed or written down explicitly
- `test.md` reflects what changed and what remains

## Suggested Start Order

1. read `test.md` and map all 19 rules to current files
2. audit `discover.rs` / `facts.rs` / `inputs.rs` / `parse.rs` for owned-root and validation-root splits
3. convert all `rs_test_*_tests.rs` files to `*_tests/`
4. harden the highest-risk rules first:
   - `RS-TEST-08`
   - `RS-TEST-09`
   - `RS-TEST-13`
   - `RS-TEST-15`
   - `RS-TEST-17`
   - `RS-TEST-19`
5. finish fail-closed and exact-severity coverage before stopping
