Summary:
- Migrated G3TS from the old flat validate commands to nested `g3ts validate repo` and `g3ts validate workspace`.
- Added top-level `g3ts --version`, updated hook-source enforcement, refreshed G3TS fixture baselines, and installed the new local `g3ts` binary.

Decisions:
- Removed the old `g3ts validate-repo` command shape instead of keeping an alias because aliases make hook contracts and downstream agent instructions ambiguous.
- Kept repo validation and workspace validation separate. Repo validation checks repo-level hook/tool/topology/marker-pair invariants; workspace validation checks one adopted TypeScript unit.
- Added `scripts/verify-g3ts-validate-command-surface.py` so the command-surface migration is mechanically checked.
- Updated `packages/ts/hooks/g3ts-hooks-contract-types/Cargo.toml` to resolver 3 because G3RS blocked validation of the touched workspace under the runtime compatibility rule.

Key files:
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`
- `behavior/fixtures/g3ts-cli-output`
- `behavior/fixtures/g3ts-rule`
- `behavior/fixtures/g3ts-validate-repo`
- `scripts/verify-g3ts-validate-command-surface.py`
- `.plans/2026-05-18-141200-g3ts-validate-command-surface.md`

Verification:
- `python3 scripts/verify-g3ts-validate-command-surface.py`: pass.
- `cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml`: pass.
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-ts/Cargo.toml -- -D warnings`: pass.
- `fixture3 check --all`: pass.
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`: pass.
- `g3rs validate workspace --path apps/guardrail3-ts`: exit 0 with existing warnings only.
- `g3rs validate workspace --path packages/ts/hooks/g3ts-hooks-source-checks`: exit 0 with existing warnings only.
- `g3rs validate workspace --path packages/ts/hooks/g3ts-hooks-contract-types`: exit 0 with existing warnings only.

Known signals:
- `g3ts validate repo --path .` now runs the new command but reports real repo findings in this repository's current tree, including stale hook command wiring and marker-pair issues under fixture reduction scratch paths. This commit changes the CLI contract; it does not make this repository pass G3TS repo validation.
- `git status` still shows many deleted old `.plans/2026-03...` files that were already present before this change. They were not staged.
