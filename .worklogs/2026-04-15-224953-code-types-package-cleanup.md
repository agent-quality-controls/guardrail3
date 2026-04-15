Summary
- Cleaned `packages/rs/code/g3rs-code-types` until `validate` returned `No findings.`
- Converted the package from old single-crate shape into the current workspace-root `*-types` shape and split the public types into `src/types.rs`.

Decisions made
- Kept this package as a single shared crate at the workspace root because that is the established shape for clean `*-types` packages.
- Added explicit `publish = false`, standard feature gates, workspace-scoped lint policy, and root policy files because the old manifest shape was the source of most findings.
- Kept `module_name_repetitions = "allow"` only with a documented waiver because the shared type names intentionally carry family context.

Key files for context
- `packages/rs/code/g3rs-code-types/Cargo.toml`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-types/src/types.rs`
- `packages/rs/code/g3rs-code-types/guardrail3-rs.toml`

Next steps
- Continue with the next `packages/rs/code/*` package.
- Stop only if a rule is genuinely contradictory or clearly outdated.
