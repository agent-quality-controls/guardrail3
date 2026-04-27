# Clippy And Deny Agent Brief

> Historical hardening brief. Use:
> - [`.plans/todo/checks/rs/clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md)
> - [`.plans/todo/checks/rs/deny.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/deny.md)
> as the current source of truth before acting on details here.

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
- deny generator/checker drift on canonical bans has been reduced and then resolved:
  - a real generator bug was fixed so `build_deny_toml("library", ...)` now emits the library-profile ban set instead of the service set
  - `lazy_static` is now emitted by the generator as part of the canonical deny baseline
  - `RS-DENY-09` now has exact service/library parity instead of an audited delta
- architectural verification corrected one earlier bad assumption and fixed the real issue:
  - in `tests/fixtures/r_arch_01/golden/`, `packages/shared-types` is a workspace member, not a standalone policy root
  - the actual bug was profile resolution, not standalone-root classification in that fixture
  - `clippy/facts.rs` and `deny/facts.rs` now apply `[rust.packages]` to the root config when the root config is package-driven
  - `adapters/inbound/cli/generate_helpers.rs` now generates per-app `deny.toml` from the app's effective profile instead of the outer default profile
  - direct regression tests now pin both facts-layer root-profile resolution and per-app deny generation

Already migrated to rule-specific `*_tests/` directories:
- all `RS-CLIPPY-*` rules (`01` through `22`)
- all `RS-DENY-*` rules (`01` through `30`)
- `RS-CLIPPY-12`
- `g3rs-clippy/local-policy-root`
- `RS-CLIPPY-14`
- `RS-CLIPPY-19`
- `RS-CLIPPY-01`
- `g3rs-clippy/max-struct-bools`
- `g3rs-clippy/max-fn-params-bools`
- `RS-CLIPPY-04`
- `RS-CLIPPY-05`
- `g3rs-clippy/package-native-policy`
- `RS-CLIPPY-07`
- `RS-CLIPPY-08`
- `g3rs-clippy/type-complexity-threshold`
- `g3rs-clippy/missing-method-ban`
- `g3rs-clippy/missing-type-ban`
- `g3rs-clippy/no-op-placeholder`
- `RS-CLIPPY-16`
- `g3rs-clippy/avoid-breaking-exported-api`
- `RS-CLIPPY-18`
- `RS-CLIPPY-20`
- `g3rs-clippy/policy-context-parseable`
- `g3rs-clippy/forbid-clippy-conf-dir-override`
- `RS-DENY-02`
- `RS-DENY-03`
- `RS-DENY-01`
- `g3rs-deny/deprecated-advisories`
- `g3rs-deny/advisories-baseline`
- `g3rs-deny/stricter-advisories-inventory`
- `g3rs-deny/graph-all-features`
- `g3rs-deny/graph-no-default-features`
- `g3rs-deny/highlight-inventory`
- `g3rs-deny/allow-wildcard-paths`
- `g3rs-deny/wildcards-inventory`
- `g3rs-deny/license-allow-baseline`
- `g3rs-deny/copyleft-allowlist`
- `g3rs-deny/unknown-sources-policy`
- `RS-DENY-17`
- `g3rs-deny/skip-hygiene`
- `g3rs-deny/ignore-hygiene`
- `RS-DENY-25`
- `RS-DENY-26`
- `g3rs-deny/license-exceptions-inventory`
- `RS-DENY-09`
- `g3rs-deny/confidence-threshold`
- `g3rs-deny/allow-git-inventory`
- `g3rs-deny/extra-feature-bans-inventory`
- `g3rs-deny/tokio-full-ban`
- `g3rs-deny/duplicate-entries`
- `g3rs-deny/unknown-keys`
- `g3rs-deny/allow-override-channel`
- `g3rs-deny/extra-deny-bans-inventory`
- `RS-DENY-30`

What those migrations now prove in business terms:
- clippy placement and local-policy-root rules no longer test only one toy case; they now cover filename variants, same-root precedence, parse failure, and exact missing managed sections
  - allowed-root controls now prove validation roots, workspace roots, and true standalone package roots stay out of forbidden-placement results
  - local-policy-root coverage now also proves the validation-root config itself is not treated as a local replacement root
