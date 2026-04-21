## Goal

Remove the warning-only assertion-helper cope-outs introduced in the new `ts/package` wave so the remaining warning list reflects only genuinely necessary escape hatches.

## Approach

1. Replace `panic!()` / `unreachable!()`-based control-flow shortcuts in:
   - `packages/parsers/package-json-parser/crates/assertions/src/parser.rs`
   - `packages/ts/package/g3ts-package-ingestion/crates/assertions/src/run.rs`
2. Keep the assertion behavior the same, but express failure through direct `match` / `let else` assertions.
3. Re-run tests and `g3rs validate` on:
   - `packages/parsers/package-json-parser`
   - `packages/ts/package/g3ts-package-ingestion`
   - `packages/ts/package/g3ts-package-config-checks`
   - `apps/guardrail3-ts`
4. Produce a fresh warning inventory for the touched `ts/package` slice and classify anything left as either necessary or still cleanup debt.

## Key decisions

- Do not add waivers.
  - Reason: these warnings are in assertion helpers and are not legitimate architectural escape hatches.

- Keep the fix local to the assertion crates.
  - Reason: the problem is not in parser or ingestion runtime logic, only in helper implementation style.

## Files to modify

- `packages/parsers/package-json-parser/crates/assertions/src/parser.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/assertions/src/run.rs`
- `.worklogs/...`
