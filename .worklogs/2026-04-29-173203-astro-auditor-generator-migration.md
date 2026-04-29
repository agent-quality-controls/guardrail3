## Summary

Migrated Astro sitemap, robots, and llms artifact validation out of app-facing checker CLIs and into Astro build lifecycle integrations. G3TS now enforces generator/auditor packages, Astro integration wiring, strict static config, Syncpack pins/bans, and removed checker CLI script targets instead of running artifact checker CLIs.

## Decisions made

- Replaced `g3ts-astro-sitemap-checks`, `g3ts-astro-robots-checks`, and `g3ts-astro-llms-checks` with `g3ts-astro-sitemap-auditor`, `g3ts-astro-robots-auditor`, and `g3ts-astro-llms-auditor`.
- Renamed the old `g3ts-astro-llms` integration package to `g3ts-astro-llms-generator`.
- Kept rendered HTML validation delegated to `@nuasite/checks`; G3TS only enforces Nuasite package/config wiring.
- Made auditor integrations validate config at construction so JS callers cannot bypass required keys or hide unknown keys.
- Kept imported config objects unsupported for this migration because the Astro config parser does not resolve imported object literals.
- Rejected old checker CLI script targets using all parsed script tool invocations, including nested shell commands.

## Key files for context

- `.plans/2026-04-29-122938-astro-auditor-generator-architecture.md`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-auditor/src/index.ts`
- `packages/ts/astro/robots/g3ts-astro-robots-auditor/src/index.ts`
- `packages/ts/astro/llms/g3ts-astro-llms-auditor/src/index.ts`
- `packages/ts/astro/integrations/g3ts-astro-llms-generator/src/index.ts`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/forbidden_script_targets.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`

## Verification

- `npm test` in `g3ts-astro-sitemap-auditor`
- `npm test` in `g3ts-astro-robots-auditor`
- `npm test` in `g3ts-astro-llms-auditor`
- `npm test` in `g3ts-astro-llms-generator`
- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-config-checks/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-ingestion/Cargo.toml --offline --locked`
- `cargo test --manifest-path packages/ts/astro/seo/g3ts-astro-seo-config-checks/Cargo.toml --workspace --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `g3rs validate --path apps/guardrail3-ts`
- `g3rs validate --path packages/ts/astro/seo/g3ts-astro-seo-config-checks`
- `g3rs validate --path packages/ts/astro/setup/g3ts-astro-setup-config-checks`
- `g3rs validate --path packages/ts/astro/setup/g3ts-astro-setup-ingestion`
- `g3rs validate --path packages/parsers/package-script-command-parser`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- `git diff --check`

## Published packages

- `g3ts-astro-sitemap-auditor@0.1.5`
- `g3ts-astro-robots-auditor@0.1.4`
- `g3ts-astro-llms-auditor@0.1.5`
- `g3ts-astro-llms-generator@0.1.2`

## Next steps

- Update landing to remove old checker packages/scripts and wire the new generator/auditor integrations with inline static config.
- Keep future artifact semantic validation out of G3TS unless it can be delegated to Astro integrations, ESLint, Syncpack, Nuasite, or the future Rust auditor pipeline.
