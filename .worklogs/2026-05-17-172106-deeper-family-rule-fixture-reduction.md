# Summary

Reduced the `g3rs-rules` fixture corpus with a second, deeper fixture3 reduction pass.

The first reduction was too conservative because it skipped clean copied fixtures and used exact-output preservation. This pass added a semantic rule-set oracle, reduced clean fixtures, removed ignored build artifacts, and approved the new fixture output after verification.

# Decisions Made

- Kept exact-output reduction for the first safe pass.
- Added a semantic reducer oracle for deeper minimization.
- Semantic reduction preserves:
  - zero vs nonzero exit class
  - emitted rule IDs grouped by severity
- Semantic reduction intentionally does not preserve:
  - duplicate finding count
  - exact diagnostic text
  - exact numeric counts inside diagnostics
- Removed ignored `target/` artifacts created by fixture replay from release fixtures before measuring maintained fixture size.

# Reduction Result

Measured against commit `867e8902e` under `behavior/fixtures/g3rs-rules`:

- Files: 1,352 to 306, reduction 1,046 files, 77.4%.
- Folders: 600 to 257, reduction 343 folders, 57.2%.
- Text lines: 36,618 to 6,522, reduction 30,096 lines, 82.2%.
- Nonblank lines: 33,127 to 5,904, reduction 27,223 lines, 82.2%.

# Verification

- `fixture3 check --suite g3rs-rule-fixtures --json`
- `fixture3 check --all --json`
- `python3 scripts/behavior/verify-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-rule-coverage.py`
- `python3 -m py_compile scripts/behavior/reduce-broken-family-rule-fixtures.py scripts/behavior/reduce-g3rs-fixture-oracle.py scripts/behavior/reduce-g3rs-fixture-rule-set-oracle.py`
- `git diff --check`

# Key Files For Context

- `.plans/2026-05-17-163104-deeper-fixture-reduction.md`
- `scripts/behavior/reduce-broken-family-rule-fixtures.py`
- `scripts/behavior/reduce-g3rs-fixture-oracle.py`
- `scripts/behavior/reduce-g3rs-fixture-rule-set-oracle.py`
- `behavior/fixtures/g3rs-rules`
- `behavior/golden/g3rs-rule-fixtures/approved.normalized.json`

# Next Steps

- If folder-count reduction must also exceed 70%, the remaining work is fixture design, not more directory/file reduction. The remaining folders mostly encode distinct Rust path shapes that the rules inspect.
