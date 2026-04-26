# Summary

Added `TS-ASTRO-CONFIG-24` to validate the structural shape of the minimal Astro policy paths. The rule rejects absolute paths, parent traversal, backslashes, glob metacharacters in directory fields, and overlapping `content_root` / `content_adapter` directories.

# Decisions

- Kept this as a pure config check over `G3TsAstroPolicySnapshot`.
- Checked discovered route overlap in a later rule instead of attempting theoretical glob-overlap analysis here.
- Rejected backslashes explicitly to avoid platform-specific path ambiguity in app policy.

# Key Files

- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_24_strict_policy_paths.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

# Verification

- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`

# Next Steps

- Implement `TS-ASTRO-CONFIG-25` by matching discovered files against `content_routes`, `non_content_routes`, and `endpoints`.
