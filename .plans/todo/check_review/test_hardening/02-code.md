# Code Hardening Lane

## Focus

Attack source-level rules as bypass surfaces, not stylistic hints.

The lane is now in repeated adversarial convergence mode, not structural migration mode.

## Main attack classes

- suppression tricks
- attribute placement tricks
- aliasing/import tricks
- grouped `use` forms
- nested-module placement
- test-vs-prod path confusion
- parse/read failures
- public-API leakage edge cases

## Priority rule groups

### Escape hatches / bypasses
- `RS-CODE-01..08`
- `RS-CODE-17..24`

### Public API and organization
- `RS-CODE-25..29`

## Explicit gaps to close

- deepen later-numbered bypass coverage where only one exploit shape is exercised
- strengthen inventory tests that still rely on counts where exact owned hit sets are practical
- keep legacy compatibility paths from drifting away from the hardened family semantics

## Success condition

Each rule has:
- golden coverage
- one attack vector applied across all relevant source files in the golden tree
- exact file hit sets
- false-positive controls for similar legal syntax
- repeated attack passes no longer finding material rule-local bugs

## Audit Snapshot

Current family state from `apps/guardrail3/crates/app/rs/families/code/`:

- the live runtime is under `crates/runtime/src/`
- rule-specific `*_tests/` directories exist across the implemented rules
- sibling `crates/assertions/` and `test_support/` packages exist, so this lane is about hardening and ownership cleanup, not first-time structural migration
- the reusable mixed-monorepo golden tree already exists at `apps/guardrail3/tests/fixtures/r_arch_01/golden/`
- broad golden-tree mutation coverage is still uneven, and exact owned hit-set / non-hit-set assertions are still missing for parts of the family

## Reuse / Replace Decision

Keep and expand:

- direct rule-local snippet tests that already prove a distinct semantic branch
- the small number of family-level tests that already prove discovery/profile/fail-closed behavior

Replace structurally:

- every `*_tests.rs` sidecar file
- happy-path-only snippet tests that do not map cleanly to a real attack vector
- loose `any()` presence assertions where exact file ownership should be asserted

## Known Blockers To Resolve During Migration

- the shared parser gaps from the original brief are now mostly closed, so new attack passes should focus on rule-local completeness instead of reopening already-fixed parser bugs
- the current gap is no longer placeholder content; it is the remaining work to broaden later-rule bypass vectors and exact owned-hit assertions across the populated golden tree
- targeted `cargo test` execution can still be blocked by unrelated dirty-tree compile failures outside `rs/code`; when that happens, use `cargo check -p guardrail3 --lib` to keep verifying the `rs/code` slice and document the external blocker explicitly

Current note:

- the `guardrail3-app-rs-family-code` package test suite is green again, including the `RS-CODE-30` fail-closed harness on current `ProjectTree::new(...)` / `DirEntry::new(...)` APIs
- unreadable active config files scanned for exception-comment inventory now fail closed through `RS-CODE-30` instead of being silently skipped
- `RS-CODE-25` should be treated as a legacy non-firing compatibility rule; active weak public error-form ownership is `RS-CODE-33`

## Coverage Matrix

Legend:

- `direct` = current rule-local typed-input snippet tests exist
- `family` = current family/orchestrator behavior is exercised
- `fp` = some false-positive / non-hit control exists
- `exact` = current tests assert exact count/message/severity for at least one branch
- `missing` = still absent from the hardening contract

