Summary

Fixed all failed non-legacy Rust workspaces found by the repo-wide G3RS sweep. The work standardized test sidecar layout, moved semantic result assertions into assertion crates, fixed source ownership bugs, removed stale waivers, and converted the remaining TS Astro directive DTO failures to private-field APIs.

Decisions Made

- Hook contract packages now use `contract_tests/` sidecars owned by `contract.rs` instead of `lib_tests/` under crate facades, because `lib.rs` must stay a facade and hook contract behavior belongs to the contract module.
- Runtime test sidecars now delegate result-shape assertions to assertion crates. This keeps runtime tests and external tests using the same proof surface instead of duplicating `G3CheckResult` checks.
- Virtual Cargo workspaces no longer own `src/lib.rs` or default binary roots. Only manifests with `[package]` can own package source roots.
- Public assertion re-export facades are allowed only when the re-export target resolves to a proof-bearing assertion item, not just when a local alias has a proof-looking name.
- `pub use` import counting is exempt only for facade files, not for arbitrary source files.
- `G3TsAstroContentEslintDirectiveInput` and `G3TsAstroSeoEslintDirectiveInput` now use private fields with constructors and getters because those structs exceeded the public named-field error threshold.

Key Files

- `.plans/2026-04-30-172431-g3rs-all-workspace-failures.md`
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/assertions/src/contract_required_tools_installed/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/assertions/src/required_contract_command_present/rule.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/classify.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/runtime_assertions_split/assertions_violations.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/core.rs`
- `packages/ts/astro/content/g3ts-astro-content-types/src/types.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-types/src/types.rs`

Verification

- Ran focused `cargo test --manifest-path ... --workspace --offline` on the repaired parser, code, hook, release, and TS Astro packages.
- Ran focused `g3rs validate --path ...` on each repaired workspace.
- Ran a full non-legacy workspace sweep over every `Cargo.toml` with `[workspace]`; every workspace passed.
- Adversarial agents reviewed the hook config, hook source, and TS directive fixes with no blocking findings after fixes.

Next Steps

- The remaining G3RS warnings are not failed workspaces. They are inventory or pre-existing type-shape warnings, mostly schema mirror structs and broad TS Astro snapshot structs.
- If those warnings should become blocking later, split or encapsulate the affected type packages in a separate targeted change.
