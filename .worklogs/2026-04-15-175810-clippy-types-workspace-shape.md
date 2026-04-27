Summary

Normalized `packages/rs/clippy/g3rs-clippy-types` into an explicit one-crate Rust workspace and added the missing workspace-root policy files. This removed the old root-shape noise and narrowed the package down to one remaining rule question about public fields on shared input structs.

Decisions made

- Kept the package as a single crate, but made it an explicit workspace because active topology, release, and apparch normalization all expect workspace-local roots.
- Marked the crate `publish = false` and added explicit root policy files instead of leaving publish and policy intent implicit.
- Split `src/lib.rs` into a facade plus `src/types.rs` so the package obeys the facade-only rule.
- Added a waiver reason for the intentional `module_name_repetitions` allow.

Key files for context

- `packages/rs/clippy/g3rs-clippy-types/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-types/guardrail3-rs.toml`
- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-types/src/types.rs`

Next steps

- Decide `g3rs-code/ast-31-public-struct-named-fields` for shared types crates.
- Current remaining findings are only the public-field signals on:
  - `G3RsClippyCargoConfigOverride`
  - `G3RsClippyConfigChecksInput`
  - `G3RsClippyShadowedConfig`
  - `G3RsClippyFileTreeChecksInput`
- If the rule stands, add constructors/accessors and update all call sites.
- If the rule is too strict for shared data-only types crates, change the rule instead of forcing boilerplate everywhere.
