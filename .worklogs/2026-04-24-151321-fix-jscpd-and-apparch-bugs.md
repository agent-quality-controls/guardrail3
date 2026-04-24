## Summary

Fixed two real guardrail bugs in the TS legacy surface. `ts/jscpd` now resolves the nearest ancestor `.jscpd.json` when validating a nested app path, and `ts/apparch` no longer rejects valid TSX just because tree-sitter misparses bare `&` inside JSX text.

## Decisions made

- Fixed JSCPD at ingestion-time config discovery.
  - Reason: the failure was that app-root validation could not see the real duplication-policy config above the app root.
  - Kept root-local config precedence over ancestors.
- Fixed apparch at parser-error classification, not by disabling parse validation.
  - Reason: the real file was valid TSX, but tree-sitter emitted `ERROR` nodes for bare `&` in JSX text.
  - Tolerated only that specific TSX grammar mismatch class.
  - Kept real syntax errors failing ingestion.
- Added red tests first for both bugs, then widened them to cover the broader class.
  - JSCPD:
    - ancestor root config resolves
    - ancestor parse errors surface
    - nested local root still wins over ancestor
  - apparch:
    - bare `&` JSX text passes
    - phrase text like `AI Search & GEO` passes
    - real TSX syntax breaks still fail

## Key files for context

- `.plans/2026-04-24-150254-fix-jscpd-and-apparch-bugs.md`
- `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/src/run.rs`
- `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/apparch/g3ts-apparch-ingestion/crates/runtime/src/source.rs`
- `packages/ts/apparch/g3ts-apparch-ingestion/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test -q --manifest-path packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/Cargo.toml`
- `cargo test -q --manifest-path packages/ts/apparch/g3ts-apparch-ingestion/crates/runtime/Cargo.toml`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family jscpd --inventory`
  - result: resolves `../../.jscpd.json` and reports all JSCPD checks green
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family apparch --inventory`
  - result: apparch ingestion succeeds and reports only info findings
- installed CLI refreshed:
  - `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --force`
  - `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family jscpd --inventory`
  - `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family apparch --inventory`
- Adversarial review:
  - found one real missing scenario in JSCPD: nested local config precedence over ancestor
  - found one real over-narrow assumption in apparch: the JSX text mismatch is broader than a single `\"&\"` token
  - both were fixed
  - final pass found no remaining concrete gap in the touched bug classes

## Next steps

- If TS work continues, the next live TS bug to pressure-test is still the repo-root `jscpd` CLI hang, which is separate from the app-root false missing report fixed here.
