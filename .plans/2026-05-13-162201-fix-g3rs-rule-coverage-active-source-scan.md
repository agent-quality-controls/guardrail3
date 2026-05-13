# Fix G3RS Rule Coverage Active Source Scan

## Goal

Make `scripts/behavior/verify-rule-coverage.py` count only runtime rule source files as active `g3rs-*/*` rule IDs.

## Problem

The first coverage matrix pass scanned every Rust file under `packages/rs` and `apps/guardrail3-rs`.

That incorrectly counted rule IDs that appear only in:

- sidecar test modules
- integration tests
- assertion crates
- generated build output

Those IDs cannot be emitted by the runtime CLI. Fixture work based on those IDs would create fake coverage targets.

## Approach

- Update `verify-rule-coverage.py` with one `is_active_rule_source(path)` filter.
- Exclude `target` and `.cargo-target` paths.
- Exclude path components named `tests`, `rule_tests`, and `contract_tests`.
- Exclude path components ending in `_tests`.
- Exclude assertion crates and assertion modules by excluding path component `assertions`.
- Regenerate `behavior/coverage/g3rs-rule-coverage.toml` from the corrected active source set.
- Update the matrix manifest expected counts.
- Update the prose matrix plan so its measured counts and missing hook IDs match the corrected scanner.

## Key Decisions

- Do not implement fixtures for test-only hook IDs.
- Do not hide assertion crates behind a special status. They are not active runtime rules.
- Keep baseline parsing unchanged. Replay baselines are still the source of current behavior.

## Files To Modify

- `scripts/behavior/verify-rule-coverage.py`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md.manifest.toml`