| Rule | Current coverage | Major missing hardening coverage | Migration note |
|------|------------------|----------------------------------|----------------|
| `RS-CODE-01` | `direct`, `exact` | golden, broad attack vector, exact owned hit set, exact owned non-hit set, inline/nested module breadth | reuse severity split idea, rebuild around multi-file allow placements |
| `RS-CODE-02` | `direct`, `exact` | golden, broad attack vector, owned hit/non-hit sets, false-positive controls | keep inventory branch idea, broaden to crate-wide exemption ownership |
| `RS-CODE-03` | `direct`, `exact` | golden, broad attack vector, grouped/aliased attrs, nested placements, owned hit/non-hit sets | high-risk because of `same_line_reason()` and helper dependence |
| `RS-CODE-04` | `direct`, `exact` | golden, broad documented-bypass attack, malformed-reason controls, owned hit/non-hit sets | pair with `RS-CODE-03` migration |
| `RS-CODE-05` | `direct`, `exact` | golden, broad attack vector, whole-type ownership, false-positive controls, owned hit/non-hit sets | likely semantic bug surface |
| `RS-CODE-06` | `direct`, `exact` | golden, broad attack vector, whole-type ownership, reasoned-warning coverage, owned hit/non-hit sets | likely semantic bug surface |
| `RS-CODE-07` | `direct`, `family`, `exact` | golden config tree, broad multi-file inventory mutation, exact owned hit/non-hit sets, false-positive controls | preserve nested config collection coverage |
| `RS-CODE-08` | `direct`, `exact` | golden, broad attack vector, grouped/aliased cfg attr forms, separation from `RS-CODE-18`, owned hit/non-hit sets | helper-backed, migrate with `RS-CODE-18` |
| `RS-CODE-09` | `direct`, `fp`, `exact` | golden, broad multi-file mutation, exact owned hit set, comment-count false positives | keep test-file exemption branch |
| `RS-CODE-10` | `direct`, `exact` | golden, threshold-boundary controls, grouped-import semantics, owned hit/non-hit sets | pair with `RS-CODE-11` |
| `RS-CODE-11` | `direct`, `exact` | golden, threshold-boundary controls, grouped-import semantics, owned hit/non-hit sets | pair with `RS-CODE-10` |
| `RS-CODE-12` | `direct`, `exact` | golden, family-level Cargo ownership, missing-lint control, malformed-input split vs `RS-CODE-30` | preserve severity split and inventory behavior |
| `RS-CODE-13` | `direct`, `exact` | golden, broad attack vector, nested placements, false-positive controls, owned hit/non-hit sets | keep macro severity split branches |
| `RS-CODE-15` | `direct`, `fp`, `exact` | golden, broad attack vector, grouped import/call breadth, exact owned hit/non-hit sets | existing cfg-test skip is reusable |
| `RS-CODE-16` | `direct`, `fp`, `exact` | golden, broad attack vector, nested placements, exact owned hit/non-hit sets | existing test-file skip is reusable |
| `RS-CODE-17` | `direct`, `exact` | golden, threshold boundary, multi-file impl attacks, owned hit/non-hit sets | migrate with suppression lane |
| `RS-CODE-18` | `direct`, `fp`, `exact` | golden, broader always-true predicate matrix, owned hit/non-hit sets, nested placements | high-risk helper/parser surface |
| `RS-CODE-19` | `direct`, `exact` | golden, threshold-boundary controls, broad inventory mutation, owned hit/non-hit sets | current struct/enum branches are reusable seeds |
| `RS-CODE-20` | `direct`, `golden`, `fp`, `exact` | optional cfg_attr combinatorics only | foreign-mod ownership is now intentionally centralized here; `RS-CODE-03/04` do not own `ForeignMod` attrs and `RS-CODE-18` no longer claims always-true foreign-mod cfg_attr |
| `RS-CODE-21` | `direct`, `golden`, `fp`, `exact` | final targeted cargo verification, then optional convergence pass for any real misses it exposes | parser now covers nested/grouped glob imports and cfg(test) suppression; fixture-level metadata assertions were tightened to the family standard |
| `RS-CODE-22` | `direct`, `exact` | golden, grouped lint-list attacks, broader crate/item placement attacks, owned hit/non-hit sets | high-risk because of `same_line_reason()` |
| `RS-CODE-23` | `direct`, `exact` | golden, broad include/path-traversal attacks across owned files, exact owned hit/non-hit sets | existing severity split branches are reusable |
| `RS-CODE-24` | `direct`, `exact` | golden, path traversal vs reason coverage across placements, owned hit/non-hit sets | high-risk because of `same_line_reason()` |
| `RS-CODE-25` | `direct`, `family`, `exact` | golden library tree, broad API attack vector, owned hit/non-hit sets, false-positive controls | preserve current profile-resolution family test |
| `RS-CODE-26` | `direct`, `exact` | golden library tree, broad API leakage attack vector, exact owned hit/non-hit sets | migrate with library lane |
| `RS-CODE-27` | `direct`, `exact` | golden library tree, broader allowed-item controls, exact owned hit/non-hit sets | existing illegal-body branches are reusable |
| `RS-CODE-29` | `direct`, `exact` | golden library tree, threshold-boundary controls, exact owned hit/non-hit sets | keep warn/error threshold branches |
| `RS-CODE-30` | `direct`, `family`, `exact` | golden, unreadable-file coverage, malformed Cargo coverage, exact owned hit set, exact non-hit set | first fail-closed hardening target |

