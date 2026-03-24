# Hexarch Execution Plan

This is the full execution order for hardening `rs/hexarch`.

It is intentionally exhaustive.

The goal is not to choose a subset.
The goal is to close the entire family lane in a controlled order without losing track of any rule, attack class, or semantic bug.

## First action

Start by building the family coverage matrix before changing behavior.

Reason:
- it prevents blind porting
- it forces every old test idea into a current rule/attack-vector bucket
- it exposes where current tests are only unit-shaped and where they actually exercise orchestrator behavior

## Ordered execution plan

### Exhaustive agent protocol

For the final hardening phase, every rule must go through this explicit sequence:
1. Round 1 with 4 attack agents
2. Local fixes/tests/docs update
3. Round 2 with 4 fresh attack agents
4. Local fixes/tests/docs update
5. If round 2 still finds meaningful hardness improvements, keep running more attack rounds for that same rule until the last fresh round does not produce a worthwhile strengthening change
6. Only then move to the next rule

The 4 agent roles per round are:
- intent vs implementation
- missing scenarios / old-corpus parity
- false positives / ownership boundaries
- fixture and mutation realism

1. Build a rule inventory for `RS-HEXARCH-01..25`.
   For each rule, record:
   - production file
   - current test file
   - expected attack classes from the lane docs
   - whether coverage is currently pure-rule only, fixture-backed, or absent
   - whether fail-closed behavior is expected

2. Mine the old corpus and map every old test into the new family.
   Sources:
   - `apps/guardrail3/tests/unit/rs_arch_01/`
   - `apps/guardrail3/tests/unit/test_hex_arch_checks.rs`
   - `apps/guardrail3/tests/fixtures/r_arch_01/`
   For each old test, record:
   - old source file and test name
   - current `RS-HEXARCH-*` rule target
   - attack vector represented
   - whether it is still valid under the current rule contract
   - whether the new family already covers it

3. Write the missing-coverage matrix into the hexarch lane file.
   Add a rule-by-rule checklist in `.plans/todo/check_review/test_hardening/01-hexarch.md` showing:
   - golden coverage
   - broad attack-vector coverage
   - multi-root coverage
   - nested-root coverage
   - false-positive coverage
   - fail-closed coverage
   - severity exactness
   - implementation bug found / fixed / still open

4. Normalize the test module structure rule by rule.
   For every `RS-HEXARCH-*` rule:
   - replace `*_tests.rs` with a `*_tests/` directory
   - add `mod.rs`
   - split files by semantic attack class where needed
   - keep the production file wired to the new module directory
   Do this while hardening the rule, not as a blind rename sweep.

5. Harden shared test support before broad migration if gaps appear.
   Expand `apps/guardrail3/crates/app/rs/checks/rs/hexarch/test_support.rs` only where needed to support:
   - exact hit-set assertions
   - exact non-hit-set assertions
   - multi-root fixture mutation helpers
   - nested-root mutation helpers
   - severity assertions
   - fail-closed file/config mutation helpers
   Test helpers must not hide rule semantics.

6. Fix family-level fail-open behavior in source parsing before finalizing source-rule tests.
   Audit and correct `apps/guardrail3/crates/app/rs/checks/rs/hexarch/source_facts.rs` so unreadable or unparsable Rust source cannot silently suppress `RS-HEXARCH-22` or `RS-HEXARCH-23`.
   If this needs new facts or a new failure input path, add them explicitly and test them directly.

7. Fix family-level fail-open behavior in boundary-config parsing before finalizing boundary tests.
   Audit and correct `apps/guardrail3/crates/app/rs/checks/rs/hexarch/dependency_facts.rs` so malformed `guardrail3.toml` cannot silently behave like “config absent but valid”.
   Decide and implement the explicit fail-closed reporting path.

8. Harden the structural root rules `RS-HEXARCH-01..06`.
   For each rule:
   - add golden coverage
   - add a broad attack that mutates all owned top-level Rust hex roots
   - add a nested-root attack where applicable
   - add false-positive coverage for non-owned or non-Rust roots
   - add exact hit/non-hit assertions
   - add severity exactness assertions

