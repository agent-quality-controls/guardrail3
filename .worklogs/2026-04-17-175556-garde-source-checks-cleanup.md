Summary

Cleaned `packages/rs/garde/g3rs-garde-source-checks` to the current package shape and brought its runtime sidecars onto the owned-rule and shared-assertions contracts. The package now passes its workspace tests and validates with no findings.

Decisions made

- Replaced the old flat assertions files with per-rule assertion modules plus a shared `run` assertions module so sidecar tests target owned `rule` modules directly.
- Split the old non-facade `parse/mod.rs` into a facade `parse/mod.rs` and implementation `parse/analysis.rs`, preserving the existing parser logic while making the module layout comply with facade rules.
- Removed the old runtime `test_support.rs` and moved fixture ownership into rule-local helper files plus test-only support inside each owned `rule.rs`, because the test filetree rules reject sidecars reaching sideways into `run` or `fs`.
- Added a centralized runtime `fs.rs` boundary and routed source analysis plus test fixture file operations through it, instead of leaving scattered `std::fs` calls in runtime support.
- Kept the package-local `crates/types` crate as a thin feature-gated re-export boundary over `g3rs-garde-types` rather than inventing another local types model.

Key files for context

- `packages/rs/garde/g3rs-garde-source-checks/Cargo.toml`
- `packages/rs/garde/g3rs-garde-source-checks/guardrail3-rs.toml`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/fs.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/parse/analysis.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_ast_04_query_as_inventory/rule.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_ast_04_query_as_inventory/rule_tests/helpers.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/assertions/src/run.rs`

Next steps

- Commit this slice by itself.
- Re-run the next dirty garde package root before leaving the family.
- Do not change rules unless the next package exposes a real contradiction after package-local cleanup.
