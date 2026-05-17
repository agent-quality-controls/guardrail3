# Finish Rust Family Rule Fixtures

## Summary

Finished the Rust family-rule fixture migration across all active Rust families. The `g3rs-rule` suite now contains 61 fixtures covering every active Rust rule that is reachable through the public CLI, with one clean golden fixture per family and the smallest broken fixture groups currently encoded in the family fixture manifest.

## Decisions Made

- Completed the remaining families from the all-family fixture plan: apparch, garde, code, test, release, and hooks.
- Kept inventory-only rules out of broken fixture expectations when the public CLI emits them only as Info.
- Recorded currently CLI-unreachable hook rules in the manifest instead of adding fake fixtures that could not make the public CLI emit them.
- Used `fixture3` golden output as the external behavior contract, not internal ingestion serialization.
- Removed generated `target/` directories from older broad fixtures after direct validation commands wrote build output into fixture folders.

## Key Files

- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md.manifest.toml`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `behavior/fixtures/g3rs-rule`
- `behavior/golden/g3rs-rule/approved.normalized.json`
- `scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`

## Verification

```text
$ fixture3 check --suite g3rs-rule
status: matched
fixtures: 61

$ python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py
family-rule-fixtures: PASS families:apparch,arch,cargo,clippy,code,deny,deps,fmt,garde,hooks,release,test,toolchain,topology fixtures:61

$ python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py
behavior-rule-coverage: PASS source:267 covered:250 replaced:17 planned:0

$ python3 scripts/behavior/verify-kept-test-dispositions.py
kept-test-dispositions: PASS

$ python3 scripts/behavior/verify-test-deletion.py
behavior-test-deletion: PASS rows:1577 active:740 replaceable:864 kept:713

$ bash scripts/behavior/verify-all.sh
family-rule-fixtures: PASS families:apparch,arch,cargo,clippy,code,deny,deps,fmt,garde,hooks,release,test,toolchain,topology fixtures:61
behavior-test-deletion: PASS rows:1577 active:740 replaceable:864 kept:713

$ g3rs validate repo --path "$PWD"
No findings.

$ git diff --check
exit 0
```

## Next Steps

- Use the family fixture manifests as the source of truth for deleting replaceable tests.
- Do not add internal serialization fixtures unless a public CLI behavior cannot expose the same rule contract.
