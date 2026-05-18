# G3TS Tool Family Hardening

## Goal

Finish the narrow tool-wiring contract for the three simplest G3TS tool families:

- `g3ts-fmt`
- `g3ts-spelling`
- `g3ts-typecov`

These families must not reimplement Prettier, CSpell, or type-coverage.
They prove that each tool is installed, configured, wired through a standard script, and reachable from `validate` and hooks.

## Shared Rules

- The app/package root is adopted only when it has `package.json` and `guardrail3-ts.toml`.
- Each family owns its own package, config, script, and hook contract.
- Each family delegates the actual check to the external tool.
- G3TS must reject hidden no-op scripts, any `||` fallback in the checked script path, unparseable package scripts, and bare tool invocations that cannot run a meaningful check.
- G3TS must not require optional autofix scripts as validation gates.
- The generated G3TS pre-commit hook must exclude `behavior/fixtures/**` from package-lock integrity work and workspace validation routing. Fixture `package.json` and `guardrail3-ts.toml` files are test data, not repo install roots or live workspaces. They must still pass generic staged-file gates such as merge-conflict marker checks, secret scanning, and file-size checks.
- The generated G3TS pre-commit hook and `--staged` filter must trigger on every config file named by the family hook contracts, not only source files and `package.json`.
- `package-json-parser` owns dependency declaration extraction and dependency specifier classification.
- `syncpack-config-parser` owns Syncpack selector matching, including first matching version-group behavior and `packages`, `dependencies`, `dependencyTypes`, and `specifierTypes` selectors.
- FMT, spelling, and typecov families must consume those shared parser facts instead of duplicating selector or specifier parsing.

## FMT

### Required contract

- `prettier` must be installed directly.
- A Prettier config must exist in the app/package root.
- `format:check` must exist.
- `format:check` must run `prettier --check ...` fail-closed, with at least one explicit checked target.
- `validate` must reach `format:check` or a direct `prettier --check ...` invocation fail-closed.
- `validate` may reach `format:check` through `pnpm run format:check`, `pnpm format:check`, `yarn run format:check`, `yarn format:check`, `bun run format:check`, or `bun format:check`.
- Bare `validate = "format:check"` must not count as a package-script invocation.
- Syncpack must pin `prettier`.
- The Syncpack pin must apply to the current package name, dependency specifier scope, and actual dependency lane.
- Hook contract must still include the Prettier gate.
- Hook triggers must include root and nested `package.json`, `guardrail3-ts.toml`, `.syncpackrc`, and Prettier config files.
- The generated repo hook and CLI `--staged` file filter must include `.syncpackrc`, `prettier.config.*`, and `.prettierrc*` so those changes route to the owning workspace.

### Explicit non-contract

- `format` / `prettier --write` is allowed but not required.
- G3TS must not fail a project only because it lacks an autofix script.

### Implementation notes

- Update `g3ts-fmt/format-scripts` or replace it with a single-purpose `g3ts-fmt/format-check-script`.
- Existing fixtures must include a passing root with no `format` script.
- Broken fixtures must prove missing `format:check`, hidden `||` fallbacks, and targetless `prettier --check` still fail.
- Broken fixtures must prove target detection does not treat a value passed to a Prettier option as a checked target.
- Clean fixtures must prove boolean Prettier options such as `--cache` do not consume the following checked target.
- Broken fixtures must prove Syncpack pins only count when their `packages`, `specifierTypes`, and `dependencyTypes` scope applies to the current package and dependency lane.
- G3TS only checks that a Prettier config file exists. It does not parse executable Prettier config files; bad config content is delegated to `prettier --check`.

## Spelling

### Required contract

- `cspell` must be installed directly.
- A CSpell config must exist and parse when the config format has a parser.
- `spellcheck` must exist.
- `spellcheck` must run `cspell` fail-closed.
- `spellcheck` must pass at least one explicit target such as `.`, a file, or a glob.
- `validate` must reach `spellcheck` or a direct target-bearing `cspell` invocation fail-closed.
- `validate` may reach `spellcheck` through `pnpm run spellcheck`, `pnpm spellcheck`, `yarn run spellcheck`, `yarn spellcheck`, `bun run spellcheck`, or `bun spellcheck`.
- Bare `validate = "spellcheck"` must not count as a package-script invocation.
- Syncpack must pin `cspell`.
- The Syncpack pin must apply to the current package name, dependency specifier scope, and actual dependency lane.
- Hook contract must still include the CSpell gate.
- Hook triggers must include root and nested `package.json`, `guardrail3-ts.toml`, `.syncpackrc`, and CSpell config files.
- The generated repo hook and CLI `--staged` file filter must include `.syncpackrc`, `cspell.json`, `.cspell.json`, `cspell.config.*`, `cspell.yaml`, and `cspell.yml` so those changes route to the owning workspace.

