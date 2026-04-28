# Goal

Finish the remaining enforceable Astro plan slice after crawler artifacts:

- enforce visible ESLint suppression handling for Astro delegated rules
- enforce one standard app-level `validate` script that runs all required validators fail-closed

# Approach

## ESLint Suppression Wiring

- Extend Astro setup ESLint facts to carry rule severities and rule options for the Astro, TS, and TSX source probes.
- Keep parsing in `eslint-config-parser`; Astro ingestion only copies typed parser facts into family snapshots.
- Add setup checks:
  - `g3ts-astro-setup/eslint-comments-plugin-package-present`
  - `g3ts-astro-setup/eslint-disable-descriptions-required`
  - `g3ts-astro-setup/unused-eslint-disables-fail`
- Add protected-disable checks in the owning Astro subfamilies:
  - content checks protected content source lanes
  - mdx checks MDX content lane and component-map lane
  - seo checks SEO source lanes
- Require `@eslint-community/eslint-comments/no-restricted-disable` at warn or error with options covering the protected delegated rule names for each family.
- Do not parse raw ESLint source inside check rules. Checks consume typed facts only.
- Inventory ESLint disable directives through the shared `eslint-directive-parser` package in ingestion, not by ad hoc text scanning in family checks.
- If directive inventory parsing is unsupported, ambiguous, unreadable, or parser-error for an included source file, fail closed instead of reporting "no directives".

## Validate Script

- Add setup check `g3ts-astro-setup/validate-script`.
- Use existing parsed package-script command facts only.
- Require `package.json` script `validate`.
- Require the transitive command graph from `validate` to invoke:
  - `eslint`
  - `syncpack lint`
  - `astro check`
  - `astro build`
- Allow package-manager script calls only when the parser normalized them into child script tool invocations.
- Reject parser blockers and `||` fail-open separators on the reachable script graph.
- Reject unsafe validation-like sibling scripts: `check`, `verify`, `ci`, `precommit`, `lint:all`.

# Files To Modify

- `packages/ts/astro/setup/g3ts-astro-setup-types/src/types.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/eslint.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/*`
- `packages/ts/astro/content/g3ts-astro-content-types/src/types.rs`
- `packages/ts/astro/content/g3ts-astro-content-ingestion/src/eslint.rs`
- `packages/ts/astro/content/g3ts-astro-content-config-checks/crates/runtime/src/*`
- `packages/ts/astro/mdx/g3ts-astro-mdx-types/src/types.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/eslint.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/*`
- `packages/ts/astro/seo/g3ts-astro-seo-types/src/types.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion/src/eslint.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/*`

# Verification

- cargo tests for touched Astro setup/content/mdx/seo packages
- cargo test for `apps/guardrail3-ts`
- install local G3TS CLI after changes
- run G3RS on touched packages
- run G3TS against landing
- adversarial review against `.plans/2026-04-28-105113-astro-content-style-next-rules.md`
