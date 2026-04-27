# Goal
Remove the duplicate `RS-CODE-FILETREE-35` rule from the active `code` family and add exact crate-level waivers for `g3rs-arch/structural-split` so intentionally large family runtime crates can be documented rather than forcing a global weakening.

# Approach
1. Add failing proofs for both changes.
   - In `packages/rs/code/g3rs-code-ingestion/.../file_tree.rs`, replace the old structural-cap expectations with assertions that oversized trees no longer emit `RS-CODE-FILETREE-35`.
   - In `packages/rs/arch/g3rs-arch-file-tree-checks/.../rs_arch_07a_structural_split_tests/mod.rs`, add cases proving a matching waiver should suppress the structural-split error and that non-matching waivers should not.
   - In `packages/rs/arch/g3rs-arch-ingestion/...`, add a pipeline test proving `guardrail3-rs.toml` waivers reach the filetree lane.
2. Remove `RS-CODE-FILETREE-35` at the family root.
   - Delete the rule module and tests.
   - Remove runtime wiring.
   - Update ingestion tests that still assert the deleted rule.
3. Add typed rust-policy state to the `arch` filetree boundary.
   - Extend `g3rs-arch-types` with a narrow rust-policy state for the filetree lane.
   - Parse `guardrail3-rs.toml` once in `g3rs-arch-ingestion` and thread the state into `G3RsArchFileTreeChecksInput`.
4. Teach `g3rs-arch/structural-split` to honor exact waivers.
   - Match on `rule`, `file`, and `selector`.
   - Stand down only for the exact crate.
   - Emit the normal error when no exact waiver matches.
5. Verify with targeted package tests and CLI validation against the clippy package workspace.

# Key Decisions
- Keep the structural split policy in `arch`, not `code`, because it is the stronger architectural rule and the user explicitly chose to drop the duplicate generic cap.
- Add waiver support only to `g3rs-arch/structural-split`, not a generic ignore system for the whole family.
- Reuse existing `guardrail3-rs.toml` waiver schema instead of inventing a new file or selector format.

# Alternatives Considered
- Keep both rules and change severities: rejected because it still produces duplicate signal on the same condition.
- Weaken `g3rs-arch/structural-split` globally for runtime crates: rejected because the user wants an explicit exception, not a global carveout.
- Add app-layer filtering for waivers: rejected because family applicability and waiver semantics belong inside the family packages.

# Files To Modify
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/lib.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/rs_code_filetree_35_root_structural_cap.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/rs_code_filetree_35_root_structural_cap_tests/mod.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/file_tree.rs`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/test_support.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split_tests/mod.rs`
- `packages/rs/arch/g3rs-arch-ingestion/Cargo.toml`
- `packages/rs/arch/g3rs-arch-types/Cargo.toml`
