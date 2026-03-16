# Step 33: Fix Issues Found by Adversarial Review

## Goal
Fix every issue found in step 32. Small agents, one issue per agent.

## Task
For EACH issue from step 32's reports:
1. Create a targeted agent with:
   - The specific issue description
   - The file(s) to read
   - The exact fix to make
   - The verification command
2. Agent makes the fix
3. Verify: `cargo test && sh golden-tests/compare.sh`
4. Commit

## Rules
- Max 3 changes per agent
- Every fix has a regression test FIRST (write the test, see it fail, then fix)
- Commit after each fix, not in batch

## On Failure
If a fix breaks golden tests or other tests, the fix is wrong. Revert and try again.
