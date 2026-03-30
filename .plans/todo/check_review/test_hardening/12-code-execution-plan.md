# Code Hardening Execution Plan

This is the working sequence for the `rs/code` hardening pass.

It is intentionally exhaustive.

The goal is not to choose a subset. The goal is to close the entire lane, step by step, without skipping structural migration, coverage rebuilding, semantic bug fixing, or plan reconciliation.

## Current checkpoint

The early structural phases below are largely complete.

The lane is now in the repeated adversarial convergence phase:
- one rule at a time
- verify when repo blockers permit
- attack from four angles
- fix rule-local bugs
- rerun until convergence

Use the `test-attack` skill for this phase.

Current resume point:
- `RS-CODE-20` converged
- `RS-CODE-21..30` exactness batch in progress

## Phase 0 — Lock Context

1. Re-read the active inputs before making code changes:
   - `AGENTS.md`
   - `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
   - `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
   - `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
   - `.plans/todo/check_review/test_hardening/02-code.md`
   - `.plans/todo/check_review/test_hardening/12-code-agent-brief.md`
   - `.plans/todo/checks/rs/code.md`
2. Read the current `rs/code` family implementation under `apps/guardrail3/crates/app/rs/checks/rs/code/`.
3. Read the old adversarial source corpus:
   - `apps/guardrail3/tests/unit/test_source_scan.rs`
   - `apps/guardrail3/crates/app/rs/validate/source_scan.rs`
   - `apps/guardrail3/crates/app/rs/validate/ast_helpers.rs`
   - `apps/guardrail3/crates/app/rs/validate/ast_visitors.rs`
4. Confirm current repo state and avoid touching unrelated user files.

## Phase 1 — Inventory The Current Family

1. Enumerate every production rule file `RS-CODE-01..30`.
2. Enumerate every current test sidecar file.
3. Record which rules already have:
   - direct rule-local unit coverage
   - family-level or orchestrator coverage
   - any fail-closed coverage
   - any false-positive coverage
   - any exact hit-set assertions
4. Record where the family still violates the structural hardening contract:
   - `*_tests.rs` sidecars still present
   - no rule-specific `*_tests/` directories
   - no golden-tree attack modules
   - loose assertions
5. Record implementation hotspots likely to fail once the tests become adversarial:
   - legacy `ast_helpers` dependence in `parse.rs`
   - source-line reason matching
   - grouped/aliased import handling
   - grouped/aliased attribute handling
   - whole-type `#[garde(skip)]`
   - test/prod path classification
   - fail-closed read/parse behavior

## Phase 2 — Build The Coverage Matrix

1. Create a rule-by-rule matrix for `RS-CODE-01..30`.
2. For each rule, mark coverage against the shared attack model:
   - golden
   - attack vector
   - owned hit set
   - owned non-hit set
   - multi-root
   - nested-root
   - false-positive control
   - fail-closed
   - precedence / inheritance / shadowing
   - severity exactness
3. Map every old validator-era test idea into:
   - current rule ID
   - attack vector represented
   - whether still valid
   - whether needs expansion to a broad golden-tree mutation
4. Record gaps in `.plans/todo/check_review/test_hardening/02-code.md` as work proceeds.

## Phase 3 — Design The New Test Layout

1. Define one rule-specific directory per rule:
   - `rs_code_01_crate_level_allow_tests/`
   - `...`
   - `rs_code_30_input_failures_tests/`
2. For each rule directory, define the needed files by attack class:
   - `mod.rs`
   - `golden.rs`
   - `bypasses.rs`
   - `false_positives.rs`
   - `fail_closed.rs`
   - `severity_exactness.rs`
   - additional files only where the rule needs them
3. Keep helper usage minimal and semantic:
   - no family-wide grouped test file
   - no helper abstractions that hide what attack is being applied
   - only small fixture builders/assertion helpers if they keep rule semantics obvious
4. Update each production rule file to point at its new test directory module instead of `*_tests.rs`.

