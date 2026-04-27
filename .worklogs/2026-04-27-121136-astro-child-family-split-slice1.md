Summary
- Planned the Astro child-family split and implemented Slice 1: nested Astro policy config for routes, content, MDX, SEO, and generated state.
- G3TS now rejects old flat Astro policy fields, requires `profile = "strict-static-content"`, preserves plural content adapters, and requires ESLint adapter modules to exactly match configured adapter roots.

Decisions made
- Kept the existing Astro rule packages as a temporary bridge instead of creating child-family packages in this slice, because the first durable boundary is the typed config shape and ingestion contract.
- Used nested `guardrail3-ts.toml` keys instead of route-class taxonomy fields; old flat keys are preserved only as parser extras and no longer configure checks.
- Made content adapters plural end-to-end. Every configured adapter root must resolve to source files, and delegated ESLint config must list matching `approvedContentAdapterModules`.
- Carried `required_collections` and `collection_fields` into the Astro policy snapshot so the later Astro content child family can consume them without reparsing.

Key files for context
- `.plans/2026-04-27-113651-astro-child-family-split.md`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_18_content_adapter_rule.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_23_strict_content_policy.rs`

Verification
- `cargo test --workspace` in `packages/parsers/guardrail3-rs-toml-parser`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-file-tree-checks`
- `cargo test --workspace` in `apps/guardrail3-ts`
- `g3rs validate --path packages/parsers/guardrail3-rs-toml-parser`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/ts/astro/g3ts-astro-file-tree-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- Adversarial review after fixes returned `CLEAN`.

Next steps
- Create actual Astro child-family packages for setup/content/MDX/SEO and migrate checks out of the flat Astro package.
- Update the landing app to the nested `guardrail3-ts.toml` policy and matching ESLint adapter/helper module options.
