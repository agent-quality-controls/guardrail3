## Goal

Fix the two confirmed guardrail bugs:

- `TS-JSCPD-CONFIG-01` incorrectly reports missing root `.jscpd.json` when validating a nested app path under a repo that has the config at an ancestor root.
- `ts/apparch` ingestion rejects valid TSX like the real `apps/landing/src/app/blog/_sections/hub-hero.tsx`.

## Approach

1. Add a red ingestion test for `ts/jscpd`.
   - Target: `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/src/run_tests/cases.rs`
   - Build a repo root fixture with `.jscpd.json` and a nested app directory.
   - Crawl the nested app directory and prove ingestion should still resolve the ancestor config.
   - Then fix ingestion in `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/src/run.rs` at the discovery layer, not in the checks.

2. Add a red ingestion test for `ts/apparch`.
   - Target: `packages/ts/apparch/g3ts-apparch-ingestion/crates/runtime/src/run_tests/cases.rs`
   - Use a minimal TSX fixture that matches the real failure shape from `hub-hero.tsx`.
   - Prove `ingest_for_config_checks` should succeed on valid TSX.
   - Then fix the parser path in `packages/ts/apparch/g3ts-apparch-ingestion/crates/runtime/src/source.rs` at the source-ingestion layer.

3. Re-run the family test suites and the TS CLI tests.
   - `packages/ts/jscpd/g3ts-jscpd-ingestion`
   - `packages/ts/apparch/g3ts-apparch-ingestion`
   - `apps/guardrail3-ts`

4. Run an adversarial review against the plan and the touched code.
   - Confirm the JSCPD fix handles ancestor config discovery, not just one exact nesting shape.
   - Confirm the apparch fix is not a blanket "ignore parser errors" band-aid.

## Key decisions

- Fix JSCPD in ingestion, not config checks.
  - The bug is wrong config discovery, not wrong rule logic.
- Fix apparch in source parsing, not by weakening downstream checks.
  - The bug is rejecting valid source, not how edges or public items are checked.
- Keep both fixes scoped to the real failure classes.
  - No broad TS family expansion.

## Files to modify

- `.plans/2026-04-24-150254-fix-jscpd-and-apparch-bugs.md`
- `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/src/run.rs`
- `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/apparch/g3ts-apparch-ingestion/crates/runtime/src/source.rs`
- `packages/ts/apparch/g3ts-apparch-ingestion/crates/runtime/src/run_tests/cases.rs`
