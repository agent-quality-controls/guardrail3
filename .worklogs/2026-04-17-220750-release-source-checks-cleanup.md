Summary
- Cleaned `packages/rs/release/g3rs-release-source-checks` to the current internal package shape and brought it to `No findings.`
- Removed the old inline runtime test bodies and shared `test_support` file, replacing them with owned sidecars and a real shared assertions crate.

Decisions made
- Switched runtime from the local `crates/types` facade to `g3rs-release-types` directly so the runtime boundary matches the rest of the cleaned release family.
- Kept the production files flat and paired them with owned `*_tests/` sidecar directories because the runtime still has one rule per flat file and the test rules key off that shape.
- Built a small flat assertions surface (`run`, `rs_release_source_01_readme_quality`, `rs_release_source_02_input_failures`) instead of inventing a nested hierarchy for only two rules.
- Marked the facade and member crates non-publishable and copied the standard release family policy files instead of widening waivers or preserving the old publishable shell.

Key files for context
- `packages/rs/release/g3rs-release-source-checks/Cargo.toml`
- `packages/rs/release/g3rs-release-source-checks/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/Cargo.toml`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/rs_release_source_01_readme_quality.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/rs_release_source_01_readme_quality_tests/helpers.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/rs_release_source_02_input_failures.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/assertions/src/common.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/assertions/src/run.rs`

Next steps
- Continue with the last dirty release root, `packages/rs/release/g3rs-release-types`.
- Re-run the release family sweep after that package is cleaned to confirm the whole family is back to `No findings.`
