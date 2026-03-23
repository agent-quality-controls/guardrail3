# Clippy And Deny Agent Brief

You own the `rs/clippy` and `rs/deny` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md`
6. `.plans/todo/checks/rs/clippy.md`
7. `.plans/todo/checks/rs/deny.md`

## Primary code

- `apps/guardrail3/crates/app/rs/checks/rs/clippy/`
- `apps/guardrail3/crates/app/rs/checks/rs/deny/`
- `apps/guardrail3/crates/domain/modules/clippy/`
- `apps/guardrail3/crates/domain/modules/deny.rs`

## Old adversarial sources to mine

- `apps/guardrail3/tests/adversarial_config_tests.rs`
- `apps/guardrail3/tests/fixtures/adversarial-configs/`
- `apps/guardrail3/crates/app/rs/validate/clippy_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/clippy_coverage.rs`
- old deny validator files under `apps/guardrail3/crates/app/rs/validate/`
- `apps/guardrail3/tests/unit/deny_inventory_test.rs`
- `apps/guardrail3/tests/adversarial_generate.rs`

## What you are trying to prove

These families should not drift silently from the canonical generator modules, and config-policy edge cases should not slip through.

One test = one attack vector.

That vector should be applied across all relevant policy roots / config roots / profile mixes.

## Current state

This brief is no longer greenfield.

Already completed in this lane:
- a first-pass coverage ledger exists at `14-clippy-deny-coverage-matrix.md`
- a working execution order exists at `14-clippy-deny-execution-plan.md`
- support-layer handwritten canonical fixtures were removed as silent baselines:
  - clippy test support now builds from `domain/modules/clippy::build_clippy_toml(...)`
  - deny test support now builds from `domain/modules/deny::build_deny_toml(...)`
- clippy and deny test support can now copy and mutate the real `tests/fixtures/r_arch_01/golden/` multi-root scaffold and run the family checker end-to-end
- support-layer parity tests now exist for clippy and deny
- the explicit deny baseline delta is now documented and tested:
  - checker still expects audited `lazy_static`
  - generator service baseline does not currently emit it

Already migrated to rule-specific `*_tests/` directories:
- all `RS-CLIPPY-*` rules (`01` through `22`)
- `RS-CLIPPY-12`
- `RS-CLIPPY-13`
- `RS-CLIPPY-14`
- `RS-CLIPPY-19`
- `RS-CLIPPY-01`
- `RS-CLIPPY-02`
- `RS-CLIPPY-03`
- `RS-CLIPPY-04`
- `RS-CLIPPY-05`
- `RS-CLIPPY-06`
- `RS-CLIPPY-07`
- `RS-CLIPPY-08`
- `RS-CLIPPY-09`
- `RS-CLIPPY-10`
- `RS-CLIPPY-11`
- `RS-CLIPPY-15`
- `RS-CLIPPY-16`
- `RS-CLIPPY-17`
- `RS-CLIPPY-18`
- `RS-CLIPPY-20`
- `RS-CLIPPY-21`
- `RS-CLIPPY-22`
- `RS-DENY-02`
- `RS-DENY-03`
- `RS-DENY-01`
- `RS-DENY-04`
- `RS-DENY-05`
- `RS-DENY-06`
- `RS-DENY-07`
- `RS-DENY-08`
- `RS-DENY-10`
- `RS-DENY-11`
- `RS-DENY-12`
- `RS-DENY-13`
- `RS-DENY-15`
- `RS-DENY-16`
- `RS-DENY-17`
- `RS-DENY-09`
- `RS-DENY-14`
- `RS-DENY-18`
- `RS-DENY-20`
- `RS-DENY-19`
- `RS-DENY-23`
- `RS-DENY-24`
- `RS-DENY-28`
- `RS-DENY-29`
- `RS-DENY-30`

What those migrations now prove in business terms:
- clippy placement and local-policy-root rules no longer test only one toy case; they now cover filename variants, same-root precedence, parse failure, and exact missing managed sections
- clippy library global-state behavior now distinguishes library vs non-library scope and checks the real generated library baseline
- clippy unknown-key behavior now tests typo-like managed drift without overfiring on unrelated custom keys
- clippy coverage now uses the real multi-root fixture and proves:
  - a validation-root `clippy.toml` covers every Rust workspace root and standalone package root in the scaffold
  - a nearer allowed local config only takes ownership of its own root instead of spilling to siblings or ancestors
  - uncovered roots error exactly on the roots that lack an allowed covering config
- clippy threshold coverage now proves for `RS-CLIPPY-02` and `RS-CLIPPY-03`:
  - generated threshold baselines inventory cleanly
  - wrong values error
  - missing values error
  - malformed `clippy.toml` still errors through the rule-local parse-error branch
- clippy threshold coverage now also proves for `RS-CLIPPY-09`, `10`, `11`, `21`, and `22`:
  - generated threshold baselines inventory cleanly
  - wrong values error
  - missing values error
  - malformed `clippy.toml` still errors through the rule-local parse-error branch
- clippy missing-method-ban coverage now proves:
  - the generated service baseline inventories all managed method bans
  - garde-owned deserialization bans stop being required when `garde = false`
  - multiple removed required method bans each produce their own hard error
- clippy missing-type-ban coverage now proves:
  - the generated service baseline inventories all managed service type bans
  - garde-owned extractor bans stop being required when `garde = false`
  - library profile expands the required type-ban set to include global-state types
  - multiple removed required type bans each produce their own hard error
- clippy extra-method-ban coverage now proves:
  - the generated service baseline does not false-positive on managed method bans
  - project-specific extra method bans inventory cleanly
  - when `garde = false`, garde-owned method bans become project-specific extras instead of silently staying managed
- clippy extra-type-ban coverage now proves:
  - the generated service baseline does not false-positive on managed service type bans
  - project-specific extra type bans inventory cleanly
  - when `garde = false`, garde-owned extractor bans become project-specific extras
  - library-profile global-state type bans are treated as managed, not extra
- clippy reason-quality coverage now proves:
  - generated ban entries with table-format reasons stay quiet
  - plain-string entries and missing-reason table entries warn across methods, types, and macros
- clippy trivial-reason coverage now proves:
  - generated substantive reasons stay quiet
  - placeholder or trivial reasons warn across methods, types, and macros
- clippy macro-ban coverage now proves:
  - the generated macro baseline inventories every required macro ban
  - each missing required macro ban errors independently
- clippy `avoid-breaking-exported-api` coverage now proves:
  - generated explicit `false` inventories cleanly
  - `true` warns for non-published roots
  - `true` inventories for published library packages
  - missing value warns
- clippy test-relaxation coverage now proves:
  - generated baseline stays quiet
  - each enabled relaxation key warns independently
- clippy duplicate-ban coverage now proves:
  - generated baseline stays quiet
  - duplicates warn once per duplicated path per section
- clippy is now structurally migrated:
  - every `RS-CLIPPY-*` rule uses a rule-specific `*_tests/` directory
  - remaining clippy work is parity tightening and broader multi-root/profile attacks, not more sidecar conversion
- deny placement/shadowing now tests all accepted deny filename variants
- deny coverage now uses the real multi-root fixture and proves:
  - deny coverage is resolved over the actual validation root, workspace roots, and standalone package root in the scaffold
  - a nearer allowed deny config replaces ancestor coverage only for that owned root
  - malformed allowed deny configs still produce the explicit parse-error branch
  - uncovered effective roots error exactly where no allowed deny config applies
- deny canonical-ban coverage now proves:
  - the generated service and library deny baselines pass cleanly
  - library profile expands the canonical ban set to include library-IO bans
  - each missing canonical ban errors individually, including the currently audited `lazy_static` delta
  - missing `[bans]` or missing `[bans].deny` fails closed
  - weakening canonical managed wrappers still errors in this rule as a baseline-integrity failure
- deny deprecated-advisories coverage now proves:
  - the generated advisories baseline stays quiet
  - each deprecated advisory key (`vulnerability`, `notice`, `unsound`) warns independently
- deny advisories-baseline coverage now proves:
  - the generated advisories baseline stays quiet
  - missing `[advisories]` fails closed
  - missing baseline values error independently
  - weakened `unmaintained` and `yanked` values error independently
- deny stricter-advisories coverage now proves:
  - the generated advisories baseline does not over-inventory
  - `unmaintained = "deny"` and `yanked = "deny"` each inventory independently as stricter-than-baseline policy
- deny graph baseline coverage now proves:
  - the generated graph baseline stays quiet
  - missing `[graph]` fails closed
  - missing or weakened `all-features` errors
  - missing or weakened `no-default-features` errors
- deny multiple-versions floor coverage now proves:
  - the generated baseline stays quiet
  - missing `[bans]` warns
  - missing `multiple-versions` warns
  - weakened `multiple-versions` warns
- deny highlight inventory coverage now proves:
  - the generated baseline stays quiet
  - missing `highlight` inventories
  - project-specific `highlight` values inventory
- deny wildcard-path policy coverage now proves:
  - the generated baseline stays quiet
  - missing `[bans]` fails closed
  - missing or weakened `allow-wildcard-paths` errors
- deny wildcards inventory coverage now proves:
  - the generated baseline stays quiet
  - missing `wildcards` warns
  - project-specific `wildcards` values warn
- deny confidence-threshold coverage now proves:
  - the generated baseline stays quiet
  - weaker threshold values warn
  - stricter threshold values inventory
  - missing or invalid threshold values warn
- deny copyleft-allowlist coverage now proves:
  - the generated license allow-list stays quiet
  - each copyleft license added to the allow-list warns independently
- deny license-exceptions coverage now proves:
  - the generated baseline stays quiet when no exceptions exist
  - each named or crate-keyed exception entry inventories independently
- deny license baseline coverage now proves:
  - the generated license baseline passes cleanly
  - each missing baseline allowed license errors individually
  - missing `[licenses]` fails closed
  - `[licenses.private].ignore` must remain exactly `true`
- deny unknown-sources coverage now proves:
  - the generated unknown-source policy passes cleanly
  - missing `[sources]` fails closed
  - weakening either `unknown-registry` or `unknown-git` errors independently
- deny allow-git coverage now proves:
  - empty `allow-git` stays quiet
  - non-empty `allow-git` warns once per config
  - each git source is still inventoried individually
- deny sources policy now tests both accepted crates.io forms and distinct failure branches for missing crates.io, missing `[sources]`, and unexpected registries
- deny skip/ignore hygiene now tests malformed shape, missing reason, non-string reason, and valid inventory shapes
- deny unknown-key coverage now includes nested skip/ignore/exception/feature entries
- deny wrapper policy now distinguishes:
  - managed non-empty canonical wrapper drift = error
  - project-specific wrapper additions where no managed wrapper policy exists = inventory only

## Important fixture note

Do not treat the family fixture as a single-root toy.

The real fixture context is broader:
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/` is a real multi-root scaffold
- the hardening packet already expects broad golden-fixture mutation across relevant roots

