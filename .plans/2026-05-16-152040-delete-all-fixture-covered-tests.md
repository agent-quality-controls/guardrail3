# Delete all fixture-covered Rust tests

## Goal

Delete every active Rust `#[test]` function that is already replaceable by behavior fixtures, without deleting any test that still proves behavior the fixture system does not cover.

The fixture ledger row count must stay fixed at 1577. Deleted tests remain historical ledger rows. The active test count must fall from 1530 to 740 when all 790 active replaceable tests are removed.

## Current counts

- fixture ledger rows: 1577
- active Rust tests: 1530
- active replaceable tests: 790
- active kept tests: 740
- whole-file deletion candidates: 346 files
- tests inside whole-file deletion candidates: 790

## Replacement rule

A test function can be deleted only when `scripts/behavior/verify-test-deletion.py` already classifies it as replaceable.

That means:

- fixture status is `covered_hit`
- or fixture status is `covered_non_hit`
- or fixture status is `kept_compile_contract` and kept-test disposition is `covered_by_cli_output`
- or fixture status is `kept_compile_contract` and kept-test disposition is `covered_by_renderer_output`

No other status can be deleted.

## Deletion batches

Delete candidate test files package by package. After each package, regenerate ledgers and run the local package check before continuing.

The current whole-file candidate batches are:

- `packages/rs/code/g3rs-code-source-checks`: 173 tests
- `packages/rs/deny/g3rs-deny-config-checks`: 130 tests
- `packages/rs/hooks/g3rs-hooks-source-checks`: 109 tests
- `packages/rs/test/g3rs-test-file-tree-checks`: 60 tests
- `packages/rs/test/g3rs-test-source-checks`: 55 tests
- `packages/rs/release/g3rs-release-config-checks`: 35 tests
- `packages/rs/fmt/g3rs-fmt-config-checks`: 27 tests
- `packages/rs/topology/g3rs-topology-file-tree-checks`: 27 tests
- `packages/rs/clippy/g3rs-clippy-config-checks`: 25 tests
- `packages/rs/deps/g3rs-deps-config-checks`: 24 tests
- `packages/rs/garde/g3rs-garde-source-checks`: 23 tests
- `packages/rs/hooks/g3rs-hooks-config-checks`: 21 tests
- `packages/rs/hooks/g3rs-hooks-file-tree-checks`: 19 tests
- `packages/rs/toolchain/g3rs-toolchain-config-checks`: 17 tests
- `packages/rs/test/g3rs-test-config-checks`: 13 tests
- `packages/rs/garde/g3rs-garde-config-checks`: 10 tests
- `packages/rs/fmt/g3rs-fmt-filetree-checks`: 7 tests
- `packages/rs/release/g3rs-release-repo-root-checks`: 6 tests
- `packages/rs/code/g3rs-code-config-checks`: 5 tests
- `packages/rs/clippy/g3rs-clippy-filetree-checks`: 4 tests

## Edit rules

- Delete only files where every active test in the file is replaceable.
- Delete a sidecar test directory only when every active test in that directory is replaceable.
- Remove the matching `#[cfg(test)] #[path = "..."] mod ...;` declaration from the production owner after deleting its sidecar test directory.
- Do not remove production code.
- Do not remove kept tests.
- Do not remove ledger rows for deleted tests.

## Verification after each batch

Run:

```sh
python3 scripts/behavior/classify-test-fixture-ledger.py
python3 scripts/behavior/classify-kept-test-dispositions.py
cargo test --manifest-path <package>/Cargo.toml --workspace --all-targets --all-features
cargo clippy --manifest-path <package>/Cargo.toml --workspace --all-targets --all-features -- -D warnings
python3 scripts/behavior/classify-test-fixture-ledger.py --check
python3 scripts/behavior/verify-test-fixture-ledger.py --strict
python3 scripts/behavior/classify-kept-test-dispositions.py --check
python3 scripts/behavior/verify-kept-test-dispositions.py
python3 scripts/behavior/verify-test-deletion.py
```

## Final verification

Run:

```sh
bash scripts/behavior/verify-all.sh
g3rs validate repo --path "$PWD"
git diff --check
```

The final `verify-test-deletion.py` output must include:

```text
rows:1577 active:740 replaceable:837 kept:740
```