- clippy library global-state behavior now distinguishes library vs non-library scope and checks the real generated library baseline
- clippy unknown-key behavior now tests typo-like managed drift without overfiring on unrelated custom keys
- clippy coverage now uses the real multi-root fixture and proves:
  - a validation-root `clippy.toml` covers every Rust workspace root in the shared scaffold
  - a nearer allowed local config only takes ownership of its own root instead of spilling to siblings or ancestors
  - uncovered roots error exactly on the roots that lack an allowed covering config
  - non-Rust roots in the shared multi-root fixture stay out of clippy coverage results
- clippy threshold coverage now proves for `g3rs-clippy/max-struct-bools` and `g3rs-clippy/max-fn-params-bools`:
  - generated threshold baselines inventory cleanly
  - wrong values error
  - missing values error
  - malformed `clippy.toml` still errors through the rule-local parse-error branch
  - the same managed threshold baseline is now verified at local policy roots too
- clippy threshold coverage now also proves for `g3rs-clippy/type-complexity-threshold`, `10`, `11`, `21`, and `22`:
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
- clippy rule-local parity is now explicitly pinned for:
  - `RS-CLIPPY-04`
  - `RS-CLIPPY-05`
  - `g3rs-clippy/package-native-policy`
  - `RS-CLIPPY-07`
  - `RS-CLIPPY-14`
  - `RS-CLIPPY-20`
  - those suites now prove the generated method/type/macro/global-state baselines match the checker-owned managed sets exactly
- clippy reason-quality coverage now proves:
  - generated ban entries with table-format reasons stay quiet
  - plain-string entries and missing-reason table entries warn across methods, types, and macros
- clippy trivial-reason coverage now proves:
  - generated substantive reasons stay quiet
  - placeholder or trivial reasons warn across methods, types, and macros
- clippy heuristic parity is now explicitly pinned for:
  - `RS-CLIPPY-08`
  - `g3rs-clippy/no-op-placeholder`
  - `RS-CLIPPY-16`
  - `g3rs-clippy/avoid-breaking-exported-api`
  - `RS-CLIPPY-18`
  - `RS-CLIPPY-19`
  - those suites now prove the generator emits reasoned table entries, substantive reasons, expected managed booleans, duplicate-free ban sections, and only managed top-level keys
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
  - deny coverage is resolved over the actual validation root and workspace roots in the scaffold
  - a nearer allowed deny config replaces ancestor coverage only for that owned workspace root
  - a local `.cargo/deny.toml` variant still owns its nearest workspace root when it is the closest config
  - same-root coverage chooses the highest-precedence accepted deny filename
  - malformed allowed deny configs still produce the explicit parse-error branch
  - uncovered effective roots error exactly where no allowed deny config applies
- deny canonical-ban coverage now proves:
  - the generated service and library deny baselines pass cleanly
  - library profile expands the canonical ban set to include library-IO bans
  - each missing canonical ban errors individually, including `lazy_static`
  - missing `[bans]` or missing `[bans].deny` fails closed
  - weakening canonical managed wrappers still errors in this rule as a baseline-integrity failure
  - parity tightening and follow-up generator fixes now prove exact parity:
    - service generator ban set matches the checker-owned canonical set exactly
    - library generator ban set matches the checker-owned canonical set exactly
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
- deny tokio feature policy coverage now proves:
  - the generated tokio feature baseline stays quiet
  - missing `tokio` `full` denial warns
  - drifting tokio allow-list warns
  - exact generated tokio allow and deny sets are now pinned in the rule-local suite
  - a broken local tokio policy only warns on the owned local config, not on ancestor-covered siblings
- deny extra-feature-ban coverage now proves:
  - the generated tokio-only feature baseline stays quiet
  - each non-tokio feature-ban entry inventories independently
  - the canonical feature-ban baseline is now pinned to a single tokio-managed entry
  - a local extra feature-ban entry inventories only on the owned local root that replaced ancestor policy
- deny allow-override coverage now proves:
  - canonical configs with no `[bans].allow` stay quiet
  - non-empty `[bans].allow` errors
  - each allow-entry that overrides a canonical or actual deny entry errors independently
- deny ban-reason coverage now proves:
  - the generated canonical deny entries stay quiet
  - each deny entry lacking a non-empty reason inventories independently
