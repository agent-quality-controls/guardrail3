# Summary

Ran an adversarial fmt hardening pass by adding tests first and fixing only the behavior that those tests actually broke. The real package bug was deleted-after-crawl root `rustfmt.toml` handling: config ingestion was still aborting instead of degrading to `ParseError` state like the other root manifests.

## Decisions made

- Treated deleted-after-crawl root files as the same rule-owned blocker class as unreadable files.
  - Why: the package model already preserves missing/parse/blocker states inside rule inputs, and a stale crawl should not crash config ingestion for `rustfmt.toml` only.
- Corrected the bad pipeline expectation instead of changing code.
  - Why: `RS-FMT-CONFIG-04` only applies when rustfmt `edition` exists. The failing test was asserting behavior the rule does not claim.
- Added more branch and boundary tests after the first bug fix without changing logic again.
  - Why: the convergence agents found unpinned branches, not more proved bugs.

## Key files for context

- `.plans/2026-04-12-153538-fmt-adversarial-hardening.md`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/ingest_tests/filetree.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_02_extra_settings/rule_tests/skip_macro_invocations.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_03_nightly_keys_on_stable/rule_tests/no_nightly_keys.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_03_nightly_keys_on_stable/rule_tests/non_stable.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_04_edition_mismatch/rule_tests/no_rustfmt_edition.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_04_edition_mismatch/rule_tests/package_edition.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_04_edition_mismatch/rule_tests/precedence.rs`

## Next steps

- The concrete fmt package bug found in this pass is fixed.
- If fmt gets another attack pass, the remaining worthwhile hardening is mostly exact-result pipeline assertions in `ingest_tests/pipeline.rs`.
