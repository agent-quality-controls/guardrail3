# TS Tooling Family Buildout

## Goal

Build the missing easy TS families that are not superseded by Astro or style work:

- `ts/fmt`
- `ts/spelling`
- `ts/typecov`
- `ts/package` cleanup

Do not build:

- `ts/content`
- a separate `ts/css`
- `ts/code` in this pass

`ts/code` is intentionally excluded from this first pass because it is the source-parsing-heavy family. This plan starts with delegated-tool wiring families where G3TS can verify installation, config, scripts, hooks, and Syncpack policy without reimplementing the tools.

## Architecture Rules

- Use the current package structure:
  - `packages/ts/<family>/g3ts-<family>-types`
  - `packages/ts/<family>/g3ts-<family>-ingestion`
  - `packages/ts/<family>/g3ts-<family>-config-checks`
  - optional `packages/ts/<family>/g3ts-<family>-hook-contract`
- Rules emit semantic IDs:
  - `g3ts-fmt/...`
  - `g3ts-spelling/...`
  - `g3ts-typecov/...`
  - `g3ts-package/...`
- No numbered rule IDs.
- No family-local parsing of config formats.
- Ingestion may call shared parser packages only:
  - `package-json-parser`
  - `package-script-command-parser`
  - `syncpack-config-parser`
  - new parser packages only if a dedicated config format must be parsed and no existing parser covers it.
- G3TS checks wiring and fail-closed contracts.
- Delegated tools do the real work:
  - Prettier formats source.
  - cspell checks spelling.
  - type-coverage measures type coverage.
  - Syncpack owns dependency floors and package bans.
- Package-specific checks stay in `ts/package`.
- Tool-specific package/script checks move out of `ts/package`.

## Shared Pattern

Each new family gets:

- types crate with DTO structs in `src/types.rs` and facade-only `src/lib.rs`
- ingestion package:
  - app package root discovery from the crawl
  - `package.json` facts through `package-json-parser`
  - script facts through `package-script-command-parser`
  - Syncpack facts through `syncpack-config-parser` when the family owns a package floor
  - family config facts only if the tool has a config file surface
- config-checks package:
  - one semantic rule module per rule where practical
  - `run.rs` only orchestrates rule modules
  - rule-specific sidecar tests
  - assertions crate helpers for result-shape proof
- hook-contract package when the family must be routed through hooks
- CLI integration:
  - add `SupportedFamily` variant
  - add family CLI name
  - add runner wiring
  - add selection tests

## `ts/fmt`

### Ownership

Owns formatting tool wiring, not formatting semantics.

### Delegation

G3TS delegates formatting to Prettier. G3TS does not parse or format TS/CSS/MDX source.

### Inputs

- app package root
- package dependencies/devDependencies
- package scripts and parsed command invocations
- Prettier config file presence and parseability
- Syncpack version-group facts for Prettier package pinning

### Config files

Accept one local app-root config:

- `prettier.config.js`
- `prettier.config.cjs`
- `prettier.config.mjs`
- `.prettierrc`
- `.prettierrc.json`
- `.prettierrc.yaml`
- `.prettierrc.yml`
- `.prettierrc.toml`

For this pass:

- existence is required
- Prettier config semantics are delegated to Prettier itself
- JS/MJS/CJS/JSON/YAML/TOML config files are treated as present and not evaluated by G3TS
- a future `prettier-config-parser` package may add parseability checks, but the fmt family must not parse Prettier config ad hoc

### Required packages

- `prettier`

### Required scripts

Standard script names:

- `format`
- `format:check`

`format:check` must execute `prettier` with:

- `--check`

`format` must execute `prettier` with:

- `--write`

### Validate contract

The app `validate` script must reach `format:check` or a direct `prettier --check` invocation.

Any reachable `||` fail-open chain invalidates the contract.

### Syncpack contract

`.syncpackrc` must have a non-ignored version group that pins:

- `prettier`

### Hook contract

Require hook routing for:

