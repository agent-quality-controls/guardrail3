# Release Hardening Execution Plan

This is the working execution order for the `rs/release` hardening pass.

It is intentionally exhaustive.

The goal is not to decide what to skip.
The goal is to close every structural, semantic, and documentation gap named in the release hardening brief and the active release family plan.

## Scope

This plan covers:

- `RS-RELEASE-01..12`
- `RS-PUB-01..14`
- `RS-BIN-01..03`
- release-family facts, inputs, and support code where required to make the rules actually satisfy the plan
- release-family hardening docs that must be updated as findings are closed

This plan does not split work into “important” vs “optional”.
Everything listed here is expected to be done.

The starting-state audit for this plan lives in:

- `.plans/todo/check_review/test_hardening/13-release-audit-matrix.md`

The current progress snapshot for handoff lives in:

- `.plans/todo/check_review/test_hardening/13-release-agent-brief.md`

## Operating rules

1. Do not treat existing `*_tests.rs` files as acceptable end state.
2. Do not preserve a weak implementation just because a legacy test expected it.
3. Do not add broad “some release error exists” assertions.
4. Do not do a blind rename sweep without strengthening semantics at the same time.
5. For each touched rule, end with:
   - a rule-specific `*_tests/` directory
   - golden coverage
   - at least one real attack-vector test
   - exact-hit and exact-non-hit assertions
   - implementation fixes if the tests expose a checker bug

## Execution order

## Progress snapshot

Completed:

- Phase 1: contract reconstruction
- Phase 2: full family audit matrix
- Phase 3: fact/support inspection for known bug zones
- repo config batch
  - `RS-RELEASE-01`
  - `RS-RELEASE-02`
  - `RS-RELEASE-03`
  - `RS-RELEASE-04`
- workflow semantics batch
  - `RS-RELEASE-05`
  - `RS-RELEASE-06`
  - `RS-RELEASE-07`
  - `RS-BIN-01`
  - `RS-BIN-02`
- remaining config-only rule batch
  - `RS-RELEASE-08`
  - `RS-RELEASE-09`
  - `RS-RELEASE-10`
  - `RS-PUB-01`
  - `RS-PUB-02`
  - `RS-PUB-03`
  - `RS-PUB-06`
  - `RS-PUB-07`
  - `RS-PUB-08`
  - `RS-PUB-12`
  - `RS-PUB-13`
  - `RS-PUB-14`
  - `RS-BIN-03`
- README and publishability batch
  - `RS-PUB-04`
  - `RS-PUB-05`
  - `RS-RELEASE-11`
- inherited-edge batch
  - `RS-PUB-10`
  - `RS-PUB-11`
- fail-closed batch
  - `RS-RELEASE-12`
- richer-fixture dry-run batch
  - `RS-PUB-09`
- lane-doc update in `.plans/todo/check_review/test_hardening/03-release.md`

Still open:

- no concrete release-family rule bug is currently queued
- remaining work, if any, is deeper workflow semantic modeling rather than another known false-positive or false-negative

### Phase 1: Reconstruct the exact contract

