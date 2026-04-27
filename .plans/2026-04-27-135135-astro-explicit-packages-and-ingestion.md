# Goal

Make the Astro split match the Rust family architecture.

The active runner must explicitly compose ingestion and check packages. It must not hide Astro behind an aggregate `g3ts-astro-checks` package.

# Architecture

Keep these real check packages:

- `packages/ts/astro/setup/g3ts-astro-setup-config-checks`
- `packages/ts/astro/setup/g3ts-astro-setup-file-tree-checks`
- `packages/ts/astro/content/g3ts-astro-content-config-checks`
- `packages/ts/astro/content/g3ts-astro-content-file-tree-checks`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks`
- `packages/ts/astro/state/g3ts-astro-state-file-tree-checks`

Add real ingestion packages:

- `packages/ts/astro/setup/g3ts-astro-setup-ingestion`
- `packages/ts/astro/content/g3ts-astro-content-ingestion`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion`
- `packages/ts/astro/state/g3ts-astro-state-ingestion`

Each ingestion package owns only the typed input creation for its matching check package or packages.

# Remove From Active Runner

- Stop using `g3ts-astro-checks` from `apps/guardrail3-ts`.
- Keep the package only if needed temporarily by old code, but it must not be in the app runner dependency path.

# Runner Shape

`apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs` must explicitly do:

- setup ingestion then setup config/file-tree checks
- content ingestion then content config/file-tree checks
- MDX ingestion then MDX config checks
- SEO ingestion then SEO config checks
- state ingestion then state file-tree checks

This mirrors Rust runner architecture:

- runner calls ingestion packages
- runner passes typed inputs to check packages
- check packages do not discover or parse
- aggregate facade packages do not hide dependency edges

# Dependency Count

If `family-runner-structure` exceeds the current dependency-count policy, add a local waiver in `apps/guardrail3-ts/guardrail3-rs.toml`.

Reason: this runner explicitly aggregates multiple independent structure families and subfamilies. Hiding those dependencies behind a facade is worse architecture than a justified local waiver.

# Verification

- `cargo test --workspace` in every new ingestion package
- `cargo test --workspace` in every Astro check package
- `cargo test --workspace` in `apps/guardrail3-ts`
- `g3rs validate --path` for every new ingestion package, every Astro check package, and `apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- grep confirms active app runner no longer references `g3ts-astro-checks`
