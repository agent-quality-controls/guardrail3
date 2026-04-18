# Websmasher architecture handoff

## What websmasher is

User submits a URL, we crawl it, deliver a full audit report (AI search readiness, SEO, etc.) at app.websmasher.com. Behind login, dashboard with query submission and report viewing.

## Services

4 apps in a monorepo:
- **web** - Next.js dashboard (app.websmasher.com) - auth, reports, crawl submission
- **landing** - Next.js marketing site
- **backend** - Rust/Axum business logic, scheduling, background jobs
- **crawler** - Python/FastAPI crawl engine

Template repo: `/Users/tartakovsky/Projects/websmasher/websmasher/`

## Decisions made

### 1. Backend architecture (apparch)

**Plan**: `.plans/2026-04-11-144026-apparch-rule-family.md`

3 layers + direction split for Rust backends:

```
types/              # structs, enums, traits, errors. Depends on nothing.
logic/              # all business logic, workflows, orchestration. Depends on types only.
io/
  inbound/          # HTTP, CLI, gRPC. Depends on types + logic + io/outbound (composition root).
  outbound/         # DB, APIs, fs. Depends on types only.
```

Key decisions:
- Dropped the hexarch 4-layer interior (domain/ports/app) because placement required semantic judgment agents can't reliably make
- Merged domain + ports + app into types/ + logic/ because they have identical dependency (types only) and the dyn Trait distinction is semantic not mechanical
- io/inbound is the composition root (creates outbound impls, passes as trait objects to logic)
- 4 enforceable rules: RS-APPARCH-01 through RS-APPARCH-04

### 2. Frontend architecture (apparch for Next.js)

**Plan**: `.plans/2026-04-11-213610-apparch-nextjs-frontend.md`

5 layers:

```
src/
  types/          # TypeScript types, Zod schemas
  logic/          # pure business functions
  adapters/       # DB queries (Drizzle), API clients (outbound I/O)
  components/     # pure React components (props in, JSX out)
  app/            # Next.js router, pages, server actions (inbound I/O + composition root)
```

Enforced via ESLint boundaries plugin. Same dependency matrix as backend plus components/ layer.

### 3. State management

**Plan**: `.plans/2026-04-12-124350-websmasher-frontend-arch.md` (section 1)

Three state layers:
- **Server state**: TanStack Query v5 (polling, mutations, client-side cache)
- **Client UI state**: Zustand (sidebar, filters, modals - selective subscriptions)
- **URL state**: nuqs (type-safe search params, shareable URLs)

Production-validated combo (Sentry, Supabase, Vercel, Clerk use it).

### 4. Data fetching & mutations

**Plan**: `.plans/2026-04-12-124350-websmasher-frontend-arch.md` (section 2)

- RSC for initial page load (no separate SSR strategy - App Router merges SSR into server components)
- TanStack Query takes over on client via initialData handoff
- Simple mutations: server actions via next-safe-action (Zod validation, type-safe errors)
- Complex mutations: TanStack Query useMutation (optimistic updates, rollback, retry)
- Both go through adapters/ layer

### 5. Component organization

**Plan**: `.plans/2026-04-12-124350-websmasher-frontend-arch.md` (section 3)

Feature-based:
```
components/
  ui/           # shadcn managed, never edit
  report/       # report viewer components
  dashboard/    # dashboard components
  crawl/        # crawl submission + status
  shared/       # truly shared across 3+ features
```

Components never co-locate in app/ routes. Always in components/.

### 6. Design system

**Plan**: `.plans/2026-04-12-124350-websmasher-frontend-arch.md` (section 4, rewritten)

Stack: **Primer token taxonomy + shadcn/ui components + Tailwind v4 + next-themes**

NOT using Primer's React components. Only their token naming scheme. shadcn provides the component library (Radix a11y). Tailwind provides utility generation.

### 7. Design tokens

**Repo**: `github.com/websmasher/design-tokens` (private)
**Local**: `/Users/tartakovsky/Projects/websmasher/design-tokens/`

Three-layer cascade (base -> semantic -> component) + themes:

```
src/
  base/               # raw scales - ONLY place raw values exist
    scale.css         # 23 size steps (1px through 128px + negatives)
    palette.css       # 13 hue ramps x 10 steps + alpha variants (130+ solid + 20 alpha)
    typography.css    # font size/weight/line-height/family scales
    motion.css        # duration + easing scales
  semantic/           # purpose-based tokens -> var(--base-*)
    spacing.css       # Primer taxonomy: stack/control/controlStack/overlay/spinner (~70 tokens)
    color.css         # Primer taxonomy: bgColor/fgColor/borderColor/control/checked/track/knob (~100 tokens)
    typography.css    # Primer taxonomy: display/title/subtitle/body/caption/code (~45 tokens)
    radius.css        # small/medium/large/full (5 tokens)
    border.css        # widths + focus outline (9 tokens)
    shadow.css        # inset/resting/floating levels (8 tokens)
    z-index.css       # 8 stacking layers
    motion.css        # duration + easing by purpose (10 tokens)
  component/          # per-component tokens -> var(--semantic-*)
    button.css        # sizing, padding, gap, radius, colors per variant
    card.css          # padding, radius, shadow
    input.css         # sizing, padding, radius, shadow
    overlay.css       # padding, radius, shadow levels per type
  themes/
    light.css         # imports palette + semantic color + shadow (default)
    dark.css          # overrides semantic color + shadow tokens in .dark scope
```