## First Migration Batch

The first structural migration batch is:

1. `RS-CODE-30`
2. `RS-CODE-01..08`
3. `RS-CODE-17..24`

Reason:

- these are the highest-value bypass and fail-closed surfaces
- they carry most of the parser/helper risk
- they are where broad golden-tree mutations are most likely to expose semantic bugs

## Specific Test Ideas To Preserve From The Current Suite

- `RS-CODE-01` prod/test severity split
- `RS-CODE-07` nested config-file collection
- `RS-CODE-09` test-file exemption
- `RS-CODE-14` allow-scoped unwrap non-hit
- `RS-CODE-15` cfg-test non-hit
- `RS-CODE-16` test-file non-hit
- `RS-CODE-18` genuine-condition non-hit cases
- `RS-CODE-21` grouped `use std::{fs::*, ...}` detection
- `RS-CODE-22` `forbid(unsafe_code)` special-case inventory
- `RS-CODE-23` severity split between direct include, build-script include, and traversal warning
- `RS-CODE-25` family-level library profile resolution
- `RS-CODE-30` family-level source parse failure and `guardrail3.toml` parse failure

## Immediate Next Step

Start the first migration batch by:

1. designing minimal shared golden-tree fixture support for `rs/code`
2. converting `RS-CODE-30` to a `rs_code_30_input_failures_tests/` directory first
3. using that first migration to lock the directory/module pattern and exact hit-set assertion style for the rest of the family

## Progress

- structural migration is no longer the main issue in this lane
- the live hardening work is now about parser/model correctness, adversarial breadth, and real repo-root debt
- recent family work has focused on shared parser fixes captured in [`FIXES.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/FIXES.md), not on adding the first `*_tests/` directories

## Adversarial Loop Protocol

Use this per rule now:

1. reconstruct rule intent from:
   - `code.md`
   - the family hardening docs
   - the production error message / semantics
2. run one targeted verification pass if unrelated repo blockers permit
3. attack the rule from four angles:
   - completeness
   - missing scenarios
   - pattern parity
   - false positives / exactness
4. fix real rule bugs before widening tests
5. rerun until the remaining findings are just combinatorial expansion with no new bug class

Use the `test-attack` skill as the default analysis frame for this loop.

## Resume Point

Current in-flight batch: `RS-CODE-21..30`.

Latest batch status:
- `RS-CODE-21` now has stronger direct, golden, false-positive, and inventory exactness
- `RS-CODE-22..30` have been tightened in the same pass toward exact metadata assertions and true zero-hit baselines
- the next gate is focused compile/test verification on `target/code`, not more speculative rewrites

Do not reopen `RS-CODE-20` unless a new plan/code contradiction appears.
