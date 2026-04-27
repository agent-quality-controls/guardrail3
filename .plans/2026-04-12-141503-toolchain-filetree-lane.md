# Toolchain Filetree Lane

## Goal

Migrate the remaining live toolchain rules into package-owned filetree checks:

- `g3rs-toolchain/exists` - `rust-toolchain.toml` exists
- `g3rs-toolchain/legacy-file` - legacy `rust-toolchain` file is warned/error when present

The result should be a real `g3rs-toolchain-filetree-checks` package plus a real
`g3rs-toolchain-ingestion::ingest_for_file_tree_checks(...)` entry point.

## Approach

1. Add failing tests first.
   - Package rule tests for existence and legacy-file behavior.
   - Ingestion pipeline tests for root discovery, missing modern file, legacy-only,
     and both-files conflict.
2. Add lane-pure filetree types in `g3rs-toolchain-types`.
   - Keep config and source types unchanged.
   - Add only the facts needed for the two filetree rules.
3. Add the new filetree checks package.
   - One production rule file per rule.
   - One sidecar test module directory per rule.
   - Use `RS-TOOLCHAIN-FILETREE-*` IDs.
4. Wire ingestion.
   - Select only root `rust-toolchain.toml` and root `rust-toolchain`.
   - Do not parse either file for filetree checks.
   - Remove the filetree-not-implemented error path.
5. Verify mechanically.
   - `cargo test --workspace -q` in toolchain packages
   - `git diff --check`

## Key decisions

- Filetree lane only.
  - These rules check file presence and file conflict, not config contents.
- Root-only scope.
  - The package model validates one pointed workspace root, so the filetree lane
    should only examine root toolchain files.
- No parse dependency for filetree.
  - `g3rs-toolchain/exists` and `04` should still report even if the file
    contents are malformed.

## Alternatives considered

- Extending `g3rs-toolchain-config-checks` with these rules.
  - Rejected because the rules do not inspect config semantics.
- Keeping filetree as a stub and relying on the old app inventory only.
  - Rejected because these are still live unmigrated package targets.

## Files to modify

- `packages/rs/toolchain/g3rs-toolchain-types/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/**`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/**` (new)
- `packages/rs/toolchain/g3rs-toolchain-config-checks/README.md`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/README.md`