- `package.json`
- `.syncpackrc`
- Prettier config files
- `*.ts`
- `*.tsx`
- `*.astro`
- `*.md`
- `*.mdx`
- `*.css`
- `*.json`
- `*.yml`
- `*.yaml`

Hook command must run the project validation path that includes `format:check`.

### Rules

- `g3ts-fmt/policy-configured`
  - error when no app package root can be evaluated for formatter policy.
- `g3ts-fmt/prettier-package-present`
  - error when `prettier` is not in local app dependencies/devDependencies.
- `g3ts-fmt/prettier-config-present`
  - error when no accepted Prettier config file exists at the app root.
- `g3ts-fmt/format-scripts`
  - error when `format` or `format:check` is missing or not parsed.
- `g3ts-fmt/format-check-fail-closed`
  - error when `format:check` does not invoke `prettier --check` or is guarded by `||`.
- `g3ts-fmt/validate-runs-format-check`
  - error when `validate` does not reach `format:check` or direct `prettier --check`.
- `g3ts-fmt/syncpack-prettier-pin`
  - error when Syncpack does not pin `prettier`.

## `ts/spelling`

### Ownership

Owns spelling tool wiring, not dictionary correctness.

### Delegation

G3TS delegates spelling analysis to cspell.

### Inputs

- app package root
- package dependencies/devDependencies
- package scripts and parsed command invocations
- cspell config presence and parseability
- Syncpack facts for cspell package pinning

### Config files

Accept one local app-root config:

- `cspell.json`
- `cspell.config.json`
- `.cspell.json`
- `cspell.yaml`
- `cspell.yml`

For this pass:

- JSON configs must be parseable by `serde_json` or a shared parser if present
- YAML config existence is accepted and delegated to cspell runtime unless a shared YAML parser already exists

### Required packages

- `cspell`

### Required scripts

Standard script name:

- `spellcheck`

`spellcheck` must execute `cspell`.

### Validate contract

The app `validate` script must reach `spellcheck` or a direct `cspell` invocation.

Any reachable `||` fail-open chain invalidates the contract.

### Syncpack contract

`.syncpackrc` must have a non-ignored version group that pins:

- `cspell`

### Hook contract

Require hook routing for:

- `package.json`
- `.syncpackrc`
- cspell config files
- authored text/source surfaces:
  - `*.ts`
  - `*.tsx`
  - `*.astro`
  - `*.md`
  - `*.mdx`
  - `*.json`
  - `*.yml`
  - `*.yaml`

Hook command must run the project validation path that includes `spellcheck`.

### Rules

- `g3ts-spelling/policy-configured`
  - error when no app package root can be evaluated for spelling policy.
- `g3ts-spelling/cspell-package-present`
  - error when `cspell` is not in local app dependencies/devDependencies.
- `g3ts-spelling/cspell-config-present`
  - error when no accepted cspell config exists at the app root.
- `g3ts-spelling/spellcheck-script`
  - error when `spellcheck` is missing or not parsed.
- `g3ts-spelling/spellcheck-fail-closed`
  - error when `spellcheck` does not invoke `cspell` or is guarded by `||`.
- `g3ts-spelling/validate-runs-spellcheck`
  - error when `validate` does not reach `spellcheck` or direct `cspell`.
- `g3ts-spelling/syncpack-cspell-pin`
  - error when Syncpack does not pin `cspell`.

## `ts/typecov`

### Ownership

Owns explicit type-coverage tool wiring. It does not own compiler strictness.

### Delegation

G3TS delegates measurement to `type-coverage`.

### Inputs

- app package root
- package dependencies/devDependencies
- package scripts and parsed command invocations
- package/config threshold policy
- Syncpack facts for type-coverage package pinning

### Required packages

- `type-coverage`

### Required scripts

Standard script name:

- `typecov`

`typecov` must execute `type-coverage`.

### Threshold

`typecov` must include a fail-closed threshold:

- `--at-least <number>`

Initial threshold policy:

- require `--at-least 100`

