# Plan: apparch for Next.js frontend apps

## Goal

Define the apparch layer structure for Next.js/TypeScript frontend applications. Same principles as backend apparch (mechanically determinable placement, enforced dependency direction) adapted for the frontend reality: UI rendering layer, Next.js app router conventions, ESLint-based enforcement instead of Cargo.toml.

## Architecture

```
src/
  types/          # TypeScript types, Zod schemas, interfaces, error types
  logic/          # Pure business functions (validation, scoring, transforms, formatting)
  adapters/       # DB queries (Drizzle), external API clients, backend service calls
  components/     # Pure React components (props in, JSX out)
  app/            # Next.js app router: pages, layouts, server actions, middleware
```

### Layer roles

**types/** - Pure data definitions. TypeScript interfaces, type aliases, Zod schemas, error types, constants, enums. No functions with logic (except Zod refinements). No imports from other layers.

**logic/** - Pure business computation. Validation functions, score calculations, data transformations, formatting helpers. Depends on types/ only. No database calls, no React, no Next.js APIs. Testable with plain unit tests.

**adapters/** - Outbound I/O. Drizzle database queries, fetch calls to the Rust backend or third-party APIs. Each adapter function takes typed input, returns typed output. Depends on types/ only. Does not call logic/ - it just fetches/persists data.

**components/** - Pure UI. React components that receive props and render JSX. Can import from types/ (for prop types) and logic/ (for formatting, validation). Must NOT import from adapters/ - no database calls in components. Both server components and client components live here, but neither calls the database directly.

**app/** - Next.js app router. Pages, layouts, loading states, error boundaries, server actions, middleware. This is the composition root and inbound I/O layer. Pages call adapters/ for data, pass results to components/, use logic/ for transforms. Server actions orchestrate mutations: validate input (logic/), persist (adapters/), revalidate. Depends on everything.

### Dependency matrix

```
              types  logic  adapters  components  app
types           -     NO      NO         NO       NO
logic          YES     -      NO         NO       NO
adapters       YES    NO       -         NO       NO
components     YES   YES      NO          -       NO
app            YES   YES     YES        YES        -
```

No cycles. Same shape as backend apparch where adapters/ = io/outbound and app/ = io/inbound.

### Placement test

- Is it a type, interface, Zod schema, or constant? -> `types/`
- Is it a pure function (no DB, no React, no fetch)? -> `logic/`
- Does it query a database or call an external API? -> `adapters/`
- Is it a React component that receives props and renders? -> `components/`
- Is it a page, layout, server action, or route handler? -> `app/`

### Mapping to backend apparch

| Backend (Rust)    | Frontend (Next.js) | Role                          |
|-------------------|--------------------|-------------------------------|
| types/            | types/             | Pure data definitions         |
| logic/            | logic/             | Pure business computation     |
| io/outbound       | adapters/          | External I/O (DB, APIs)       |
| (no equivalent)   | components/        | Pure UI rendering             |
| io/inbound        | app/               | Entry points + composition    |

The frontend adds one layer (components/) that backends don't need because backends don't render UI.

## Next.js-specific considerations

### Server Components vs Client Components

Both can live in components/. The architecture rule is about IMPORTS, not about where code runs:
- Server component in components/: receives props, renders JSX. Fine.
- Client component in components/: receives props, has state/effects, renders JSX. Fine.
- Server component that calls the database directly: VIOLATION. Must go in app/ (page) or the query must be extracted to adapters/.

### Server Actions

Server actions are mutations triggered by the client. They live in app/ (co-located with the pages that use them, or in a shared app/actions/ directory). A server action typically:
1. Validates input (calls logic/)
2. Persists data (calls adapters/)
3. Revalidates cache (Next.js API)

This is the orchestration pattern - same as io/inbound in backend apparch.

### Route Handlers (API routes)

Route handlers (app/api/**/route.ts) are io/inbound - they receive HTTP requests and compose responses using logic/ and adapters/. Same rules as pages.

### Middleware

Next.js middleware (middleware.ts at src root or in app/) is io/inbound. Auth checks, redirects, header manipulation. Can import from types/ and logic/ but should not call adapters/ directly (middleware runs on the edge, DB access may not be available).

## Enforcement

### ESLint boundaries plugin

Enforce import rules via eslint-plugin-boundaries (already used in the websmasher template):

```js
// eslint.config.mjs (conceptual)
{
  rules: {
    'boundaries/element-types': [/* define types, logic, adapters, components, app */],
    'boundaries/entry-point': [/* barrel imports only */],
    'boundaries/external': [/* restrict external deps per layer */],
  }
}
```

Import rules to enforce:
- types/: cannot import from logic/, adapters/, components/, app/
- logic/: can import from types/ only
- adapters/: can import from types/ only
- components/: can import from types/, logic/ only
- app/: can import from anything

### External dependency restrictions per layer

| Layer        | Allowed external imports                                    |
|--------------|-------------------------------------------------------------|
| types/       | zod (for schemas)                                           |
| logic/       | zod, pure utility libs                                      |
| adapters/    | drizzle, database drivers, fetch/HTTP clients               |
| components/  | react, UI libs (shadcn, lucide-react), zod (for form validation) |
| app/         | next, react, anything                                       |

This prevents framework dependencies from leaking into pure layers. logic/ must not import React. adapters/ must not import React. types/ must not import drizzle.

## File structure example (websmasher dashboard)

```
src/
  types/
    report.ts           # Report, AuditResult, Score types + Zod schemas
    user.ts             # User, Session types
    crawl.ts            # CrawlRequest, CrawlStatus types
    errors.ts           # AppError, ValidationError types

  logic/
    scoring.ts          # calculateScore(), aggregateResults()
    validation.ts       # validateUrl(), validateReportParams()
    formatting.ts       # formatScore(), formatDate(), truncateUrl()

  adapters/
    db/
      reports.ts        # getReports(), createReport(), getReportById()
      users.ts          # getUserByEmail(), createUser()
    api/
      crawler.ts        # submitCrawl(), getCrawlStatus() (calls Rust backend)

  components/
    report-card.tsx     # displays a single report summary
    report-list.tsx     # lists reports
    score-badge.tsx     # colored score indicator
    url-input.tsx       # URL entry form component
    dashboard-shell.tsx # layout wrapper

  app/
    layout.tsx          # root layout (auth provider, theme)
    middleware.ts       # auth redirect
    (auth)/
      login/page.tsx
      register/page.tsx
    (dashboard)/
      page.tsx          # dashboard: calls adapters/db/reports, renders components/
      reports/
        [id]/page.tsx   # report detail: calls adapters/db/reports, renders components/
      new/
        page.tsx        # new report form
        actions.ts      # server action: validate (logic/) -> submit crawl (adapters/api/) -> save (adapters/db/)
    api/
      webhooks/
        crawl-complete/route.ts  # webhook from crawler backend
```

## Relation to backend apparch

This is the same architecture with the same dependency rules. The enforcement mechanism differs (ESLint vs Cargo.toml) but the layers, dependency matrix, and placement test are equivalent plus one additional layer (components/).

A guardrail3 check family for this would be a TypeScript variant of apparch, checking:
- Import boundaries via ESLint config presence and correctness
- Or direct import graph analysis on TypeScript source

That's a separate implementation concern from the Rust apparch family.
