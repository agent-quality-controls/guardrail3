# G3RS Hook Replay Stage Two

## Goal

Make the corrected Stage 2 hook replay targets visible through public `g3rs validate-repo --inventory` baselines.

## Corrected Targets

The active source scanner now reports 39 absent runtime IDs.

Stage 2 owns:

- 13 family hook contract IDs
- 9 hooks IDs

Stage 2 does not own IDs that only appear in tests or assertion crates.

## Approach

- Add hook-contract inventory output inside `g3rs-hooks-config-checks`.
- Emit one inventory row per loaded `G3HookRequirement`.
- Use each requirement's own `id`, for example `g3rs-fmt/hook-contract`.
- Keep this under the hooks family run path because `validate-repo` already loads every Rust family hook contract there.
- Add rule-specific sidecar tests for the new hook-contract inventory rule.
- Expand `R15-hooks-reachable-no-root-cargo` required results to include the 13 hook-contract inventory rows.
- Add `R16-hooks-required-steps-present-but-weakened` for real hook command findings that can coexist in one pre-commit script without hiding each other.
- Add `R17-hooks-modular-scripts-invalid` for modular hook script findings that need a `pre-commit.d` directory and cannot be emitted by `R16`.
- Regenerate validate-repo baselines.
- Update `behavior/coverage/g3rs-rule-coverage.toml` so newly emitted IDs move from planned to covered.

## Fixture Split

`R16-hooks-required-steps-present-but-weakened` must cover:

- `g3rs-hooks/cargo-dupes-excludes`
- `g3rs-hooks/cargo-dupes-installed`
- `g3rs-hooks/clippy-denies-warnings`
- `g3rs-hooks/contract-critical-command-not-fail-open`
- `g3rs-hooks/executable-command-context-only`
- `g3rs-hooks/guardrail-binary-available`
- `g3rs-hooks/no-fail-open-wrappers`

`R17-hooks-modular-scripts-invalid` must cover:

- `g3rs-hooks/modular-scripts-executable`
- `g3rs-hooks/real-dispatcher-syntax-only`

## Files To Modify

- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/hook_contract_inventory/*`
- `behavior/fixtures/g3rs-validate-repo/R16-hooks-required-steps-present-but-weakened/*`
- `behavior/fixtures/g3rs-validate-repo/R17-hooks-modular-scripts-invalid/*`
- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `behavior/baselines/g3rs-validate-repo/*`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md.manifest.toml`