If this proves too strict for real app roots, the waiver mechanism is the escape hatch. Do not weaken the default.

### Validate contract

The app `validate` script must reach `typecov` or a direct `type-coverage --at-least 100` invocation.

Any reachable `||` fail-open chain invalidates the contract.

### Syncpack contract

`.syncpackrc` must have a non-ignored version group that pins:

- `type-coverage`

### Hook contract

Require hook routing for:

- `package.json`
- `.syncpackrc`
- `tsconfig*.json`
- `*.ts`
- `*.tsx`
- `*.astro`

Hook command must run the project validation path that includes `typecov`.

### Rules

- `g3ts-typecov/policy-configured`
  - error when no app package root can be evaluated for type coverage policy.
- `g3ts-typecov/package-present`
  - error when `type-coverage` is not in local app dependencies/devDependencies.
- `g3ts-typecov/script-present`
  - error when `typecov` is missing or not parsed.
- `g3ts-typecov/threshold-fail-closed`
  - error when `typecov` does not invoke `type-coverage --at-least 100` or is guarded by `||`.
- `g3ts-typecov/validate-runs-typecov`
  - error when `validate` does not reach `typecov` or direct `type-coverage --at-least 100`.
- `g3ts-typecov/syncpack-type-coverage-pin`
  - error when Syncpack does not pin `type-coverage`.

## `ts/package` Cleanup

### Ownership

Keep only generic package manifest policy here.

### Current Useful Rules To Keep

- root `package.json` exists
- root `package.json` parseable
- root private policy
- root package manager policy
- root engines policy
- root scripts policy for generic scripts:
  - `lint`
  - `typecheck`
  - `validate`
- root pnpm policy
- local banned dependencies

### Add Or Tighten

- `g3ts-package/validate-script-present`
  - require standard `validate` script name.
- `g3ts-package/validate-script-fail-closed`
  - parse `validate` through `package-script-command-parser`.
  - reject unsupported parse or reachable `||`.
  - do not require specific tools here.
- `g3ts-package/package-policy-not-tool-dump`
  - ensure package checks do not emit formatter/spelling/typecov/style-specific package findings.
  - This is mostly an internal code-ownership cleanup and test assertion.

### Move Out

If any package-family code still checks these packages, move or delete it:

- `prettier` -> `ts/fmt`
- `cspell` -> `ts/spelling`
- `type-coverage` -> `ts/typecov`
- `size-limit` -> later `ts/size`
- Stylelint packages -> current `ts/style`, not `ts/package`

## Implementation Order

1. `ts/fmt`
2. `ts/spelling`
3. `ts/package` cleanup
4. `ts/typecov`

Reason:

- `fmt` and `spelling` are the same delegated-tool pattern and prove the new family shape.
- `package` cleanup is easiest after the tool families exist, because ownership boundaries are concrete.
- `typecov` is still delegated, but threshold semantics are more likely to surface app friction, so do it after the pattern is proven.

## Verification

For each family:

- `cargo test --manifest-path packages/ts/<family>/g3ts-<family>-types/Cargo.toml`
- `cargo test --manifest-path packages/ts/<family>/g3ts-<family>-ingestion/Cargo.toml`
- `cargo test --manifest-path packages/ts/<family>/g3ts-<family>-config-checks/Cargo.toml`
- `g3rs validate --path packages/ts/<family>/g3ts-<family>-types --inventory`
- `g3rs validate --path packages/ts/<family>/g3ts-<family>-ingestion --inventory`
- `g3rs validate --path packages/ts/<family>/g3ts-<family>-config-checks --inventory`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `g3rs validate --path apps/guardrail3-ts --inventory`
- install local G3TS after every CLI wiring change:
  - `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- run on real apps where available:
  - `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family <family> --inventory`

## Expected App Impact

New families should intentionally fail on apps that do not wire the delegated tool.

The errors must tell the agent exactly:

- which package to install
- which config file is missing
- which script name is required
- which command must appear in the script
- which Syncpack pin is missing
- which validation route is not fail-closed
