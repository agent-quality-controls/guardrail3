# G3TS Fixture Hardening

## Goal

Finish the post-coverage hardening pass for the G3TS fixture migration.

The target state is:

- every G3TS family remains covered by `g3ts-rule`
- no G3TS test deletion starts until fixture coverage is stable
- fixture minimization is attempted only where it can preserve approved output
- inactive clean fixtures are either upgraded or explicitly accepted as inactive no-op fixtures
- all verification runs through `fixture3` and deterministic behavior scripts

## Current Verified State

- `g3ts-rule` exists and has `82` fixtures.
- `g3ts-validate-repo` exists and passes.
- `g3ts-cli-output` exists and passes.
- `scripts/behavior/verify-all.sh` passes on commit `92e76d287`.
- The old `scripts/verify/all.sh` is not the active fixture gate. It belongs to the earlier routing-slice verifier and still checks stale `g3rs validate --path` CLI expectations.

## Decisions

- Do not edit the old routing-slice verifier in this pass. It is not the active fixture gate for G3TS family-rule fixtures.
- Do not delete G3TS tests in this pass. First harden and reduce the fixtures.
- Do not reduce clean golden fixtures. A clean fixture exists to prove no findings for a valid or intentionally inactive surface.
- Reduce only broken G3TS fixtures where the suite output can prove the same behavior after removals.
- If a fixture is already tiny, reduction may be a no-op. Record that from reducer output instead of hand-guessing.

## Work Items

### 1. Classify Clean Fixtures

Read every `behavior/fixtures/g3ts-rule/<family>/<family>-R00-clean-golden/repo`.

Classify each clean fixture as:

- `active-clean`: contains enough app/package surface for that family to run its positive checks and exit zero
- `inactive-clean`: contains no app/package surface and only proves the family does not false-positive on irrelevant roots

Write the classification into the existing G3TS family-rule fixture manifest.

The verifier must reject:

- a completed family without exactly one clean fixture
- a clean fixture with no explicit `clean_kind`
- a `clean_kind` value outside `active-clean` or `inactive-clean`

### 2. Reduce Broken Fixtures

Run `fixture3 reduce` on every non-clean fixture under `behavior/fixtures/g3ts-rule`.

The committed `g3ts-rule` suite has one approved output file for all fixtures. `fixture3 reduce` reduces one fixture tree at a time. Therefore reduction must use a temporary single-fixture manifest whose approved output contains only the record for the fixture being reduced.

Required permanent helper:

- `scripts/behavior/fixture3-g3ts-single-fixture-replay.py`
- input: the reducer's trial file list
- behavior: locate exactly one `fixture.toml` in that trial tree, delegate to `scripts/behavior/fixture3-g3ts-fixture-replay.py`, and restore the original fixture ID because reducer trial directories are scratch paths
- failure mode: if `fixture.toml` is removed or duplicated, emit invalid JSON behavior and exit nonzero so the reducer rejects that trial

Required reducer command shape:

```bash
fixture3 reduce \
  --suite g3ts-rule \
  --manifest .fixture3/reduce/g3ts-rule/<fixture-id>/manifest.fixture3.yaml \
  --fixture-root <fixture> \
  --work-dir .fixture3/reduce/g3ts-rule/<fixture-id>
```

After each reducer run:

- inspect `.fixture3/reduce/g3ts-rule/<fixture-id>/reduce-report.json`
- copy the accepted removals back to `<fixture>` only when the report says the approved output is preserved
- never copy `trial-current`
- rerun `fixture3 check --suite g3ts-rule`

### 3. Verify Fixture Contract

Run:

```bash
fixture3 check --suite g3ts-rule
fixture3 check --suite g3ts-validate-repo
fixture3 check --suite g3ts-cli-output
python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py
python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py
scripts/behavior/verify-all.sh
```

All must pass before commit.

### 4. Commit and Push

Commit only after a worklog is added.

Push to `origin/development`.

## Non-Goals

- Do not push to `main`.
- Do not delete unit tests yet.
- Do not create internal ingestion fixtures.
- Do not serialize internal G3TS structs.
- Do not weaken fixture coverage to make reduction pass.

## Execution Notes

- Reducer support requires the single-fixture replay helper because the normal `g3ts-rule` suite compares all `82` fixtures at once.
- Reducer output accepted `0` removals from broken fixtures.
- Four fixtures were not reducer-eligible under `fixture3 reduce` because their behavior depends on files under generated-style dependency folders that the reducer excludes from trial copies:
  - `eslint-R20-parse-error`
  - `eslint-R30-weak-baseline`
  - `eslint-R40-broken-carveouts`
  - `style-R40-incomplete-stylelint`
- No fixture content should be changed by this pass.
