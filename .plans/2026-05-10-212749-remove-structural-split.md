# Remove `g3rs-arch/structural-split` rule and dead shadow infrastructure

## Goal

Delete the `g3rs-arch/structural-split` enforcement rule (depth ≤ 3, ≤ 4 sibling dirs, ≤ 10 sibling `.rs` files) and all of its supporting infrastructure across the repo. The rule is overly restrictive in practice (50 workspaces waive it). Also remove the dead shadow metrics in `g3rs-code-ingestion` that compute the same numbers but are never consumed by any rule.

## Scope

### Active enforcement (must be removed)

- `g3rs-arch/structural-split` rule emitter and its tests:
  - `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/structural_split.rs`
  - `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/structural_split_tests/`
  - `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/structural_split.rs`
- `mod structural_split;` in `g3rs-arch-file-tree-checks/{runtime/src/lib.rs, assertions/src/lib.rs}`
- `crate::structural_split::check(...)` call in `g3rs-arch-file-tree-checks/runtime/src/run.rs`

### Type fields and ingestion compute (active surface)

- `G3RsArchFileTreeCrate { max_sibling_rs_file_count, max_sibling_dir_count, max_module_depth }` in `packages/rs/arch/g3rs-arch-types/src/types.rs:43-45`
- The mirror block at `types.rs:170-172` (likely an analyzed-source variant)
- The `expect` reason at `types.rs:40` cited those names; remove the attribute or rewrite reason
- `g3rs-arch-ingestion/crates/runtime/src/file_tree.rs:87-89` populates the three fields from `node.structure.*`
- `g3rs-arch-ingestion/crates/runtime/src/workspace/run.rs:290-306` computes via `measure_max_sibling_counts`/`measure_module_depth`
- `g3rs-arch-ingestion/crates/runtime/src/workspace/structure.rs` defines `measure_max_sibling_counts` (line 44) and `measure_module_depth` (line 85). Delete both helpers and any of their private callees that become unused.

### Dead shadow (no consumer; pure delete)

- `G3RsCodeRoot { max_module_depth, max_sibling_dirs, max_sibling_rs_files }` in `packages/rs/code/g3rs-code-types/src/types.rs:149-153`
- `g3rs-code-ingestion/crates/runtime/src/run.rs:135-191` compute and zero-init sites
- `g3rs-code-ingestion/crates/assertions/src/run_file_tree.rs:25-27` assertions
- `module_depth` helper at `g3rs-code-ingestion/crates/runtime/src/run.rs:279` if unused after the field removals

### Waivers (50 workspaces)

Strip every `[[waivers]]` entry where `rule = "g3rs-arch/structural-split"` from each `guardrail3-rs.toml`. List captured at `.baselines/structural-split-waivers.txt`. After removal, count must be 0.

If a workspace's only `[[waivers]]` table was the structural-split entry, delete the section header too. Don't leave dangling empty sections.

## Approach

Manifest-driven, three phases.

### Phase 1: baseline + manifest

- Plan: `.plans/2026-05-10-212749-remove-structural-split.md` (this file)
- Manifest: `.plans/2026-05-10-212749-remove-structural-split.manifest.toml`
- Verifier: `scripts/verify-remove-structural-split.py` (single-file, reads its own manifest)
- Baseline file: `.baselines/structural-split-waivers.txt` (50 entries captured)

### Phase 2: parallel agents

Five independent agents:

| Agent | Scope |
|---|---|
| **A: rule package** | Delete the rule emitter + tests + assertions module; strip mod/use/call lines in `lib.rs`/`run.rs`. |
| **B: arch types + ingestion** | Remove the three fields from both blocks of `g3rs-arch-types/src/types.rs`; remove the `#[expect]` reason that named them; gut `arch-ingestion/runtime/src/{file_tree.rs, workspace/run.rs, workspace/structure.rs}`. Delete helpers `measure_max_sibling_counts`/`measure_module_depth` and any private callees that become dead. |
| **C: code ingestion shadow** | Remove the three fields from `g3rs-code-types/src/types.rs:149-153`; delete the compute and zero-init sites in `code-ingestion/runtime/src/run.rs`; delete the matching assertions in `code-ingestion/assertions/src/run_file_tree.rs:25-27`. Drop `module_depth` helper if unused. |
| **D: waiver cleanup** | Read `.baselines/structural-split-waivers.txt`. For each file, parse TOML, remove `[[waivers]]` entries where `rule == "g3rs-arch/structural-split"`. Drop empty `[[waivers]]` blocks. Preserve other waivers verbatim. |
| **E: rule registry / docs** | Search the repo for any catalog, registry, doc, or test that lists `g3rs-arch/structural-split` and remove the entry. |

Each agent must run `cargo clippy` + `cargo fmt` for its touched workspaces and report exit codes.

### Phase 3: verification

- Run `python3 scripts/verify-remove-structural-split.py` - all layers must PASS.
- Run `apps/guardrail3-rs/target/release/g3rs validate --path <every adopted RS workspace>` - all must exit 0.
- Run `scripts/verify/all.sh` - 8 layers PASS (existing manifest must continue to pass).
- Adversarial review: send two reviewers to find missed references, dead test silences, leftover catalog entries, accidental sibling-rule damage.

## Key decisions

- **No backwards-compat shims.** No deprecation warning, no rule-id alias. The rule is gone.
- **Field removal, not rename to `_unused`.** Three callers exist: the rule (deleted), the ingestion populator (deleted), the assertion test in `g3rs-code-ingestion/assertions` (deleted). No external consumers.
- **Waiver removal, not soft-archive.** Leaving stale waivers referencing a non-existent rule is dead code in policy form.
- **`module_depth` helper at `g3rs-code-ingestion/runtime/src/run.rs:279`.** Drop only if unused after the field removals; if a sibling computation still calls it, keep.
- **`structure.rs` private callees of `measure_max_sibling_counts`.** Recurse into the file and remove only what becomes dead. Don't drag in unrelated helpers.

## Files to modify (concrete list)

```
DELETE:
  packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/structural_split.rs
  packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/structural_split_tests/   (whole dir)
  packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/structural_split.rs

EDIT:
  packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/lib.rs           (drop "mod structural_split;")
  packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/run.rs           (drop crate::structural_split::check call)
  packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/lib.rs        (drop "pub mod structural_split;")
  packages/rs/arch/g3rs-arch-types/src/types.rs                                   (drop fields x2; drop "expect" reason)
  packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree.rs            (drop populator block)
  packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/run.rs        (drop computes + use line)
  packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/structure.rs  (drop measure_*; private callees if dead)
  packages/rs/code/g3rs-code-types/src/types.rs                                   (drop fields)
  packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs                  (drop computes, init zeros, module_depth fn if unused)
  packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run_file_tree.rs     (drop 3 assert_eq lines)
  50x guardrail3-rs.toml files listed in .baselines/structural-split-waivers.txt  (drop matching [[waivers]] entries)
```

## Done condition

- `grep -rn "structural[_-]split" packages/ apps/ scripts/` returns 0 hits (excluding this plan and the manifest/baseline files).
- `grep -rn "max_module_depth\|max_sibling_rs_file_count\|max_sibling_dir_count\|max_sibling_dirs\|max_sibling_rs_files" packages/ apps/` returns 0 hits.
- `python3 scripts/verify-remove-structural-split.py` exits 0.
- `scripts/verify/all.sh` exits 0.
- Every adopted RS workspace exits 0 on `g3rs validate --path <ws>`.
