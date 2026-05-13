# Summary

Compacted the G3RS L30-L39 behavior fixtures from 22 fixtures to 5 fixtures without allowing hidden Error/Warn rows. Added count-sensitive baseline verification and a compaction manifest verifier so future fixture merges must prove removed fixtures are represented and forbidden family pollution is absent.

# Decisions Made

- Kept separate fixtures only where state is mutually exclusive or where a family would pollute another family's branch.
- Removed the release missing-member fixture from L30-L39 coverage because it is a topology-invalid side effect, not an independent release required-input branch.
- Kept `R20`'s marker under `behavior/fixtures` because it proves `validate-repo` ignores adoption markers inside fixture directories.
- Changed runtime fixture copying to dereference shared symlinks and ignore `.git` and `target` so replay fixtures are portable and do not copy build outputs.
- Made `required_results` count-sensitive so duplicate Error/Warn rows require duplicate manifest rows.

# Key Files

- `.plans/2026-05-13-004723-g3rs-behavior-fixture-compaction.md`
- `.plans/2026-05-13-004723-g3rs-behavior-fixture-compaction.md.manifest.toml`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `scripts/behavior/baseline_common.py`
- `scripts/behavior/verify-baselines.py`
- `scripts/behavior/verify-compaction.py`
- `scripts/behavior/verify-all.sh`
- `behavior/fixtures/g3rs`
- `behavior/baselines/g3rs`

# Verification

- `scripts/behavior/verify-all.sh`
- `python3 -m py_compile scripts/behavior/*.py`
- `git diff --check`
- `find behavior/fixtures/g3rs behavior/fixtures/g3rs-validate-repo -path '*/target/*' -o -name target`
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate-repo`
- Adversarial review `019e1ebd-80c2-7792-b9cb-01817ce05035`: PASS.

# Next Steps

- Continue fixture-layer migration at the next behavior layer using the same count-sensitive `required_results` verifier.
