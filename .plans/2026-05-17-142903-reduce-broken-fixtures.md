# Goal

Reduce copied broken family-rule fixture trees without changing approved fixture behavior.

# Scope

- Include only broken fixture roots under `behavior/fixtures/g3rs-rule/*/*`.
- Exclude every fixture whose directory name contains `R00-clean-golden`.
- Do not edit `behavior/golden/**`.
- Do not edit clean golden fixture roots.

# Approach

1. Discover broken fixture roots from `behavior/fixtures/g3rs-rule/*/*/fixture.toml`.
2. For each root, create scratch space under `.fixture3/reduce-broken-family-rule-fixtures/<root>`.
3. For each root, write a generated one-fixture `fixture3` manifest in scratch.
4. For each root, write a generated approved output in scratch by running the replay command only for that root.
5. Normalize the scratch approved output by:
   - replacing `fixture_id` with `fixture-root`
   - removing `fixture_hash`
6. Run `fixture3 reduce` against the scratch one-fixture suite, not against the full committed suite.
7. Apply only accepted removals from the report to the original fixture root.
8. Re-run the one-fixture behavior oracle after applying removals.
9. If the normalized behavior output differs, restore that fixture root from scratch backup and stop.
10. After all fixture roots are reduced, regenerate the committed full-suite approved output if and only if the only required drift is fixture-file identity/hash drift from the reduced files.
11. After all fixture roots are reduced, run:
   - `fixture3 check --suite g3rs-rule --json`
   - `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
   - `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`

# Key Decisions

- Use `fixture3 reduce` as the minimizer, not custom minimization logic.
- Use a generated scratch one-fixture suite because the committed suite output covers all fixtures and includes fixture hashes.
- Preserve behavior by comparing normalized replay output, not fixture identity.
- Keep clean golden fixtures unchanged because their output can be weaker than their intended structural proof.
- Keep any automation deterministic and local to behavior fixture maintenance.
- Do not let reduction edit committed golden output directly.
- Regenerate committed golden output only after behavior-equivalent reductions are accepted, because changed fixture files necessarily change `fixture_hash`.

# Files To Modify

- `behavior/fixtures/g3rs-rule/**` broken fixture roots only.
- Optional helper script under `scripts/behavior/` if repeated reduction needs automation.
- Worklog under `.worklogs/` before commit.
