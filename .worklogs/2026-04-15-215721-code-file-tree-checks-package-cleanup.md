Summary
- Cleaned `packages/rs/code/g3rs-code-file-tree-checks` to `No findings.` The package now uses `g3rs-code-types` directly, is explicitly unpublished, and no longer carries an unused public assertions helper or a fake wrapper `types` crate.

Decisions made
- Deleted the local wrapper `crates/types` crate and depended on `g3rs-code-types` directly. Rejected keeping the wrapper because it only reexported shared code-family types and added fake structure.
- Marked the whole workspace unpublished with explicit `publish = false`.
- Kept the assertions crate as a minimal scaffold only. Rejected inventing test helpers because the package has no runtime tests yet.
- Removed the exported `common` helper and the public field bag `ExpectedRuleResult` because they were unused and only created false signal.

Key files for context
- `.plans/2026-04-15-215605-code-file-tree-checks-package-cleanup.md`
- `packages/rs/code/g3rs-code-file-tree-checks/Cargo.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/src/lib.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/assertions/Cargo.toml`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/assertions/src/lib.rs`

Next steps
- Move to `packages/rs/code/g3rs-code-ingestion`.
- Stop only on the next real outdated or contradictory rule.