## Phase 4 — Create Or Reuse Fixture Infrastructure

1. Inspect the existing family tests for reusable direct-input builders.
2. Reuse the existing mixed-monorepo golden tree at `apps/guardrail3/tests/fixtures/r_arch_01/golden/` instead of inventing a second code fixture.
3. Populate that existing golden tree with realistic source files where it still contains comment-only placeholders, and keep extending it until all major app/package slices have believable code.
4. Add the missing root-level config files the tree needs to behave like a plausible mixed Rust/TypeScript project:
   - `guardrail3.toml`
   - `package.json`
   - `pnpm-workspace.yaml`
   - `tsconfig.base.json`
5. Decide what shared fixture support is necessary for broad golden-tree tests.
6. Add only the minimum shared support needed to:
   - build a representative Rust project tree
   - apply one attack vector across all relevant files
   - run the family checker
   - assert exact hit sets and exact non-hit sets
7. Ensure shared support does not hide:
   - which files were mutated
   - which rule is being attacked
   - why a file should or should not hit

### Fixture population target

The target is not a perfect copy of `steady-parent`.

The target is:

- one realistic mixed-monorepo golden tree
- multiple Rust roots with real library and binary code
- multiple TypeScript apps with real routes/components/modules
- shared TS and Rust packages
- enough clean, legal code that each `RS-CODE-*` attack can mutate a believable source surface

## Phase 5 — Migrate The Tests Structurally

1. Convert every rule from `*_tests.rs` to `*_tests/`.
2. Keep compilation passing as the migration proceeds.
3. Remove the old `*_tests.rs` files only after the new rule-specific module directory is wired and compiling.
4. Do not leave any mixed old/new structure behind when the pass is complete.

Status:
- complete for `RS-CODE-01..30`

## Phase 6 — Harden Suppression Rules (`RS-CODE-01..08`)

### `RS-CODE-01` crate-level `#![allow(...)]`

1. Add golden coverage.
2. Add attacks for:
   - crate-level `#![allow(...)]` in owned prod files
   - inline module `#![allow(...)]`
   - nested module placement
   - prod/test severity distinction
3. Add false-positive controls for legal inner attributes that should not hit.
4. Assert exact severities and file sets.

### `RS-CODE-02` justified `#![allow(unused_crate_dependencies)]`

1. Add golden coverage.
2. Add attacks for:
   - present exemption inventory where expected
   - illegal variants that should not be treated as this inventory rule
3. Add exact info-level assertions.
4. Add non-hit controls for unrelated `allow` attributes.

### `RS-CODE-03` item-level `#[allow(...)]` without reason

1. Add golden coverage.
2. Add attacks for:
   - attributes on functions, impl items, modules, structs, enums, traits, extern blocks
   - grouped attributes
   - aliased or formatted variants that should still be detected
   - nested module placements
3. Assert exact hits and exact error severity.
4. Add false-positive controls for documented cases.

### `RS-CODE-04` item-level `#[allow(...)]` with reason

1. Add golden coverage.
2. Add documented-bypass attacks proving the cases are surfaced exactly where expected in normal output.
3. Add false-positive controls for malformed or empty reasons.
4. Assert exact warning severity and hit set.

### `RS-CODE-05` non-primitive `#[garde(skip)]` without comment

1. Add golden coverage.
2. Add attacks for:
   - field-level `#[garde(skip)]`
   - whole-type ownership cases
   - nested type placements
3. Verify current ownership behavior for whole-type skip.
4. If the implementation misses whole-type cases, fix the rule or parser support.
5. Add false-positive controls for primitive fields and legal cases.

### `RS-CODE-06` non-primitive `#[garde(skip)]` with comment

1. Add golden coverage.
2. Add attacks for comment forms that look documented but do not satisfy the contract.
3. Add attacks for exact `// reason:` cases that should remain visible as warnings.
4. Include whole-type and nested placements.
5. Add exact severity and hit-set assertions for both error and warning branches.

### `RS-CODE-07` exception comment inventory