- deny duplicate-entry coverage now proves:
  - the generated canonical deny config stays quiet
  - duplicate deny, skip, advisory-ignore, and feature-ban entries each warn once per duplicated identity
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
  - and now pins exact generated registry-list parity in the rule-local suite
  - and now proves a broken local registry allow-list only errors on the owned local root that replaced ancestor coverage
- deny skip/ignore hygiene now tests malformed shape, missing reason, non-string reason, and valid inventory shapes
  - plain-string skip and ignore entries are now treated explicitly as supported valid inventory forms
  - malformed `[bans].skip` and `[advisories].ignore` container types now warn instead of failing open
- deny unknown-key coverage now includes nested skip/ignore/exception/feature entries
  - and now also covers unknown keys in the core `[bans]` and `[sources]` sections using structured mutations
- deny wrapper policy now distinguishes:
  - managed non-empty canonical wrapper drift = error
  - project-specific wrapper additions where no managed wrapper policy exists = inventory only
  - exact generated wrapper policy is now pinned against the checker-owned canonical wrapper map
  - and now proves managed wrapper drift in a local workspace-root config errors only on that owned local root

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

Current targeted verification for this lane now passes, including:
- `cargo test -p guardrail3 --lib rs_deny_19_`
- `cargo test -p guardrail3 --lib rs_deny_21_`
- `cargo test -p guardrail3 --lib rs_deny_22_`
- `cargo test -p guardrail3 --lib rs_deny_25_`
- `cargo test -p guardrail3 --lib rs_deny_26_`
- `cargo test -p guardrail3 --lib rs_deny_30_`

## Known gaps already identified

### Clippy
- generator/checker still disagree on some global-state bans
- no direct generator-vs-checker parity test
- `RS-CLIPPY-19` is intentionally temporary and must be tested honestly, not overclaimed

### Deny
- generation still uses wrong effective-profile logic in mixed setups
- no strong generator-vs-checker parity test
- canonical deny fixture has drifted before
- `g3rs-deny/tokio-full-ban` still needs explicit policy resolution if not already settled during the pass

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

This is complete for both clippy and deny at the structural migration level.

## Done means

- every `RS-CLIPPY-*` and `RS-DENY-*` rule has a `*_tests/` directory
- parity tests exist against the generator baseline
- drift-prone hardcoded fixtures are removed or parity-checked
- every touched rule has golden + attack-vector coverage

## Do not

- let the checker and generator keep separate silent baselines
- write tests that only look for broad family output
- hide policy decisions inside test fixtures

## Current unverified additions

None for deny at the moment.

`g3rs-deny/license-exceptions-inventory` breadth is now cargo-verified too:
- distinct-near-duplicate non-hit coverage passes
- same-identity-different-shape duplicate coverage for skip and advisory-ignore entries passes

## Next session start here

1. Read:
   - `14-clippy-deny-coverage-matrix.md`
   - `14-clippy-deny-execution-plan.md`
   - `04-clippy-and-deny.md`

2. Continue from the remaining semantic gaps, but pivot the attack model:
   - prefer the actual multi-root family fixture for remaining root/profile-sensitive rules
   - use synthetic helper trees only for isolated local branches

3. Highest-value next targets:
   - tighten parity and broader multi-root attack coverage for already-migrated rules where the matrix still calls out semantic gaps
   - especially broaden deny mixed-root/profile cases and clippy/deny generator-vs-checker parity framing
   - the most recent verified clippy ownership/root packet is:
     `RS-CLIPPY-01`, `02`, `03`, `12`, `13`
   - the most recent verified deny multi-root packet is:
     `RS-DENY-01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `10`, `11`, `12`, `13`, `15`, `16`, `17`, `19`, `21`, `22`, `23`, `24`, `25`, `26`, `28`, `29`, `30`
   - the most recent verified deny parity packet is:
     `RS-DENY-09`, `14`, `18`, `19`, `20`, `21`, `22`, `25`, `26`, `27`, `30`
   - the most recent verified clippy parity packet is:
     `RS-CLIPPY-04`, `05`, `06`, `07`, `08`, `14`, `15`, `16`, `17`, `18`, `19`, `20`

Reason:
- clippy and deny are structurally migrated now; remaining work is semantic breadth, not sidecar conversion
