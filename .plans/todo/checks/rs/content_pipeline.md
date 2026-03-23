# RS-CONTENT — Rust content pipeline checker

**Input:** Content source tree + content-pipeline config + schema definitions + generated content artifacts + Rust pipeline code
**Parser:** `ProjectTree` + TOML/JSON/YAML as needed + Rust AST for pipeline/compiler code
**Current code:** None yet — new family needed for Rust content pipeline validation

## Scope

This family is the Rust replacement for the useful parts of the old content-site / Velite / MDX planning.

It should validate a VLite-style Rust content pipeline and the safety/integrity of MDX-like content compilation, without carrying over TS/Next/npm-specific implementation details.

## Rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-CONTENT-01 | Error | One canonical content-pipeline config/entrypoint exists | Planned |
| RS-CONTENT-02 | Error | One canonical content source root exists; content is not scattered arbitrarily | Planned |
| RS-CONTENT-03 | Error | Content schemas/frontmatter schemas exist and are typed | Planned |
| RS-CONTENT-04 | Error | Required metadata fields are structurally enforced (`slug`, publish state, summary, dates, etc.) | Planned |
| RS-CONTENT-05 | Error | Slugs/route identities are unique and stable across collections/locales | Planned |
| RS-CONTENT-06 | Error | Local asset and internal-content references resolve correctly | Planned |
| RS-CONTENT-07 | Error | MDX-like compilation only allows approved component bridges/shortcodes | Planned |
| RS-CONTENT-08 | Error | Raw script/unsafe HTML/runtime-eval style escapes are banned or explicitly inventoried | Planned |
| RS-CONTENT-09 | Warn | Document structure quality: heading hierarchy, code fence languages, empty links/images | Planned |
| RS-CONTENT-10 | Warn | Generated content/types/manifests must not be hand-edited | Planned |
| RS-CONTENT-11 | Error | Content changes must be covered by the pipeline and generated outputs must stay in sync | Planned |
| RS-CONTENT-12 | Error | Pipeline/source/config parse failures fail closed | Planned |

## Notes

- “VLite-style” here means: typed content schemas, a compile step, generated artifacts, and a controlled bridge into runtime UI.
- The family should validate content-pipeline safety and integrity, not frontend component semantics in general.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Velite package presence by name | Tooling-specific; keep the pipeline invariant, not the JS implementation |
| Next.js `page.tsx` / `generateStaticParams` specifics | Framework-specific legacy TS mechanism |
