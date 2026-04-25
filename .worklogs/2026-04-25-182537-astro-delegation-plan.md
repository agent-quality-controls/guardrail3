# Summary

Documented the exact Astro delegation boundary for the next G3TS implementation pass. The plan names package versions, parser contracts, ESLint effective-config probes, Syncpack policy groups, Nuasite validator wiring, and the renamed G3TS-owned npm packages.

# Decisions Made

- G3TS-owned TypeScript npm packages use `g3ts-` names: `g3ts-eslint-plugin-astro-pipeline` and `g3ts-astro-nuasite-checks`.
- Package version and ban policy stays delegated to Syncpack; Astro checks read `syncpack-config-parser` facts instead of parsing package versions.
- Rendered HTML checks stay delegated to `@nuasite/checks@0.18.0`; G3TS enforces package/config/script setup and does not inspect built HTML.
- MDX linting is owned by `eslint-plugin-mdx`, not bare `eslint-mdx`.
- Style/Tailwind policy is explicitly outside the Astro family.

# Key Files For Context

- `.plans/2026-04-25-161058-astro-delegation-boundaries.md`
- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/parsers/astro-config-parser/crates/runtime/src/parser.rs`
- `packages/parsers/eslint-config-parser/crates/types/src/document.rs`

# Next Steps

- Rename and update the Astro pipeline ESLint package to `g3ts-eslint-plugin-astro-pipeline@0.1.5`.
- Add `packages/ts/g3ts-astro-nuasite-checks` with `structuredDataPresentCheck`.
- Extend Astro config parser, Astro ingestion, and Astro config checks to enforce the plan.
- Run mechanical verification and adversarial test attacks against the committed plan.
