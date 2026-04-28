## Summary

Implemented TypeScript hook contracts in the Rust hook-contract style and hardened Rust hook fail-open severity. Added TS hook contract packages, TS hook checks, TS hook ingestion, CLI family wiring, and Astro-owned hook contract packages.

## Decisions Made

- TS families expose no-arg `hook_contract()` functions from family-owned hook-contract packages instead of making ingestion or checks publish hook requirements.
- `g3ts-hooks` consumes parsed hook scripts from `hook-shell-parser`; comments and echoed text do not satisfy validation command requirements.
- Hook checks aggregate the effective hook surface rather than checking each script in isolation.
- App validation commands must target the app root, so `pnpm --filter wrong-app run validate` and `g3ts validate --path wrong-app` do not satisfy the landing contract.
- Rust fail-open critical hook findings are hard errors now, matching fail-closed behavior.

## Key Files For Context

- `apps/guardrail3-ts/crates/types/app-types/src/supported_family.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-contract-types/src/lib.rs`
- `packages/ts/hooks/g3ts-hooks-ingestion/src/lib.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/src/lib.rs`
- `packages/ts/hooks/g3ts-hooks-file-tree-checks/src/lib.rs`
- `packages/ts/hooks/g3ts-hooks-config-checks/src/lib.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-hook-contract/src/lib.rs`
- `packages/ts/astro/content/g3ts-astro-content-hook-contract/src/lib.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-hook-contract/src/lib.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-hook-contract/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/no_fail_open_wrappers/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/contract_critical_command_not_fail_open/rule.rs`

## Verification

- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/parsers/hook-shell-parser/Cargo.toml --workspace --offline --locked`
- `cargo clippy --manifest-path apps/guardrail3-ts/Cargo.toml -p guardrail3-ts-family-runner-hooks -p g3ts-hooks-config-checks -p g3ts-hooks-file-tree-checks -p g3ts-hooks-source-checks -p g3ts-hooks-ingestion -p g3ts-hooks-types -p g3ts-hooks-contract-types --all-targets --offline --locked -- -D warnings`
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family hooks --inventory`

## Real Repo Result

`g3ts` now reads the landing app's Git-root `.githooks/pre-commit` correctly. The landing repo currently fails the new hooks family because its hook does not run `g3ts validate --path ...`, does not run the app-level `validate` script, and does not route the Astro hook contract trigger patterns.

## Next Steps

- Update the landing repo hook to satisfy `g3ts-hooks`.
- If the trigger routing checks become too noisy, replace the current executable-command trigger evidence with a stricter control-flow-aware rule.
