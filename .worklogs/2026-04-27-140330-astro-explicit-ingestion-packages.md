Summary

Removed the active `g3ts-astro-checks` aggregate package and rewired the TS structure runner to call Astro setup, content, MDX, SEO, and state ingestion/check packages explicitly. Added per-area Astro ingestion packages and fixed the Rust arch dependency-count rule so explicit waivers work instead of forcing dependencies to be hidden behind facade packages.

Decisions made

- Deleted `packages/ts/astro/g3ts-astro-checks` because it hid the actual Astro package graph behind one aggregate check package.
- Added `g3ts-astro-setup-ingestion`, `g3ts-astro-content-ingestion`, `g3ts-astro-mdx-ingestion`, `g3ts-astro-seo-ingestion`, and `g3ts-astro-state-ingestion`.
- Kept app runner dependencies explicit and added a local `g3rs-arch/dependency-count-split` waiver for `family-runner-structure`; this is better than hiding Astro dependencies behind an aggregate package.
- Added Rust arch config waiver support for `g3rs-arch/dependency-count-split` with selector `dependency-count`, including a regression test.
- Removed direct app references to the old shared Astro ingestion and the deleted aggregate Astro checks package.

Key files for context

- apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs
- apps/guardrail3-ts/crates/logic/family-runner-structure/Cargo.toml
- apps/guardrail3-ts/guardrail3-rs.toml
- packages/ts/astro/setup/g3ts-astro-setup-ingestion
- packages/ts/astro/content/g3ts-astro-content-ingestion
- packages/ts/astro/mdx/g3ts-astro-mdx-ingestion
- packages/ts/astro/seo/g3ts-astro-seo-ingestion
- packages/ts/astro/state/g3ts-astro-state-ingestion
- packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_07b_dependency_count_split.rs
- packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/config_tests/pipeline.rs

Verification

- `cargo test --workspace` passed in `apps/guardrail3-ts`.
- `cargo test --workspace` passed in each new Astro ingestion package.
- `cargo test --workspace` passed in `packages/rs/arch/g3rs-arch-ingestion`.
- `cargo test --workspace` passed in `packages/rs/arch/g3rs-arch-config-checks`.
- Reinstalled local `g3rs`.
- `g3rs validate --path` passed for the changed Rust arch packages, all new Astro ingestion packages, and `apps/guardrail3-ts`.
- Reinstalled local `g3ts`.
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory` passed.
- grep confirms the active app no longer references `g3ts-astro-checks`, `g3ts_astro_checks`, or direct `g3ts-astro-ingestion`.

Next steps

- The old `g3ts-astro-ingestion` package is now a shared parsing/fact provider used by the per-area ingestion packages. If stricter ownership is needed, split its internals into explicit parser-support modules and move per-area selection logic out of that shared package.
