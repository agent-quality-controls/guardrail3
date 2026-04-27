# Clippy And Deny Hardening Lane

> Historical hardening checkpoint. Current rule inventory and live semantics now live in:
> - [`.plans/todo/checks/rs/clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md)
> - [`.plans/todo/checks/rs/deny.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/deny.md)
>
> Use this file as migration history, not current source of truth.

## Focus

These families are mostly implemented. The hardening work is parity and policy-edge attack coverage.

## Main attack classes

- generator/checker drift
- mixed-profile drift
- shadowing / precedence
- typo-like config drift
- malformed escape-hatch entries
- inventory vs error branch exactness

## Clippy

- add direct generator-vs-checker parity tests
- attack root resolution and mixed profile/layer cases
- verify temporary `RS-CLIPPY-19` behavior is tested honestly, not overclaimed

### Progress

- support-layer canonical fixture now comes from `domain/modules/clippy::build_clippy_toml(...)` instead of a separate handwritten test-only baseline
- clippy test support can now copy the real `tests/fixtures/r_arch_01/golden/` multi-root scaffold, mutate config files in place, and run the family checker end-to-end
- support-layer parity tests now lock:
  - generated service thresholds
  - generated service booleans
  - generated service method/type/macro path sets
- existing clippy rule tests that used string replacement on the old fixture were converted to structure-aware mutation helpers for method/type removal and extra-entry insertion
- `RS-CLIPPY-12` now uses a rule-specific test directory and proves:
  - both forbidden nested filename variants fire
  - same-root precedence conflict still fires exactly on the lower-precedence sibling
- `g3rs-clippy/local-policy-root` now uses a rule-specific test directory and proves:
  - full local baseline inventory path
  - incomplete local-root failure path with exact missing-section set
  - parse-failure path for a local policy root that replaces inherited policy
- `RS-CLIPPY-14` now uses a rule-specific test directory and proves:
  - non-library roots stay out of scope
  - real generated library-profile config does not false-positive
  - every missing global-state type ban in a library root is reported
- `RS-CLIPPY-19` now uses a rule-specific test directory and proves:
  - typo-like managed-key drift warns
  - unrelated project-specific unknown keys do not overfire this temporary heuristic
- `RS-CLIPPY-01` now uses a rule-specific test directory and proves:
  - the real multi-root fixture is fully covered when a validation-root `clippy.toml` exists
  - a nearer local config at `apps/devctl/` replaces ancestor coverage only for its owned workspace root
  - uncovered-root errors fire only for the specific roots that lack an allowed covering config
- facts-layer verification corrected the mixed-root/profile model:
  - in the shared golden fixture, `packages/shared-types` is a workspace member, not a standalone policy root
  - the real bug was root package-profile resolution, not standalone-root coverage in that fixture
  - `clippy/facts.rs` and `deny/facts.rs` now treat the root config as package-profile-driven when `[rust.packages]` owns root generation
  - `generate_helpers.rs` now generates per-app `deny.toml` from the app's effective profile instead of the outer default profile
- `g3rs-clippy/max-struct-bools` now uses a rule-specific test directory and proves:
  - the generated `max-struct-bools` baseline inventories cleanly
  - wrong values error
  - missing values error
  - malformed `clippy.toml` errors through the rule-local parse-error branch
- `g3rs-clippy/max-fn-params-bools` now uses a rule-specific test directory and proves:
  - the generated `max-fn-params-bools` baseline inventories cleanly
  - wrong values error
  - missing values error
  - malformed `clippy.toml` errors through the rule-local parse-error branch
- `RS-CLIPPY-04` now uses a rule-specific test directory and proves:
  - the generated service baseline inventories the managed method-ban set
  - garde-owned deserialization method bans are not required when `garde = false`
  - each missing required method ban produces its own hard error
- `RS-CLIPPY-05` now uses a rule-specific test directory and proves:
  - the generated service baseline inventories the managed service type-ban set
  - garde-owned extractor type bans are not required when `garde = false`
  - library profile expands the required type-ban set to include global-state types
  - each missing required type ban produces its own hard error