### Explicit non-contract

- G3TS does not judge the spelling dictionary contents.
- G3TS does not scan source text itself.

### Implementation notes

- Current target detection already rejects bare `cspell`; preserve and fixture it.
- Add or update a broken fixture where `spellcheck = "cspell"` fails even when package/config exist.
- Broken fixtures must prove invalid JSON CSpell config, hidden `||` fallbacks, and targetless direct `validate = "cspell"` fail.
- Broken fixtures must prove target detection does not treat a value passed to a CSpell option as a checked target.
- Broken fixtures must prove Syncpack pins only count when their `packages`, `specifierTypes`, and `dependencyTypes` scope applies to the current package and dependency lane.

## Type Coverage

### Required contract

- `type-coverage` must be installed directly.
- `guardrail3-ts.toml` must define `[typecov] minimum = <integer>`.
- `minimum` must be an integer from `0` to `100`.
- `typecov` script must exist.
- `typecov` must run `type-coverage --strict --at-least <n>` fail-closed.
- `<n>` must be at least `[typecov].minimum`.
- `validate` must reach `typecov` or a direct `type-coverage --strict --at-least <n>` invocation fail-closed.
- `validate` may reach `typecov` through `pnpm run typecov`, `pnpm typecov`, `yarn run typecov`, `yarn typecov`, `bun run typecov`, or `bun typecov`.
- Bare `validate = "typecov"` must not count as a package-script invocation.
- Syncpack must pin `type-coverage`.
- The Syncpack pin must apply to the current package name, dependency specifier scope, and actual dependency lane.
- Hook contract must still include the type-coverage gate.
- Hook triggers must include root and nested `package.json`, `guardrail3-ts.toml`, `.syncpackrc`, and `tsconfig*.json`.
- The generated repo hook and CLI `--staged` file filter must include `.syncpackrc` and `tsconfig*.json` so those changes route to the owning workspace.

### Explicit non-contract

- G3TS does not calculate type coverage itself.
- G3TS does not require a separate type-coverage config file.
- G3TS does not hardcode `100`; the threshold is a project policy in `guardrail3-ts.toml`.

### Implementation notes

- Extend `g3ts-toml-parser` with a typed `[typecov]` table.
- Extend `g3ts-typecov-ingestion` to ingest that policy.
- Extend `g3ts-typecov-types` with policy surface state and threshold snapshot.
- Update typecov rules to compare script threshold against policy threshold.
- Fixtures must cover:
  - valid minimum `100`
  - valid non-100 minimum
  - missing `[typecov]`
  - invalid minimum outside `0..=100`
  - script missing `--strict`
  - script threshold below configured minimum
  - script threshold above 100
  - duplicate `--at-least` where any effective threshold is below policy
  - duplicate `--at-least` where any threshold value is invalid
  - attached `--at-least=<n>` threshold values
  - validate not reaching typecov
  - `validate = "typecov"` does not count as running the package script
  - Syncpack pins only count when their `packages`, `specifierTypes`, and `dependencyTypes` scope applies to the current package and dependency lane

## Worker Split

### G3TS-FMT agent

Owned paths:

- `packages/ts/fmt/**`
- `packages/parsers/package-json-parser/**`
- `packages/parsers/syncpack-config-parser/**`
- `behavior/fixtures/g3ts-rule/fmt/**`

Must not edit:

- `behavior/golden/**`
- other family packages

### G3TS-SPELLING agent

Owned paths:

- `packages/ts/spelling/**`
- `packages/parsers/package-json-parser/**`
- `packages/parsers/syncpack-config-parser/**`
- `behavior/fixtures/g3ts-rule/spelling/**`

Must not edit:

- `behavior/golden/**`
- other family packages

### G3TS-TYPECOV agent

Owned paths:

- `packages/ts/typecov/**`
- `packages/parsers/g3ts-toml-parser/**`
- `packages/parsers/package-json-parser/**`
- `packages/parsers/syncpack-config-parser/**`
- `behavior/fixtures/g3ts-rule/typecov/**`

Must not edit:

- `behavior/golden/**`
- other family packages except the parser listed above

## Verification

After workers finish, the integrator must run:

```bash
python3 scripts/verify-g3ts-tool-family-hardening.py
fixture3 check --suite g3ts-rule
cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml
cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-ts/Cargo.toml -- -D warnings
g3ts validate repo --path .
g3rs validate repo --path .
```

The integrator owns golden approval after reviewing fixture diffs.
