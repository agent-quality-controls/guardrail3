Summary
- Cleaned `packages/rs/code/g3rs-code-source-checks` until `validate` returned `No findings.`
- Removed the local `types` wrapper crate, reshaped shared assertions into nested rule modules, and finished the parser split so the package satisfies `code`, `arch`, and `test`.

Decisions made
- Removed `crates/types` and used `g3rs-code-types` directly because the local crate was only a wrapper.
- Kept one-rule-per-module layout in runtime and assertions, then added `g3rs-arch/structural-split` waivers for those two crates because the structure is intentional.
- Split `parse/attrs` into `attrs/mod.rs`, `attrs/policies.rs`, and `attrs/public_surface.rs` because the right fix for the size warning was a real module split, not a waiver.
- Turned rule-sidecar helper logic into local `#[cfg(test)]` functions inside each rule file and removed `rule_tests/helpers.rs` files so sidecars stay focused on cases.

Key files for context
- `packages/rs/code/g3rs-code-source-checks/Cargo.toml`
- `packages/rs/code/g3rs-code-source-checks/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/mod.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/assertions/src/lib.rs`

Next steps
- Continue package-by-package cleanup in the `code` family.
- Stop only when a rule is genuinely unclear, contradictory, or clearly outdated.
