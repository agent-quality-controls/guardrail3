# TS-SEO — Route metadata and SEO surface checker

**Input:** discovered TS apps, route files, metadata generators, SEO artifacts, selected TS/TSX source files
**Parser:** directory discovery + JSON + targeted TS/TSX AST/config inspection
**Current code:** no cohesive family yet; today SEO/static-route ideas live only in planning docs
**Owned root:** TS package/app roots with public web/content routing surfaces

## Owns

- sitemap presence/ownership
- robots presence/ownership
- page/layout metadata ownership
- structured-data presence when required by the chosen SEO contract
- static-route/prerender ownership where that is part of the site contract
- route-surface completeness for public web/content pages
  - explicit static-route generation where required
  - canonical route metadata ownership

## Does not own

- content pipeline/model structure
  - that belongs to `ts/content`
- locale/message completeness
  - that belongs to `ts/i18n`
- generic accessibility or CSS tooling
  - that belongs to `ts/css`

## Contract direction

This family exists because SEO/site-surface rules are real product concerns, but they are not the same as content-model checks.

It should own the public discovery/indexing surface:
- routes
- metadata
- sitemap
- robots

Any dependency on content facts should be a fact dependency, not a reason to collapse the families together.
