Summary

Cleaned `packages/rs/fmt/g3rs-fmt-types` by converting it into an explicit one-crate workspace and adding the missing workspace-root policy files. No new rule bug showed up there after the root-shape noise was removed.

Decisions made

- Reused the same one-crate-workspace shape already proven on `g3rs-clippy-types`.
- Kept the crate unpublished with `publish = false`.
- Split `src/lib.rs` into a facade plus `src/types.rs` so the package obeys the facade-only rule.
- Added a waiver reason for the intentional `module_name_repetitions` allow.

Key files for context

- `packages/rs/fmt/g3rs-fmt-types/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-types/guardrail3-rs.toml`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`

Next steps

- Continue package-by-package outside clippy and fmt types.
- Watch for the next real rule contradiction instead of more old root-shape cleanup.
