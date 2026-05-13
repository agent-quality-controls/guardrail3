# Goal

Make L50 represent only delegated external tool availability, and remove the same unrelated cargo-format gate noise from L60.

# Current Evidence

- The only G3RS source of host executable availability findings is `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run.rs`.
- `discover_installed_tools` checks exactly:
  - `cargo-deny`
  - `cargo-machete`
  - `cargo-dupes`
  - `gitleaks`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/run.rs` runs exactly four installed-tool rules.
- Current `L50-required-inputs-valid-delegated-tools-missing` already captures all four missing-tool findings.
- Current L50 baseline also has `cargo fmt --all -- --check` stderr noise because the fixture source file is not rustfmt-clean.
- Current L60 baseline has the same unrelated `cargo fmt --all -- --check` stderr noise from the same empty source-file shape.
- The behavior runner's `path_without_delegated_tools` mode uses a hand-written executable whitelist. After fixing the source file, that whitelist hides normal build tools such as `cc` and `xcrun`, so `cargo test` can fail for reasons unrelated to delegated-tool availability.

# Decisions

- Do not add another L50 fixture.
- Do not split per missing tool. The four tool checks do not hide each other because they are independent results over the same `installed_tools` set.
- Do not use `--rules-only` for L50. The public behavior should stay `g3rs validate --path . --inventory` because missing delegated tools should be visible through normal validate.
- Fix the L50 fixture source so cargo gates do not add unrelated format noise.
- Fix the L60 fixture source for the same reason.
- Replace the runner whitelist with a filtered copy of the current executable PATH surface. Exclude only the delegated tool names that the L50 fixture is meant to hide.
- Keep L50 manifest `required_results` unchanged except for baseline hash drift.
- Keep L60 manifest `required_results` unchanged except for baseline hash drift.

# Files To Modify

- `scripts/behavior/baseline_common.py`
- `behavior/fixtures/g3rs/L50-required-inputs-valid-delegated-tools-missing/repo/src/lib.rs`
- `behavior/baselines/g3rs/L50-required-inputs-valid-delegated-tools-missing/command-00.json`
- `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid/repo/src/lib.rs`
- `behavior/baselines/g3rs/L60-delegated-tools-present-policy-invalid/command-00.json`
- `.worklogs/<timestamp>-fix-g3rs-l50-delegated-tool-fixture.md`

# Verification

- `scripts/behavior/verify-all.sh`
- Direct baseline inspection must show L50 `stderr` has no `cargo gate failed`, no rustfmt diff, and no linker/tool-not-found error.
- Direct baseline inspection must show L60 `stderr` has no `cargo gate failed`, no rustfmt diff, and no linker/tool-not-found error.
- Direct baseline inspection must show L50 Error/Warn rows are only:
  - `g3rs-deps/cargo-deny-installed`
  - `g3rs-deps/cargo-machete-installed`
  - `g3rs-deps/cargo-dupes-installed`
  - `g3rs-deps/gitleaks-installed`
- Direct baseline inspection must show L60 Error/Warn rows are only:
  - `g3rs-clippy/too-many-lines-threshold`
- `python3 -m py_compile scripts/behavior/*.py`
- `git diff --check`
- `g3rs validate-repo`
