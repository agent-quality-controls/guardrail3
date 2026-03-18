# TypeScript Project Types + Content Site Guardrails

## Overview

TypeScript apps are not all the same. A marketing site has completely different guardrails than an API service. guardrail3 needs per-app type configuration that applies the right checks.

## Project Types

### `service` — HTTP server, API backend
Existing checks: T-ARCH-01/02 (hex arch), T50 (route wrappers), full ESLint strict.

Structure:
```
apps/my-api/src/
  modules/
    domain/           Pure types
    ports/            Interfaces
    application/      Use cases (not "app" — Next.js reserves it)
    adapters/         HTTP handlers, DB clients
```

### `content` — Marketing site, blog, docs (NEW)
Static-first Next.js with Velite content pipeline.

Structure:
```
apps/landing/
  src/
    app/              Next.js routes (locale-first)
    components/       UI components
    lib/              Business logic (API wrappers, helpers)
    content/          Velite schemas + content validation
    types/            Content type definitions
  content/            Content source (MDX, JSON)
```

### `library` — Shared package, no I/O
Existing: dependency allowlist, no side effects.

### `extension` — VS Code extension, CLI plugin (future)
Different I/O rules, no route wrappers.

## Config

```toml
[typescript.apps.admin]
type = "service"

[typescript.apps.landing]
type = "content"

[typescript.packages.generator]
type = "library"
```

## Content Site Checks (T-CONTENT-XX)

### Build-time Content Safety

| Check | What | Detection |
|-------|------|-----------|
| T-CONTENT-01 | Velite config exists | `velite.config.ts` or `velite.config.js` at app root |
| T-CONTENT-02 | Content schemas validate with Zod | `velite-schemas.ts` or similar imports `zod` |
| T-CONTENT-03 | Velite prepare hook exists | `velitePrepare` or equivalent cross-collection validation function |

### Static Generation Enforcement

| Check | What | Detection |
|-------|------|-----------|
| T-CONTENT-04 | All `page.tsx` export `generateStaticParams` | AST scan: each `page.tsx` in `app/` must export `generateStaticParams` function |
| T-CONTENT-05 | All `page.tsx` export `dynamicParams = false` | AST scan: `export const dynamicParams = false` in each `page.tsx` |
| T-CONTENT-06 | All `page.tsx` export `generateMetadata` or inherit | AST scan: `generateMetadata` exported, or parent layout has it |

### SEO Completeness

| Check | What | Detection |
|-------|------|-----------|
| T-CONTENT-07 | Sitemap exists | `sitemap.ts` or `sitemap.xml` in `app/` |
| T-CONTENT-08 | Robots config exists | `robots.ts` or `robots.txt` in `app/` |
| T-CONTENT-09 | JSON-LD structured data on content pages | AST scan: `application/ld+json` script in page components |

### i18n Correctness

| Check | What | Detection |
|-------|------|-----------|
| T-CONTENT-10 | No hardcoded locale in URLs | Scan for `"/en/"`, `"/fr/"` etc. in source — should use locale helpers |
| T-CONTENT-11 | Locale routing configured | `middleware.ts` uses `next-intl` or equivalent |
| T-CONTENT-12 | Messages files exist for all configured locales | Check `messages/*.json` matches locale config |

### Content Architecture

| Check | What | Detection |
|-------|------|-----------|
| T-CONTENT-13 | Content in `content/` directory (not scattered in `src/`) | Scan for `.mdx`, `.json` content files outside `content/` |
| T-CONTENT-14 | No direct database imports | Scan for `prisma`, `drizzle`, `knex`, `pg`, `mysql` in deps |
| T-CONTENT-15 | No server-side state | Scan for global mutable state patterns |

### API Route Safety (content sites have simple proxy routes)

| Check | What | Detection |
|-------|------|-----------|
| T-CONTENT-16 | API routes validate input before external calls | AST scan: POST handlers must have validation before fetch/API calls |
| T-CONTENT-17 | API routes return proper error responses | Check for try/catch or error handling patterns |

### Image Handling

| Check | What | Detection |
|-------|------|-----------|
| T-CONTENT-18 | Consistent image component usage | If project uses CDN images (unoptimized: true in next.config), flag `next/image` imports |
| T-CONTENT-19 | Images have alt text | AST scan: `<img>` or image components must have `alt` prop |

## Implementation Plan

### Phase 1: Config schema
- Add `[typescript.apps.*]` to guardrail3.toml types
- Add `type` field: `"service"` | `"content"` | `"library"`
- T-ARCH checks only fire on `type = "service"` apps
- T-CONTENT checks only fire on `type = "content"` apps

### Phase 2: Content checks (tree-sitter based)
- T-CONTENT-01 through T-CONTENT-03: file existence checks
- T-CONTENT-04 through T-CONTENT-06: AST scan of page.tsx files for exports
- T-CONTENT-07 through T-CONTENT-09: file existence + AST

### Phase 3: Content safety
- T-CONTENT-10 through T-CONTENT-15: source scan + dependency checks

### Phase 4: API + image checks
- T-CONTENT-16 through T-CONTENT-19: AST-based validation

## Detection: How to Tell if an App is Content vs Service

If not configured in guardrail3.toml, auto-detect:
- Has `velite` in deps → content
- Has `src/modules/domain/` → service
- Has neither → unknown (don't apply type-specific checks, only generic)
- Has `engines.vscode` in package.json → extension

## Interaction with Existing Checks

Content sites SKIP:
- T-ARCH-01/02 (hex arch — not applicable)
- T50 (route wrappers — no HTTP framework)
- T-TEST-01 (Stryker — limited value for content sites)

Content sites KEEP:
- T1-T8 (ESLint config)
- T9-T10 (tsconfig strict)
- T23-T31 (source scan — eslint-disable, process.env, any types)
- T32 (file length)
- T-TEST-02/03 (test files, test runner)

## Depends On
- Per-app TS config in guardrail3.toml (Phase 1)
- tree-sitter AST infrastructure for scanning page.tsx exports
