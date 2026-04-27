# Goal

Make strict Astro apps prove that delegated validators are actually executable through app package scripts.

# Approach

- Add setup config checks for validator execution:
  - `g3ts-astro-setup/lint-script`: `package.json` must contain a safe `lint` script that invokes `eslint`.
  - `g3ts-astro-setup/syncpack-lint-script`: `package.json` must contain a safe `lint:packages` script that invokes `syncpack lint`.
- Keep the existing execution contracts:
  - `g3ts-astro-setup/astro-check-present` already requires a safe `astro check` invocation.
  - `g3ts-astro-seo/nuasite-checks` already requires a safe `build` script invoking `astro build`, which executes Nuasite during Astro build.
- Use existing parsed package-script facts from `package-script-command-parser`.
- Do not inspect raw script strings.
- Add tests that first prove missing scripts fail, unsafe `|| true` scripts fail, and valid scripts pass.

# Key Decisions

- This belongs to Astro setup because it is the app-level execution contract for all delegated Astro validators.
- ESLint content/MDX/SEO rule wiring remains in content/MDX/SEO subfamilies.
- Syncpack package policy content remains in setup config checks, but app execution of `syncpack lint` also belongs to setup.

# Files To Modify

- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/lib.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/ts_astro_config_33_lint_script.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/ts_astro_config_34_syncpack_lint_script.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/lib_tests/cases.rs`
- `.worklogs/<timestamp>-astro-validator-execution-scripts.md`

# Verification

- `cargo test --package g3ts-astro-setup-config-checks-runtime`
- `cargo test --workspace --offline --locked` in `apps/guardrail3-ts`
- `g3rs validate --path` for touched packages.
- Install local `g3ts`.
- Run `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`.
