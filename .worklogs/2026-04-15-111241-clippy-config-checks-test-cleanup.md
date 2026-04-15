Summary
- Fixed the clippy config-checks package so the `test` family passes cleanly.
- Moved shared proof into the assertions crate, moved generic input builders into a sibling `test_support` crate, and removed the old runtime-local test support split.

Decisions made
- Kept one shared assertions surface for both internal sidecar tests and external tests so proof stays in one place.
- Kept `test_support` generic. It only builds inputs and override facts. It does not own semantic result checks.
- Switched nested `01..08` sidecars to call `crates/assertions/src/<rule>/rule.rs` directly.
- Reworked `09..21` tests to stop inspecting `CheckResult` directly and call shared assertions helpers instead.
- Moved nested assertions facades from `foo.rs` plus `foo/` to `foo/mod.rs` plus `foo/rule.rs` so the package also stays clean under arch file layout.
- Added a crate-level waiver for the assertions crate structural split rule because one shared assertions module per rule is the intended shape here.

Key files for context
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/test_support/src/input.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/guardrail3-rs.toml`

Next steps
- Package validates cleanly now.
- If another family package shows the same test split problem, reuse this exact shape: runtime + assertions + test_support.
