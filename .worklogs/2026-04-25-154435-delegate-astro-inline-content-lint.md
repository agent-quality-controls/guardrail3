# Summary

Delegated Astro public-copy enforcement to `eslint-plugin-i18next/no-literal-string` through `eslint-plugin-astro-pipeline` instead of adding a custom source AST scanner. G3TS now enforces the delegated rule as an effective Astro content-pipeline requirement and rejects unsafe option shapes that would hide public copy.

# Decisions Made

- `eslint-plugin-astro-pipeline` owns the exact `eslint-plugin-i18next` dependency and exposes `configs["strict-content"]`; Astro apps should install the Astro pipeline plugin, not `eslint-plugin-i18next` directly.
- G3TS enforces `eslint-plugin-astro-pipeline` `0.1.4` through Syncpack and bans direct app deps on `eslint-plugin-i18next`, `eslint-mdx`, `velite`, and `next`.
- G3TS still does not scan source for copy. It reads effective ESLint config facts and marks `i18next/no-literal-string` effective only when the rule is `error`, scoped to present Astro/TS/TSX lanes, and configured without broad `words`, `jsx-components`, `callees`, `object-properties`, or `jsx-attributes` allowlists that would hide authored copy.
- Unsupported i18next regex-like pattern shapes fail closed in G3TS, except the exact known i18next punctuation token pattern needed for structural strings.
- Removed the abandoned custom `astro-pipeline/no-inline-public-content` plan and kept the delegated implementation plan as the committed design source.

# Key Files For Context

- `.plans/2026-04-25-144144-delegate-inline-content-lint.md`
- `packages/ts/eslint-plugin-astro-pipeline/src/configs/recommended.ts`
- `packages/ts/eslint-plugin-astro-pipeline/tests/strict-content-config.test.ts`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`

# Verification

- `npm test` in `packages/ts/eslint-plugin-astro-pipeline`
- `npm pack --dry-run` in `packages/ts/eslint-plugin-astro-pipeline`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `git diff --check`
- Adversarial review passes after iterative fixes; final narrow pass returned `PASS`.

# Next Steps

- Publish `eslint-plugin-astro-pipeline@0.1.4` when ready.
- Run G3TS against the landing app after updating its ESLint config to use `configs["strict-content"]`.