1. Read the current source-of-truth docs again in one pass:
   - `AGENTS.md`
   - `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
   - `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
   - `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
   - `.plans/todo/check_review/test_hardening/03-release.md`
   - `.plans/todo/check_review/test_hardening/13-release-agent-brief.md`
   - `.plans/todo/checks/rs/release.md`
2. Restate the release-family intent in concrete terms:
   - what each repo rule should detect
   - what each publishable-crate rule should detect
   - what each release-edge rule should detect
   - what each binary-workflow rule should detect
   - what should explicitly not count as a hit
3. Extract every already-declared gap from the docs into one working checklist:
   - workflow prose/comment false positives
   - missing executable-step detection
   - inherited local path-edge handling
   - publishability inference bugs
   - `readme = false`
   - malformed config and partial-facts fail-closed behavior
   - false positives for non-publishable crates
   - aggregate-heavy rule input concerns
   - semantic `release-plz.toml` and `cliff.toml` baseline gaps

### Phase 2: Build the family audit matrix

4. Enumerate every production rule file in `apps/guardrail3/crates/app/rs/checks/rs/release/`.
5. For each rule, record:
   - rule ID
   - source file
   - current test file
   - current test shape
   - whether the current test shape violates the `*_tests/` directory requirement
   - known attack classes that should apply
   - current attack classes actually covered
   - missing attack classes
   - likely implementation-risk areas
6. Review the old adversarial sources and map each old test idea to:
   - current rule ID
   - attack vector
   - still valid / partially valid / obsolete
7. Use the matrix to define the exact conversion list for all 29 rules.

### Phase 3: Inspect family internals before rewriting tests

8. Read `mod.rs`, `facts.rs`, `inputs.rs`, `release_support.rs`, and `test_support.rs`.
9. Identify which known gaps are test-only and which require implementation changes.
10. Specifically inspect:
   - workflow analysis collection and matching
   - publishability inference logic
   - README path and content handling
   - dependency-edge extraction, including inherited workspace dependencies
   - release input failure collection
   - repo/workflow inventory facts used by binary rules
11. Write down where the current family shape is too aggregate-heavy and whether it blocks exact rule-local attack tests.

### Phase 4: Establish the target test harness shape

12. Define the rule-specific test directory pattern to use consistently:
   - `rule_tests/mod.rs`
   - `rule_tests/golden.rs`
   - attack files grouped by semantic vector
   - false-positive or fail-closed files where needed
13. Preserve helper reuse only in ways that keep test semantics visible.
14. Decide how to share golden-fixture construction across release rules without hiding what each mutation breaks.
15. Keep helper code in support modules, but keep rule semantics in the attack files.

### Phase 5: Convert repo-level rules and harden them

16. Convert `RS-RELEASE-01` to a rule-specific test directory and add:
   - golden coverage
   - missing-license attack
   - exact inventory vs failure assertions
17. Convert `RS-RELEASE-02` similarly and attack:
   - missing `release-plz.toml`
   - malformed `release-plz.toml` interplay with fail-closed handling
18. Convert `RS-RELEASE-03` and attack:
   - publishable crate omitted from release-plz coverage
   - partial package coverage
   - malformed release-plz file
   - non-publishable crates not causing false positives
19. Convert `RS-RELEASE-04` and attack:
   - missing `cliff.toml`
   - malformed `cliff.toml`
20. Convert `RS-RELEASE-05` and harden against:
   - comment-only mention of `release-plz`
   - prose-only mention in non-executable YAML fields
   - step names or descriptions that mention `release-plz` without executing it
   - workflows that use the tool in a real executable step and should pass
21. Convert `RS-RELEASE-06` and harden against:
   - comments or prose mentioning `cargo publish --dry-run`
   - command text in non-executable fields
   - real execution under a script wrapper or shell command
22. Convert `RS-RELEASE-07` and harden against:
   - stray `CARGO_REGISTRY_TOKEN` text in comments or unrelated env locations
   - workflow text that mentions the token but does not wire it into execution context
   - real registry-token wiring in the release flow
23. Convert `RS-RELEASE-08` and ensure:
   - tool present baseline
   - tool missing case
   - exact severity and no unrelated release hits
24. Convert `RS-RELEASE-09` and `RS-RELEASE-10` into directory tests with exact inventory assertions.
25. Convert `RS-RELEASE-11` and attack:
   - internal crates accidentally publishable
   - non-publishable internal crates not falsely hit
   - actually publishable crates with valid metadata not falsely hit
26. Convert `RS-RELEASE-12` and harden fail-closed behavior across:
   - unreadable Cargo manifests
   - unparsable Cargo manifests
   - unreadable README content
   - unparsable workflow YAML
   - unreadable or malformed `release-plz.toml`
    - unreadable or malformed `cliff.toml`
    - partial-facts cases where one bad file must not silently erase the rest of the family’s output

Status:

- `RS-RELEASE-01` completed
- `RS-RELEASE-02` completed
- `RS-RELEASE-03` completed
- `RS-RELEASE-04` completed
- `RS-RELEASE-05` completed
- `RS-RELEASE-06` completed
- `RS-RELEASE-07` completed
- `RS-RELEASE-08` completed
- `RS-RELEASE-09` completed
- `RS-RELEASE-10` completed
- `RS-RELEASE-11` completed
- `RS-RELEASE-12` completed for malformed-config, partial-facts, unreadable-README, and unreadable cached config/workflow coverage

### Phase 6: Convert publishable-crate rules and harden them

27. Convert `RS-PUB-01` through `RS-PUB-03` with:
   - golden coverage
   - missing required metadata attacks
   - non-publishable false-positive checks
28. Convert `RS-PUB-04` and fix the `readme = false` policy bug:
   - establish expected behavior from the family plan
   - add attack cases for explicit string path, default README, missing README, and `readme = false`
   - ensure non-publishable crates do not trigger false positives
29. Convert `RS-PUB-05` and harden README quality checks against:
   - unreadable README
   - too-short README
   - heading-less README
   - `readme = false` interaction
   - non-publishable crate false positives
30. Convert `RS-PUB-06` and `RS-PUB-07` with exact count and severity assertions.
31. Convert `RS-PUB-08` and attack:
   - invalid semver
   - `workspace = true` valid inheritance
   - non-publishable crate behavior
32. Convert `RS-PUB-09` and verify:
   - thorough-mode-only behavior
   - exact command outcome handling
   - no legacy stderr-text heuristics
33. Convert `RS-PUB-12`, `RS-PUB-13`, and `RS-PUB-14` with exact inventory and false-positive coverage.

Status:

- `RS-PUB-01` completed
- `RS-PUB-02` completed
- `RS-PUB-03` completed
- `RS-PUB-04` completed
- `RS-PUB-05` completed except for true unreadable-file attacks
- `RS-PUB-06` completed
- `RS-PUB-07` completed
- `RS-PUB-08` completed
- `RS-PUB-09` completed with real richer-fixture dry-run pass/fail coverage
- `RS-PUB-12` completed
- `RS-PUB-13` completed
- `RS-PUB-14` completed

### Phase 7: Convert release-edge rules and fix inherited-edge semantics

34. Convert `RS-PUB-10` and attack every relevant dependency surface:
   - normal dependencies
   - build dependencies
   - target-specific dependencies
   - inherited `workspace = true` local path edges
   - local path to non-publishable crates
   - local path to publishable crates
35. Fix fact extraction if inherited workspace dependency edges are currently invisible or under-classified.
36. Convert `RS-PUB-11` and attack:
   - direct local publishable crate edges with incompatible versions
   - inherited `workspace = true` version/path edges
   - compatible versions that should not hit
   - non-publishable dependency edges that should not produce false positives for this rule
37. Fix the edge collector and/or rule inputs if current facts are too lossy to express the inherited-edge cases exactly.

Status:

- `RS-PUB-10` completed
- `RS-PUB-11` completed
- inherited `workspace = true` path-edge extraction fix is already landed in `release_support.rs`

### Phase 8: Convert binary release rules and harden workflow semantics

38. Convert `RS-BIN-01` and harden against:
   - comments or prose naming release actions
   - fake use of release actions outside executable workflow context
   - real binary release workflow using executable release steps
39. Convert `RS-BIN-02` and harden against:
   - fake Linux target strings in comments or display text
   - real Linux target presence in executable workflow configuration
40. Convert `RS-BIN-03` with:
  - binary crate with correct binstall metadata
  - binary crate missing metadata
  - library crate false-positive protection

Status:

- `RS-BIN-01` completed
- `RS-BIN-02` completed
- `RS-BIN-03` completed

### Phase 9: Remove old test structure completely

41. Delete every release-family `*_tests.rs` file after its replacement directory is in place.
42. Update every production rule file’s `#[cfg(test)]` module path to the new directory `mod.rs`.
43. Ensure no release-family rule ends with a flat `*_tests.rs` file.
44. Ensure there are no grouped family-wide rule test files left behind.