1. Add golden coverage.
2. Add multi-file inventory attacks across all relevant config files.
3. Add false-positive controls for similar comments that are not real exception markers.
4. Assert exact file and line ownership.

### `RS-CODE-08` genuine `cfg_attr(..., allow(...))` inventory

1. Add golden coverage.
2. Add attacks for:
   - conditionally real `cfg_attr`
   - grouped and aliased attribute syntax
   - nested placements
3. Add controls separating this inventory rule from `RS-CODE-18` always-true bypasses.
4. Assert exact info severity and hit set.

Status:
- the suppression tranche has already been through structural hardening
- continue reopening individual rules only if a later adversarial pass exposes a real boundary bug

## Phase 7 — Harden Structure Rules (`RS-CODE-09..12`)

### `RS-CODE-09` file length

1. Add golden coverage.
2. Add attacks across all relevant non-test files in the fixture.
3. Add controls proving comments and legal syntax do not overcount.
4. Add exact hit/non-hit assertions for prod vs test paths.

### `RS-CODE-10` use count error

1. Add golden coverage.
2. Add attacks using grouped imports, split imports, and nested layout variants.
3. Verify threshold accounting on AST structure rather than raw lines.
4. Assert exact error hits.

### `RS-CODE-11` use count warning

1. Add golden coverage.
2. Add threshold-boundary attacks.
3. Add false-positive controls for grouped import shapes that should not count incorrectly.
4. Assert exact warn severity.

### `RS-CODE-12` unsafe_code lint

1. Add golden coverage.
2. Add attacks for:
   - workspace lint set to `deny`
   - workspace lint set to `forbid`
   - missing lint
   - malformed Cargo input triggering `RS-CODE-30` instead
3. Assert exact severity split and exact file ownership.

Status:
- structurally hardened
- only reopen when attack loops surface a concrete miss

## Phase 8 — Harden Quality Rules (`RS-CODE-13..16`)

### `RS-CODE-13` todo/unimplemented/unreachable inventory

1. Add golden coverage.
2. Add attacks for macro placement in nested items and non-test code.
3. Add controls for similar legal identifiers/macros that should not hit.
4. Assert exact warn/info severity per macro type.

### `RS-CODE-14` unwrap/expect

1. Add golden coverage.
2. Add attacks for direct calls, chained calls, and nested placements.
3. Add controls for unrelated methods with similar names.
4. Assert exact hits and warn severity.

### `RS-CODE-15` direct `std::fs` usage

1. Add golden coverage.
2. Add attacks for:
   - direct imports
   - grouped `use std::{fs, ...}`
   - inline calls
   - grouped glob variants
   - nested placements
3. Preserve exemptions for `src/fs.rs` and tests.
4. Add false-positive controls for legal references that should not hit.

### `RS-CODE-16` panic! macro

1. Add golden coverage.
2. Add attacks for panic in nested scopes and non-test code.
3. Add controls for test-only panic and lookalike identifiers.
4. Assert exact info/warn contract currently defined by the rule.

## Phase 9 — Harden Escape-Hatch Rules (`RS-CODE-17..24`)

### `RS-CODE-17` blanket `#[allow]` on impl block

1. Add golden coverage.
2. Add attacks for impl blocks with more than three methods across multiple owned files.
3. Add threshold-boundary controls.
4. Add nested and trait-impl variants where relevant.

### `RS-CODE-18` always-true `cfg_attr` bypass

1. Add golden coverage.
2. Add attacks for:
   - `all()`
   - `any(unix, windows)`
   - `not(nonexistent_target)`
   - other semantically always-true forms supported by the current parser
3. Separate these from genuinely conditional `RS-CODE-08` cases.
4. Fix parser/helper logic if adversarial cases slip through.

### `RS-CODE-19` large type inventory

1. Add golden coverage.
2. Add attacks for struct and enum thresholds.
3. Add boundary controls for legal sizes.
4. Assert exact inventory hit sets.

### `RS-CODE-20` `#[allow]` on extern blocks

