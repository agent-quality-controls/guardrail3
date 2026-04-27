Summary

Fixed `g3rs-arch/structural-split` so it catches oversized module folders anywhere inside a crate, not only at the crate root. Added failing pipeline tests for nested `.rs` file piles and nested directory piles, then changed arch ingestion to compute recursive maximum sibling counts and threaded those corrected facts into the pure rule.

Decisions made

- Fixed the bug in arch ingestion, not in the rule.
  - The rule was already correct for the facts it received.
  - The broken part was the aggregate: ingestion only counted siblings at `src/` or crate root.
- Renamed the crate structure fields from `sibling_*` to `max_sibling_*`.
  - Reason: after the fix, the values represent recursive maxima across the crate tree, not one root-level folder count.
- Proved the bug at the pipeline level first.
  - The failure mode was an incomplete orchestrator fact, so pure rule tests alone were not sufficient.

Key files for context

- `.plans/2026-04-20-170554-fix-arch-structural-split-recursive-scan.md`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split.rs`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`

Verification

- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-file-tree-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-types/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/rs/arch/g3rs-arch-ingestion/Cargo.toml`
- `g3rs validate --path packages/rs/arch/g3rs-arch-ingestion`
- `g3rs validate --path packages/rs/arch/g3rs-arch-file-tree-checks`
- `g3rs validate --path packages/rs/arch/g3rs-arch-types`

Next steps

- Revisit the TS ESLint `full_config` split separately. That change satisfied the current root-level shape rule, but it would still violate the stronger recursive intent that this bug fix restores for arch structural checks.
