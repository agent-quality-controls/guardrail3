Summary
- Split Astro from one runnable structure family into setup, content, MDX, SEO, and state families while keeping `--family astro` as a CLI alias that expands to those concrete families.
- Removed the hidden aggregate `g3ts-astro-ingestion`/support ingestion layer and replaced it with per-subfamily ingestion packages plus shared surface readers.
- Narrowed the Astro content/MDX/SEO approved-source contracts so content adapters, MDX component maps, and SEO helper surfaces are no longer carried in one combined approved-surface bag.

Decisions Made
- Kept `--family astro` as an alias, not a real `SupportedFamily`, because app agents need one command to validate all Astro subfamilies.
- Kept shared parser/surface readers in `g3ts-astro-check-support`; this package reads shared files once but no longer exports aggregate `collect_config_facts` or `collect_file_tree_facts`.
- Regenerated local Astro package lockfiles so package-local `cargo test --workspace --locked --no-run` is reproducible.

Key Files For Context
- `apps/guardrail3-ts/crates/types/app-types/src/supported_family.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-check-support/src/surfaces/mod.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/run.rs`
- `packages/ts/astro/content/g3ts-astro-content-ingestion/src/run.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/run.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion/src/run.rs`
- `packages/ts/astro/state/g3ts-astro-state-ingestion/src/run.rs`

Next Steps
- Finish shrinking the remaining mixed content/MDX/SEO rules that still take full family input because they compare integration and ESLint contracts together.
- Split file-tree rule entrypoints so rules receive the exact root list or route list they check, not the whole file-tree package input.
- Re-run adversarial review after those last rule-entrypoint reductions.
