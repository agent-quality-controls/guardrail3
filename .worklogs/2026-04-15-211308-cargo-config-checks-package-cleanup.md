Summary
- Cleaned `packages/rs/cargo/g3rs-cargo-config-checks` until full validation returned `No findings.` The package now uses the shared `g3rs-cargo-types` crate directly, has shared assertions and generic test support in the expected places, and its runtime sidecars route proof through the assertions crate.

Decisions made
- Deleted the local wrapper `crates/types` crate and depended on `g3rs-cargo-types` directly. Rejected keeping a one-line wrapper because it added structure without owning any boundary.
- Added `crates/test_support` only for generic builders and parsing helpers. Rejected canned fixture exports after `RS-TEST-FILETREE-18` correctly showed that hardcoded string fixtures do not belong in shared test support.
- Moved flat test files to `cases.rs` sidecars and removed `#[path]` wiring. Rejected keeping the old test module pattern because it kept tripping source and arch rules for the wrong shape.
- Kept proof in the shared assertions crate, but used the existing `define_result_assertions!` helpers instead of inventing new local wrappers. The remaining false negatives were fixed in `RS-TEST-SOURCE-07`, not by bending the package.
- Added an assertions structural-split waiver. Rejected splitting the assertions crate because the package intentionally keeps one assertions module per cargo rule.

Key files for context
- `.plans/2026-04-15-202821-cargo-config-checks-package-cleanup.md`
- `packages/rs/cargo/g3rs-cargo-config-checks/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-config-checks/guardrail3-rs.toml`
- `packages/rs/cargo/g3rs-cargo-config-checks/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/assertions/src/common.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/test_support/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/test_support/src/input.rs`

Next steps
- Continue package-by-package cleanup with the next cargo package.
- Stop only on the next real rule contradiction or stale rule contract.
