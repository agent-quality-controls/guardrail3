Summary

Cleaned `packages/rs/hooks/g3rs-hooks-source-checks` to the current package shape and restored the package after the bulk runtime rule split. The package now passes both `cargo test` and `guardrail3-rs validate` with no findings.

Decisions made

- Converted runtime rule modules from facade-owning `mod.rs` plus `tests/` sidecars to facade-only `mod.rs`, sibling `rule.rs`, and owned `rule_tests/` sidecars.
- Moved assertions from the old flat `src/<rule>.rs` shape to owned `src/<rule>/rule.rs` modules so runtime sidecars and external tests share the same proof surface.
- Kept `hook_rs_09_clippy_denies_warnings` support as a real sibling module under the rule package instead of using `#[path]`, and tightened internal visibility instead of widening the API.
- Removed the stale `hook_rs_13_cargo_dupes_excludes/support` tree because the rule no longer uses it and the leftover facade violated `mod.rs` rules.
- Switched runtime off the local types crate and onto `g3rs-hooks-types`, and feature-gated the local re-export facade instead of keeping open exports.

Key files for context

- `packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-source-checks/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/mod.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/support.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/assertions/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/assertions/src/common.rs`

Next steps

- Continue package-by-package in `packages/rs/hooks`, starting with the next dirty root after `g3rs-hooks-source-checks`.
- Re-run the full family sweep after the hooks slice is cleaned to catch any remaining old-shape debt in later families.
