# Summary

Added Astro setup checks that require strict Astro apps to expose fail-closed package scripts for delegated validators.

# Decisions Made

- Added `g3ts-astro-setup/lint-script` for a safe `lint` script invoking `eslint`.
- Added `g3ts-astro-setup/syncpack-lint-script` for a safe `lint:packages` script invoking `syncpack lint`.
- Kept the implementation in Astro setup because this is the app-level execution contract for delegated Astro validation.
- Reused parsed package-script facts from `package-script-command-parser`; rejected raw script substring checks.

# Key Files

- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/ts_astro_config_33_lint_script.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/ts_astro_config_34_syncpack_lint_script.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/lib_tests/cases.rs`

# Verification

- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/Cargo.toml`
- `cargo test --workspace --offline --locked` in `apps/guardrail3-ts`
- `g3rs validate --path packages/ts/astro/setup/g3ts-astro-setup-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

# Next Steps

- Add visible suppression/waiver checks for `eslint-disable` escapes in strict Astro content apps.
