# TS-TESTS — TypeScript test quality checker

**Input:** test files, package.json/test-runner config
**Parser:** tree-sitter TypeScript/TSX + targeted config/script inspection
**Current code:** `app/ts/validate/test_checks.rs`
**Owned root:** TS package/app root

## Owns

- mutation-test config presence
- mutation-test package/script/config coherence
- test-file presence
- test-runner configuration
- test-runner package/script/config coherence
- test-file naming and placement policy
- test-file/source co-location or canonical test-dir policy
- minimum assertion-bearing test surface
- `.skip()` / `.only()` policy
- `test.todo()` / equivalent unfinished-test inventory or policy
- coverage-threshold configuration when coverage is part of the tests contract
- other explicit test-surface rules already carried by `test_checks.rs`

## Does not own

- general source-scan rules
- auxiliary tool packages except where they are required by the tests family itself
- architecture boundary policy
  - that belongs to `ts/hexarch` or `ts/libarch`

## Contract direction

This should stay its own family.
The current implementation is already cohesive enough to justify it.

But the target contract is broader than the current runtime:
- config existence alone is not enough
- mutation/coverage tooling must be wired coherently
- test files must not merely exist; they must look like real tests
