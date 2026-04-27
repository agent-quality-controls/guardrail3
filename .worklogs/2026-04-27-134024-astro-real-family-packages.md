Summary

Replaced the temporary Astro runner-only split with actual Rust package folders for Astro setup, content, MDX, SEO, and generated-state checks. The active G3TS app now calls `g3ts-astro-checks`, which composes those packages; it no longer calls the old flat `g3ts-astro-config-checks` and `g3ts-astro-file-tree-checks` packages.

Decisions made

- Created separate package folders for setup config, setup file-tree, content config, content file-tree, MDX config, SEO config, and state file-tree checks.
- Added `g3ts-astro-check-support` for shared pure helper functions used by multiple Astro packages, then split it into `core`, `eslint`, and `support_nuasite` modules so `lib.rs` remains facade-only.
- Added `g3ts-astro-checks` as the package-level Astro facade. Directly wiring all Astro packages into the app runner would increase the app runner dependency surface; the facade keeps the app runner simple while preserving real package boundaries.
- Kept the old flat Astro packages in the repository as legacy code, but removed them from the active G3TS app path.
- Added package-local assertion crates and package-local runtime tests for the new Astro packages.

Key files for context

- apps/guardrail3-ts/Cargo.toml
- apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs
- packages/ts/astro/g3ts-astro-checks/src/run.rs
- packages/ts/astro/g3ts-astro-check-support/src/lib.rs
- packages/ts/astro/setup/g3ts-astro-setup-config-checks
- packages/ts/astro/content/g3ts-astro-content-config-checks
- packages/ts/astro/mdx/g3ts-astro-mdx-config-checks
- packages/ts/astro/seo/g3ts-astro-seo-config-checks
- packages/ts/astro/state/g3ts-astro-state-file-tree-checks
- .plans/2026-04-27-130312-astro-real-family-packages.md

Verification

- `cargo test --workspace` passed in every new Astro package workspace.
- `cargo test --workspace` passed in `apps/guardrail3-ts`.
- `g3rs validate --path` passed for every new Astro package and `apps/guardrail3-ts`.
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force` passed.
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory` passed and emitted the new setup/content/MDX/SEO rule IDs.
- `rg` found no active app or new-package references to the old flat Astro check package names.

Next steps

- Replace copied golden config test builders with smaller package-local builders so g3rs no longer prints warning-level findings for the documented test-module `dead_code` allow.
- Continue moving any remaining Astro rules out of the old flat legacy packages only when those rules become active again.
