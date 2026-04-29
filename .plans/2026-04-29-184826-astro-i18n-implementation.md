# Goal

Implement the first Astro i18n guardrail layer from `.plans/2026-04-29-175900-astro-i18n-policy-guardrails.md`.

The end state:

- `g3ts-eslint-plugin-astro-i18n-policy` exists, tests pass, and is publishable.
- `astro-i18n` exists as a separate G3TS Astro child family with types, ingestion, config checks, and hook contract.
- G3TS enforces i18n tool/package/config wiring instead of parsing app source itself.
- Local `g3ts` includes the new family and reports missing i18n wiring on Astro apps.

# Approach

1. Add the ESLint plugin package under `packages/ts/g3ts-eslint-plugin-astro-i18n-policy`.
   - Implement `no-unlocalized-internal-hrefs`.
   - Implement `no-inline-image-alt`.
   - Implement `require-content-image-key`.
   - Use `@typescript-eslint/utils`, `@typescript-eslint/parser`, `astro-eslint-parser`, `eslint`, and `minimatch`.
   - Do not add a CLI.

2. Add Astro i18n family packages under `packages/ts/astro/i18n`.
   - `g3ts-astro-i18n-types`
   - `g3ts-astro-i18n-ingestion`
   - `g3ts-astro-i18n-config-checks`
   - `g3ts-astro-i18n-hook-contract`

3. Extend `guardrail3-rs-toml-parser` only if the existing typed model does not expose `[ts.astro.i18n]`.
   - Parser owns TOML shape.
   - Family ingestion consumes parser facts.
   - Checks do not parse TOML.

4. Extend `eslint-config-parser` only if it cannot expose the effective rule options needed for:
   - `astro-i18n-policy/*`
   - `i18next/no-literal-string`
   - `no-restricted-syntax`
   - `@eslint-community/eslint-comments/no-restricted-disable`

5. Wire the family into G3TS.
   - Add `SupportedFamily::AstroI18n`.
   - CLI name: `astro-i18n`.
   - Include it in `--family astro` expansion if that expansion exists; otherwise include it in default all-family runs and allow direct `--family astro-i18n`.
   - Add it to the structure family runner.
   - Add its hook contract to hooks aggregation.

6. Add Syncpack required pin:
   - `g3ts-eslint-plugin-astro-i18n-policy`

7. Verify:
   - ESLint plugin tests.
   - Cargo tests for parser/family/app crates touched.
   - `cargo install` local `g3ts`.
   - `g3rs validate --path apps/guardrail3-ts --inventory`.
   - `g3ts validate` against the landing app worktree.

# Key Decisions

- Source AST checks go into ESLint, not Rust G3TS.
- G3TS checks only packages, policy config, effective ESLint config, and hook contract.
- No hidden defaults in custom ESLint rules. Missing required options reports a config error from the rule.
- No route inference in the plugin. The app must provide locales, content route prefixes, allowed unprefixed routes, and approved helpers/components.
- No rendered-output i18n auditing in this pass.

# Files To Modify

- `packages/ts/g3ts-eslint-plugin-astro-i18n-policy/**`
- `packages/ts/astro/i18n/g3ts-astro-i18n-types/**`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/**`
- `packages/ts/astro/i18n/g3ts-astro-i18n-config-checks/**`
- `packages/ts/astro/i18n/g3ts-astro-i18n-hook-contract/**`
- `packages/parsers/guardrail3-rs-toml-parser/**`
- `packages/parsers/eslint-config-parser/**`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`
- `apps/guardrail3-ts/**`
- `.worklogs/<timestamp>-astro-i18n-implementation.md`
