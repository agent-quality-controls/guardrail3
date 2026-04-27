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
- RS-PUB-01 → g3rs-release/description-present (description)
- RS-PUB-02 → g3rs-release/license-present (license)
- RS-PUB-03 → g3rs-release/repository-present (repository)
- RS-PUB-06 → g3rs-release/keywords-present (keywords)
- RS-PUB-07 → g3rs-release/categories-present (categories)
- RS-PUB-08 → g3rs-release/valid-semver (semver)
- RS-PUB-13 → g3rs-release/docs-rs-metadata (docs.rs metadata)
- RS-BIN-03 → g3rs-release/binstall-metadata (binstall metadata)
- RS-RELEASE-11 → g3rs-release/accidentally-publishable (accidentally publishable)
- RS-RELEASE-03 → g3rs-release/release-plz-baseline (release-plz baseline)
- RS-RELEASE-04 → g3rs-release/cliff-baseline (cliff baseline)

## Outstanding
- g3rs-release-config-checks has 0 tests — check implementations need test coverage
- Checks not yet wired into the app (old rules still run inline)

## Key Files
- `packages/release-plz-toml-parser/crates/parser/types/src/release_plz_toml.rs`
- `packages/cliff-toml-parser/crates/parser/types/src/cliff_toml.rs`
- `packages/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/g3rs-release-config-ingestion/crates/runtime/src/run.rs`
