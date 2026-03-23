# Hooks Execution Plan

This is the exhaustive execution order for the hook migration and hardening lane.

It is not a prioritization document.
It is the step-by-step sequence for completing all required hook work end to end.

## First action

1. Read and restate the live contract before writing code.
   - `AGENTS.md`
   - `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
   - `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
   - `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
   - `.plans/todo/check_review/test_hardening/05-hooks.md`
   - `.plans/todo/checks/hooks/shared.md`
   - `.plans/todo/checks/hooks/rs.md`
   - `.plans/todo/check_review/01-hooks-and-cli.md`

## Full execution order

2. Inventory every current hook-related code path and record ownership.
   - legacy hook validation under `apps/guardrail3/crates/app/hooks/`
   - old Rust-side mutation-hook detection under `apps/guardrail3/crates/app/rs/validate/`
   - hook generation under `apps/guardrail3/crates/adapters/inbound/cli/`
   - hook template/module content under `apps/guardrail3/crates/domain/modules/`
   - any repo fixtures or `.claude/` hook examples

3. Freeze the migration boundary.
   - decide the canonical new-family location for hooks
   - define which legacy files become read-only migration sources
   - define which routing entrypoints continue to call hooks during the transition

4. Build the hook coverage matrix before changing semantics.
   - map every old hook check to planned `HOOK-SHARED-*` or `HOOK-RS-*`
   - mark missing rules
   - mark overclaimed rules
   - mark rules that currently rely on substring matching
   - mark generator-only behavior with no checker ownership

5. Define the new family data model.
   - shared facts for pre-commit file, modular scripts, permissions, hook path config, tool availability, and generated-template expectations
   - typed inputs for one local assertion at a time
   - explicit input-failure facts for unreadable files, malformed scripts, parse failures, and missing expected surfaces

6. Define the executable-command parsing model before porting any semantic rule.
   - shell line classification
   - comment detection
   - shebang extraction
   - command extraction from executable lines only
   - wrapper detection such as `|| true`, `|| :`, `|| echo ...`, unconditional `exit 0`
   - dispatcher-shape detection
   - trigger-condition extraction for staged-file/config-change logic

7. Decide the parser contract and test it in isolation.
   - what counts as an executable command line
   - what counts as an inert line
   - how multiline shell blocks are represented
   - how sourced modular scripts contribute commands
   - how command equivalence is normalized across wrapped forms

8. Create golden hook fixtures that represent the intended valid baseline.
   - root dispatcher form
   - modular `pre-commit.d/` form
   - Rust-enabled hook path
   - prerequisite-tool checks
   - config-change trigger logic
   - correct `workspace_root`

9. Create the new hook family scaffolding.
   - orchestrator module
   - `facts.rs`
   - `inputs.rs`
   - one production file per rule
   - one rule-specific `*_tests/` directory per rule

10. Implement fail-closed family plumbing before individual rules.
   - missing hook file surfaces an owned failure
   - unreadable hook files surface an owned failure
   - unreadable modular scripts surface an owned failure
   - parse failures surface an owned failure
   - no silent fallback to empty string content

11. Migrate the shared structural baseline rules.
   - `HOOK-SHARED-01`
   - `HOOK-SHARED-02`
   - `HOOK-SHARED-03`
   - `HOOK-SHARED-04`
   - `HOOK-SHARED-05`
   - `HOOK-SHARED-06`
   - `HOOK-SHARED-07`
   - `HOOK-SHARED-08`
   - `HOOK-SHARED-09`
   - keep them architecture-correct even if some remain mostly inventory-oriented

12. Migrate the shared shell-safety and file-integrity rules.
   - `HOOK-SHARED-10`
   - `HOOK-SHARED-11`
   - `HOOK-SHARED-12`
   - `HOOK-SHARED-13`
   - `HOOK-SHARED-14`

13. Migrate the shared semantic command-detection rules.
   - `HOOK-SHARED-15`
   - `HOOK-SHARED-16`
   - `HOOK-SHARED-17`
   - `HOOK-SHARED-18`
   - `HOOK-SHARED-19`
   - `HOOK-SHARED-20`
   - `HOOK-SHARED-21`

14. Migrate the Rust hook-step presence rules on top of the executable-command model.
   - `HOOK-RS-01`
   - `HOOK-RS-02`
   - `HOOK-RS-03`
   - `HOOK-RS-04`
   - `HOOK-RS-05`
   - `HOOK-RS-08`
   - `HOOK-RS-11`
   - `HOOK-RS-12`

15. Migrate the Rust tool-availability and fail-closed rules.
   - `HOOK-RS-06`
   - `HOOK-RS-14`
   - `HOOK-RS-15`
   - ensure missing `guardrail3` is not treated as a warning-only skip
   - ensure missing `cargo-dupes` is owned by the Rust hook family

16. Migrate the Rust command-shape correctness rules.
   - `HOOK-RS-07`
   - `HOOK-RS-09`
   - `HOOK-RS-10`
   - `HOOK-RS-13`
   - exact `cargo clippy -D warnings` semantics
   - workspace-aware `cargo test --workspace` semantics
   - `cargo-dupes --exclude-tests` semantics

17. Migrate the Rust trigger-completeness rule.
   - `HOOK-RS-16`
   - define the canonical Rust config file set
   - prove config-only changes trigger Rust validation
   - prove `.rs`-only assumptions are not the semantic core

18. Build attack-vector tests for every shared rule.
   - comments/prose masquerading as commands
   - fake dispatcher text
   - wrong shebang
   - non-executable modular script
   - unconditional `exit 0`
   - fail-open wrappers
   - `--no-verify` instructions
   - misleading lockfile prose
   - nearby valid structures that should not fire

19. Build attack-vector tests for every Rust hook rule.
   - commented-out Rust steps
   - wrapped or weakened `guardrail3` calls
   - `cargo clippy` without deny-warnings
   - `cargo test` with workspace-skipping forms
   - missing `gitleaks`
   - wrong duplication tool
   - missing `cargo-dupes`
   - config-change trigger omissions

20. Enforce exact-result assertions across the hook lane.
   - exact owned hit set
   - exact owned non-hit set
   - exact rule IDs
   - exact severities
   - no loose “contains some result” assertions

21. Port old test ideas deliberately rather than mechanically.
   - map each old test to a current rule
   - keep only still-valid attack vectors
   - drop tests that only proved substring presence

22. Reconcile generator and checker semantics.
   - unify all generation paths on one `workspace_root` contract
   - remove template/checker drift
   - ensure generated hooks satisfy the new rules
   - fix the stylelint shell-logic bug called out in the review backlog

23. Reconcile prerequisite-tool ownership between shared and Rust families.
   - shared ownership for generic tool/runtime requirements
   - Rust ownership for `guardrail3`, `cargo-dupes`, and Rust-only execution prerequisites
   - no checker blind spots where generation assumes a tool but validation does not

24. Reconcile CLI/report routing with the migrated hook families.
   - stop treating hooks as a stale sidecar under coarse `domains.code`
   - make hook reporting align with the Rust-only architecture
   - remove stale family/domain naming where it hides hook semantics

25. Align `RS-TEST-08` with the same executable-command model.
   - reuse the parser if possible
   - otherwise match its semantics exactly
   - eliminate mutation-hook detection that still relies on coarse non-comment substring logic

26. Remove or quarantine superseded legacy hook semantics.
   - no dual semantic cores
   - no old `contains()` path left as the real authority
   - keep migration shims only where routing still requires them

27. Update the active hook planning docs with closure status.
   - what is now implemented
   - what gaps were closed
   - what policy questions remain
   - what follow-up items, if any, still belong in `check_review`

28. Run full verification.
   - rule-level tests
   - family-level tests if present
   - targeted CLI validation paths
   - generated hook output spot-checks against the new checker

29. Do the final cleanup pass.
   - remove dead helpers
   - remove stale legacy test files
   - ensure every rule uses the required `*_tests/` directory layout
   - ensure file naming matches the one-rule/one-test structure

30. Write the completion summary back into the hook lane documents.
   - what changed
   - which files now define the migrated hook system
   - remaining risks, if any

## Immediate next implementation step

If starting the actual code pass now, the first code step is:

1. build the hook coverage matrix and choose the canonical new-family module path
2. then implement the executable-command parser and its isolated tests
3. only after that start migrating individual hook rules
