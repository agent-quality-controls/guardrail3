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

- `TS-ASTRO-SETUP-CONFIG-01`
- `TS-ASTRO-SETUP-CONFIG-02`
- `TS-ASTRO-SETUP-CONFIG-03`
- `TS-ASTRO-SETUP-CONFIG-05`
- `TS-ASTRO-SETUP-CONFIG-06`
- `TS-ASTRO-SETUP-CONFIG-07`
- `TS-ASTRO-SETUP-CONFIG-09`
- `TS-ASTRO-SETUP-CONFIG-10`
- `TS-ASTRO-SETUP-CONFIG-11`
- `TS-ASTRO-SETUP-CONFIG-12`
- `TS-ASTRO-SETUP-CONFIG-21`

Astro setup file-tree:

- `TS-ASTRO-SETUP-FILETREE-01`
- `TS-ASTRO-SETUP-FILETREE-03`

Astro content config:

- `TS-ASTRO-CONTENT-CONFIG-18`
- `TS-ASTRO-CONTENT-CONFIG-19`
- `TS-ASTRO-CONTENT-CONFIG-23`
- `TS-ASTRO-CONTENT-CONFIG-24`
- `TS-ASTRO-CONTENT-CONFIG-25`
- `TS-ASTRO-CONTENT-CONFIG-26`
- `TS-ASTRO-CONTENT-CONFIG-27`
- `TS-ASTRO-CONTENT-CONFIG-28`

Astro content file-tree:

- `TS-ASTRO-CONTENT-FILETREE-02`
- `TS-ASTRO-CONTENT-FILETREE-04`
- `TS-ASTRO-CONTENT-FILETREE-05`
- `TS-ASTRO-CONTENT-FILETREE-06`

Astro MDX config:

- `TS-ASTRO-MDX-CONFIG-20`
- `TS-ASTRO-MDX-CONFIG-24`
- `TS-ASTRO-MDX-CONFIG-29`
- `TS-ASTRO-MDX-CONFIG-30`

Astro SEO config:

- `TS-ASTRO-SEO-CONFIG-13`
- `TS-ASTRO-SEO-CONFIG-14`
- `TS-ASTRO-SEO-CONFIG-15`
- `TS-ASTRO-SEO-CONFIG-16`
- `TS-ASTRO-SEO-CONFIG-17`
- `TS-ASTRO-SEO-CONFIG-22`
- `TS-ASTRO-SEO-CONFIG-24`
- `TS-ASTRO-SEO-CONFIG-29`
- `TS-ASTRO-SEO-CONFIG-31`
- `TS-ASTRO-SEO-CONFIG-32`

Astro state file-tree:

- `TS-ASTRO-STATE-FILETREE-11`
- `TS-ASTRO-STATE-FILETREE-12`

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
