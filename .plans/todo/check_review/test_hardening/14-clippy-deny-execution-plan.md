# Clippy And Deny Execution Plan

This is the working order of operations for fully hardening the `rs/clippy` and `rs/deny` families.

It is not a prioritization document. The intent is to do all of this, in this order, until the lane is actually finished.

## Progress snapshot

Completed so far:
- rule inventory and first-pass coverage matrix
- generator-backed parity support for clippy and deny
- migration of:
  - `RS-CLIPPY-12`
  - `g3rs-clippy/local-policy-root`
  - `RS-CLIPPY-14`
  - `RS-CLIPPY-19`
  - `RS-CLIPPY-01`
  - `RS-CLIPPY-04`
  - `RS-CLIPPY-05`
  - `RS-DENY-02`
  - `RS-DENY-03`
  - `g3rs-deny/tokio-full-ban`
  - `g3rs-deny/duplicate-entries`
  - `g3rs-deny/unknown-keys`
  - `g3rs-deny/allow-override-channel`
  - `g3rs-deny/extra-deny-bans-inventory`
  - `RS-DENY-30`
  - `RS-DENY-01`
  - `RS-DENY-09`
  - `g3rs-deny/confidence-threshold`
  - `g3rs-deny/allow-git-inventory`
  - `g3rs-deny/extra-feature-bans-inventory`
- support helpers that copy the real `tests/fixtures/r_arch_01/golden/` scaffold and run clippy/deny families end-to-end

Current next rule targets in this order:
- refresh `14-clippy-deny-coverage-matrix.md` to current migrated directory state
- audit remaining unmigrated clippy rules after `RS-CLIPPY-05`
- audit remaining unmigrated deny rules after `g3rs-deny/extra-feature-bans-inventory`

## First action

1. Build the complete rule inventory and coverage matrix before changing test structure.
   - list every `RS-CLIPPY-*` rule file
   - list every `RS-DENY-*` rule file
   - list the current sidecar test file for each rule
   - list the current attack classes covered by each rule
   - list the attack classes missing for each rule
   - map old adversarial tests and old validator tests onto current rule IDs and current family semantics

This first step prevents blind directory churn and makes the migration measurable.

## Inputs to read and mine

2. Re-read the family contract and hardening contract together:
   - `00-shared-test-story.md`
   - `04-clippy-and-deny.md`
   - `14-clippy-deny-agent-brief.md`
   - `99-family-agent-playbook.md`
   - `.plans/todo/checks/rs/clippy.md`
   - `.plans/todo/checks/rs/deny.md`

3. Read the current clippy family code under `apps/guardrail3/crates/app/rs/checks/rs/clippy/`.

4. Read the current deny family code under `apps/guardrail3/crates/app/rs/checks/rs/deny/`.

5. Read the generator baselines and canonical module sources:
   - `apps/guardrail3/crates/domain/modules/clippy/`
   - `apps/guardrail3/crates/domain/modules/deny.rs`

6. Mine all older adversarial and validator-era sources for attack vectors:
   - `apps/guardrail3/tests/adversarial_config_tests.rs`
   - `apps/guardrail3/tests/fixtures/adversarial-configs/`
   - `apps/guardrail3/crates/app/rs/validate/clippy_checks.rs`
   - `apps/guardrail3/crates/app/rs/validate/clippy_coverage.rs`
   - old deny validator files under `apps/guardrail3/crates/app/rs/validate/`
   - `apps/guardrail3/tests/unit/deny_inventory_test.rs`
   - `apps/guardrail3/tests/adversarial_generate.rs`

## Coverage matrix and migration ledger

7. Create a clippy matrix with one row per rule and these columns:
   - rule ID
   - production file
   - current test file
   - golden coverage present or absent
   - parity coverage present or absent
   - root-resolution coverage present or absent
   - mixed-profile coverage present or absent
   - malformed-input coverage present or absent
   - false-positive coverage present or absent
   - severity exactness present or absent
   - known policy question

8. Create the same matrix for deny.

9. For every old test found, record:
   - which current rule it belongs to
   - which attack vector it represents
   - whether that vector is still valid
   - whether it should become golden, attack, fail-closed, false-positive, parity, precedence, or severity coverage

10. Mark the current structural gap explicitly:
   - every clippy rule still has a flat `*_tests.rs`
   - every deny rule still has a flat `*_tests.rs`
   - every touched rule must end with a rule-specific `*_tests/` directory

## Shared test infrastructure before mass migration

