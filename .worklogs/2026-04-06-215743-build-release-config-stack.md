# Build release config stack: 2 parsers + checks + ingestion

**Date:** 2026-04-06 21:57
**Scope:** 4 new packages

## Summary
Extracted 11 config-only rules from the release family into the g3rs pipeline. Built 2 new typed parser crates (release-plz.toml and cliff.toml), the checks package with all 11 rules, and the ingestion package. Unified RS-BIN/RS-PUB/RS-RELEASE naming into RS-RELEASE-CONFIG-XX.

## Packages built
1. **release-plz-toml-parser** — typed parser for release-plz.toml (ReleasePlzToml, workspace, package entries). 5 tests.
2. **cliff-toml-parser** — typed parser for cliff.toml (CliffToml, git section, commit parsers). 7 tests.
3. **g3rs-release-config-checks** — 11 checks: per-crate metadata (01-09) + per-repo config baselines (10-11). 0 tests (implementations only, tests TODO).
4. **g3rs-release-config-ingestion** — selects Cargo.toml (required) + release-plz.toml + cliff.toml (optional). 7 tests.

## Check mapping (old → new)
- RS-PUB-01 → RS-RELEASE-CONFIG-01 (description)
- RS-PUB-02 → RS-RELEASE-CONFIG-02 (license)
- RS-PUB-03 → RS-RELEASE-CONFIG-03 (repository)
- RS-PUB-06 → RS-RELEASE-CONFIG-04 (keywords)
- RS-PUB-07 → RS-RELEASE-CONFIG-05 (categories)
- RS-PUB-08 → RS-RELEASE-CONFIG-06 (semver)
- RS-PUB-13 → RS-RELEASE-CONFIG-07 (docs.rs metadata)
- RS-BIN-03 → RS-RELEASE-CONFIG-08 (binstall metadata)
- RS-RELEASE-11 → RS-RELEASE-CONFIG-09 (accidentally publishable)
- RS-RELEASE-03 → RS-RELEASE-CONFIG-10 (release-plz baseline)
- RS-RELEASE-04 → RS-RELEASE-CONFIG-11 (cliff baseline)

## Outstanding
- g3rs-release-config-checks has 0 tests — check implementations need test coverage
- Checks not yet wired into the app (old rules still run inline)

## Key Files
- `packages/release-plz-toml-parser/crates/parser/types/src/release_plz_toml.rs`
- `packages/cliff-toml-parser/crates/parser/types/src/cliff_toml.rs`
- `packages/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/g3rs-release-config-ingestion/crates/runtime/src/run.rs`