9. Rebuild `RS-HEXARCH-01` around exhaustive root-existence attacks.
   Cover:
   - missing `crates/` across all owned app roots
   - file/symlink replacement cases if still valid under `ProjectTree`
   - valid `.gitkeep` presence behavior
   - non-owned root non-hits

10. Rebuild `RS-HEXARCH-02` around exact top-level crate-dir ownership.
   Cover:
   - missing required dirs across all owned roots
   - illegal extra siblings across all owned roots
   - nested-root parity
   - optional `crates/macros/`
   - false positives against non-owned roots
   - file/symlink replacement edge cases from the old corpus if still applicable

11. Rebuild `RS-HEXARCH-03` for directional container parity.
   Cover:
   - missing `inbound/` and `outbound/` across all matching containers
   - nested-root parity
   - illegal extra siblings
   - replaced-dir edge cases from the old corpus

12. Rebuild `RS-HEXARCH-04` for loose-file attacks.
   Cover:
   - broad loose-file insertion in all structural/container dirs that should reject it
   - `.gitkeep` allow behavior
   - nearby valid files outside owned dirs as false-positive controls

13. Rebuild `RS-HEXARCH-05` for empty-container attacks.
   Cover:
   - emptying all matching containers at once
   - allowed `.gitkeep` semantics
   - nested-root parity where relevant

14. Rebuild `RS-HEXARCH-06` for leaf-validity attacks.
   Cover:
   - invalid leaves everywhere the rule owns
   - valid leaf alternatives
   - nested hex-in-hex allowance
   - false positives around non-leaf directories

15. Harden workspace-coverage rules `RS-HEXARCH-07..11`.
   For each rule:
   - add golden coverage
   - mutate all applicable app workspaces together
   - assert exact owned hit/non-hit sets
   - add malformed-manifest fail-closed coverage where expected
   - assert exact severity

16. Rebuild `RS-HEXARCH-07` for missing member coverage.
   Cover:
   - removing owned crate dirs from workspace membership everywhere
   - nested root interactions if any member discovery depends on them
   - exact expected app hit set

17. Rebuild `RS-HEXARCH-08` for app workspace declaration correctness.
   Cover:
   - app `Cargo.toml` missing `[workspace]`
   - malformed app `Cargo.toml`
   - false positives for valid workspace manifests

18. Rebuild `RS-HEXARCH-09` for extra workspace member attacks.
   Cover:
   - adding extra members in all app workspaces
   - exact owned hit set
   - non-hit controls for valid members

19. Rebuild `RS-HEXARCH-10` for out-of-boundary member attacks.
   Cover:
   - members escaping the app boundary everywhere
   - exact offending member paths
   - false positives for valid in-boundary members

20. Rebuild `RS-HEXARCH-11` for root-workspace leakage.
   Cover:
   - root workspace including app crates
   - exact hit/non-hit behavior
   - valid root workspace controls

21. Rebuild `RS-HEXARCH-12` for banned app-level `src/`.
   Cover:
   - adding `src/` to all owned app roots at once
   - non-hit controls for source dirs inside actual crates
   - exact app hit set

22. Harden dependency and boundary rules `RS-HEXARCH-13..25`.
   For each rule:
   - golden coverage
   - at least one broad attack vector
   - exact edge/member/file hit sets
   - exact non-hit sets
   - severity exactness
   - fail-closed coverage for malformed inputs when the rule depends on parsing

23. Rebuild `RS-HEXARCH-13` for illegal direction permutations.
   Cover:
   - all illegal layer-direction edges the policy forbids
   - broad mutation across all matching member pairs in the fixture
   - allowed-direction controls

24. Rebuild `RS-HEXARCH-14` as exact inventory behavior.
   Cover:
   - golden inventory expectations
   - exact count / exact edge assertions
   - non-inventory rule separation so attack tests do not blur `Info` inventory semantics

25. Rebuild `RS-HEXARCH-15` for boundary-config semantics.
   Cover:
   - missing config entries for all app boundaries
   - valid config presence controls
   - malformed `guardrail3.toml` fail-closed behavior
   - exact severity and file attribution

26. Rebuild `RS-HEXARCH-16` for patch/replace bypasses.
   Cover:
   - `[patch.*]` path overrides into layered trees
   - `[replace]` path overrides
   - multiple workspace roots if applicable
   - valid patch targets outside owned layered trees as non-hits