- `g3rs-clippy/package-native-policy` now uses a rule-specific test directory and proves:
  - the generated service method baseline does not false-positive
  - project-specific extra method bans inventory
  - `garde = false` converts garde-owned method bans into project-specific extras
- `RS-CLIPPY-07` now uses a rule-specific test directory and proves:
  - the generated service type baseline does not false-positive
  - project-specific extra type bans inventory
  - `garde = false` converts garde-owned extractor type bans into project-specific extras
  - library-profile global-state type bans stay managed instead of being inventoried as extras
- `RS-CLIPPY-08` now uses a rule-specific test directory and proves:
  - generated reasoned table-format ban entries do not false-positive
  - plain-string and missing-reason entries warn across methods, types, and macros
- `g3rs-clippy/no-op-placeholder` now uses a rule-specific test directory and proves:
  - generated substantive reasons do not false-positive
  - trivial or placeholder reasons warn across methods, types, and macros
- `RS-CLIPPY-20` now uses a rule-specific test directory and proves:
  - the generated macro baseline inventories every required macro ban
  - each missing required macro ban produces its own hard error
- `RS-CLIPPY-16` now uses a rule-specific test directory and proves:
  - generated explicit `avoid-breaking-exported-api = false` inventories cleanly
  - `true` warns for non-published roots
  - `true` inventories for published library packages
  - missing value warns
- `g3rs-clippy/avoid-breaking-exported-api` now uses a rule-specific test directory and proves:
  - generated test-relaxation baseline stays quiet
  - each enabled test-relaxation key warns independently
- `RS-CLIPPY-18` now uses a rule-specific test directory and proves:
  - generated ban baseline stays quiet
  - duplicates warn once per duplicated path per section
- rule-local parity is now verified for:
  - `RS-CLIPPY-04`
  - `RS-CLIPPY-05`
  - `g3rs-clippy/package-native-policy`
  - `RS-CLIPPY-07`
  - `RS-CLIPPY-08`
  - `RS-CLIPPY-14`
  - `g3rs-clippy/no-op-placeholder`
  - `RS-CLIPPY-16`
  - `g3rs-clippy/avoid-breaking-exported-api`
  - `RS-CLIPPY-18`
  - `RS-CLIPPY-19`
  - `RS-CLIPPY-20`
  - these suites now pin exact generator-vs-checker parity for managed method, type, macro, and library-global-state ban sets, plus heuristic assumptions about reasoned entries, managed booleans, duplicate-free baselines, and managed top-level keys

### Remaining gaps

- rule-local parity tests still need to be added where the real attack surface lives, especially:
- multi-root and mixed-profile attack tests are still largely absent where semantics depend on root ownership
- severity exactness is still too local and not yet asserted against broad owned hit/non-hit sets
- clippy is now structurally migrated; remaining clippy work is parity tightening and broader attack coverage, not more flat-sidecar conversion

## Deny

- add direct generator-vs-checker parity tests
- attack mixed workspace profile selection
- attack nested config placement, same-root precedence, malformed exceptions/skips/ignores/wrappers
- resolve and test the `g3rs-deny/tokio-full-ban` policy decision explicitly

### Progress

- support-layer canonical service fixture now comes from `domain/modules/deny::build_deny_toml(...)` instead of a separate handwritten test-only baseline
- deny test support can now copy the real `tests/fixtures/r_arch_01/golden/` multi-root scaffold, mutate config files in place, and run the family checker end-to-end
- support-layer parity tests now lock:
  - graph settings
  - bans settings
  - sources settings
  - license allow-list
  - advisories baseline
  - tokio feature-ban allow set
- the deny wrapper test that depended on the old handwritten fixture was adjusted to mutate the generated deny baseline directly
- `RS-DENY-02` now uses a rule-specific test directory and proves:
  - all three accepted deny filenames are rejected when placed at a forbidden nested member root
  - allowed validation and workspace roots do not false-positive as forbidden deny locations
