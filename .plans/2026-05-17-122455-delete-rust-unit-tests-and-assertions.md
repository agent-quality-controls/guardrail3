# Delete Rust Unit Tests And Assertion Crates

## Goal

Move G3RS to external behavior verification only.

The active Rust test surface must no longer contain unit-test sidecars, inline `#[cfg(test)]` modules, or local `crates/assertions` helper crates. The remaining verification contract is `fixture3` plus behavior verifier scripts.

## Scope

In scope:

- `apps/guardrail3-rs`
- `packages/rs`
- `packages/shared`
- `packages/parsers`

Out of scope:

- `packages/ts`
- `apps/guardrail3-ts`
- `legacy`
- `node_modules`
- `behavior/fixtures`
- `behavior/golden`
- behavior verifier scripts

Reason: current G3RS fixtures prove the Rust CLI external behavior. They do not prove TS package behavior, legacy app behavior, or third-party vendored package behavior.

## Approach

1. Remove Rust assertion helper crates.
   - Delete every in-scope `crates/assertions` directory.
   - Remove `"crates/assertions"` from package workspace members.
   - Remove local assertion crate dev-dependencies from runtime crates.
   - Remove assertion crate names from `guardrail3-rs.toml` adoption/allow lists.

2. Remove unit-test sidecars.
   - Delete every in-scope directory named `*_tests`, `tests`, or `contract_tests` when it belongs to source test code.
   - Do not delete fixture directories under `behavior`.

3. Remove test module wiring from runtime source.
   - Remove `#[cfg(test)]` plus `#[path = "..._tests/mod.rs"] mod ...;` blocks.
   - Remove `#[cfg(test)] mod tests;` blocks.
   - Remove test-only public modules used only for test support.

4. Regenerate or clean workspace metadata.
   - Run cargo metadata/check commands to expose stale workspace members or dev-dependencies.
   - Regenerate Cargo.lock files only where lockfiles changed because assertion crates were removed.

5. Verify external behavior stayed stable.
   - `fixture3 check --suite g3rs-rule-fixtures`
   - `fixture3 check --all`
   - `python3 scripts/behavior/verify-family-rule-fixtures.py`
   - `python3 scripts/behavior/verify-rule-coverage.py`
   - `python3 scripts/behavior/verify-test-deletion.py`
   - `bash scripts/behavior/verify-all.sh`
   - `g3rs validate repo --path "$PWD"`

## Done Criteria

- No in-scope `crates/assertions` directories remain.
- No in-scope `*_tests`, `tests`, or `contract_tests` source directories remain.
- No in-scope Rust source file contains test module wiring.
- No in-scope Cargo.toml references local assertion crates.
- All behavior verifiers pass.
