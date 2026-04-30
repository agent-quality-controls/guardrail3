# ESLint ESM plugin package resolution

## Goal

G3TS must recognize ESLint plugin package identity when the plugin package is ESM-only and exposes its entrypoint through the package `exports.import` condition. The style rule `g3ts-style/tailwind-ban-eslint-rule` must not fail when ESLint proves `eslint-plugin-tailwind-ban` is active and effective.

## Approach

- Add a parser regression test in `packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs`.
- The test creates a fake `eslint-plugin-tailwind-ban` package with `"type": "module"` and `"exports": { ".": { "import": "./index.js" } }`.
- The fake ESLint effective config returns the actual imported plugin object under namespace `tailwind-ban`.
- The parser must record `plugin_package_names["tailwind-ban"] = ["eslint-plugin-tailwind-ban"]`.
- Fix `packages/parsers/eslint-config-parser/crates/runtime/src/parser.rs` in the shared Node helper.
- Keep package identity proof strict: do not trust namespace, metadata, or matching shape alone. A package name is accepted only when importing the resolved package yields the same effective plugin object exported by the package.

## Key decisions

- Fix the shared ESLint parser, not the style check. Style should consume parser facts only.
- Keep CommonJS resolution support because existing tests cover CommonJS plugins and nested config-local packages.
- Add ESM-aware package import resolution only for the package identity proof path.
- Do not add a Tailwind-specific branch.

## Files to modify

- `packages/parsers/eslint-config-parser/crates/runtime/src/parser.rs`
- `packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs`
- `.worklogs/*-eslint-esm-plugin-resolution.md`
