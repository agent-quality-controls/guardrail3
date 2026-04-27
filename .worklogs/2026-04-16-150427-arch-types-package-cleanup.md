Summary

Normalized `packages/rs/arch/g3rs-arch-types` into the current shared `*-types` workspace shape and kept all arch dependents green. The main package-specific cleanup was shrinking the giant shared crate record and reducing the lib facade export surface so the package validates cleanly without waivers.

Decisions made

- Kept the root fix in the package, not in the code rule.
  - Why: `g3rs-code/ast-19-large-type-inventory` and `g3rs-code/ast-11-many-use-imports` were flagging real package shape issues, not a bad rule.
  - Rejected: waiving the large shared record or the crowded lib facade.
- Split `G3RsArchCrateNode` into nested transport records:
  - `G3RsArchFeatureContract`
  - `G3RsArchDependencyCounts`
  - `G3RsArchCrateStructure`
  - Why: this keeps the shared type as plain transport data but removes the 20-field top-level bag.
  - Rejected: inventing constructors or getters for a shared data carrier.
- Reduced the crate-root facade to only the three public input types:
  - `G3RsArchConfigChecksInput`
  - `G3RsArchFileTreeChecksInput`
  - `G3RsArchSourceChecksInput`
  - Everything else now lives under `g3rs_arch_types::types::...`.
  - Why: this fixes the top-level import-count warning without broad re-exports.
  - Rejected: `pub use types::*`, because arch explicitly forbids broad re-exports.

Key files for context

- `packages/rs/arch/g3rs-arch-types/Cargo.toml`
- `packages/rs/arch/g3rs-arch-types/src/lib.rs`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `packages/rs/arch/g3rs-arch-types/guardrail3-rs.toml`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`

Next steps

- Continue the package-by-package cleanup pass after `packages/rs/arch/g3rs-arch-types`.
- Stop only on the next real rule contradiction or false positive.
