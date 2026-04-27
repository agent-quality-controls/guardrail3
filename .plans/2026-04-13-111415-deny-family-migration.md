## Goal

Finish the deny family under the package model:

- migrate the remaining old app deny config rules into package-owned config checks
- add the missing deny filetree lane for coverage and same-root conflict signaling
- fix the known package logic bugs in deny config checks
- prove the fixes with failing tests first, then rerun deny-specific adversarial review until clean

## Approach

1. Add failing tests for the concrete deny package bugs and gaps
   - `g3rs-deny/unknown-keys` missing unknown-key coverage for typed nested tables
   - `g3rs-deny/stricter-advisories-inventory` behavior/name mismatch around `unmaintained`
   - missing tests for malformed skip/ignore entries, tokio allow-list drift, unreadable selected deny file
2. Migrate remaining config rules from old app code into `g3rs-deny-config-checks`
   - `g3rs-deny/ban-baseline-complete` package successor for old `RS-DENY-09`
   - `g3rs-deny/license-exceptions-inventory` package successor for old `RS-DENY-17`
   - `g3rs-deny/allow-override-channel` package successor for old `RS-DENY-25`
   - `g3rs-deny/extra-deny-bans-inventory` package successor for old `RS-DENY-26`
   - `g3rs-deny/wrappers` package successor for old `RS-DENY-30`
3. Build deny filetree lane
   - add `g3rs-deny-filetree-checks`
   - extend `g3rs-deny-types`
   - implement `g3rs-deny-ingestion::ingest_for_file_tree_checks(...)`
   - migrate old app `RS-DENY-01` and `RS-DENY-03` into lane-scoped filetree rules
4. Update deny package docs to reflect the real package surface
5. Run package tests, then rerun adversarial deny agents and close any remaining gaps

## Key decisions

- Keep deny under two lanes only:
  - config for deny.toml contents
  - filetree for coverage, same-root conflict, and input-failure signaling
- No source lane
  - deny rules inspect deny config and workspace/file placement, not Rust source files
- Keep typed parsing in ingestion and pure checks in packages
  - config package receives typed parsed `DenyToml`
  - filetree lane receives minimal root/conflict/failure facts
- Use lane-scoped package rule IDs only
  - no legacy `RS-DENY-*` IDs inside package rules

## Alternatives considered

- Leaving `RS-DENY-01` and `RS-DENY-03` in app-only bridge code
  - rejected because they are filetree/input-failure checks and belong in the package model
- Preserving the old `RS-DENY-03` stricter-advisories semantics without checking the parser types
  - rejected because the current package rule name must match real typed behavior

## Files to modify

- `packages/rs/deny/g3rs-deny-types/src/lib.rs`
- `packages/rs/deny/g3rs-deny-config-checks/**`
- `packages/rs/deny/g3rs-deny-ingestion/**`
- `packages/rs/deny/g3rs-deny-filetree-checks/**` (new)
- `packages/rs/deny/g3rs-deny-config-checks/README.md`
- `packages/rs/deny/g3rs-deny-ingestion/README.md`