27. Rebuild `RS-HEXARCH-17` for inherited workspace dependency bypasses.
   Cover:
   - `workspace = true` edges resolving to illegal directions
   - allowed inherited edges as controls
   - exact source/target assertions

28. Rebuild `RS-HEXARCH-18` for renamed dependency bypasses.
   Cover:
   - alias + `package` renames that cross forbidden directions
   - controls proving aliasing does not overfire on valid targets

29. Rebuild `RS-HEXARCH-19` for same-layer cycles.
   Cover:
   - real same-layer cycles
   - non-cycle same-layer chains
   - cross-layer behavior if intentionally out of scope

30. Rebuild `RS-HEXARCH-20` for dev-dependency direction violations.
   Cover:
   - illegal dev edges
   - contrast with normal dependency severity differences
   - exact warning severity

31. Rebuild `RS-HEXARCH-21` for domain purity.
   Cover:
   - forbidden external deps
   - `optional = true` deps must still be checked
   - allowed pure-crate allowlist controls
   - user-configured `allowed_deps`
   - workspace path deps to allowed layers
   - false-positive controls for valid domain manifests

32. Rebuild `RS-HEXARCH-22` for ports trait-dominance.
   Cover:
   - impl-heavy ports crates through the real source collector path
   - multiple source files within one crate
   - false positives for DTO-heavy but trait-dominant ports crates if still allowed by policy
   - unreadable/unparsable Rust source fail-closed behavior

33. Rebuild `RS-HEXARCH-23` for adapter public-trait bans.
   Cover:
   - `pub trait` in adapter crates through the real source collector path
   - `pub(crate) trait` as explicit non-hit
   - multi-file adapter crates
   - unreadable/unparsable Rust source fail-closed behavior

34. Rebuild `RS-HEXARCH-24` for cross-app boundary leaks.
   Cover:
   - path deps crossing app boundaries
   - exact source app and target app assertions
   - controls for allowed package/shared dependencies

35. Rebuild `RS-HEXARCH-25` for target-specific direction checks.
   Cover:
   - `target.'cfg(...)'.dependencies`
   - `target.'cfg(...)'.dev-dependencies`
   - `target.'cfg(...)'.build-dependencies` if the rule/facts own them
   - controls for valid target-specific edges

36. Add explicit false-positive sweeps after all rule rewrites.
   Run targeted attacks against nearby-valid structures and confirm rules do not overfire on:
   - non-Rust roots
   - non-owned directories
   - optional `macros/`
   - valid package/shared crates
   - valid inherited dependency cases

37. Add exact severity verification everywhere it is currently implicit.
   Every hardened rule should prove:
   - exact rule ID
   - exact severity
   - exact file/path attribution where applicable

38. Review orchestrator/rule boundaries while hardening.
   If tests expose rule files rediscovering state or collectors leaking semantics:
   - fix the architecture violation in code
   - record the change in the lane file
   Do not patch around architectural drift with test-only workarounds.

39. Update `.plans/todo/check_review/test_hardening/01-hexarch.md` continuously as gaps close.
   For each rule or rule group, mark:
   - closed gaps
   - remaining gaps
   - semantic bugs found
   - policy questions, if any

40. Finish with a full family review against the done criteria.
   Confirm:
   - every `RS-HEXARCH-*` rule has a rule-specific `*_tests/` directory
   - every rule has golden coverage
   - every rule has at least one real attack-vector test
   - exact owned hit/non-hit assertions are used
   - all discovered semantic bugs were fixed or written into the lane file

## Practical first implementation slice

If resuming from zero context, do these first:

1. Build the coverage matrix and old-to-new test mapping.
2. Convert the worst current test-shape offenders into directory modules while touching them.
3. Fix fail-open behavior in `source_facts.rs`.
4. Fix fail-open behavior in `dependency_facts.rs` for malformed `guardrail3.toml`.
5. Rebuild `RS-HEXARCH-01..06`.
6. Rebuild `RS-HEXARCH-07..12`.
7. Rebuild `RS-HEXARCH-13..20`.
8. Rebuild `RS-HEXARCH-21..25`.
9. Run a final false-positive and severity sweep.
