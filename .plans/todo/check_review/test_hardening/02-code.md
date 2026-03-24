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

- retire legacy `ast_helpers`
- whole-type `#[garde(skip)]` ownership
- grouped/aliased attribute edge cases
- fail-closed parsing already added in `RS-CODE-30`; deepen adversarial coverage

## Success condition

Each rule has:
- golden coverage
- one attack vector applied across all relevant source files in the golden tree
- exact file hit sets
- false-positive controls for similar legal syntax
- repeated attack passes no longer finding material rule-local bugs

## Audit Snapshot

Current family state from `apps/guardrail3/crates/app/rs/checks/rs/code/`:

- the family started with legacy `*_tests.rs` sidecars everywhere
- a first migration batch is now using rule-specific `*_tests/` directories, but most rules still need conversion
- most rules only have direct typed-input tests against one tiny snippet
- only a few rules currently touch family-level behavior at all
- there is effectively no broad golden-tree mutation coverage yet
- exact owned hit-set and owned non-hit-set assertions are mostly absent
- the reusable mixed-monorepo golden tree already exists at `apps/guardrail3/tests/fixtures/r_arch_01/golden/` and is now populated with realistic source across the main app/package slices, but it still needs to be consumed by broad `rs/code` mutation tests

## Reuse / Replace Decision

Keep and expand:

- direct rule-local snippet tests that already prove a distinct semantic branch
- the small number of family-level tests that already prove discovery/profile/fail-closed behavior

Replace structurally:

- every `*_tests.rs` sidecar file
- happy-path-only snippet tests that do not map cleanly to a real attack vector
- loose `any()` presence assertions where exact file ownership should be asserted

## Known Blockers To Resolve During Migration

- `parse.rs` still depends on legacy `validate::ast_helpers` for:
  - crate-level allow discovery
  - inline-module allow discovery
  - item-level allow discovery
  - `cfg_attr(..., allow(...))` discovery
  - `#[garde(skip)]` discovery
- `same_line_reason()` is still raw-line comment matching; adversarial tests should attack formatting and placement edge cases
- `discover.rs::is_test_path()` is still heuristic string matching and needs prod/test confusion attacks
- whole-type `#[garde(skip)]` ownership is still an explicit open gap from the brief
- grouped and aliased attribute/import handling is a likely parser/helper debt surface
- the current gap is no longer placeholder content; it is the remaining work to rewrite rule tests so they mutate the populated golden tree broadly instead of relying on direct snippets
- targeted `cargo test` execution can still be blocked by unrelated dirty-tree compile failures outside `rs/code`; when that happens, use `cargo check -p guardrail3 --lib` to keep verifying the `rs/code` slice and document the external blocker explicitly

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
| `RS-CODE-04` | `direct`, `exact` | golden, broad inventory attack, malformed-reason controls, owned hit/non-hit sets | pair with `RS-CODE-03` migration |
| `RS-CODE-05` | `direct`, `exact` | golden, broad attack vector, whole-type ownership, false-positive controls, owned hit/non-hit sets | likely semantic bug surface |
| `RS-CODE-06` | `direct`, `exact` | golden, broad attack vector, whole-type ownership, comment-shape controls, owned hit/non-hit sets | likely semantic bug surface |
| `RS-CODE-07` | `direct`, `family`, `exact` | golden config tree, broad multi-file inventory mutation, exact owned hit/non-hit sets, false-positive controls | preserve nested config collection coverage |
| `RS-CODE-08` | `direct`, `exact` | golden, broad attack vector, grouped/aliased cfg attr forms, separation from `RS-CODE-18`, owned hit/non-hit sets | helper-backed, migrate with `RS-CODE-18` |
| `RS-CODE-09` | `direct`, `fp`, `exact` | golden, broad multi-file mutation, exact owned hit set, comment-count false positives | keep test-file exemption branch |
| `RS-CODE-10` | `direct`, `exact` | golden, threshold-boundary controls, grouped-import semantics, owned hit/non-hit sets | pair with `RS-CODE-11` |
| `RS-CODE-11` | `direct`, `exact` | golden, threshold-boundary controls, grouped-import semantics, owned hit/non-hit sets | pair with `RS-CODE-10` |
| `RS-CODE-12` | `direct`, `exact` | golden, family-level Cargo ownership, missing-lint control, malformed-input split vs `RS-CODE-30` | preserve severity split and inventory behavior |
| `RS-CODE-13` | `direct`, `exact` | golden, broad attack vector, nested placements, false-positive controls, owned hit/non-hit sets | keep macro severity split branches |
| `RS-CODE-14` | `direct`, `fp`, `exact` | golden, broad attack vector, exact owned hit/non-hit sets, chained/nested placements | existing skip branch is reusable |
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
| `RS-CODE-28` | `direct`, `exact` | golden library tree, broad inline-module attack vector, exact owned hit/non-hit sets | migrate with library lane |
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

