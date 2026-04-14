# Summary
Removed the duplicate `RS-CODE-FILETREE-35` rule from the active `code` family and added exact crate-level waivers to `RS-ARCH-FILETREE-07`. The `arch` filetree lane now parses `guardrail3-rs.toml` once and honors `rule + file + selector` waivers, and the clippy config-checks workspace now documents its intentional runtime-crate exception there.

# Decisions Made
- Kept structural split ownership in `arch` and dropped the duplicate `code` structural-cap rule entirely. Rejected keeping both rules with different messaging because it still duplicates the same signal.
- Added a narrow `G3RsArchRustPolicyState` only to the `arch` filetree boundary. Rejected app-layer waiver handling because family waiver semantics belong inside the family package.
- Matched waivers exactly on `RS-ARCH-FILETREE-07` + `Cargo.toml` path + `structural-split` selector. Rejected broad family or crate-name wildcard ignores because the user wanted an explicit documented exception.
- Applied the new waiver only to `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`. Rejected global weakening of the structural-split rule.

# Key Files For Context
- `.plans/2026-04-14-194705-remove-code-35-add-arch-structural-waivers.md`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split_tests/mod.rs`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/file_tree.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/guardrail3-rs.toml`

# Next Steps
- Decide whether `RS-CODE-SOURCE-31` should stand on assertion-helper DTOs or be scoped away from that support surface.
- Decide whether `RS-ARCH-SOURCE-04` should treat `*_tests/mod.rs` as part of the same facade-only contract or recognize the sidecar test-directory pattern.
- If more family workspaces need intentional large runtime crates, add explicit `RS-ARCH-FILETREE-07` waivers in their local `guardrail3-rs.toml` files instead of weakening the rule.