Implication for the next session:
- stop defaulting to only tiny synthetic helper trees for root/profile-sensitive rules
- use the real multi-root fixture for the remaining coverage, placement, profile-resolution, and broad policy attacks whenever the rule semantics depend on cross-root ownership
- keep tiny helper-built tests only for isolated local branches where that is genuinely clearer

## Verification blockers currently present

These are repo-wide and not caused by this lane, but they block normal cargo verification:
- `cargo test -p guardrail3 --lib rs_clippy_01_coverage` is currently blocked before reaching this lane by unrelated unresolved imports in:
  - `apps/guardrail3/crates/app/rs/checks/rs/hexarch/test_support.rs` (`find_edge` dead code)
  - `apps/guardrail3/crates/app/rs/checks/rs/release/facts.rs` (`cliff_parsed` dead code)
  - `apps/guardrail3/crates/app/rs/checks/rs/release/test_support.rs` (`workflow`, `failure`, `failure_input` dead code)

## Known gaps already identified

### Clippy
- generator/checker still disagree on some global-state bans
- no direct generator-vs-checker parity test
- `RS-CLIPPY-19` is intentionally temporary and must be tested honestly, not overclaimed

### Deny
- generation still uses wrong effective-profile logic in mixed setups
- no strong generator-vs-checker parity test
- canonical deny fixture has drifted before
- `RS-DENY-19` still needs explicit policy resolution if not already settled during the pass

