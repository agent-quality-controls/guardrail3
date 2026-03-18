# Improve all CheckResult messages to be self-explanatory

**Date:** 2026-03-16 21:01
**Task:** Make every CheckResult message answer: WHAT is this? WHY does it matter? WHAT to do?

## Goal
Every CheckResult in every check file must be self-explanatory to an agent that has never seen guardrail3. The summary line format (when >3 items are grouped) must also include the context.

## Approach

### Files to modify (all in apps/guardrail3/src/app/rs/validate/)

1. **config_files.rs** - R1, R2, R3, R21, R24 messages
2. **clippy_coverage.rs** - R4, R5, R6, R7 messages
3. **deny_audit.rs** - R8, R9, R10, R11 messages
4. **deny_bans.rs** - R12, R13, R17, R18 messages
5. **deny_inventory.rs** - R19, R20 messages
6. **deny_licenses.rs** - R14, R15, R16 messages
7. **cargo_lints.rs** - R26, R27, R28, R29 messages
8. **allow_checks.rs** - R30-R37 messages
9. **structure_checks.rs** - R38, R40, R41, R42, R53 messages
10. **code_quality_checks.rs** - R43, R44, R49, R58 messages
11. **dependency_scan.rs** - R45-R50 messages
12. **source_scan.rs** - no CheckResults directly (orchestrator)
13. **toolchain_check.rs** - R25 messages
14. **rustfmt_check.rs** - R22, R23 messages
15. **workspace_metadata.rs** - R55, R56, R57 messages
16. **garde_checks.rs** - R-GARDE-01 to R-GARDE-05 messages
17. **test_checks.rs** - R-TEST-01 to R-TEST-04, R-TEST-09 messages
18. **test_quality_checks.rs** - R-TEST-05 to R-TEST-08 messages
19. **release_checks.rs** - R-PUB-12 message
20. **release_crate_checks.rs** - R-PUB-01 to R-PUB-05, R-PUB-08 messages
21. **release_crate_deps.rs** - R-PUB-06, R-PUB-07, R-PUB-09 to R-PUB-11 messages
22. **release_bin_checks.rs** - R-BIN-01 to R-BIN-03 messages
23. **release_repo_checks.rs** - R-REL-01 to R-REL-08 messages
24. **hex_arch_checks.rs** - R-ARCH-01 to R-ARCH-04 messages
25. **dependency_allowlist.rs** - R-DEPS-01, R-DEPS-02 messages

### Also fix the summary line in text.rs
The summary line template needs to include the full context too.

### Key pattern
For every message, ensure it contains:
- WHAT: what the check found
- WHY: what breaks or goes wrong if ignored
- WHAT TO DO: exact remediation action (for errors/warnings)
- For inventory/info items: explain what this IS and that it's an approved/expected state

## Risks
- Many string changes = risk of compilation errors from unterminated strings
- Must not change check logic, only message text
