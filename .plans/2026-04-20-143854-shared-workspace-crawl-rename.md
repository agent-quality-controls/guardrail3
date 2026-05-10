## Goal

Rename `packages/rs/g3rs-workspace-crawl` into a permanently neutral shared package and update all Rust and TypeScript consumers to use that neutral boundary.

End state:

- package root is `packages/shared/g3-workspace-crawl`
- crate names are `g3-workspace-crawl`, `g3-workspace-crawl-runtime`, `g3-workspace-crawl-types`, `g3-workspace-crawl-assertions`
- Rust module paths use `g3_workspace_crawl`
- public types are neutral (`G3WorkspaceCrawl`, `G3WorkspaceEntry`, ...)
- no TS package depends on an `rs`-scoped crawl package anymore

## Approach

1. Move the package root from `packages/rs` to `packages/shared`.
2. Rename package metadata, READMEs, and guardrail allowlists to the neutral name.
3. Rename public type names from `G3Rs*` to `G3*` neutral crawl names.
4. Update all Rust app/family consumers and the TS eslint scaffold to the new crate/module/type names.
5. Regenerate lockfiles as needed through normal cargo commands.
6. Verify with formatting, compile/tests, and `g3rs validate` on affected roots.

## Key Decisions

- Keep a single shared crawl package forever.
  - Reason: crawl semantics are intentionally neutral and should not fork by language.
- Neutralize both the package name and the exported type names.
  - Reason: leaving `G3RsWorkspaceCrawl` inside a shared package would still leak Rust branding into TS consumers.
- Keep the crawl package narrow.
  - It continues to own only filesystem crawl facts and simple path queries.

## Alternatives Considered

- Keep the current package but let TS depend on it.
  - Rejected: wrong boundary, even if behavior is neutral.
- Duplicate a TS crawl package.
  - Rejected: same logic, worse architecture, guaranteed drift.

## Files To Modify

- `packages/rs/g3rs-workspace-crawl/**` -> moved to `packages/shared/g3-workspace-crawl/**`
- all Cargo manifests and source imports referencing `g3rs-workspace-crawl` or `g3rs_workspace_crawl`
- any guardrail allowlists and docs that still point at the old name/path
