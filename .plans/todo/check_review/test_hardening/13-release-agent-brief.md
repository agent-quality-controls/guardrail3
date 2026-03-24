# Release Agent Brief

You own the `rs/release` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/03-release.md`
6. `.plans/todo/checks/rs/release.md`

## Primary code

- `apps/guardrail3/crates/app/rs/checks/rs/release/`

## Old adversarial sources to mine

- `apps/guardrail3/tests/unit/test_release_checks.rs`
- `apps/guardrail3/tests/unit/test_release_repo_checks.rs`
- `apps/guardrail3/tests/unit/test_release_crate_checks.rs`
- `apps/guardrail3/tests/unit/test_release_crate_deps.rs`
- `apps/guardrail3/tests/unit/test_release_bin_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/release_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/release_repo_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/release_crate_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/release_crate_deps.rs`
- `apps/guardrail3/crates/app/rs/validate/release_bin_checks.rs`

## What you are trying to prove

The family should detect real release-wiring failures, not comments/prose that accidentally contain the right strings.

One test = one attack vector.

That vector should be applied across all relevant release surfaces:
- repo configs
- publishable crates
- dependent publishable crates
- binary workflow targets

## Known gaps already identified

- tests are still `*_tests.rs` instead of rule-specific `*_tests/` directories
- workflow rules still overclaim semantic strength while using broad string matching
- `readme = false` is still buggy
- `RS-PUB-10/11` still miss `workspace = true` inherited local path edges
- `RS-RELEASE-12` is only partially fail-closed
- some rule inputs are still too aggregate-heavy
- semantic `release-plz.toml` / `cliff.toml` baseline is still only partly real

## Current status

Closed in the current pass:

- workflow semantics were tightened enough to stop counting:
  - comments
  - prose/display text
  - `echo ...` fake command mentions
- `readme = false` is no longer treated as implicit fallback to default `README.md`
- inherited `workspace = true` path edges are now surfaced into release-edge facts for `RS-PUB-10/11`
- `RS-RELEASE-12` now has synthetic `ProjectTree` fail-closed coverage for malformed config content and partial-facts behavior
- `RS-RELEASE-12` now also fail-closes unreadable cached Cargo/release/workflow files instead of silently skipping them
- `RS-RELEASE-03` now enforces the canonical `release-plz.toml` workspace baseline
- `RS-RELEASE-04` now enforces the canonical `cliff.toml` git baseline and parser coverage
- `RS-RELEASE-01` now only inventories canonical root license filenames
- `RS-BIN-01` now rejects unrelated build/release jobs but accepts split build/publish jobs linked by `needs:`
- `RS-BIN-02` now follows the linked release path for Linux detection and also accepts Linux from matrix-driven `runs-on`
- `RS-PUB-13` now correctly recognizes nested `[package.metadata.docs.rs]` TOML
- `RS-PUB-02`, `RS-PUB-13`, `RS-PUB-14`, `RS-BIN-03`, and `RS-RELEASE-12` now have stronger family-level manifest/fixture coverage for the fixed semantics
- the binary helper layer has now been pushed further through strict rule-by-rule agent attack rounds:
  - `RS-BIN-01` respects crate targeting through `-p`, `--package=`, `--bin`, `--bin=`, and `--manifest-path`
  - `RS-BIN-01` no longer credits generic release builds to every binary crate in multi-binary repos
  - `RS-BIN-01` exact-matches real release actions instead of raw substrings, and accepts both `action-gh-release` and `release-action`
  - `RS-BIN-02` ignores echo/prose/`--target-dir` Linux noise, supports matrix `include`, and ties Linux-attribution to the current crate’s actual build step
  - `RS-BIN-03` now has collector-backed warn/no-emit coverage for autodiscovered and explicit-bin manifests, not just stubbed fact tests

Rules already migrated to rule-specific `*_tests/` directories:

- `RS-RELEASE-01`
- `RS-RELEASE-02`
- `RS-RELEASE-03`
- `RS-RELEASE-04`
- `RS-RELEASE-05`
- `RS-RELEASE-06`
- `RS-RELEASE-07`
- `RS-RELEASE-08`
- `RS-RELEASE-09`
- `RS-RELEASE-10`
- `RS-RELEASE-11`
- `RS-RELEASE-12`
- `RS-PUB-01`
- `RS-PUB-02`
- `RS-PUB-03`
- `RS-PUB-04`
- `RS-PUB-05`
- `RS-PUB-06`
- `RS-PUB-07`
- `RS-PUB-08`
- `RS-PUB-09`
- `RS-PUB-10`
- `RS-PUB-11`
- `RS-PUB-12`
- `RS-PUB-13`
- `RS-PUB-14`
- `RS-BIN-01`
- `RS-BIN-02`
- `RS-BIN-03`

## Fixture boundary

Current fixture reality:

- config-only synthetic trees are still used for parse/fail-closed and narrow manifest attacks
- `tests/fixtures/r_arch_01/golden` is now a richer runnable Rust fixture with real crate bodies

Safe to continue under that assumption:

- workflow YAML semantics
- repo config existence and coverage rules
- release-plz/cliff existence and malformed-config coverage
- publishability inference from manifests
- workspace dependency inheritance
- most inventory and metadata rules
- synthetic `ProjectTree` fail-closed tests

Already completed under that assumption:

- all config-only release-family rules

Already completed with the richer fixture:

- `RS-PUB-09` real `cargo publish --dry-run` hardening
- unreadable README permission-failure hardening for `RS-RELEASE-12`

Remaining open design limitation:

- workflow rules now consume preserved parsed workflow structure, but semantic matching still relies on release-family helpers rather than a fuller Actions execution model
- the strict four-agent adversarial loop has now converged across all `RS-RELEASE-*`, `RS-PUB-*`, and `RS-BIN-*` rules; new findings have stopped surfacing concrete checker bugs and are now mostly combinatorial test variants

Closed from the adversarial pass:

- shell-wrapped real `release-plz` execution is now recognized by `RS-RELEASE-05`
- shell-wrapped real `cargo publish --dry-run` execution is now recognized by `RS-RELEASE-06`
- `RS-RELEASE-07` no longer treats `CARGO_REGISTRY_TOKEN` on `release-pr`-only steps as sufficient
- workspace metadata inheritance from `[workspace.package]` is now honored for `RS-PUB-01/02/03/06/07` and `RS-RELEASE-11`
- `publish = []` now suppresses publishable-crate scope correctly, both directly and through `publish.workspace = true`
- `WorkflowFacts` now preserves parsed workflow structure, and `RS-RELEASE-05/06/07` plus `RS-BIN-01/02` now evaluate that structure directly instead of consuming pre-collapsed booleans

If a new session reaches one of those boundaries, it should stop and call that out explicitly instead of faking realism.

## Verification note

- `cargo check -p guardrail3 --lib` passes after the current release-family changes
- `cargo check -p guardrail3 --tests` now passes again
- `cargo test -p guardrail3 --lib --no-run` passes
- targeted release-family tests are green for `RS-RELEASE-01`, `RS-RELEASE-03`, `RS-RELEASE-04`, `RS-RELEASE-05`, `RS-RELEASE-06`, `RS-RELEASE-07`, `RS-PUB-02`, `RS-PUB-13`, `RS-PUB-14`, `RS-RELEASE-12`, `RS-BIN-01`, `RS-BIN-02`, and `RS-BIN-03`

## Required attack classes

- fake workflow hits via comments or prose
- missing real executable release step
- inherited path-edge attacks
- publishability inference bugs
- `readme = false`
- malformed release config / partial facts
- false positives for non-publishable crates

## Structural requirement

Every rule must end with a rule-specific `*_tests/` directory.

Do not leave `*_tests.rs` rule files in place.

## Done means

- every `RS-RELEASE-*`, `RS-PUB-*`, and `RS-BIN-*` rule has a `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- prose/comments cannot satisfy workflow rules in the hardened suite
- inherited path-edge cases are attacked directly
- `RS-PUB-09` uses real richer-fixture dry-run coverage instead of stub-only command outcomes

## Do not

- preserve old substring heuristics just because they existed before
- write tests that only prove “some release error exists”
- silently narrow publishability policy