1. Add golden coverage.
2. Add attacks for `extern "C"` blocks in multiple placements.
3. Add controls for unrelated foreign items that should not hit.
4. Verify the AST ownership is complete for the current parser.

### `RS-CODE-21` `use std::fs::*`

1. Add golden coverage.
2. Add attacks for:
   - direct glob import
   - grouped `use std::{fs::*, ...}`
   - aliasing forms that still represent the same bypass surface
3. Add controls for legal non-fs glob imports and non-glob fs imports.
4. Fix import-tree traversal if grouped forms are currently missed.

### `RS-CODE-22` undocumented `#[deny]` / `#[forbid]`

1. Add golden coverage.
2. Add attacks for item-level and crate-level forms.
3. Add the special-case control for `#![forbid(unsafe_code)]`.
4. Add grouped lint list and nested placement attacks.

### `RS-CODE-23` `include!` bypass

1. Add golden coverage.
2. Add attacks for:
   - direct `include!()`
   - build-script `OUT_DIR` exception inventory
   - `include_str!` and `include_bytes!` path traversal
   - nested placements
3. Add exact severity split assertions.
4. Add controls for legal include patterns.

### `RS-CODE-24` `#[path = ...]`

1. Add golden coverage.
2. Add attacks for:
   - path traversal with `..`
   - non-traversal `#[path]` that still requires reason handling
   - nested module placements
3. Add exact warn/error split assertions.
4. Add controls for standard module layout that should not hit.

## Phase 10 — Harden Public API / Organization Rules (`RS-CODE-25..29`)

### `RS-CODE-25` public `Result<_, String | Box<dyn Error>>`

1. Add golden coverage.
2. Add attacks across all library-profile files where public API is owned.
3. Add controls for non-public functions and acceptable error types.
4. Verify profile gating is correct and exact.

### `RS-CODE-26` `pub use foo::*` in `lib.rs`

1. Add golden coverage.
2. Add attacks for facade re-export leakage patterns.
3. Add controls for explicit re-exports that are legal.
4. Assert exact library-only ownership.

### `RS-CODE-27` facade-only `lib.rs`

1. Add golden coverage.
2. Add attacks for illegal function bodies, impl blocks, and other body-bearing items.
3. Add controls for allowed doc comments, types, consts, mods, and `pub use`.
4. Assert exact hit and non-hit sets.

### `RS-CODE-28` inline `pub mod` in `lib.rs`

1. Add golden coverage.
2. Add attacks for inline public module bodies.
3. Add controls for private inline modules if legal and for file-backed modules.
4. Assert exact library-surface ownership.

### `RS-CODE-29` trait too large

1. Add golden coverage.
2. Add attacks for warn and error thresholds.
3. Add controls for traits below each threshold and non-public contexts if the rule excludes them.
4. Assert exact severity split.

## Phase 11 — Deepen Fail-Closed Coverage (`RS-CODE-30`)

1. Add golden coverage.
2. Add attacks for:
   - unreadable Rust source
   - unparsable Rust source
   - malformed `Cargo.toml`
   - malformed `guardrail3.toml`
   - any additional code-family policy input that can currently fail open
3. Assert that failures are surfaced instead of silently skipped.
4. Assert exact file ownership and exact error messages where practical.
5. Verify no other rule silently swallows parse/read failures before `RS-CODE-30`.

## Phase 12 — Close Parser / Helper Debt Exposed By The Tests

1. Audit `parse.rs` and identify every current dependency on legacy `validate::ast_helpers`.
2. For each helper-backed surface, decide whether to:
   - migrate logic into `rs/code/parse.rs`
   - keep the helper temporarily but document the debt explicitly
3. Prefer migrating the code-family-specific logic out of legacy helpers when the hardening tests expose semantic gaps.
4. Fix:
   - grouped attribute parsing gaps
   - aliasing gaps
   - whole-type `#[garde(skip)]` gaps
   - foreign-mod attribute ownership gaps
   - cfg_attr classification gaps
