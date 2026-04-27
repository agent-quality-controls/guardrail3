Goal

Move Astro ESLint semantic/effective surface computation out of `g3ts-astro-check-support` and into setup/content/MDX/SEO ingestion packages.

Approach

- Keep shared support limited to raw ESLint config selection, parsing, and small typed probe readers.
- Move effective rule helpers into package-local ingestion code.
- Add setup/content/MDX/SEO local ESLint ingestion modules and route each `run.rs` through local functions.
- Remove `eslint_effective.rs` and stop exporting concrete `ingest_*_eslint_surface` from shared support.
- Update dependencies only where local ingestion now directly uses parser/runtime crates.

Key decisions

- Preserve existing public snapshot types because check crates already consume them.
- Avoid touching existing check rule behavior except through moved ingestion ownership.
- Do not revert unrelated worktree edits.

Files to modify

- `packages/ts/astro/g3ts-astro-check-support/src/surfaces/mod.rs`
- `packages/ts/astro/g3ts-astro-check-support/src/surfaces/eslint.rs`
- `packages/ts/astro/g3ts-astro-check-support/src/surfaces/eslint_effective.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/*`
- `packages/ts/astro/content/g3ts-astro-content-ingestion/src/*`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/*`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion/src/*`
- affected `Cargo.toml` files if direct parser deps are needed
