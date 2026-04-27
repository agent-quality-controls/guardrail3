# Goal

Remove the global `g3ts-astro-ingestion` package from the active architecture and from the filesystem.

# Problem

The current tree still has:

- `packages/ts/astro/g3ts-astro-ingestion`
- `packages/ts/astro/g3ts-astro-config-checks`
- `packages/ts/astro/g3ts-astro-file-tree-checks`

That preserves the old fake Astro family packages next to the real setup/content/MDX/SEO/state packages. The app no longer calls the flat check packages, but the per-area ingestion packages still delegate to global `g3ts-astro-ingestion`, so the split is not real.

# Correct Structure

Keep:

- `packages/ts/astro/g3ts-astro-types`
- `packages/ts/astro/g3ts-astro-check-support`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks`
- `packages/ts/astro/setup/g3ts-astro-setup-file-tree-checks`
- `packages/ts/astro/content/g3ts-astro-content-ingestion`
- `packages/ts/astro/content/g3ts-astro-content-config-checks`
- `packages/ts/astro/content/g3ts-astro-content-file-tree-checks`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks`
- `packages/ts/astro/state/g3ts-astro-state-ingestion`
- `packages/ts/astro/state/g3ts-astro-state-file-tree-checks`

Delete:

- `packages/ts/astro/g3ts-astro-ingestion`
- `packages/ts/astro/g3ts-astro-config-checks`
- `packages/ts/astro/g3ts-astro-file-tree-checks`

# Implementation

- Move the current shared Astro ingestion implementation into a module inside each per-area ingestion package.
- For this cleanup pass, duplicate the private ingestion implementation where needed rather than keeping a global fake ingestion family. Duplication is ugly but honest package ownership is more important here than keeping a central aggregate package.
- Per-area ingestion packages expose only the inputs needed by their own checks.
- Remove all references to `g3ts-astro-ingestion`.
- Remove all references to the deleted flat config/file-tree packages.

# Verification

- grep finds no `g3ts-astro-ingestion`, `g3ts_astro_ingestion`, `g3ts-astro-config-checks`, `g3ts-astro-file-tree-checks`, `g3ts_astro_config_checks`, or `g3ts_astro_file_tree_checks`.
- `cargo test --workspace` passes in every Astro per-area ingestion package.
- `cargo test --workspace` passes in `apps/guardrail3-ts`.
- `g3rs validate --path` passes for every Astro package and `apps/guardrail3-ts`.
- local `g3ts` install passes.
- landing Astro validation passes.
