# Goal

Make the L40-L70 G3RS behavior fixtures execute their intended workspace instead of failing on a nonexistent nested `repo` path.

# Problem

The L40-L70 fixtures use:

```toml
run_from = "repo"
commands = [
  ["validate", "--path", "repo", "--inventory"],
]
```

`run_from = "repo"` already sets the command cwd to the fixture repo. `--path repo` therefore points to `repo/repo`, which does not exist. The baselines currently prove only `path is not a directory: repo`, not the intended fixture layers.

# Approach

- Change L40-L70 fixture commands from `--path repo` to `--path .`.
- Keep each fixture's existing `run_from = "repo"` setting.
- Fix behavior baseline normalization if the corrected fixtures expose volatile Cargo timing or test-binary hashes.
- Implement `runner_mode = "path_without_delegated_tools"` in the behavior runner so L50 actually hides delegated binaries while keeping the Rust toolchain available.
- Remove irrelevant cargo-deny unused allowlist warnings from L50-L70 fixture inputs.
- Add verifier checks for volatile baseline text so regenerated baselines cannot pass with nondeterministic Cargo output.
- Regenerate behavior baselines.
- Run `scripts/behavior/verify-all.sh`.
- Inspect the regenerated L40-L70 outputs and report whether they now exercise real behavior.

# Files To Modify

- `behavior/fixtures/g3rs/L40-required-inputs-present-invalid/fixture.toml`
- `behavior/fixtures/g3rs/L50-required-inputs-valid-delegated-tools-missing/fixture.toml`
- `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid/fixture.toml`
- `behavior/fixtures/g3rs/L70-delegated-policy-valid-project-policy-violated/fixture.toml`
- `behavior/baselines/g3rs/L40-required-inputs-present-invalid/command-00.json`
- `behavior/baselines/g3rs/L50-required-inputs-valid-delegated-tools-missing/command-00.json`
- `behavior/baselines/g3rs/L60-delegated-tools-present-policy-invalid/command-00.json`
- `behavior/baselines/g3rs/L70-delegated-policy-valid-project-policy-violated/command-00.json`
- `scripts/behavior/baseline_common.py`
- `scripts/behavior/verify-baselines.py`
