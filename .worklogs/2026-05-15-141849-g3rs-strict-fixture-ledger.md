# Summary

Closed the remaining unclassified Rust test-fixture ledger rows by adding focused behavior replay fixtures and exact classifier mappings.

# Decisions Made

- Added six `g3rs validate` fixtures because the missing rows required distinct fixture states for deny sections, deny value branches, fmt edition precedence, and nested topology context.
- Added one `g3rs validate-repo` fixture because path-qualified hook tools and safe bypass-looking comments are repo-level hook behavior.
- Kept exact fixture output pinned in manifests so every emitted Error/Warn row is reviewed through fixture manifests, not only through golden output.
- Made `scripts/behavior/verify-all.sh` require strict ledger verification so future work cannot reintroduce unclassified rows.

# Key Files

- `.plans/2026-05-15-133355-g3rs-unclassified-fixture-coverage.md`
- `.plans/2026-05-15-133355-g3rs-unclassified-fixture-coverage.md.manifest.toml`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
- `scripts/behavior/classify-test-fixture-ledger.py`
- `scripts/behavior/verify-unclassified-fixture-coverage.py`
- `scripts/behavior/verify-all.sh`

# Verification

- `fixture3 check --all`
- `python3 scripts/behavior/verify-fixtures.py`
- `python3 scripts/behavior/verify-fixtures.py --manifest .plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `python3 scripts/behavior/verify-rule-coverage.py`
- `python3 scripts/behavior/verify-unclassified-fixture-coverage.py`
- `python3 scripts/behavior/verify-test-fixture-ledger.py --strict`
- `scripts/behavior/verify-all.sh`
- `git diff --check`

# Next Steps

- Use the strict fixture ledger as the baseline for deciding which legacy tests remain as compile contracts and which can be deleted after review.
