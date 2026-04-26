# Goal

Fix G3TS bugs found by the landing agent while configuring the published Astro packages.

# Bugs

- `TS-ASTRO-CONFIG-02` can report missing `astro check` when a different package script contains unsupported guardrail-related shell syntax.
- Astro Syncpack policy appears order-sensitive for the required pin block, forcing apps to order canonical dependency groups exactly instead of validating set membership.
- `TS-ASTRO-CONFIG-17` requires `astro-seo@1.1.0`, but that package publishes `exports: "./index.ts"`, which is brittle for non-Astro tooling.

# Approach

1. Add failing tests first.
   - Package script parser: prove one safe `astro check` fact plus one unrelated unsupported `astro dev` fact should still satisfy `has_safe_tool_invocation(..., "astro", "check")`.
   - Astro ingestion: prove Syncpack required pins pass when canonical groups are present in a different order.

2. Fix at the architectural boundary.
   - Script parser owns safe invocation semantics. Change `has_safe_tool_invocation` so unsupported or parse-error facts only block when they contain or appear to contain the requested tool invocation. Do not let unrelated scripts poison the target tool query.
   - Astro ingestion owns Astro Syncpack contract matching from parsed Syncpack facts. Change the required-pin and forbidden-dep checks to validate exact canonical groups by set membership inside the Astro policy prefix, not by positional order.

3. Keep delegated inline-copy enforcement.
   - Do not remove `eslint-plugin-i18next`.
   - The current plan delegates literal public-copy policing to `eslint-plugin-i18next`; G3TS should enforce package presence, Syncpack pinning, and effective ESLint config.

4. Remove the brittle `astro-seo` requirement.
   - Keep `schema-dts` as the typed JSON-LD package.
   - Keep rendered SEO validation delegated to `@nuasite/checks`.
   - Do not require a specific SEO component package until the selected package has a tooling-safe compiled export surface.

# Files To Modify

- `packages/parsers/package-script-command-parser/crates/runtime/src/parser.rs`
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_17_seo_packages.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/*`