5. Re-run the hardened rule tests after each parser/helper change.

## Phase 13 — Verify Discovery / Scope Boundaries

1. Attack `discover.rs` test classification heuristics.
2. Verify prod/test path handling for:
   - `/tests/`
   - `/test/`
   - `__tests__`
   - `_test.rs`
   - `_tests.rs`
   - `/tests.rs`
3. Add controls for similar-but-prod paths that should not be treated as tests.
4. Verify fixture paths remain excluded and do not leak into owned hit sets.
5. Verify profile gating for library-only rules is correct.

## Phase 14 — Tighten Assertion Quality

1. Replace loose assertions in touched tests with:
   - exact result count
   - exact file hit set
   - exact rule ID
   - exact severity
2. Add explicit owned non-hit assertions for:
   - non-Rust roots
   - test files when exempt
   - sibling-family config files when out of scope
   - similar legal syntax
3. Remove any remaining “some result exists” style assertions in the hardened area.

## Phase 15 — Run Full Family Verification

1. Run the `rs/code` test suite.
2. Run any broader Rust test target needed to ensure the family still compiles and integrates.
3. Fix regressions immediately when they reflect real semantic problems.
4. If a failure reflects ambiguous policy rather than a bug, record it in the lane file instead of silently changing the rule contract.

## Phase 16 — Reconcile Plans And Residual Gaps

1. Update `.plans/todo/check_review/test_hardening/02-code.md` with:
   - closed gaps
   - remaining gaps
   - semantic bugs found
   - parser/helper debt still open
2. Update `.plans/todo/checks/rs/code.md` if implementation reality changed in a policy-relevant way.
3. Record any remaining cross-family blocker explicitly rather than leaving it implicit in code comments.

## Phase 17 — Final Closure Checks

1. Confirm every `RS-CODE-*` rule ends with a rule-specific `*_tests/` directory.
2. Confirm every rule has:
   - golden coverage
   - at least one real attack-vector test
   - exact-result assertions where practical
   - false-positive control where relevant
3. Confirm `rs/code` no longer depends on accidental shallow coverage.
4. Confirm any remaining legacy-helper dependence is explicit and documented.
5. Only then treat the lane as complete.

## First Concrete Move

The first concrete move is:

1. produce the full rule-by-rule coverage matrix for `RS-CODE-01..30`
2. identify which current tests can be reused versus replaced
3. identify which parser/helper gaps will block migration
4. then begin structural migration rule by rule starting with `RS-CODE-01..08`, `RS-CODE-17..24`, and `RS-CODE-30`

This is first because the family currently still uses `*_tests.rs` throughout, so changing individual assertions before the matrix and layout pass would create churn without closing the lane structurally.

## Current Status

Completed so far:

1. the coverage matrix was built
2. the migration queue and parser/helper blocker list were written down
3. `RS-CODE-30`, `RS-CODE-01`, `RS-CODE-02`, `RS-CODE-03`, `RS-CODE-04`, `RS-CODE-05`, `RS-CODE-06`, and `RS-CODE-08` were converted to rule-specific `*_tests/` directories
4. a minimal `test_support.rs` was added for exact rule-hit file assertions

Next concrete move:

1. finish populating `apps/guardrail3/tests/fixtures/r_arch_01/golden/` so no app/package slice still depends on placeholder comments
2. use that populated fixture as the shared golden source tree for the remaining `rs/code` hardening work

Progress within that move:

1. root mixed-monorepo configs were added to the golden tree
2. `apps/backend` now has populated Rust planning-service code plus a small MCP inbound slice
3. `apps/worker` now has populated Rust queue-processing code
4. `apps/devctl` now has populated Rust workspace-doctor CLI code
5. `apps/landing`, `apps/admin`, and `apps/portal` now have real TS/TSX routes, modules, adapters, and UI components
6. `packages/shared-types` and `packages/ui-kit` now contain real shared package code
7. comment-only source placeholders have been eliminated from the golden tree
