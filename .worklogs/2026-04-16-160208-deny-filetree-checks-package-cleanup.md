Summary

Cleaned `packages/rs/deny/g3rs-deny-filetree-checks` until workspace tests and full `validate` returned clean. The package now follows the current deny file-module sidecar pattern, uses shared assertions for the combined `run` proof, and no longer carries the fake local `types` crate.

Decisions made

- Removed the fake local `crates/types` crate and wired the root facade and runtime directly to `g3rs-deny-types`.
  - Why: the local crate was only a wrapper and created fake arch, release, and apparch problems.
  - Rejected: keeping the wrapper and waiving its findings.
- Added a real `crates/test_support` crate and moved the old runtime-local `test_support.rs` there.
  - Why: sidecars must not import sibling local modules from runtime.
  - Rejected: leaving `test_support.rs` in runtime and patching around `RS-TEST-FILETREE-03`.
- Kept the approved file-module sidecar shape with:
  - `#[path = "..._tests/mod.rs"]`
  - owned sidecar module names such as `run_tests`
  - same-line `// reason:` comments
  - Why: plain `mod tests;` was the stale package shape and kept tripping both code and test rules.
- Added `crates/assertions/src/run.rs` and moved the combined run proof there.
  - Why: the `run` sidecar must use its own shared assertions module, not sibling rule assertions.
  - Rejected: keeping the proof inline in `run_tests/mod.rs`.

Key files for context

- `packages/rs/deny/g3rs-deny-filetree-checks/Cargo.toml`
- `packages/rs/deny/g3rs-deny-filetree-checks/guardrail3-rs.toml`
- `packages/rs/deny/g3rs-deny-filetree-checks/src/lib.rs`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/assertions/src/run.rs`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/test_support/src/input.rs`

Next steps

- Commit this package slice as a standalone cleanup.
- Continue the package-by-package pass from the next deny package.
- Stop only on the next real contradictory rule or false positive.
