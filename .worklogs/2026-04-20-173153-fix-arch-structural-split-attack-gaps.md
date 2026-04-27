Summary

Closed the attack findings against the recursive `g3rs-arch/structural-split` fix. Arch ingestion now measures structural complexity from the actual active code roots, ignores inactive `src` trees for custom root-level libs, ignores `tests/examples/benches/target` during recursive structural counting, and pins those behaviors with new pipeline regressions.

Decisions made

- Fixed the attack gaps in ingestion, not in the pure rule.
  - The rule still consumes precomputed crate facts.
  - The bug was in how ingestion selected roots and traversed the tree.
- Introduced entrypoint-aware structure roots.
  - Library roots come from `lib_rs_rel`.
  - `src` is included only when it is actually active, such as `src/lib.rs`, `src/main.rs`, or the fallback no-custom-root case.
- Split `workspace` into `workspace/mod.rs`, `workspace/run.rs`, and `workspace/structure.rs`.
  - Reason: the attack follow-up fix pushed `workspace` over both the code-length and facade-only rules.
- Added pipeline regressions instead of relying on synthetic rule tests.
  - Reason: the attacks were against ingestion behavior, not just the pure rule comparison logic.

Key files for context

- `.plans/2026-04-20-170554-fix-arch-structural-split-recursive-scan.md`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/mod.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/structure.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree_tests/pipeline.rs`

Verification

- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-file-tree-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-types/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/rs/arch/g3rs-arch-ingestion/Cargo.toml`
- `g3rs validate --path packages/rs/arch/g3rs-arch-ingestion`
- `g3rs validate --path packages/rs/arch/g3rs-arch-file-tree-checks`
- `g3rs validate --path packages/rs/arch/g3rs-arch-types`

Next steps

- Re-run adversarial review if we touch the structure root logic again.
- If we later want module-tree-only semantics instead of source-root semantics, that needs a larger redesign based on parsed module reachability rather than filesystem heuristics.
