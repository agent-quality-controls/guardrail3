# Clippy And Deny Hardening Lane

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
- `RS-CLIPPY-13` now uses a rule-specific test directory and proves:
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
  - nearer local configs at `apps/devctl/` and `packages/shared-types/` replace ancestor coverage only for those owned roots
  - uncovered-root errors fire only for the specific roots that lack an allowed covering config
- `RS-CLIPPY-02` now uses a rule-specific test directory and proves:
  - the generated `max-struct-bools` baseline inventories cleanly
  - wrong values error
  - missing values error
  - malformed `clippy.toml` errors through the rule-local parse-error branch
- `RS-CLIPPY-03` now uses a rule-specific test directory and proves:
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
- `RS-CLIPPY-06` now uses a rule-specific test directory and proves:
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
- `RS-CLIPPY-15` now uses a rule-specific test directory and proves:
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
- `RS-CLIPPY-17` now uses a rule-specific test directory and proves:
  - generated test-relaxation baseline stays quiet
  - each enabled test-relaxation key warns independently
- `RS-CLIPPY-18` now uses a rule-specific test directory and proves:
  - generated ban baseline stays quiet
  - duplicates warn once per duplicated path per section

### Remaining gaps

- rule-local parity tests still need to be added where the real attack surface lives, especially:
  - `RS-CLIPPY-01`
  - `RS-CLIPPY-04`
  - `RS-CLIPPY-05`
  - `RS-CLIPPY-06`
  - `RS-CLIPPY-07`
  - `RS-CLIPPY-08`
  - `RS-CLIPPY-12`
  - `RS-CLIPPY-13`
  - `RS-CLIPPY-14`
  - `RS-CLIPPY-15`
  - `RS-CLIPPY-16`
  - `RS-CLIPPY-17`
  - `RS-CLIPPY-18`
  - `RS-CLIPPY-19`
  - `RS-CLIPPY-20`
  - `RS-CLIPPY-02`
  - `RS-CLIPPY-03`
- multi-root and mixed-profile attack tests are still largely absent where semantics depend on root ownership
- severity exactness is still too local and not yet asserted against broad owned hit/non-hit sets
- clippy is now structurally migrated; remaining clippy work is parity tightening and broader attack coverage, not more flat-sidecar conversion

## Deny

- add direct generator-vs-checker parity tests
- attack mixed workspace profile selection
- attack nested config placement, same-root precedence, malformed exceptions/skips/ignores/wrappers
- resolve and test the `RS-DENY-19` policy decision explicitly

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
- `RS-DENY-02` now uses a rule-specific test directory and proves all three accepted deny filenames are rejected when placed at a forbidden nested member root
- `RS-DENY-03` now uses a rule-specific test directory and proves:
  - all three deny filename variants fire shadowing errors when nested below an allowed root
  - same-root multi-file conflicts still fire exactly once with the expected precedence-set message
- `RS-DENY-19` now uses a rule-specific test directory and proves:
  - both accepted crates.io allow-list forms are tolerated
  - missing sources section, missing crates.io, and unexpected extra registries all error distinctly
- `RS-DENY-23` now uses a rule-specific test directory and proves:
  - malformed skip entries warn
  - missing skip reasons warn
  - non-string skip reasons warn
  - supported skip entry shapes inventory cleanly, including legacy `name` + `version`
- `RS-DENY-24` now uses a rule-specific test directory and proves:
  - malformed ignore entries warn
  - missing ignore reasons warn
  - non-string ignore reasons warn
  - supported ignore entry shapes inventory cleanly for both plain-string and table forms
- `RS-DENY-28` now uses a rule-specific test directory and proves:
  - top-level and core-section unknown keys warn
  - nested unknown keys in skip, ignore, license exceptions, and feature-ban entries warn
- `RS-DENY-29` now uses a rule-specific test directory and proves:
  - ignore accumulation warns only above threshold
  - mixed valid ignore-entry shapes at threshold do not overfire
- `RS-DENY-30` now uses a rule-specific test directory and proves:
  - canonical bans with managed non-empty wrapper policy error on wrapper drift
  - canonical bans with empty wrapper policy inventory project-specific wrapper additions
  - non-canonical bans inventory project-specific wrappers instead of being treated as managed drift
- `RS-DENY-01` now uses a rule-specific test directory and proves:
  - deny coverage is checked against the actual validation root, workspace roots, and standalone package root in the multi-root scaffold
  - nearer allowed deny configs at `apps/devctl/` and `packages/shared-types/` replace ancestor coverage only for those owned roots
  - malformed allowed deny configs still surface the explicit parse-error branch
  - uncovered-root errors fire only for the specific effective roots that lack a covering deny config
