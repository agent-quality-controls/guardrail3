# Goal

Remove `g3ts-arch/structural-split` completely.

# Approach

- Remove the rule module from `g3ts-arch-file-tree-checks`.
- Remove the rule invocation from the file-tree check runner.
- Remove rule-specific tests that expect `g3ts-arch/structural-split`.
- Remove `G3TsArchSourceTree` and `source_tree` ingestion because they only exist to feed this removed rule.
- Keep `g3ts-arch/declared-entrypoint-exists` unchanged.

# Key decisions

- Do not replace the threshold with an Astro waiver or Astro profile. The sibling-directory threshold is not proving architectural quality across real TS packages or Astro apps.
- Do not keep source-tree metrics as unused facts. Dead facts make future rules look more justified than they are.
- Do not remove Rust `g3rs-arch/structural-split` in this change. The user asked about the current TS/Astro failure, and Rust has separate historical waiver behavior.

# Files to modify

- `packages/ts/arch/g3ts-arch-types/src/types.rs`
- `packages/ts/arch/g3ts-arch-types/src/lib.rs`
- `packages/ts/arch/g3ts-arch-ingestion/crates/runtime/src/file_tree.rs`
- `packages/ts/arch/g3ts-arch-ingestion/crates/runtime/src/run.rs`
- `packages/ts/arch/g3ts-arch-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/arch/g3ts-arch-file-tree-checks/crates/runtime/src/lib.rs`
- `packages/ts/arch/g3ts-arch-file-tree-checks/crates/runtime/src/run.rs`
- `packages/ts/arch/g3ts-arch-file-tree-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/arch/g3ts-arch-file-tree-checks/crates/runtime/src/structural_split.rs`
