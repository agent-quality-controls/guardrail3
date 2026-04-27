# Goal

Replace the temporary Astro runner-only split with real package structure.

# Target Structure

Create actual Rust crates under `packages/ts/astro`:

- `setup/g3ts-astro-setup-config-checks`
- `setup/g3ts-astro-setup-file-tree-checks`
- `content/g3ts-astro-content-config-checks`
- `content/g3ts-astro-content-file-tree-checks`
- `mdx/g3ts-astro-mdx-config-checks`
- `seo/g3ts-astro-seo-config-checks`
- `state/g3ts-astro-state-file-tree-checks`
- `g3ts-astro-check-support`
- `g3ts-astro-checks`

These are not "slices" inside a flat crate. They are separate package folders with their own `Cargo.toml`, runtime crate, facade crate, and tests.

# Approach

- Move child-owned rule modules from the flat Astro config/file-tree crates into the new child package runtime crates.
- Extract shared helper functions into `g3ts-astro-check-support` instead of copying a broad `support.rs` into every package.
- Keep the old flat `g3ts-astro-config-checks` and `g3ts-astro-file-tree-checks` only as legacy aggregate packages if needed by old tests; stop using them from the main G3TS app.
- Wire `apps/guardrail3-ts` to depend on `g3ts-astro-checks`, a package-level Astro facade.
- `g3ts-astro-checks` owns the fan-out to setup, content, MDX, SEO, and state packages. This keeps the app runner below the Rust dependency-count policy while preserving real package boundaries.
- Remove child-runner ID remapping from the flat packages as the main app no longer needs that bridge.
- Each new child package returns its own child-prefixed IDs directly.

# Rule Ownership

Astro setup config:

- `g3ts-astro-setup/astro-package-present`
- `g3ts-astro-setup/astro-check-present`
- `g3ts-astro-setup/astro-eslint-plugin-package-present`
- `g3ts-astro-setup/astro-eslint-plugin-wired`
- `TS-ASTRO-SETUP-CONFIG-06`
- `TS-ASTRO-SETUP-CONFIG-07`
- `g3ts-astro-setup/syncpack-stack-pins`
- `g3ts-astro-setup/syncpack-forbidden-deps`
- `g3ts-astro-setup/site-url`
- `g3ts-astro-setup/static-output`
- `g3ts-astro-setup/required-integrations`

Astro setup file-tree:

- `g3ts-astro-setup/astro-config-exists`
- `TS-ASTRO-SETUP-FILETREE-03`

Astro content config:

- `g3ts-astro-content/content-adapter-rule`
- `g3ts-astro-content/inline-copy-rule`
- `g3ts-astro-content/strict-content-policy`
- `g3ts-astro-content/strict-policy-paths`
- `g3ts-astro-content/route-scope-overlap`
- `g3ts-astro-content/policy-eslint-coverage`
- `g3ts-astro-content/content-adapter-exists`
- `g3ts-astro-content/content-adapter-astro-content`

Astro content file-tree:

- `g3ts-astro-content/content-config-exists`
- `g3ts-astro-content/no-route-markdown-pages`
- `g3ts-astro-content/no-velite-config`
- `g3ts-astro-content/no-velite-output`

Astro MDX config:

- `g3ts-astro-mdx/mdx-lane`
- `g3ts-astro-mdx/strict-policy-paths`
- `g3ts-astro-mdx/policy-helper-surfaces`
- `g3ts-astro-mdx/mdx-component-map-rule`

Astro SEO config:

- `g3ts-astro-seo/nuasite-checks`
- `g3ts-astro-seo/sitemap-integration`
- `g3ts-astro-seo/robots-integration`
- `g3ts-astro-seo/llms-txt`
- `g3ts-astro-seo/seo-packages`
- `g3ts-astro-seo/structured-data-check`
- `g3ts-astro-seo/strict-policy-paths`
- `g3ts-astro-seo/policy-helper-surfaces`
- `g3ts-astro-seo/metadata-helper-rule`
- `g3ts-astro-seo/json-ld-helper-rule`

Astro state file-tree:

- `g3ts-astro-state/no-legacy-parallel-state`
- `g3ts-astro-state/configured-forbidden-state`

# Files To Modify

- New package folders under `packages/ts/astro/{setup,content,mdx,seo,state}`
- `apps/guardrail3-ts/Cargo.toml`
- `apps/guardrail3-ts/crates/logic/family-runner-structure/Cargo.toml`
- `apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs`
- CLI tests that assert Astro child output order

# Verification

- `cargo test --workspace` in each new package
- `cargo test --workspace` in `apps/guardrail3-ts`
- `g3rs validate --path` for each new package and `apps/guardrail3-ts`
- local `cargo install` of G3TS
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- adversarial review of package structure and app wiring
