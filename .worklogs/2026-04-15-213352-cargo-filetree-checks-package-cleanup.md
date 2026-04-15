Summary
- Cleaned `packages/rs/cargo/g3rs-cargo-filetree-checks` to `No findings.` The package now uses the agreed file-module sidecar shape for `run.rs` and routes proof through the shared assertions crate.

Decisions made
- Switched `run.rs` back to `#[cfg(test)] #[path = "run_tests/mod.rs"] mod run_tests;`. Rejected `mod tests;` plus `run/tests` because that was the awkward shape we decided against.
- Kept the package-level changes already in progress:
  - direct use of `g3rs-cargo-types`
  - no local wrapper `types` crate
  - shared assertions in `crates/assertions/src/run.rs`
- Reused the existing `run` assertions module instead of adding new local proof helpers.

Key files for context
- `.plans/2026-04-15-211448-cargo-filetree-checks-package-cleanup.md`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/guardrail3-rs.toml`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/assertions/src/run.rs`

Next steps
- Move to the next cargo package.
- Stop only on the next real outdated or contradictory rule.