- `RS-CODE-30` has been converted from `rs_code_30_input_failures_tests.rs` to `rs_code_30_input_failures_tests/`
- `RS-CODE-01` has been converted from `rs_code_01_crate_level_allow_tests.rs` to `rs_code_01_crate_level_allow_tests/`
- `RS-CODE-02` has been converted from `rs_code_02_unused_crate_dependencies_allow_tests.rs` to `rs_code_02_unused_crate_dependencies_allow_tests/`
- `RS-CODE-03` has been converted from `rs_code_03_item_level_allow_without_reason_tests.rs` to `rs_code_03_item_level_allow_without_reason_tests/`
- `RS-CODE-04` has been converted from `rs_code_04_item_level_allow_with_reason_tests.rs` to `rs_code_04_item_level_allow_with_reason_tests/`
- `RS-CODE-05` has been converted from `rs_code_05_garde_skip_without_comment_tests.rs` to `rs_code_05_garde_skip_without_comment_tests/`
- `RS-CODE-06` has been converted from `rs_code_06_garde_skip_with_comment_tests.rs` to `rs_code_06_garde_skip_with_comment_tests/`
- `RS-CODE-08` has been converted from `rs_code_08_cfg_attr_allow_inventory_tests.rs` to `rs_code_08_cfg_attr_allow_inventory_tests/`
- `apps/guardrail3/crates/app/rs/checks/rs/code/test_support.rs` now exists as the first minimal shared support for temp roots and exact rule-hit file sets
- fixture population is now complete enough for hardening use:
  - root golden config files exist (`guardrail3.toml`, `package.json`, `pnpm-workspace.yaml`, `tsconfig.base.json`)
  - `apps/backend` contains real Rust planning-service code, including domain, app, ports, REST adapter, and MCP adapter slices
  - `apps/worker` contains real Rust queue-processing code with retry, skip, and dead-letter behavior
  - `apps/devctl` contains real Rust workspace-doctor code with probe, doctor-report, and CLI summary flows
  - `apps/landing`, `apps/admin`, and `apps/portal` contain real TS/TSX route, module, adapter, and UI code rather than placeholder comments
  - `packages/shared-types` and `packages/ui-kit` now contain real shared Rust/TS package code
  - there are no remaining comment-only source placeholders under `apps/guardrail3/tests/fixtures/r_arch_01/golden/`
- targeted fixture verification succeeded for the populated Rust slices:
  - `cargo test --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/backend/Cargo.toml -p backend-app-commands -p backend-adapters-inbound-rest -p backend-app-queries`
  - `cargo test --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/worker/Cargo.toml -p worker-app-processor -p worker-adapters-inbound-poller`
  - `cargo test --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/devctl/Cargo.toml -p devctl-app-core -p devctl-adapters-inbound-cli`
  - `cargo check --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound/transport/Cargo.toml`
  - `cargo check --manifest-path apps/guardrail3/tests/fixtures/r_arch_01/golden/packages/shared-types/Cargo.toml`
- cargo-based verification of the new modules is currently blocked by unrelated existing repo failures:
  - `unused-crate-dependencies` failures in non-`rs/code` targets
  - `dead-code` failures in hook-family placeholder structs during `--lib` test builds
- structural migration is complete across `RS-CODE-01..30`
- `RS-CODE-01..19` have already been pushed through repeated adversarial hardening loops in this lane
- `RS-CODE-20` has now also been converged:
  - prior compile bugs in its sidecar tests were fixed
  - `RS-CODE-03/04` overlap on `ForeignMod` attrs was removed
  - `#[cfg_attr(..., allow(...))]` on foreign mods is now owned by `RS-CODE-20`
  - always-true foreign-mod cfg_attr no longer double-reports with `RS-CODE-18`
  - direct, golden, false-positive, and inventory coverage are all materially stronger now
- current verification state for the lane:
  - `cargo check -p guardrail3 --lib` passes
  - `cargo check -p guardrail3 --tests` is blocked by unrelated code in `apps/guardrail3/crates/app/rs/checks/rs/release/rs_bin_01_binary_release_workflow_tests/bypasses.rs`

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
