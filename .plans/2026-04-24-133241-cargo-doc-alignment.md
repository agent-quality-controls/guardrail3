## Goal

Align the cargo family docs with the cargo implementation that actually exists in this repo.

## Approach

- Update the live by-family cargo plan.
  - Remove stale references to non-existent `apps/guardrail3/.../families/cargo` paths.
  - Describe the current extracted package implementation under `packages/rs/cargo`.
  - Describe the current root-scoped ingestion shape instead of the stale workspace-routing shape.
- Update the package READMEs so they match the current contract:
  - root `Cargo.toml` is the owned policy root
  - that root may be a workspace root or a standalone package root
  - member surfaces are only ingested when the root is a workspace root
  - source checks are not implemented
- Keep the older detailed cargo ledger as historical/reference only.

## Key decisions

- Document current code, not aspirational routing.
  - The live by-family plan currently describes behavior and paths that do not exist in this repo.
- Do not rewrite cargo architecture in this pass.
  - This is a doc-alignment change only.
- Keep the distinction between:
  - current extracted package implementation
  - historical ledger / migration notes

## Files to modify

- `.plans/by_family/rs/cargo.md`
- `packages/rs/cargo/g3rs-cargo-ingestion/README.md`
- `packages/rs/cargo/g3rs-cargo-config-checks/README.md`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/README.md`
