# Harden rs/garde family tests

**Date:** 2026-03-24 08:50
**Task:** Harden the `rs/garde` checker family tests according to the family agent playbook and shared test story.

## Goal
To convert all `rs/garde` rules to use rule-specific `*_tests/` directories, add comprehensive golden and attack-vector tests covering exact owned hits/non-hits, and fix any semantic bugs found during the process.

## Input Information
- `19-garde-agent-brief.md`
- `AGENTS.md`
- `2026-03-21-153251-checker-architecture.md`
- `00-shared-test-story.md`
- `99-family-agent-playbook.md`
- `rs/garde.md`

## Approach

### Step-by-step plan
1.  **Audit Current State:** Explore `apps/guardrail3/crates/app/rs/checks/rs/garde/` to map existing rules (`RS-GARDE-01` to `RS-GARDE-10`) to their current sidecar test files (`*_tests.rs`).
2.  **Audit Old Tests:** Map old legacy tests from `apps/guardrail3/tests/unit/test_garde_checks.rs` to current rule IDs and attack vectors.
3.  **Convert Test Layout:** For each rule, create the `rs_garde_XX_..._tests/` directory structure.
    -   Move existing logic or seed logic into the new modular layout (`golden.rs`, attack vectors, `false_positives.rs`, `fail_closed.rs`, etc.).
4.  **Harden High-Risk Rules First:**
    -   `RS-GARDE-05` (Struct derive inventory)
    -   `RS-GARDE-07` (Manual Deserialize impl)
    -   `RS-GARDE-08` (Enum derive inventory)
    -   `RS-GARDE-10` (Input failures)
5.  **Harden Remaining Rules:** Harden `RS-GARDE-01`, `02`, `03`, `04`, `06`, `09`.
6.  **Verify Assertions:** Ensure all tests assert exact rule ID, severity, target sets, and cover multi-root / fail-closed scenarios.
7.  **Bug Fixes:** Address any semantic bugs uncovered during testing.
8.  **Update Documentation:** Update `.plans/todo/checks/rs/garde.md` with closed/remaining gaps.

### Key decisions
-   **Test Layout:** Use the mandated `*_tests/` folder pattern for every rule to isolate tests by attack class.
-   **Delegation vs Inline:** As an orchestrator, I will delegate the hardening of specific rules or rule groups to background agents (via `generalist` sub-agent) since this involves multiple files and extensive test generation.

## Architectural Considerations
This aligns with the `ProjectTree` checker architecture where rules are pure functions. Tests will construct minimal typed inputs for rules, and orchestrator tests will handle `ProjectTree` extraction verification.

## Risks & Edge Cases
-   Properly identifying covering `clippy.toml` configs in a multi-root workspace during tests.
-   Avoiding false positives on C-like enums in `RS-GARDE-08`.
-   Ensuring `RS-GARDE-10` properly surfaces source parsing and config failures.

## Files to Modify
-   `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_*.rs` (Update test module paths)
-   `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_*_tests/*` (Create new test files)
-   `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_*_tests.rs` (Remove old sidecar files)
-   `.plans/todo/checks/rs/garde.md` (Update status)
