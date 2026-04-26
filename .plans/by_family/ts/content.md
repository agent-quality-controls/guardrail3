# TS-CONTENT

Status: planned framework-independent family. No cohesive runtime exists yet.

Current source of truth:

- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- this file for compact family status
- `.plans/todo/checks/ts/content.md` is legacy background and must not be treated as authoritative where it conflicts with the boundary plan
- legacy mentions of Velite, Contentlayer, content-pipeline presence, API route safety, generated artifact freshness, or image component choice are not final `TS-CONTENT` scope

## Boundary

`TS-CONTENT` owns authored-content product invariants that survive a framework replacement.

If removing Astro changes the rule, the rule belongs to `TS-ASTRO`, not `TS-CONTENT`.

`TS-CONTENT` must not require:

- Astro collections
- `astro:content`
- `src/content.config.ts`
- `.astro` route files
- Astro integrations
- Astro route adapters
- Astro MDX integration
- Astro output mode
- Astro package pins

Those are `TS-ASTRO`.

## Intended Inputs

The family should consume normalized content facts produced by framework families or shared parsers.

Future minimal input shape:

- `schema_validated`
- `root_rel_path`
- `content_domain`
- `entry_rel_path`
- `entry_kind`
- `slug`
- `canonical_path`
- `status`
- `title_present`
- `description_present`
- `published_at`
- `updated_at`
- `asset_refs`
- `internal_links`
- `rich_component_refs`
- `generated`
- `preview_mode`

The family must not parse Astro collection config directly.
The family must not require framework validator package/config names directly.

## Planned Rules

- `TS-CONTENT-01` - content roots are explicitly declared or routed from shared app classification.
- `TS-CONTENT-02` - content entries have required identity fields: `slug`, `title`, `description`, `status`, `publishedAt`, and `canonicalPath`.
- `TS-CONTENT-03` - slugs are unique within a content domain.
- `TS-CONTENT-04` - canonical paths are unique across public content.
- `TS-CONTENT-05` - drafts are not public unless preview mode is explicit.
- `TS-CONTENT-06` - published content has valid dates.
- `TS-CONTENT-07` - `updatedAt` is not earlier than `publishedAt`.
- `TS-CONTENT-08` - required landing/blog/docs fields are present for declared entry kinds.
- `TS-CONTENT-09` - referenced content assets exist.
- `TS-CONTENT-10` - required image alt/caption fields are present.
- `TS-CONTENT-11` - internal content links resolve.
- `TS-CONTENT-12` - internal content anchors resolve where feasible.
- `TS-CONTENT-13` - rich text uses only approved portable component names or shortcodes.
- `TS-CONTENT-14` - unsafe HTML in authored content is rejected unless waived.
- `TS-CONTENT-15` - generated content artifacts are declared as generated and are not hand-authored.

## Delegation

Prefer delegated validators:

- schema validation through JSON Schema, Zod, or a framework-owned schema output
- Markdown/MDX checks through remark/rehype plugins
- link checking through a dedicated link checker or remark plugin
- image/reference checks through a dedicated content asset validator where available
- draft and future-date policy through normalized content facts, not framework route parsing

Framework families or shared ingestion should enforce the delegated validator setup and expose normalized facts to `TS-CONTENT`.

`TS-CONTENT` can require framework-neutral facts such as `schema_validated = true`, but it cannot require Astro collections, Velite config, Contentlayer config, Zod files, or a specific package directly.

If a content-validation script is required, `TS-CONTENT` should require the framework-neutral result it produces:

- slug uniqueness facts
- draft/public facts
- future-date facts
- internal-link facts
- image-reference facts

It must not own the Astro command, Astro config, or Astro route wiring that produced those facts.

## Not Owned Here

- Astro collection setup
- Astro content adapter setup
- Astro MDX wiring
- SEO rendered tags
- sitemap and robots generation
- spelling
- style/design-token policy
- generic package manager policy

## Implementation Order

Do not implement this family first for Astro apps.

Immediate work stays in `TS-ASTRO` until strict Astro local-content profile, route classes, adapter globs, plugin option checks, and Astro waivers exist.

After that, implement `TS-CONTENT` over framework-neutral `ContentEntry` facts.