- `RS-DENY-03` now uses a rule-specific test directory and proves:
  - all three deny filename variants fire shadowing errors when nested below an allowed root
  - same-root multi-file conflicts still fire exactly once with the expected precedence-set message
  - allowed local policy roots do not false-positive as shadowing
- `g3rs-deny/tokio-full-ban` now uses a rule-specific test directory and proves:
  - both accepted crates.io allow-list forms are tolerated
  - missing sources section, missing crates.io, and unexpected extra registries all error distinctly
- `g3rs-deny/duplicate-entries` now uses a rule-specific test directory and proves:
  - malformed skip entries warn
  - missing skip reasons warn
  - non-string skip reasons warn
  - supported skip entry shapes inventory cleanly, including legacy `name` + `version`
  - malformed `[bans].skip` container shape now warns instead of failing open
  - a local skip inventory entry only reports on the owned local root that replaced ancestor coverage
- `g3rs-deny/unknown-keys` now uses a rule-specific test directory and proves:
  - malformed ignore entries warn
  - missing ignore reasons warn
  - non-string ignore reasons warn
  - supported ignore entry shapes inventory cleanly for both plain-string and table forms
  - malformed `[advisories].ignore` container shape now warns instead of failing open
  - a local advisory ignore inventory entry only reports on the owned local root that replaced ancestor coverage
- `g3rs-deny/allow-override-channel` now uses a rule-specific test directory and proves:
  - top-level and core-section unknown keys warn
  - nested unknown keys in skip, ignore, license exceptions, and feature-ban entries warn
  - unknown keys in `[bans]` and `[sources]` are now covered explicitly with structured config mutation
  - local unknown-key drift only warns on the owned local root that replaced ancestor coverage
- `g3rs-deny/extra-deny-bans-inventory` now uses a rule-specific test directory and proves:
  - ignore accumulation warns only above threshold
  - mixed valid ignore-entry shapes at threshold do not overfire
  - malformed ignore entries still count toward accumulation threshold because the rule owns container size, not entry validity
  - a local oversized ignore list only warns on the owned local root that replaced ancestor coverage
- `RS-DENY-30` now uses a rule-specific test directory and proves:
  - canonical bans with managed non-empty wrapper policy error on wrapper drift
  - canonical bans with empty wrapper policy inventory project-specific wrapper additions
  - non-canonical bans inventory project-specific wrappers instead of being treated as managed drift
  - managed wrapper drift in a local workspace-root config errors only on the owned local root
  - a local canonical wrapper baseline does not false-positive
- `RS-DENY-01` now uses a rule-specific test directory and proves:
  - deny coverage is checked against the actual validation root and workspace roots in the multi-root scaffold
  - a nearer allowed deny config at `apps/devctl/` replaces ancestor coverage only for that owned workspace root
  - a local `.cargo/deny.toml` variant still owns its nearest workspace root when it is the closest config
  - same-root coverage chooses the highest-precedence accepted deny filename
  - malformed allowed deny configs still surface the explicit parse-error branch
  - uncovered-root errors fire only for the specific effective roots that lack a covering deny config
- `RS-DENY-09` now uses a rule-specific test directory and proves:
  - generated service and library deny baselines pass without false positives
  - library profile requires the extra library-IO canonical bans
  - missing canonical bans each produce their own hard error
  - missing `[bans]` and missing `[bans].deny` fail closed
  - canonical managed wrapper drift still errors as baseline weakening
  - exact generator-vs-checker parity is now pinned for both service and library profiles
- `g3rs-deny/confidence-threshold` now uses a rule-specific test directory and proves:
  - generated license baseline passes without false positives
  - missing baseline allowed licenses error
  - missing `[licenses]` fails closed
  - `[licenses.private].ignore` must stay exactly `true`
  - exact generated allow-list and `private.ignore` parity is now pinned in the rule-local suite
