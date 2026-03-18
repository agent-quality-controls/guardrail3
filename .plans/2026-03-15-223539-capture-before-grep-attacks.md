# Step 02: Capture "Before" Results for Adversarial Grep Attack Fixtures

**Date:** 2026-03-15 22:35
**Task:** Run current grep-based guardrail3 against every adversarial fixture and record what checks fire.

## Goal
Create an integration test `tests/adversarial_grep_attacks.rs` that runs each fixture through `guardrail3 rs validate --format json`, collects which check IDs fire, and produces a `BEFORE_SUMMARY.md` documenting current grep behavior per fixture. The test PASSes with current behavior -- it documents, not asserts correctness.

## Approach

### Step-by-step plan
1. Create `tests/adversarial_grep_attacks.rs` following the pattern from `tests/adversarial_fixtures.rs`
2. For each of the 40 Rust fixtures (rust-allow/10, rust-code-quality/10, rust-structural/10, edge-cases/10), create a temp project and run guardrail3
3. Collect all fired check IDs per fixture
4. The test function for each fixture documents what the current grep tool does
5. Create `tests/fixtures/grep-attacks/BEFORE_SUMMARY.md` from running the binary manually first, then hard-code expected behavior into the test

### Key decisions
- **One test per fixture:** Matches existing adversarial_fixtures.rs pattern. Each test documents current behavior.
- **Skip TypeScript fixtures:** Task says Rust source scan checks R30-R58, typescript fixtures are for TS checks.
- **Test documents, not asserts correctness:** Tests pass by recording what grep does. After migration, we compare.

## Files to Modify
- `tests/adversarial_grep_attacks.rs` -- NEW: integration test file
- `tests/fixtures/grep-attacks/BEFORE_SUMMARY.md` -- NEW: summary of current grep behavior