### Phase 10: Fix implementation exposed by the hardened suite

45. Patch `facts.rs`, `inputs.rs`, and `release_support.rs` wherever the hardened tests expose actual checker bugs.
46. Expected implementation-fix areas to verify and close explicitly:
   - `readme = false`
   - workflow command-context detection
   - registry-token wiring semantics
   - inherited local path-edge extraction
   - publishability inference edge cases
   - fail-closed behavior for malformed or unreadable inputs
   - partial-facts behavior that currently over-skips or under-reports
47. If any rule still depends on aggregate-heavy facts that make exact local assertions impossible, refactor the family facts or inputs enough to restore clean rule-local tests.

### Phase 11: Add semantic baseline checks if they are still part of the live release contract

48. Reconcile the release-family plan’s note about semantic `release-plz.toml` and `cliff.toml` baselines with current implementation.
49. If those semantic checks are already part of the intended live contract, implement them and test them directly.
50. If they are not yet active policy, record the gap explicitly in the lane doc instead of leaving it implicit.

### Phase 12: Verify exhaustively

51. Run release-family targeted tests repeatedly during the conversion, not only at the end.
52. Run the final release-family test set after all rule migrations and fixes.
53. Confirm exact expectations:
   - every rule has golden coverage
   - every rule has at least one attack-vector test
   - every touched test asserts exact owned hits and non-hits
   - workflow prose/comments cannot satisfy workflow rules
   - inherited path-edge cases are covered directly
   - non-publishable crates are protected against false positives
