# RS-FRONTEND-ROUTES — Rust frontend routes and SEO checker

**Input:** Route definitions + content-derived route manifests + metadata generators + sitemap/robots outputs
**Parser:** Rust AST + generated route/content facts + structured config/artifact parsing
**Current code:** None yet — new family needed for Rust frontend routing/SEO validation

## Scope

This family carries the route/SEO side of the old content-site planning into Rust frontend code.

It should validate typed routing completeness, metadata completeness, and sitewide SEO artifacts for Rust frontend/content apps.

## Rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-FRONTEND-ROUTES-01 | Error | All routable content/pages map to typed routes or an explicit generated route manifest | Planned |
| RS-FRONTEND-ROUTES-02 | Error | Dynamic/content-derived routes have explicit prerender/route inventory and do not fail open on unknown params | Planned |
| RS-FRONTEND-ROUTES-03 | Warn | Routable pages provide required metadata (title/description/canonical or approved equivalent) | Planned |
| RS-FRONTEND-ROUTES-04 | Warn | Sitemap generation exists and covers routable public content | Planned |
| RS-FRONTEND-ROUTES-05 | Warn | Robots policy exists and is generated/owned centrally | Planned |
| RS-FRONTEND-ROUTES-06 | Info | Structured-data support exists for content/article/docs pages where applicable | Planned |
| RS-FRONTEND-ROUTES-07 | Error | Route/metadata/sitemap input failures fail closed | Planned |

## Notes

- This is not just “SEO exists”; it should check that route and metadata ownership is explicit and tied to the Rust frontend/content model.
- Structured-data coverage should be profile/site-type aware, not universal by default.
