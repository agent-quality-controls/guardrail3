# Summary

Deleted the active Rust unit-test surface from `apps/guardrail3-rs`, `packages/rs`, `packages/shared`, and `packages/parsers`.
The remaining verification surface is fixture-driven CLI behavior plus the behavior ledger scripts.

# Decisions Made

- Removed `crates/assertions`, `crates/test_support`, sidecar `*_tests`, `tests`, and `contract_tests` directories from the active Rust/G3RS scope.
- Removed assertion and test-support workspace members, dev-dependencies, guardrail allowlist entries, and stale release metadata.
- Regenerated active Rust lockfiles so deleted helper crates are not left in dependency metadata.
- Removed the `g3rs-report-output` fixture3 suite because it executed a deleted assertion crate and constructed internal `ValidateReport` objects instead of testing the external CLI surface.
- Fixed two post-deletion production issues exposed by the managed hook: `reason-policy` now declares its remaining runtime crate as a workspace member, and hooks config check rules no longer duplicate their tool-availability implementation shape.
- Kept production rule code that mentions `#[cfg(test)]`, `#[path]`, `crates/assertions`, or `test_support` because those strings are user-repo rule behavior, not active unit tests.
- Updated behavior verification scripts so final test deletion means zero active Rust tests, not "kept tests still exist".

# Key Files

- `fixture3.yaml`
- `scripts/behavior/verify-all.sh`
- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-test-fixture-ledger.py`
- `behavior/golden/*/approved.meta.json`
- `apps/guardrail3-rs/Cargo.toml`
- `packages/rs/**/Cargo.toml`
- `packages/parsers/**/Cargo.toml`
- `packages/shared/**/Cargo.toml`

# Verification

- `cargo build --quiet --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs --bin g3rs`
- `fixture3 check --all --json`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate repo`
- `.githooks/pre-commit.d/g3rs`
- `git diff --check`
- `python3 scripts/behavior/list-rust-tests.py --format json`
- `find apps/guardrail3-rs packages/rs packages/shared packages/parsers -type d \( -name '*_tests' -o -name tests -o -name contract_tests -o -name assertions -o -name test_support \) ! -path '*/target/*'`

# Next Steps

- If future behavior gaps are found, add or modify fixture inputs and approved output instead of reintroducing unit-test or assertion-helper crates.