- `RS-DENY-09` now uses a rule-specific test directory and proves:
  - generated service and library deny baselines pass without false positives
  - library profile requires the extra library-IO canonical bans
  - missing canonical bans each produce their own hard error
  - missing `[bans]` and missing `[bans].deny` fail closed
  - canonical managed wrapper drift still errors as baseline weakening
- `RS-DENY-14` now uses a rule-specific test directory and proves:
  - generated license baseline passes without false positives
  - missing baseline allowed licenses error
  - missing `[licenses]` fails closed
  - `[licenses.private].ignore` must stay exactly `true`
- `RS-DENY-18` now uses a rule-specific test directory and proves:
  - generated unknown-source policy passes without false positives
  - missing `[sources]` fails closed
  - weakened `unknown-registry` and `unknown-git` values each error independently
- `RS-DENY-20` now uses a rule-specific test directory and proves:
  - empty `allow-git` stays quiet
  - non-empty `allow-git` warns once per config
  - each allowed git source is inventoried individually
- `RS-DENY-04` now uses a rule-specific test directory and proves:
  - the generated advisories baseline stays quiet
  - each deprecated advisory key warns independently
- `RS-DENY-05` now uses a rule-specific test directory and proves:
  - the generated advisories baseline stays quiet
  - missing `[advisories]` fails closed
  - missing baseline values error independently
  - weakened `unmaintained` and `yanked` values error independently
- `RS-DENY-06` now uses a rule-specific test directory and proves:
  - the generated advisories baseline stays quiet
  - `unmaintained = "deny"` and `yanked = "deny"` each inventory independently as stricter-than-baseline policy
- `RS-DENY-07` now uses a rule-specific test directory and proves:
  - the generated graph baseline stays quiet
  - missing `[graph]` fails closed
  - missing or weakened `all-features` errors
- `RS-DENY-08` now uses a rule-specific test directory and proves:
  - the generated graph baseline stays quiet
  - missing `[graph]` fails closed
  - missing or weakened `no-default-features` errors
- `RS-DENY-10` now uses a rule-specific test directory and proves:
  - the generated multiple-versions baseline stays quiet
  - missing `[bans]` warns
  - missing `multiple-versions` warns
  - weakened `multiple-versions` warns
- `RS-DENY-11` now uses a rule-specific test directory and proves:
  - the generated highlight baseline stays quiet
  - missing `highlight` inventories
  - project-specific `highlight` values inventory
- `RS-DENY-12` now uses a rule-specific test directory and proves:
  - the generated allow-wildcard-paths baseline stays quiet
  - missing `[bans]` fails closed
  - missing or weakened `allow-wildcard-paths` errors
- `RS-DENY-13` now uses a rule-specific test directory and proves:
  - the generated wildcards baseline stays quiet
  - missing `wildcards` warns
  - project-specific `wildcards` values warn
- `RS-DENY-15` now uses a rule-specific test directory and proves:
  - the generated confidence-threshold baseline stays quiet
  - weaker threshold values warn
  - stricter threshold values inventory
  - missing or invalid threshold values warn
- `RS-DENY-16` now uses a rule-specific test directory and proves:
  - the generated license allow-list stays quiet
  - each added copyleft license warns independently
- `RS-DENY-17` now uses a rule-specific test directory and proves:
  - the generated baseline stays quiet when no exceptions exist
  - each named or crate-keyed license exception inventories independently

### Explicit delta still present

- `expected_bans(...)` still contains one audited addition outside the current generator output: `lazy_static`
- this is now documented and asserted explicitly in support-layer parity tests instead of being hidden inside a second silent canonical fixture

### Remaining gaps

- rule-local parity tests still need to be added for:
  - `RS-DENY-01`
  - `RS-DENY-02`
  - `RS-DENY-03`
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
- `RS-DENY-19` still needs explicit policy closure around accepted crates.io forms and broader root/profile coverage
- broad root-resolution and mixed-profile mutation tests are still mostly missing
- remaining deny sidecar conversion work is now:
  - `RS-DENY-21`
  - `RS-DENY-22`
  - `RS-DENY-25`
  - `RS-DENY-26`
  - `RS-DENY-27`

## Current verification blockers

- targeted clippy/deny tests now compile past the local lane helpers, but are blocked by unrelated dead-code failures in:
  - `apps/guardrail3/crates/app/rs/checks/rs/hexarch/test_support.rs`
  - `apps/guardrail3/crates/app/rs/checks/rs/release/test_support.rs`
- a fresh `cargo test -p guardrail3 --lib rs_deny_10_ --no-run` still stops on those same unrelated dead-code failures, not on the deny hardening lane
- this remained unchanged after the `RS-DENY-15/16/17` migration; no new lane-local compile blocker surfaced

These blockers are outside the clippy/deny lane, but they currently prevent normal end-to-end cargo verification for this pass.

## Success condition

No family-local hardcoded canonical fixture can drift silently from the generator.