- `g3rs-deny/allow-git-inventory` now uses a rule-specific test directory and proves:
  - generated unknown-source policy passes without false positives
  - missing `[sources]` fails closed
  - weakened `unknown-registry` and `unknown-git` values each error independently
  - exact generated registry list and unknown-source policy parity is now pinned in the rule-local suite
- `g3rs-deny/extra-feature-bans-inventory` now uses a rule-specific test directory and proves:
  - empty `allow-git` stays quiet
  - non-empty `allow-git` warns once per config
  - each allowed git source is inventoried individually
  - exact generated empty `allow-git` baseline is now pinned in the rule-local suite
- verified deny parity packet now includes:
  - `RS-DENY-09`
  - `g3rs-deny/confidence-threshold`
  - `g3rs-deny/allow-git-inventory`
  - `g3rs-deny/extra-feature-bans-inventory`
- `g3rs-deny/deprecated-advisories` now uses a rule-specific test directory and proves:
  - the generated advisories baseline stays quiet
  - each deprecated advisory key warns independently
  - local deprecated advisory fields only warn on the owned local root that replaced ancestor coverage
- `g3rs-deny/advisories-baseline` now uses a rule-specific test directory and proves:
  - the generated advisories baseline stays quiet
  - missing `[advisories]` fails closed
  - missing baseline values error independently
  - weakened `unmaintained` and `yanked` values error independently
  - a local weakened advisory baseline only errors on the owned local root that replaced ancestor coverage
- `g3rs-deny/stricter-advisories-inventory` now uses a rule-specific test directory and proves:
  - the generated advisories baseline stays quiet
  - `unmaintained = "deny"` and `yanked = "deny"` each inventory independently as stricter-than-baseline policy
  - a local stricter advisory policy only inventories on the owned local root that replaced ancestor coverage
- `g3rs-deny/graph-all-features` now uses a rule-specific test directory and proves:
  - the generated graph baseline stays quiet
  - missing `[graph]` fails closed
  - missing or weakened `all-features` errors
  - local `all-features` drift only errors on the owned local root that replaced ancestor coverage
- `g3rs-deny/graph-no-default-features` now uses a rule-specific test directory and proves:
  - the generated graph baseline stays quiet
  - missing `[graph]` fails closed
  - missing or weakened `no-default-features` errors
  - local `no-default-features` drift only errors on the owned local root that replaced ancestor coverage
- `g3rs-deny/highlight-inventory` now uses a rule-specific test directory and proves:
  - the generated multiple-versions baseline stays quiet
  - missing `[bans]` warns
  - missing `multiple-versions` warns
  - weakened `multiple-versions` warns
  - local multiple-versions weakening only warns on the owned local root that replaced ancestor coverage
- `g3rs-deny/allow-wildcard-paths` now uses a rule-specific test directory and proves:
  - the generated highlight baseline stays quiet
  - missing `highlight` inventories
  - project-specific `highlight` values inventory
  - local highlight drift only inventories on the owned local root that replaced ancestor coverage
- `g3rs-deny/wildcards-inventory` now uses a rule-specific test directory and proves:
  - the generated allow-wildcard-paths baseline stays quiet
  - missing `[bans]` fails closed
  - missing or weakened `allow-wildcard-paths` errors
  - local allow-wildcard-paths drift only errors on the owned local root that replaced ancestor coverage
- `g3rs-deny/license-allow-baseline` now uses a rule-specific test directory and proves:
  - the generated wildcards baseline stays quiet
  - missing `wildcards` warns
  - project-specific `wildcards` values warn
  - local wildcards drift only warns on the owned local root that replaced ancestor coverage
- `g3rs-deny/copyleft-allowlist` now uses a rule-specific test directory and proves:
  - the generated confidence-threshold baseline stays quiet
  - weaker threshold values warn
  - stricter threshold values inventory
  - missing or invalid threshold values warn
  - a local weaker confidence threshold only warns on the owned local root that replaced ancestor coverage
