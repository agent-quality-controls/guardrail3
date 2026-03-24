# Deps Agent Brief

This is the droppable handoff file for the `rs/deps` hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/checks/rs/deps.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/checks/rs/deps/`

Important family files:
- `mod.rs`
- `facts.rs`
- `inputs.rs`
- `test_support.rs`

Rules:
- `rs_deps_01_cargo_deny_installed.rs`
- `rs_deps_02_cargo_machete_installed.rs`
- `rs_deps_03_cargo_dupes_installed.rs`
- `rs_deps_04_gitleaks_installed.rs`
- `rs_deps_05_dependencies_allowlisted.rs`
- `rs_deps_06_build_dependencies_allowlisted.rs`
- `rs_deps_07_dev_dependencies_allowlisted.rs`
- `rs_deps_08_library_allowlist_present.rs`
- `rs_deps_09_cargo_lock_present.rs`
- `rs_deps_10_gitignore_not_ignoring_cargo_lock.rs`
- `rs_deps_11_input_failures.rs`

## Legacy Seed Material

Relevant old seed:
- `apps/guardrail3/tests/unit/test_release_crate_deps.rs`

Use it as seed material only. Do not port it mechanically.

## Family Contract

`rs/deps` owns:
- required external Rust/tooling presence
- dependency allowlist enforcement for external crates
- allowlist coverage policy for library crates
- lockfile policy

It does not own:
- banned crates in the lockfile
- dependency direction
- release-specific dependency policy
- hook execution of tools

## Root / Crate Model

This family is not repo-root-only.

Important scope points already frozen in the plan and implementation:
- lockfile checks are per Rust root, not just repo root
- `.gitignore` checks are per Rust root and must consider relevant ancestor `.gitignore` files
- allowlist checks operate at the crate level
- `workspace = true` dependencies are not auto-skipped
- path dependencies are skipped only when they are workspace path dependencies
- renamed dependencies must be checked against real package name when present

## Fail-Closed Contract

Required inputs must surface explicitly through `RS-DEPS-11`.

That includes:
- member `Cargo.toml`
- workspace `Cargo.toml` when needed for `workspace = true` resolution
- `guardrail3.toml` when needed for profile / allowlist policy

Malformed inputs must not silently suppress dependency-policy findings.

## Known Plan-Level Additions

`RS-DEPS-12` is planned but not implemented yet:
- more than 25 unique direct dependency names on one crate

Do not invent new semantics for it during this pass unless you are explicitly implementing it.

## Main Mission

Harden `rs/deps` only.

Required outcomes:
- verify the family against the current plan
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- use exact owned hit/non-hit assertions
- fix real semantic bugs you find
- update `.plans/todo/checks/rs/deps.md` with:
  - closed gaps
  - remaining gaps
  - policy questions, if any

## Highest-Signal Attack Classes

1. tool presence exactness
- missing one required tool should only hit its own rule
- optional/recommended tool severity must stay exact

2. allowlist enforcement
- same attack vector should mutate all relevant dependency sections
- external crate vs workspace path dep separation
- `workspace = true` resolution
- renamed dependency handling
- exact crate-name ownership

3. library allowlist coverage
- library crates without allowlists should fire
- non-library crates should not overfire

4. lockfile policy
- multiple Rust roots at once
- missing lockfiles across roots
- nested `.gitignore` masking
- ancestor `.gitignore` precedence

5. fail-closed behavior
- malformed member manifest
- malformed workspace manifest for inherited dependency resolution
- malformed `guardrail3.toml`

## Required Attack Classes

Every rule should move toward:

1. golden pass
2. attack-vector test
3. exact owned hit set
4. exact owned non-hit set
5. multi-root coverage where applicable
6. inheritance / resolution coverage where applicable
7. false-positive control
8. fail-closed coverage where applicable
9. exact severity assertions

## Do Not

- add grouped family tests
- reduce exact dependency-name assertions to loose “rule appears” checks
- silently widen or narrow allowlist semantics to make tests pass
- mix `release`-family dependency rules into this family

## Done Means

The pass is not done until:

- every rule has a rule-specific `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- semantic bugs are fixed or written down explicitly
- `deps.md` reflects what changed and what remains

## Suggested Start Order

1. read `deps.md` and map all current rules to current files
2. inspect `facts.rs` first; most family mistakes here will be about dependency resolution and per-root lockfile ownership
3. map old seed tests into current attack vectors
4. verify the existing `*_tests/` directories actually match the rule/attack-vector standard
5. harden the highest-risk rules first:
   - `RS-DEPS-05`
   - `RS-DEPS-06`
   - `RS-DEPS-07`
   - `RS-DEPS-09`
   - `RS-DEPS-10`
   - `RS-DEPS-11`
