Summary

Cleaned `packages/rs/fmt/g3rs-fmt-config-checks` so the package now validates with no findings under the active rules. The main work was normalizing the old runtime/assertions test layout, adding the missing workspace-root policy files, and making publish intent explicit.

Decisions made

- Reused the cleaned clippy config package shape instead of inventing a fmt-only layout.
- Kept the local `crates/types` crate as a thin package-local facade, but moved runtime code to depend on the family-wide `g3rs-fmt-types` crate directly.
- Added a sibling `crates/test_support` crate so sidecar tests stop importing the local types crate directly.
- Moved assertions from flat files into nested `mod.rs` plus `rule.rs` directories so sidecars can call the owned shared assertions module cleanly.
- Marked the workspace and child crates unpublished with explicit `publish = false`, so release checks stop treating this package as a publish unit.
- Added structural-split waivers for the intentional large `runtime` and `assertions` crates.
- Ran an adversarial pass on the package shape after validation. No new rule contradiction appeared.

Key files for context

- `.plans/2026-04-15-184704-fmt-config-checks-package-cleanup.md`
- `packages/rs/fmt/g3rs-fmt-config-checks/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-config-checks/guardrail3-rs.toml`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_01_settings/mod.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/test_support/src/input.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/types/src/lib.rs`

Next steps

- Continue package-by-package in the fmt family.
- The next likely package is `packages/rs/fmt/g3rs-fmt-filetree-checks`.
- Keep the same loop:
  - run full validation
  - fix clear package debt
  - stop only on a real rule contradiction
