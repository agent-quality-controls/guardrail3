Summary
- Cleaned `packages/rs/apparch/g3rs-apparch-source-checks` until `validate` returned `No findings.` and the package workspace tests passed.
- Removed the fake local `types` crate, added a shared assertions crate, and moved the final source-rule proof out of the runtime sidecars.

Decisions made
- Deleted `crates/types` and used `g3rs-apparch-types` directly because the local crate was only a passthrough and created fake arch and apparch coupling.
- Kept the chosen file-module sidecar contract as `#[path = "x_tests/mod.rs"] mod x_tests;` because the package still used flat sibling sidecar directories.
- Moved all direct CheckResult assertions into the new `crates/assertions` crate so the runtime sidecars are now setup plus shared-proof calls only.
- Marked the workspace and child crates unpublished and added the standard root policy files, because this workspace is internal.

Key files for context
- `packages/rs/apparch/g3rs-apparch-source-checks/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/guardrail3-rs.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/rs_apparch_source_04_io_traits_in_types.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/rs_apparch_source_05_types_public_surface.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/assertions/src/rs_apparch_source_04_io_traits_in_types.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/assertions/src/rs_apparch_source_05_types_public_surface.rs`

Next steps
- Continue to `packages/rs/apparch/g3rs-apparch-ingestion`.
- Stop only if the next failure is a real rule contradiction or an outdated rule.