Consumer imports:
```css
@import '@websmasher/design-tokens/structure.css';     /* spacing, typography, radius, etc. */
@import '@websmasher/design-tokens/themes/light.css';  /* colors + shadows */
@import '@websmasher/design-tokens/themes/dark.css';   /* dark overrides */
```

Key design decisions:
- Primer's semantic spacing taxonomy (pattern x density: stack/control/overlay x condensed/normal/spacious)
- Color palette sourced from 3jane design system (`github.com/3jane/design_system`), 13 hues
- Alpha variants hardcoded in palette (not runtime color-mix) - 10/20/40% for status hues
- Constraint system: semantic/component tokens reference base via var(). No raw values outside base/.
- Shadows are documented exception (composite values with OKLCH, can't reference palette cleanly)
- Same-file self-references in semantic/ are accepted (shorthand tokens, aliases)

### 8. Service-to-service contracts (OpenAPI)

**Plan**: `.plans/2026-04-11-213610-apparch-nextjs-frontend.md` (service-to-service section)

Code-first OpenAPI:
- Rust: utoipa generates spec from handler annotations
- Python: FastAPI generates spec natively
- TypeScript clients generated from specs (Hey API / openapi-typescript)
- Pre-commit hook: source change -> regen spec -> regen clients -> type check consumers -> block if broken
- Specs and generated clients both committed to repo

### 9. Accessibility

shadcn/Radix handles ARIA, keyboard nav, focus management out of the box. axe-core for testing. Don't break what Radix gives you.

## Decisions NOT yet made

These were identified but not resolved:

### Forms
- React Hook Form + Zod + server actions is the likely stack
- Conform is an alternative for progressive enhancement
- No decision made on: form organization pattern, error display conventions, multi-step form approach

### Auth
- Not discussed at all
- Needs: provider choice (Clerk? NextAuth? Custom?), session strategy, protected route pattern, middleware approach

### Error handling
- Not discussed
- Needs: error boundary placement, typed error hierarchy, toast/notification pattern, error logging/monitoring

### Real-time (crawl status)
- Identified as a need (polling crawl status while waiting)
- TanStack Query polling is the likely mechanism
- Not discussed: SSE vs polling, WebSocket needs, notification when crawl completes

### Testing strategy
- Not discussed
- Needs: what gets unit tested (logic/), integration tested (adapters/), e2e tested (app/), component tested (components/)

### Tailwind v4 @theme bridge
- Design tokens exist but the Tailwind bridge (mapping tokens to @theme for utility class generation) is not implemented
- This is needed before shadcn components can consume the tokens

### stores/ directory
- Proposed Zustand stores as a top-level directory alongside the 5 apparch layers
- Dependency rules: stores/ can depend on types/ and logic/ only
- Not finalized or added to the plan

### lib/ directory
- Framework glue (env validation, cn(), auth config)
- Sits outside the apparch layers
- Not formalized

### Component token files
- Only 4 component token files exist (button, card, input, overlay)
- Needed as products require them: badge, avatar, nav, table, tab, tooltip, progress, skeleton, alert, form-label, checkbox/radio

## Prior art / reference projects

- **autosmm web** (`/Users/tartakovsky/Projects/autosmm/apps/web/`) - old web app example with repository pattern + DI
- **steady-parent landing** (`/Users/tartakovsky/Projects/steady-parent/apps/landing/`) - content pipeline, micro design tokens, pure presentational components
- **steady-parent admin** (`/Users/tartakovsky/Projects/steady-parent/apps/admin/`) - hexarch with ESLint enforcement
- **3jane design system** (`github.com/3jane/design_system`) - previous full design system, color palette source
- **websmasher template** (`/Users/tartakovsky/Projects/websmasher/websmasher/`) - monorepo template with Rust backend + Next.js frontend

## Key files

- Backend apparch plan: `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-11-144026-apparch-rule-family.md`
- Frontend apparch plan: `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-11-213610-apparch-nextjs-frontend.md`
- Frontend arch decisions: `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-12-124350-websmasher-frontend-arch.md`
- Design tokens repo: `/Users/tartakovsky/Projects/websmasher/design-tokens/`
- This handoff: `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-12-220738-websmasher-arch-handoff.md`
