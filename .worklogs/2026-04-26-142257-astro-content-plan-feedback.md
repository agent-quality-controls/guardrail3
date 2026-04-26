# Summary

Updated the Astro/content boundary plan with concrete coverage for the latest landing-agent feedback. The plan now explicitly assigns generated framework leftovers, runtime MDX eval, blog routes, metadata provenance, and typed JSON-LD helper contracts to `TS-ASTRO`, while keeping content quality facts framework-independent.

# Decisions Made

- Kept `TS-CONTENT` framework-independent because Astro collections, Astro route files, Astro adapters, and Astro integrations disappear if Astro is removed.
- Assigned `.next/**`, `.velite/**`, `.contentlayer/**`, route metadata provenance, runtime MDX execution, blog route shape, and JSON-LD helper wiring to `TS-ASTRO`.
- Kept rendered SEO validation in Nuasite and future `TS-SEO`; Astro only owns the Astro-specific wiring that makes those validators run.
- Kept slug, draft, future-date, link, and image checks as normalized content facts, not Astro route parsing.

# Key Files

- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `.plans/by_family/ts/content.md`

# Next Steps

- Implement the `TS-ASTRO` strict content profile in the order listed in the plan.
- Start with policy facts, route class facts, file-tree generated-state bans, and effective ESLint option checks.
- Keep source semantics in delegated ESLint rules and dependency policy in Syncpack.
