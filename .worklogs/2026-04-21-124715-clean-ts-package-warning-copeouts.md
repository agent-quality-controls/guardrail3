## Summary

Removed the warning-only assertion-helper cope-outs from the new `ts/package` slice. The parser and ingestion assertion crates now express failure through direct assertions and explicit matching, so the slice validates cleanly without relying on `panic!()` or `unreachable!()` shortcuts.

## Decisions made

- Fixed the warnings at the source instead of waiving them.
  - Why: assertion-helper control flow is not an architectural escape hatch.
  - Rejected: adding waivers or leaving the warnings as "acceptable" because they were just implementation laziness.

- Kept the change local to assertion crates.
  - Why: runtime parser and ingestion behavior was already correct; only helper expression style was wrong.

## Key files for context

- `.plans/2026-04-21-124611-clean-ts-package-warning-copeouts.md`
- `packages/parsers/package-json-parser/crates/assertions/src/parser.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/assertions/src/run.rs`

## Next steps

- `ts/package` slice is clean now. Move to `ts/npmrc`.
- If warning audits come up again, classify assertion-helper warnings as cleanup debt by default, not as legitimate waivers.
