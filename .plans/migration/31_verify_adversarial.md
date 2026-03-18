# Step 31: Verify Adversarial Fixtures Show Improvement

## Goal
The grep-should-fail fixtures from step 01 should now give CORRECT results. Update the test expectations.

## Task (1 agent)

1. Run `cargo test --test adversarial_grep_attacks`
2. Tests marked `GREP_BUG` should now FAIL (because the bug is fixed)
3. For each `GREP_BUG` test that fails:
   - Update the assertion to expect the CORRECT behavior
   - Change the marker from `GREP_BUG` to `FIXED`
4. For any `GREP_BUG` test that still passes (bug NOT fixed):
   - Report which check still uses grep and needs migration
   - File as a TODO for the next iteration
5. Run `cargo test --test adversarial_grep_attacks` — all must pass

## Verification
```bash
cargo test --test adversarial_grep_attacks  # all pass
# Count of FIXED vs remaining GREP_BUG:
grep -c "FIXED" tests/adversarial_grep_attacks.rs
grep -c "GREP_BUG" tests/adversarial_grep_attacks.rs
```

## On Failure
If a `GREP_BUG` is not fixed, the corresponding check wasn't migrated or the AST walker has the same bug as grep. Read the check implementation and fix.
