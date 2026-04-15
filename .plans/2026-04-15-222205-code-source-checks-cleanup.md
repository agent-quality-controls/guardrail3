Goal

Clean `packages/rs/code/g3rs-code-source-checks` until it validates with no findings, unless a real rule contradiction appears first.

Approach

1. Normalize the package root.
   - Add the missing workspace-root policy files.
   - Make publish intent explicit at the root and in child crates.
   - Keep release burden off unpublished crates.

2. Remove wrapper-only package structure.
   - Read `crates/types` and shrink or bypass it if it only re-exports `g3rs-code-types`.
   - Rewire runtime and root crates to depend on the real shared types crate directly.

3. Fix the old test shape.
   - Create owned shared assertions files under `crates/assertions/src/<rule>/rule.rs`.
   - Move final result proof out of runtime sidecars into the assertions crate.
   - Stop sidecars from importing sibling helpers, sibling assertions modules, or local types crates directly.

4. Fix runtime source structure.
   - Make `parse/comments/mod.rs` and `parse/visitors/mod.rs` facade-only.
   - Split oversized files only where the current rules require it.

5. Verify the whole package.
   - Run package tests.
   - Run `guardrail3-rs validate --path packages/rs/code/g3rs-code-source-checks`.
   - If a real rule contradiction appears during cleanup, stop and report that instead of patching around it.

Key decisions

- Treat `crates/types` as a bug if it only re-exports `g3rs-code-types`.
  - Reason: wrapper crates caused the same bad edges in earlier packages and were removed there.

- Keep the newer test contract.
  - Sidecars should call owned production `rule` code and the shared assertions crate only.
  - Final result assertions belong in the assertions crate, not in runtime sidecars.

- Fix source structure at the architectural boundary, not with waivers.
  - `mod.rs` files should become dispatchers.
  - Oversized parsing helpers should be split only as much as the current package needs.

Files to modify

- `packages/rs/code/g3rs-code-source-checks/Cargo.toml`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-source-checks/crates/assertions/Cargo.toml`
- `packages/rs/code/g3rs-code-source-checks/crates/types/Cargo.toml`
- `packages/rs/code/g3rs-code-source-checks/crates/types/src/lib.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/**`
- `packages/rs/code/g3rs-code-source-checks/crates/assertions/src/**`
- workspace-root policy files under `packages/rs/code/g3rs-code-source-checks/`
