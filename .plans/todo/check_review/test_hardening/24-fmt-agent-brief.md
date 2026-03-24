# Fmt Agent Brief

This is the droppable handoff file for the `rs/fmt` hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/checks/rs/fmt.md`
6. `.plans/todo/checks/rs/toolchain.md`
7. `.plans/todo/checks/rs/cargo.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/checks/rs/fmt/`

Important family files:
- `mod.rs`
- `facts.rs`
- `inputs.rs`

Rules:
- `rs_fmt_01_exists.rs`
- `rs_fmt_02_settings.rs`
- `rs_fmt_03_extra_settings.rs`
- `rs_fmt_04_nightly_keys_on_stable.rs`
- `rs_fmt_05_per_crate_override.rs`
- `rs_fmt_06_edition_mismatch.rs`
- `rs_fmt_07_ignore_escape_hatch.rs`
- `rs_fmt_08_dual_file_conflict.rs`

## Legacy Seed Material

Use these as seed material only:

- `apps/guardrail3/crates/app/rs/validate/config_files.rs`
- `apps/guardrail3/crates/app/rs/validate/rustfmt_check.rs`
- `apps/guardrail3/tests/adversarial_generate.rs`

Do not port old root-only helper behavior mechanically.

## Family Contract

`rs/fmt` is intentionally a repository-root family.

It owns:
- the one effective root formatting contract
- nested rustfmt override detection
- stable/nightly rustfmt policy interactions
- root Cargo edition vs rustfmt edition consistency

It does not own:
- allowed local formatting roots
- per-workspace/per-package formatting policy

Nested `rustfmt.toml` / `.rustfmt.toml` files are escape hatches, not legitimate second policy roots.

## Fail-Closed Contract

Required inputs include:
- root rustfmt config
- root `Cargo.toml` when edition comparison is needed
- root `rust-toolchain.toml` when stable/nightly interaction is needed

Malformed required inputs must not silently suppress findings.

Ownership split:
- `RS-FMT-02` owns parse failures of the root rustfmt config
- `RS-FMT-04` and `RS-FMT-06` own the secondary-file failures they need

## Known Live Gaps

The family is implemented but still structurally behind the test standard.

Highest-signal remaining gaps:
- every rule still uses `*_tests.rs`, not `*_tests/`
- root-vs-nested ownership needs harder exact-result testing
- root dual-file ambiguity and nested override interactions need broader attack coverage
- parse-failure ownership must stay rule-local and exact

## Current Test Shape

The family still uses:
- `rs_fmt_*_tests.rs`

This pass should move it to:
- one rule-specific `*_tests/` directory per rule
- one test file per attack vector

## Required Attack Classes

Every rule should move toward:

1. golden pass
2. attack-vector test
3. exact owned hit set
4. exact owned non-hit set
5. false-positive control
6. fail-closed coverage where applicable
7. exact severity assertions

Highest-signal attacks for this family:
- missing root config
- wrong/missing owned keys in one broad mutation
- extra settings that should inventory without colliding with owned-key failures
- nightly-only keys on stable
- nested local override files across multiple roots
- edition mismatch with root Cargo policy
- `ignore` escape hatch
- same-level `rustfmt.toml` and `.rustfmt.toml` conflict

## Mission

Harden `rs/fmt` only.

Required outcomes:
- verify root-only ownership against `fmt.md`
- convert every rule to a rule-specific `*_tests/` directory
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- use exact owned hit/non-hit assertions
- fix real semantic bugs you find
- update `.plans/todo/checks/rs/fmt.md` with:
  - gaps closed
  - gaps remaining
  - policy questions, if any

## Do Not

- reinterpret the family as multi-root policy ownership
- collapse parse failures into generic no-finding behavior
- leave tests as `*_tests.rs`
- reduce assertions to loose “rule appeared” checks

## Done Means

The pass is not done until:

- every fmt rule has a rule-specific `*_tests/` directory
- every fmt rule has golden coverage
- every fmt rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- semantic bugs are fixed or written down explicitly
- `fmt.md` reflects what changed and what remains

## Suggested Start Order

1. read `fmt.md` and map all 8 rules to current files
2. audit `mod.rs` / `facts.rs` / `inputs.rs` for root-only ownership
3. convert all `rs_fmt_*_tests.rs` files to `*_tests/`
4. harden the highest-risk rules first:
   - `RS-FMT-02`
   - `RS-FMT-04`
   - `RS-FMT-05`
   - `RS-FMT-06`
   - `RS-FMT-08`
5. finish parse-failure and exact-severity coverage before stopping