11. Inspect existing family test helpers in:
   - `apps/guardrail3/crates/app/rs/checks/rs/clippy/test_support.rs`
   - `apps/guardrail3/crates/app/rs/checks/rs/deny/test_support.rs`

12. Add or reshape shared helpers only where they reduce duplication without hiding rule semantics.

13. Add direct generator-vs-checker parity support for clippy.
   - the source of truth must be the generator modules, not a copied hardcoded fixture
   - parity assertions must compare exact owned outcomes, not broad family presence

14. Add direct generator-vs-checker parity support for deny under the same constraint.

15. Ensure the parity helpers can exercise:
   - policy-root placement
   - nested roots
   - mixed profile or layer selection
   - same-root precedence
   - malformed escape-hatch entries

16. Remove or neutralize any drift-prone family-local baseline fixture that can silently diverge from the generator without detection.

## Clippy execution order

17. Migrate clippy rule tests one rule at a time from `*_tests.rs` to `*_tests/` directories.

18. Use this rule order for clippy so the root and policy semantics are hardened before the lower-risk threshold checks:
   - `RS-CLIPPY-12`
   - `g3rs-clippy/local-policy-root`
   - `RS-CLIPPY-14`
   - `RS-CLIPPY-19`
   - `RS-CLIPPY-01`
   - `RS-CLIPPY-04`
   - `RS-CLIPPY-05`
   - `g3rs-clippy/package-native-policy`
   - `RS-CLIPPY-07`
   - `RS-CLIPPY-08`
   - `g3rs-clippy/no-op-placeholder`
   - `RS-CLIPPY-16`
   - `g3rs-clippy/avoid-breaking-exported-api`
   - `RS-CLIPPY-18`
   - `RS-CLIPPY-20`
   - `g3rs-clippy/max-struct-bools`
   - `g3rs-clippy/max-fn-params-bools`
   - `g3rs-clippy/type-complexity-threshold`
   - `g3rs-clippy/missing-method-ban`
   - `g3rs-clippy/missing-type-ban`
   - `g3rs-clippy/policy-context-parseable`
   - `g3rs-clippy/forbid-clippy-conf-dir-override`

19. For each clippy rule, create a rule-specific test module directory with semantic files only as needed:
   - `mod.rs`
   - `golden.rs`
   - `parity.rs`
   - `bypasses.rs`
   - `multi_root.rs`
   - `nested_root.rs`
   - `precedence.rs`
   - `false_positives.rs`
   - `fail_closed.rs`
   - `severity_exactness.rs`

20. For each clippy rule, port or rebuild golden coverage first.

21. For each clippy rule, add at least one broad attack-vector test that mutates the golden fixture everywhere the vector should matter.

22. For each clippy rule, replace loose assertions with:
   - exact owned hit set
   - exact owned non-hit set
   - exact rule ID
   - exact severity

23. For clippy root and placement rules, explicitly attack:
   - policy-root placement
   - local root baseline rules
   - shadowing by closer configs
   - same-root precedence conflicts
   - mixed profile and layer cases
   - nested root handling where applicable

24. For clippy ban and inventory rules, explicitly attack:
   - missing bans
   - extra bans
   - duplicate entries
   - malformed reasons
   - trivial reasons
   - temporary escape-hatch behavior

25. For clippy threshold rules, explicitly attack:
   - off-by-one thresholds
   - profile-specific overrides
   - mixed threshold declarations
   - similar-but-valid neighboring thresholds

26. Treat `RS-CLIPPY-19` as an honesty checkpoint.
   - verify the current temporary heuristic behavior exactly
   - do not claim stronger semantics than the implementation actually guarantees
   - if generator and checker disagree, document the gap before changing policy

27. After each clippy rule migration:
   - remove the old `*_tests.rs` entry for that rule
   - run the targeted test scope
   - fix any semantic bug found before moving on

## Deny execution order

28. Migrate deny rule tests one rule at a time from `*_tests.rs` to `*_tests/` directories.