54. Re-run any broader Rust test target needed to make sure the family still integrates with the checker entrypoint.

Current verification state:

- `cargo check -p guardrail3 --lib` passes
- `cargo check -p guardrail3 --tests` passes
- `cargo test -p guardrail3 --lib --no-run` passes
- targeted release-family suites are green for `RS-RELEASE-01`, `RS-RELEASE-03`, `RS-RELEASE-04`, `RS-RELEASE-05`, `RS-RELEASE-06`, `RS-RELEASE-07`, `RS-PUB-02`, `RS-PUB-13`, `RS-PUB-14`, `RS-RELEASE-12`, `RS-BIN-01`, `RS-BIN-02`, and `RS-BIN-03`

### Phase 13: Update planning docs as part of the work, not afterward

55. Update `.plans/todo/check_review/test_hardening/03-release.md` with:
   - closed gaps
   - remaining gaps
   - any policy questions exposed by implementation
56. Keep the lane doc concrete:
   - note what was fixed in code
   - note what was fixed only in tests
   - note any unresolved ambiguity that requires explicit product direction

### Phase 14: Finish cleanly

57. Confirm the release-family directory no longer violates the hardening structure rules.
58. Confirm the codebase still follows the one-rule-per-file rule.
59. Before any commit:
   - generate the worklog filename with the required `date` command
   - write the worklog
   - stage it with the code changes
60. In the worklog, record:
   - which rules were migrated structurally
   - which semantic checker bugs were fixed
   - which attack vectors were added
   - what remains open, if anything

## First action

The first action is:

1. build the full audit matrix for all 29 release-family rules

Reason:

- the family is still entirely in legacy `*_tests.rs` structure
- the known gaps span tests, facts, and rule behavior
- a complete matrix is the only safe way to guarantee that every rule ends with both structural compliance and semantic hardening

The second action is:

2. inspect `facts.rs` and `release_support.rs` specifically for the already-known semantic bug zones before rewriting tests blindly

Reason:

- `readme = false`
- inherited `workspace = true` edges
- workflow step semantics
- fail-closed input handling

Those areas are likely to require implementation changes, and the test rewrite should be designed to expose them directly rather than discover them accidentally halfway through the migration.

## Audit-confirmed first batch

The audit confirmed that the first implementation batch should be:

1. workflow semantics
   - `RS-RELEASE-05`
   - `RS-RELEASE-06`
   - `RS-RELEASE-07`
   - `RS-BIN-01`
   - `RS-BIN-02`
2. README and publishability semantics
   - `RS-PUB-04`
   - `RS-PUB-05`
   - `RS-RELEASE-11`
3. inherited local-edge semantics
   - `RS-PUB-10`
   - `RS-PUB-11`
4. fail-closed coverage
   - `RS-RELEASE-12`
5. remaining structural conversion across the rest of the family

Reason:

- these rules are the places where `facts.rs` and `release_support.rs` are currently weakest
- migrating them first avoids encoding current family bugs into the new hardened test directories
