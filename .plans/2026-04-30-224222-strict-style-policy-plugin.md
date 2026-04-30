# Strict style policy plugin

## Goal

Replace the weak third-party Tailwind denylist rule with a G3TS-owned ESLint plugin that catches denied static class tokens across Astro, React, and common class-composition helpers. G3TS must still only enforce delegated plugin wiring, not parse source files itself.

## Approach

- Add npm package `packages/ts/g3ts-eslint-plugin-style-policy`.
- Export namespace `style-policy`.
- Implement rule `style-policy/no-denied-class-tokens`.
- The ESLint rule consumes the AST provided by ESLint parsers and extracts only static class-token strings.
- Update G3TS style ingestion/config checks to require `g3ts-eslint-plugin-style-policy` and `style-policy/no-denied-class-tokens`.
- Stop requiring `eslint-plugin-tailwind-ban`.
- Keep denylist ownership in ESLint config. G3TS only verifies the effective ESLint rule has a non-empty `denyList`.

## Rule coverage

The rule must report denied tokens in:

- JSX `className="..."`
- Astro/JSX `class="..."`
- Astro `class:list={...}`
- expression attributes such as `className={"text-black"}`
- static template literals with no expressions
- conditional expressions with static branches
- logical expressions with static string operands
- arrays and objects used in class expressions
- configured helper calls such as `cn(...)`, `clsx(...)`, `twMerge(...)`

The rule must not pretend to understand fully dynamic classes. Dynamic strings are ignored unless they contain a visible static denied token.

## G3TS changes

- Required style packages:
  - keep `stylelint`
  - keep `stylelint-config-standard`
  - keep `stylelint-config-tailwindcss`
  - keep `@double-great/stylelint-a11y`
  - replace `eslint-plugin-tailwind-ban` with `g3ts-eslint-plugin-style-policy`
- Effective ESLint rule:
  - namespace `style-policy`
  - package identity `g3ts-eslint-plugin-style-policy`
  - rule `style-policy/no-denied-class-tokens`
  - severity `error`
  - option `denyList` has at least one non-empty string

## Files to modify

- `packages/ts/g3ts-eslint-plugin-style-policy/**`
- `packages/ts/style/g3ts-style-types/src/lib.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/roots.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- style tests under `packages/ts/style/**`

## Verification

- npm test/build for `g3ts-eslint-plugin-style-policy`
- cargo tests for `g3ts-style-ingestion`
- cargo tests for `g3ts-style-config-checks`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path` on changed Rust workspaces
- install local G3TS CLI
