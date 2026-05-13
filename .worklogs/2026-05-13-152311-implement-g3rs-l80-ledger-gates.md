# Implement G3RS L80 Ledger Gates

## Summary

Implemented the G3RS behavior replay migration safety gates from `.plans/2026-05-13-145758-g3rs-l80-and-test-ledger-next-stage.md`.

The change hardens the L80 clean fixture contract, adds a first-scope G3RS test ledger for `apps/guardrail3-rs`, and wires ledger/deletion verifiers into `scripts/behavior/verify-all.sh`.

## Decisions Made

- Kept the first ledger scope file-level and limited to the 16 planned `apps/guardrail3-rs` test files.
- Marked all first-scope rows as `unclassified` because no tests are deleted in this stage.
- Made `L80-project-policy-valid-clean` explicitly require `fixture_kind = "clean"`.
- Made forbidden-path checks use `exists() or is_symlink()` so broken `repo/target` symlinks cannot bypass the clean fixture contract.
- Did not add invalid-example fixture files because temporary ledger mutations proved the negative controls without adding persistent noise.

## Negative Controls

- Duplicate row: temp ledger copy plus duplicated first `[[test]]` row, then `python3 scripts/behavior/verify-ledger.py --ledger "$ledger"` returned exit 1.
- Invalid kind: temp ledger copy with first `kind = "garbage"`, then `python3 scripts/behavior/verify-ledger.py --ledger "$ledger"` returned exit 1.
- Invalid status: temp ledger copy with first `status = "garbage"`, then `python3 scripts/behavior/verify-ledger.py --ledger "$ledger"` returned exit 1.
- Missing fixture id: temp ledger copy with first row changed to `kind = "behavior"`, `status = "migrated_deleted"`, `fixture = "missing-fixture"`, then `python3 scripts/behavior/verify-ledger.py --ledger "$ledger"` returned exit 1.
- Deleted file marked kept: temp ledger copy with first row status changed to `kept_compile_contract`; a temporary probe removed the file and the verifier returned exit 1. The file was restored exactly from HEAD immediately after the probe.
- Existing file marked deleted: temp ledger copy with first row changed to `kind = "private_implementation_only"` and `status = "deleted_private_implementation"`, then `python3 scripts/behavior/verify-test-deletion.py --ledger "$ledger"` returned exit 1.
- Missing `fixture_kind`: temp manifest copy with `fixture_kind = "clean"` removed, then `python3 scripts/behavior/verify-fixtures.py --manifest "$tmp_manifest"` returned exit 1.
- Broken `repo/target` symlink: temporary symlink under L80 fixture, then `python3 scripts/behavior/verify-fixtures.py` returned exit 1 and the symlink was removed.

## Verification

- `python3 -m py_compile scripts/behavior/*.py`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `git diff --check`

## Adversarial Review

- Reviewer A checked ledger completeness and exact first-scope inventory: `NO MUST FIX`.
- Reviewer B found two L80 verifier bugs: missing `fixture_kind` could pass, and broken forbidden symlinks could pass.
- Reviewer B2 checked the fixes: `NO MUST FIX`.
- Reviewer C checked fail-closed ledger/deletion behavior with temp mutations: `NO MUST FIX`.

## Key Files For Context

- `behavior/migration/g3rs-test-ledger.toml`
- `scripts/behavior/verify-ledger.py`
- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-all.sh`
- `scripts/behavior/verify-fixtures.py`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`

## Next Steps

- Audit the 16 `apps/guardrail3-rs` ledger rows and classify each row as behavior, replay system, compile contract, private implementation only, or obsolete.
- Only after classification, migrate behavior rows to replay fixtures or keep compile/replay rows as explicit exceptions.
