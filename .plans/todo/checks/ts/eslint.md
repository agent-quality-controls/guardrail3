# TS-ESLINT — ESLint config and plugin checker

**Input:** `eslint.config.*`, root `package.json`
**Parser:** tree-sitter/structured `ESLint` config parser + JSON
**Current code:** `app/ts/validate/eslint_check.rs`, `eslint_plugin_checks.rs`, `eslint_parser.rs`, `eslint_rule_infra.rs`, plugin portion of `package_deps.rs`
**Owned root:** nearest TS package/app root with an `eslint` config surface

## Owns

- `ESLint` config file existence and parseability
- core plugin package presence in root `devDependencies`
- content-profile plugin package presence when content checks are enabled
- plugin version/conflict/deprecation policy where that is part of the `ESLint` surface
- plugin wiring in `eslint.config.*`
- core rule baseline / threshold / severity policy
- local rule presence/value checks that are not architecture-zone specific

## Does not own

- boundary-zone configuration and import-direction policy
  - that belongs to `ts/hexarch`
- CSS lint config
  - that belongs to `ts/css`
- TypeScript compiler settings
  - that belongs to `ts/tsconfig`

## Current old-code split to normalize

- `eslint_check.rs`
  - baseline config existence and rule/value checks
- `eslint_plugin_checks.rs`
  - plugin-wiring checks
- `package_deps.rs`
  - plugin-package presence checks
- `eslint_audit.rs`
  - only the non-architecture baseline parts should stay here; boundaries-specific checks move to `ts/hexarch`
- old plugin/version audit docs under `.plans/todo/typescript/ts_guardrails_implementation.md`
  - baseline package/version/conflict material to normalize into this family

## Contract direction

The family should end up owning the full `ESLint` baseline:
- file exists
- parses
- required plugins installed
- required plugins enabled
- required rules enabled at the correct severity/value
- conflicting or deprecated plugin packages rejected
- extra relaxations inventoried or rejected according to policy

This family is the TS equivalent of a config/checker hybrid like Rust `clippy` plus part of Rust `cargo`.
