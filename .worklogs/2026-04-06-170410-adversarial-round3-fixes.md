# Adversarial round 3: strengthen weak tests + fix handoff plans

**Date:** 2026-04-06 17:04
**Scope:** crawl tests, deny/cargo ingestion tests, handoff plans

## Summary
Round 3 adversarial attack found Phase 1 banned-dir filter had zero test coverage in isolation, several ingestion tests were vacuous (passed without exercising claimed behavior), and handoff plans would cause duplicate TOML sections. Fixed all.

## Fixes
- Crawl: added `banned_dirs_excluded_from_phase1_without_gitignore` — proves filter_entry works on un-gitignored node_modules
- Deny ingestion: `prefers_deny_toml_over_dot_variant` now writes distinguishable content and verifies parsed content matches the correct file
- Both ingestion packages: `ignored_but_recovered` tests now verify `ignore_state == Ignored` on crawl entry before ingesting
- Deny ingestion: added `nested_deny_toml_is_not_selected` test
- Handoff plans: changed "add shared=true" to "verify present" to prevent duplicate TOML sections

## Key Files
- `packages/g3rs-workspace-crawl/crates/runtime/src/crawl_tests/ignore_state.rs`
- `packages/g3rs-deny-config-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/g3rs-cargo-config-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `.plans/2026-04-06-163144-handoff-*.md`
