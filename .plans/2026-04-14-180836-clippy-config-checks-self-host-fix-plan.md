Goal

- Make `packages/rs/clippy/g3rs-clippy-config-checks` pass the active Rust-only guardrail stack when validated by `apps/guardrail3-rs`.
- Fix the failures at their architectural source instead of patching the CLI output.

Observed failure groups

- Missing root workspace policy/config files:
  - `rust-toolchain.toml`
  - `rustfmt.toml`
  - `clippy.toml`
  - deny root config
  - `guardrail3-rs.toml`
- Apparch purity failure:
  - `crates/types` depends on `g3rs-clippy-types`
- Arch/layout failures:
  - missing `all` feature in `crates/types`
  - boundary-crossing deps in `crates/assertions` and `crates/runtime`
  - facade-only failures in `lib.rs` and many `mod.rs`
  - forbidden `#[path]` usage in runtime rule files
  - structural-cap failure in `crates/runtime`
- Test-family failures:
  - ad hoc `#[cfg(test)]` declarations
  - sidecar test modules import sibling production and assertion modules directly
  - sidecar modules own semantic result assertions they should not own
- Release/package-metadata failures:
  - missing keywords/categories/docs.rs/include-exclude/README
  - publish dry-run failures
  - missing LICENSE / `release-plz.toml` / `cliff.toml`
  - missing release workflows
- Hooks failures:
  - missing `.githooks/pre-commit`
  - missing `core.hooksPath`

Approach

- Fix in this order so the package gets progressively more truthful and easier to verify:

1. Root workspace contract
   - Add the missing root files for this workspace:
     - `guardrail3-rs.toml`
     - `rust-toolchain.toml`
     - `rustfmt.toml`
     - `clippy.toml`
     - deny root config
   - Goal:
     - eliminate filetree/config hard-fail noise
     - make later architectural failures easier to read
   - Tests:
     - rerun the CLI against this workspace and prove those specific filetree findings disappear

2. Apparch purity cut
   - Inspect whether `crates/types -> g3rs-clippy-types` is a real architecture problem or a modeling problem.
   - Expected fix:
     - move the dependency out of `crates/types`, or
     - reclassify the crate layout if `crates/types` is not actually a `types/*` layer in apparch terms
   - Do not special-case apparch.
   - Fix the package boundary or the crate placement.

3. Arch surface cleanup
   - Add missing `all` feature to `crates/types`.
   - Audit dependencies in `crates/assertions` and `crates/runtime`.
   - Convert non-facade `lib.rs` / `mod.rs` surfaces to facade-only layout where required.
   - Remove `#[path]` from runtime rule files by moving tests to the expected sidecar shape.
   - Re-run family validation after each sub-batch so the arch failures shrink monotonically.

4. Test architecture migration
   - This is likely the biggest real fix batch.
   - Migrate rule-sidecar tests away from:
     - sibling assertions imports
     - sibling production imports
     - ad hoc `#[cfg(test)]`
     - semantic assertions inside sidecar modules
   - Bring them to the current repo pattern:
     - one rule file
     - one rule-specific sidecar test module directory
     - owned assertions/helpers only
   - Expect this to remove most `RS-TEST-*` and many `g3rs-arch/mod-facade-only/09` findings together.

5. Runtime structural split
   - Once test layout and facade layout are fixed, re-check whether `crates/runtime` still exceeds structural caps.
   - If yes, split the runtime by a real ownership seam instead of tuning the caps.

6. Release contract
   - Add missing metadata and README files for root and subcrates.
   - Add root `LICENSE`.
   - Add `release-plz.toml` and `cliff.toml` only if this package workspace is meant to be releasable as a standalone workspace.
   - Decision boundary:
     - if these family workspaces are internal-only, the release family may be too strict for package workspaces
     - but do not assume that yet
     - first prove whether the current intended contract is "self-hostable publishable workspace" or "internal dev workspace"

7. Hooks
   - Add the expected hook file and local hooks path config if this workspace is supposed to be self-validating in isolation.
   - If hooks are supposed to be repo-root only, that is a family-contract bug, not a package bug.
   - Decide this from the actual hooks family semantics before changing the workspace.

Key decisions to settle early

- Is each family workspace supposed to be a fully self-hosting Rust workspace under the active rules?
  - If yes, fix package by package.
  - If no, some root-scoped families (`release`, `hooks`) are overfiring on nested workspaces and should be corrected at the family level.
- Is `crates/types` in these package workspaces truly an apparch `types/*` layer?
  - If yes, purity rule stands and the dependency is wrong.
  - If no, apparch is overclassifying these internal crate layouts.

First execution slice

- Start with only these changes:
  - add root config files
  - add `guardrail3-rs.toml`
  - rerun CLI
  - inspect the `crates/types -> g3rs-clippy-types` dependency for the apparch purity fix
- Do not touch release or hooks until we see the reduced post-config failure set.

Files likely to change first

- `packages/rs/clippy/g3rs-clippy-config-checks/guardrail3-rs.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/rust-toolchain.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/rustfmt.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/clippy.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/deny.toml` or `.deny.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/types/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