## Required attack classes

- policy-root placement and shadowing
- same-root precedence conflicts
- mixed profile/layer cases
- generator/checker drift
- malformed exceptions/skips/ignores/wrappers
- severity exactness on inventory vs hard errors
- temporary-heuristic behavior for `RS-CLIPPY-19`

## Structural requirement

Every rule must end with a rule-specific `*_tests/` directory.

Do not leave `*_tests.rs` rule files in place.

This is complete for clippy and still in progress for deny. The remaining untouched deny rules still need migration.

## Done means

- every `RS-CLIPPY-*` and `RS-DENY-*` rule has a `*_tests/` directory
- parity tests exist against the generator baseline
- drift-prone hardcoded fixtures are removed or parity-checked
- every touched rule has golden + attack-vector coverage

## Do not

- let the checker and generator keep separate silent baselines
- write tests that only look for broad family output
- hide policy decisions inside test fixtures

## Next session start here

1. Read:
   - `14-clippy-deny-coverage-matrix.md`
   - `14-clippy-deny-execution-plan.md`
   - `04-clippy-and-deny.md`

2. Continue from the remaining unmigrated rules, but pivot the attack model:
   - prefer the actual multi-root family fixture for remaining root/profile-sensitive rules
   - use synthetic helper trees only for isolated local branches

3. Highest-value next targets:
   - deny remaining still-flat rules:
     `RS-DENY-21`, `22`, `25`, `26`, `27`
   - then tighten parity and broader multi-root attack coverage for already-migrated rules where the matrix still calls out semantic gaps

Reason:
- clippy is structurally migrated now; the remaining structural debt is concentrated in deny