- `g3rs-deny/unknown-sources-policy` now uses a rule-specific test directory and proves:
  - the generated license allow-list stays quiet
  - each added copyleft license warns independently
  - a local copyleft allowance only warns on the owned local root that replaced ancestor coverage
- `RS-DENY-17` now uses a rule-specific test directory and proves:
  - the generated baseline stays quiet when no exceptions exist
  - each named or crate-keyed license exception inventories independently
  - a local license exception only inventories on the owned local root that replaced ancestor coverage
- `g3rs-deny/skip-hygiene` now uses a rule-specific test directory and proves:
  - the generated tokio feature policy stays quiet
  - missing `tokio` `full` denial warns
  - drifting tokio allow-list warns
  - a broken local tokio feature policy only warns on the owned local root that replaced ancestor coverage
- `g3rs-deny/ignore-hygiene` now uses a rule-specific test directory and proves:
  - the generated tokio-only feature baseline stays quiet
  - each non-tokio feature-ban entry inventories independently
  - a local extra feature-ban entry inventories only on the owned local root that replaced ancestor coverage
- `RS-DENY-25` now uses a rule-specific test directory and proves:
  - canonical configs with no `[bans].allow` stay quiet
  - non-empty `[bans].allow` errors
  - each allow-entry that overrides a deny entry errors independently
  - a non-overlapping allow-list still errors, so the rule does not depend on override detection to fire
  - a bad local allow-list only errors on the owned local root that replaced ancestor coverage
- `RS-DENY-26` now uses a rule-specific test directory and proves:
  - canonical deny entries with reasons stay quiet
  - each deny entry lacking a non-empty reason inventories independently
  - whitespace-only reasons inventory as missing
  - a local missing ban reason only inventories on the owned local root that replaced ancestor coverage
- `g3rs-deny/license-exceptions-inventory` now uses a rule-specific test directory and proves:
  - the generated canonical deny config stays quiet
  - duplicate deny, skip, advisory-ignore, and feature-ban entries each warn once per duplicated identity
  - distinct near-duplicates do not warn
  - same identity across supported skip and advisory-ignore shapes still warns as a duplicate

### Remaining gaps

- rule-local parity tests still need to be added for:
  - `RS-DENY-30`
- `g3rs-deny/tokio-full-ban`, `21`, `22`, and `30` now have direct parity framing too; their remaining gaps are broader mixed-root/profile breadth, not silent canonical drift
- broad root-resolution and mixed-profile mutation tests are still mostly missing
- deny is now structurally migrated; remaining deny work is parity tightening and broader multi-root/profile attack coverage

## Current verification blockers

- repo-wide blockers have since been cleared; targeted deny-tail verification now passes, including:
  - `cargo test -p guardrail3 --lib rs_deny_09_`
  - `cargo test -p guardrail3 --lib rs_deny_19_`
  - `cargo test -p guardrail3 --lib rs_deny_21_`
  - `cargo test -p guardrail3 --lib rs_deny_22_`
  - `cargo test -p guardrail3 --lib rs_deny_25_`
  - `cargo test -p guardrail3 --lib rs_deny_30_`
  - `cargo test -p guardrail3 --lib rs_deny_26_`
  - `cargo test -p guardrail3 --lib rs_deny_27_`

No current external blocker is preventing targeted deny-tail verification.

Recent generator fix in this lane:
- canonical `deny.toml` generation now injects ban `reason` fields inside `[bans].deny` entries instead of emitting invalid loose inline tables under `[bans]`
- that closes the generator/checker drift exposed by `RS-DENY-26` and restores valid canonical TOML for deny test-support mutations

Recent unrelated compile unblockers fixed while finishing deny-tail verification:
- `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs`
  - removed dead `accumulated_len` assignments that were tripping `-D unused-assignments`

## Success condition

No family-local hardcoded canonical fixture can drift silently from the generator.