29. Use this rule order for deny so location, precedence, profile, and escape-hatch semantics are hardened before the more static inventory rules:
   - `RS-DENY-02`
   - `RS-DENY-03`
   - `g3rs-deny/tokio-full-ban`
   - `g3rs-deny/duplicate-entries`
   - `g3rs-deny/unknown-keys`
   - `g3rs-deny/extra-deny-bans-inventory`
   - `RS-DENY-30`
   - `g3rs-deny/allow-override-channel`
   - `RS-DENY-01`
   - `g3rs-deny/deprecated-advisories`
   - `g3rs-deny/advisories-baseline`
   - `g3rs-deny/stricter-advisories-inventory`
   - `g3rs-deny/graph-all-features`
   - `g3rs-deny/graph-no-default-features`
   - `RS-DENY-09`
   - `g3rs-deny/highlight-inventory`
   - `g3rs-deny/allow-wildcard-paths`
   - `g3rs-deny/wildcards-inventory`
   - `g3rs-deny/license-allow-baseline`
   - `g3rs-deny/confidence-threshold`
   - `g3rs-deny/copyleft-allowlist`
   - `g3rs-deny/unknown-sources-policy`
   - `RS-DENY-17`
   - `g3rs-deny/allow-git-inventory`
   - `g3rs-deny/extra-feature-bans-inventory`
   - `g3rs-deny/skip-hygiene`
   - `g3rs-deny/ignore-hygiene`
   - `RS-DENY-25`
   - `RS-DENY-26`
   - `g3rs-deny/license-exceptions-inventory`

30. For each deny rule, create a rule-specific test module directory with semantic files only as needed:
   - `mod.rs`
   - `golden.rs`
   - `parity.rs`
   - `bypasses.rs`
   - `multi_root.rs`
   - `nested_root.rs`
   - `precedence.rs`
   - `false_positives.rs`
   - `fail_closed.rs`
   - `severity_exactness.rs`

31. For each deny rule, port or rebuild golden coverage first.

32. For each deny rule, add at least one broad attack-vector test that mutates every relevant deny root affected by that vector.

33. For each deny rule, replace loose assertions with:
   - exact owned hit set
   - exact owned non-hit set
   - exact rule ID
   - exact severity

34. For deny location and precedence rules, explicitly attack:
   - policy-root placement and shadowing
   - nested config placement
   - same-root precedence conflicts
   - mixed workspace profile selection
   - profile or layer disagreement

35. For deny inventory and policy rules, explicitly attack:
   - malformed exceptions
   - malformed skips
   - malformed ignores
   - malformed wrappers
   - duplicate entries
   - unknown keys
   - exact severity for inventory-only versus hard-error branches

36. For deny generator-sensitive rules, explicitly compare checker behavior against generated baseline output instead of trusting copied fixtures.

37. Resolve `g3rs-deny/tokio-full-ban` explicitly.
   - confirm the intended policy source
   - update tests to lock that decision
   - if the correct policy is still ambiguous, stop and record the ambiguity in the lane doc instead of guessing

38. After each deny rule migration:
   - remove the old `*_tests.rs` entry for that rule
   - run the targeted test scope
   - fix any semantic bug found before moving on

## Family-wide bug fixing and drift removal

39. When parity or attack tests reveal implementation bugs, fix the implementation in the family code immediately rather than weakening the tests.

40. If a bug crosses orchestrator and rule boundaries, inspect:
   - `facts.rs`
   - `inputs.rs`
   - family support helpers
   - the rule file itself

41. Do not silently change family policy to satisfy current implementation.

42. If a rule contract is ambiguous, record the policy question in the lane doc before making a behavioral change.

43. Remove or rewrite any test that only proves broad family output rather than rule-owned exact output.

44. Remove any hidden semantic assumptions living only in test helpers or opaque fixture naming.

## Final verification and closeout

45. Run the clippy family test suite after all clippy migrations and fixes are complete.

46. Run the deny family test suite after all deny migrations and fixes are complete.

47. Run any broader Rust test scopes needed to catch cross-family fallout from shared helpers or parser behavior.

48. Verify structurally that no rule-specific `*_tests.rs` files remain under:
   - `apps/guardrail3/crates/app/rs/checks/rs/clippy/`
   - `apps/guardrail3/crates/app/rs/checks/rs/deny/`

49. Verify every rule now has:
   - a rule-specific `*_tests/` directory
   - golden coverage
   - at least one real attack-vector test
   - exact result assertions
   - exact severity assertions where severity is meaningful

50. Update `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md` with:
   - closed gaps
   - remaining gaps
   - policy decisions reached
   - unresolved policy questions
   - any generator/checker disagreement that still needs product resolution

51. Re-check the coverage matrices and mark every rule complete only when:
   - structure is migrated
   - parity risk is addressed
   - attack-vector coverage exists
   - false-positive checks exist where relevant
   - fail-closed checks exist where relevant
   - severity exactness is asserted

52. Do one final pass over the family directories to confirm there are no grouped family test files, no grouped production rule files, and no drift-prone hidden baselines left behind.
