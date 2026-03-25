# TS-CONTENT — Content pipeline and content-model checker

**Input:** discovered content apps, content config files, content roots, content-model source files
**Parser:** directory discovery + JSON + structured TS/TSX AST scans
**Current code:** content-specific portions of `jscpd_check.rs`, content gating in `app/ts/validate/mod.rs`
**Owned root:** TS package/app roots with `type = "content"` or content auto-detection

## Owns

- content-app discovery and content-type-aware checks
- content pipeline presence
  - `velite`, `contentlayer`, or equivalent configured content root
- content directory ownership
  - content files live under canonical content roots, not scattered arbitrarily
- content schema/model ownership
- content slug and content-reference integrity where the content model requires it
- generated content artifact ownership
- generated content artifact freshness/sync requirements
- content-site safety checks that are structural to the content pipeline itself
  - no direct database/persistence layer creep
  - no server-state creep in content-only surfaces
- content-site API route safety for simple proxy/form routes
  - input validation before external calls
  - explicit error-response handling contract
- content-site image policy
  - canonical image component choice for the site contract
  - required alt text on authored image surfaces

## Does not own

- CSS/style lint tooling
  - that belongs to `ts/css`
- general `ESLint` baseline or plugin wiring
  - that belongs to `ts/eslint`
- service-app hex architecture
  - that belongs to `ts/hexarch`
- library/package architecture
  - that belongs to `ts/libarch`
- locale routing or message completeness
  - that belongs to `ts/i18n`
- sitemap, robots, metadata, or static SEO surface
  - that belongs to `ts/seo`
- generic source-scan rules
  - that belongs to `ts/code`

## Current old-code split to normalize

- `jscpd_check.rs`
  - currently holds early content-only checks like Velite config
- `mod.rs`
  - currently gates content checks as one broad profile bucket
- `ts-project-types.md`
  - currently carries content-only API/image safety rules that must be pulled into this family

## Contract direction

This is the TypeScript-side analogue of the content-model architecture.

It should own the rules that answer:
- is this a content site
- does it have the required content pipeline
- is the content model rooted in the right places
- do content schemas/generated artifacts exist in the right shape
- do content-only API/image surfaces obey the content-site contract

This family should not absorb i18n or SEO just because content sites often use both.
