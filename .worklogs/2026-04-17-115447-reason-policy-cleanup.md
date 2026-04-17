Summary
- Collapsed `packages/shared/reason-policy` away from the old nested member path and rebuilt it as a proper package with a root facade plus `crates/runtime` and `crates/assertions`.
- Moved behavior out of `lib.rs`, made `ReasonPolicy` opaque, moved tests onto an owned sidecar for `validation.rs`, and put final proof into a shared assertions crate.
- Updated all dependency paths that still pointed at `shared/reason-policy/crates/reason-policy` and verified both package validation and the app workspace build.

Decisions made
- Rejected the temporary single-crate root shape. The flaw there was architectural: this package has runtime behavior plus owned sidecars, so under the chosen test architecture it needs the standard sibling `runtime` and `assertions` crates.
- Kept the package unpublished. It is an internal shared policy crate, not an external release artifact.
- Kept the logic as one runtime crate. The code is small and cohesive; the needed split was for facade/test structure, not for multiple runtime crates.

Key files for context
- `packages/shared/reason-policy/Cargo.toml`
- `packages/shared/reason-policy/guardrail3-rs.toml`
- `packages/shared/reason-policy/src/lib.rs`
- `packages/shared/reason-policy/crates/runtime/src/lib.rs`
- `packages/shared/reason-policy/crates/runtime/src/issue.rs`
- `packages/shared/reason-policy/crates/runtime/src/policy.rs`
- `packages/shared/reason-policy/crates/runtime/src/validation.rs`
- `packages/shared/reason-policy/crates/runtime/src/validation_tests/cases.rs`
- `packages/shared/reason-policy/crates/assertions/src/validation.rs`
- Cargo.toml files across `apps/` and `packages/rs/` that depend on `guardrail3-reason-policy`

Next steps
- Commit this slice by itself.
- Run the full shared-package sweep again if another shared package path still uses the old nested-member pattern.
- Continue with the next unresolved package root after `packages/shared/reason-policy`.
