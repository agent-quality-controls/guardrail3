# Split family rule fixture suite

## Goal

Keep adding minimized per-family rule fixtures without making the broad `g3rs-validate` golden exceed the repository file-size limit.

The end state is:

- old broad layer fixtures stay in `g3rs-validate`
- new per-family rule fixtures move to a separate `g3rs-rule` suite
- family-rule verification reads the family-rule suite golden output
- the existing broad suite no longer grows when a new family is covered
- no fixture output is truncated or hidden

## Problem

`behavior/golden/g3rs-validate/approved.normalized.json` is `1044867` bytes.

The pre-commit file-size limit is `1048576` bytes.

Adding clippy fixtures to the same suite would almost certainly fail the hook even if the fixtures are correct.

This is not a clippy problem. The suite currently combines two different fixture corpora:

- broad layered behavior fixtures under `behavior/fixtures/g3rs`
- minimized family rule fixtures under `behavior/fixtures/g3rs-rule`

Those corpora need separate golden files because they answer different questions.

## Approach

1. Update `fixture3.yaml`.
   - Keep `g3rs-validate` scoped to `behavior/fixtures/g3rs/*/fixture.toml`.
   - Add `g3rs-rule` scoped to `behavior/fixtures/g3rs-rule/*/*/fixture.toml`.
   - Use the same replay script and manifest for the new suite.
   - Store output under `behavior/golden/g3rs-rule`.

2. Update `scripts/behavior/verify-g3rs-family-rule-fixtures.py`.
   - Read approved output from `behavior/golden/g3rs-rule/approved.normalized.json`.
   - Keep the verifier limited to `behavior/fixtures/g3rs-rule`.
   - Do not read broad `g3rs-validate` output for family rule coverage.

3. Update `scripts/behavior/verify-all.sh`.
   - Run `fixture3 check --suite g3rs-rule`.
   - Keep existing suite checks intact.

4. Update `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`.
   - Set `target.suite = "g3rs-rule"`.
   - Change verification command to `fixture3 check --suite g3rs-rule`.
   - Keep broad corpus marked transitional.

5. Approve the new family-rule suite.
   - Record current family fixtures in the new suite.
   - Re-approve `g3rs-validate` after removing family fixture records from that suite.

6. Continue clippy family fixture work.
   - Add minimized clippy fixtures under `behavior/fixtures/g3rs-rule/clippy`.
   - Add clippy fixture rows to the family-rule manifest.
   - Mark clippy complete only after every active clippy rule is broken by at least one non-clean fixture or documented as inventory-only.

## Key decisions

- Do not truncate stdout.
  - Truncation would hide behavior and break fixture review.

- Do not raise the file-size limit.
  - The limit caught a real architecture issue: one golden was being used for two fixture corpora.

- Do not remove broad fixtures in this change.
  - The broad fixtures are transitional, but deleting them belongs to the later coverage replacement step.

- Do not split one suite per family.
  - That would multiply verifier config without solving a different problem. One family-rule suite keeps the contract simple.

## Files to modify

- `fixture3.yaml`
- `scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `scripts/behavior/verify-all.sh`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3rs-rule/clippy/**`
- `behavior/golden/g3rs-validate/**`
- `behavior/golden/g3rs-rule/**`

## Verification

Run:

```bash
fixture3 check --suite g3rs-validate
fixture3 check --suite g3rs-rule
python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py
python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py
python3 scripts/behavior/verify-kept-test-dispositions.py
python3 scripts/behavior/verify-test-deletion.py
bash scripts/behavior/verify-all.sh
g3rs validate repo --path "$PWD"
git diff --check
```
