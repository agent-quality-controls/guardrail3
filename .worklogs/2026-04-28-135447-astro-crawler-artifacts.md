# Summary

Added narrow Astro crawler artifact generation and validation packages, then wired G3TS to enforce their presence and fail-closed validate scripts. The implementation keeps sitemap, robots, and llms output parsing in npm packages while G3TS checks Astro config, package pins, integrations, and script wiring.

# Decisions

- Used `@astrojs/sitemap` and `astro-robots` as the standard generators because they own narrow Astro build output generation.
- Added `g3ts-astro-llms` as a narrow Astro integration because no existing focused llms.txt Astro generator fit the contract.
- Added split post-build checker packages for sitemap, robots, and llms instead of a combined output-check package, so each artifact has one parser and one responsibility.
- G3TS checks only wiring: package presence, Astro integration presence, static output, canonical HTTPS site, `trailingSlash: "always"`, Syncpack pins, and validate script ordering.
- Kept package-script parsing generic. The parser normalizes package manager run invocations without hardcoding Astro checker executable names.

# Key Files

- `.plans/2026-04-28-105113-astro-content-style-next-rules.md`
- `packages/ts/astro/integrations/g3ts-astro-llms`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks`
- `packages/ts/astro/robots/g3ts-astro-robots-checks`
- `packages/ts/astro/llms/g3ts-astro-llms-checks`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion`
- `packages/ts/astro/seo/g3ts-astro-seo-types`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`
- `packages/parsers/astro-config-parser`
- `packages/parsers/guardrail3-rs-toml-parser`
- `packages/parsers/package-script-command-parser`

# Verification

- `npm test` in `g3ts-astro-llms`
- `npm test` in `g3ts-astro-sitemap-checks`
- `npm test` in `g3ts-astro-robots-checks`
- `npm test` in `g3ts-astro-llms-checks`
- `cargo test --manifest-path packages/parsers/astro-config-parser/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/parsers/guardrail3-rs-toml-parser/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/ts/astro/seo/g3ts-astro-seo-config-checks/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-ingestion/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `g3rs validate` on touched parser, Astro SEO/setup, and G3TS CLI workspaces
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

# Published

- `g3ts-astro-llms@0.1.1`
- `g3ts-astro-llms-checks@0.1.1`
- `g3ts-astro-sitemap-checks@0.1.2`
- `g3ts-astro-robots-checks@0.1.2`

# Next Steps

- Implement the suppression visibility rules planned in section 1 of the Astro plan.
- Add the Astro app-level `validate` script contract from section 2.
- Build the separate TS style family instead of putting Tailwind/CSS policy into Astro.
