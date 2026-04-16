Goal
- Normalize the remaining stale internal test sidecars that still hang off `lib.rs` or facade `mod.rs` files.

Approach
- For packages where validation only reports `RS-TEST-FILETREE-02`, move `run_tests` off `lib.rs` onto `run.rs`.
- For packages where nested facades still own `rule_tests`, move those sidecars onto `rule.rs`.
- Update helper imports from `super::super::rule::check` to `super::super::check` where the sidecar move changes visibility paths.
- Re-run package tests and package validation after each package.

Key decisions
- Treat these as package-shape fixes, not rule work.
- Keep the current package architecture and only change test ownership.

Files to modify
- package-local `crates/runtime/src/lib.rs`
- package-local `crates/runtime/src/run.rs`
- package-local `crates/runtime/src/*/mod.rs`
- package-local `crates/runtime/src/*/rule.rs`
- affected `rule_tests/helpers.rs` and `rule_tests/cases.rs`
