## Goal

Close the `ts/jscpd` proof gaps found by the attack pass without widening the family scope.

End state:

- unreadable root `.jscpd.json` path is proven in ingestion
- config-check tests assert exact finding payloads for the golden and parse-error paths
- `g3ts` has a direct selected-family `jscpd` execution proof
- selected-family ordering is proven with `Jscpd` in a mixed filter set
- parser invalid-JSON test is stable against upstream serde wording drift

## Approach

1. Add an unreadable-root ingestion test under `g3ts-jscpd-ingestion`.
   - Use real file permissions on Unix to hit the live `Unreadable` branch.
2. Strengthen `g3ts-jscpd-config-checks` tests.
   - Convert the parse-error and golden-root paths from ID-only assertions to exact finding assertions.
3. Strengthen `g3ts` app-path tests.
   - Add one direct `--family jscpd` run test with exact stdout/stderr/exit-code.
   - Add one mixed-family ordering test with `Jscpd`.
4. Relax the parser invalid-JSON test to assert the stable parser prefix instead of a serde wording fragment.

## Key Decisions

- Keep this as proof tightening only.
  - Why: the attack did not find a `ts/jscpd` check bug, only missing proof.

- Use the live crawl unreadable branch instead of constructing fake family state.
  - Why: the point is to prove the real ingestion behavior, not the enum shape.

## Files To Modify

- `packages/parsers/jscpd-json-parser/crates/runtime/src/parser_tests/cases.rs`
- `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/assertions/src/run.rs`
- `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/jscpd/g3ts-jscpd-config-checks/crates/runtime/src/run_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/selection_tests/cases.rs`
- `.worklogs/**`
