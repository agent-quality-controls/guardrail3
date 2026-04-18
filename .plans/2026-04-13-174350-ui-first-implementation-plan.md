# UI-First Implementation Plan

## Goal

- Start coding with a trivial but real dashboard UI slice
- Settle the design system, token bridge, and shadcn integration before auth, DB, and workflow wiring
- Use mock data first
- Keep the work aligned with the existing Websmasher architecture decisions

## Source of truth

### Use our own plans as the primary rules

- Frontend architecture:
  - `2026-04-11-213610-apparch-nextjs-frontend.md`
  - `2026-04-12-124350-websmasher-frontend-arch.md`
- Product and design system handoff:
  - `2026-04-12-220738-websmasher-arch-handoff.md`
- Current decision snapshot:
  - `2026-04-13-140830-websmasher-mvp-architecture-decisions-so-far.md`

### Borrow patterns from references, not architecture

- From `steady-parent`:
  - token-first styling
  - global CSS imports token package first
  - Tailwind `@theme inline` bridge pattern
  - pure presentational component discipline
  - shadcn pro blocks as starting points for dashboard/application sections

- From `autosmm`:
  - shadcn component usage and organization
  - dashboard/app-shell composition
  - minimal route layouts with feature composition above them

## Non-goals for this phase

- No real auth wiring
- No real DB access
- No real repository integration
- No real polling
- No real run submission
- No real report fetching

This phase is about proving the UI system, not the product workflow.

## Use of shadcn pro blocks

- Use them selectively
- Treat them as accelerators, not final product architecture

Best source:
- `steady-parent/apps/landing/src/components/pro-blocks/application/`

Best candidates:
- `app-shells/`
- `page-headers/`
- `empty-sections/`
- `cards/`
- `table-headers/`
- `application-examples/dashboard-1.tsx`

Avoid for this phase:
- landing-page blocks
- e-commerce blocks unless a pattern is directly reusable
- auth/sign-in/sign-up blocks

Rule:
- borrow layout/composition ideas
- restyle into Websmasher tokens and primitives
- simplify aggressively instead of importing giant blocks unchanged

## Rules for this phase

- Mock data only
- Tokens are the source of truth
- No raw visual values in product components if a token should exist
- Do not edit shadcn primitives directly once generated
- Keep feature components in `components/`, not in route folders
- Keep route files in `app/` thin

## Implementation order

### Phase 1. Tailwind token bridge

Purpose:
- prove that the design token package can drive utilities and component styling

Deliverables:
- import Websmasher design tokens into global CSS
- add Tailwind v4 `@theme` bridge for the semantic tokens needed immediately
- map the first token groups:
  - background/foreground/surface
  - border/input/ring
  - spacing
  - radius
  - typography
  - status colors needed for crawl/report states

Reference pattern:
- `steady-parent/apps/landing/src/app/globals.css`

Success condition:
- a component can be styled entirely through token-backed utilities without hardcoded raw values

### Phase 2. shadcn baseline + wrappers

Purpose:
- establish the primitive component layer without locking in page implementation

Deliverables:
- install/generate the minimal shadcn primitives needed now
- keep primitives in `components/ui/`
- create product-level wrappers or usage conventions for:
  - button
  - input
  - card
  - badge
  - table
  - skeleton
  - progress
  - tabs
  - dialog or drawer if needed by shell exploration

Reference pattern:
- `autosmm/apps/web/src/components/ui/`

Rules:
- do not treat raw shadcn defaults as the final design
- primitives are the base layer, not the product language

Success condition:
- the first product components can be built from styled primitives without fighting shadcn defaults

### Phase 3. Dashboard shell

Purpose:
- lock the spatial system and visual hierarchy before feature pages multiply

Deliverables:
- app shell with:
  - sidebar
  - top header
  - page container
  - content max widths / gutters
  - desktop and mobile behavior
- no real navigation logic required yet beyond mock structure

Reference patterns:
- `autosmm/apps/web/src/app/dashboard/layout.tsx`
- `steady-parent/.../pro-blocks/application/app-shells/`


Rules:
- keep route layout thin
- put shell UI in `components/dashboard/` or `components/shared/`
- use mock nav items and mock user/org identity
- start from pro blocks if they save time, then simplify

Success condition:
- one consistent shell supports multiple fake pages without layout drift

### Phase 4. Fake crawl status page

Purpose:
- pressure-test the design system against the actual product's most important stateful UI

Deliverables:
- mocked crawl submission/status page
- mocked states:
  - queued
  - running
  - failed
  - completed
- components for:
  - status badge
  - progress card
  - run summary row
  - empty state
  - loading skeleton

Good accelerators:
- `steady-parent/.../pro-blocks/application/cards/`
- `steady-parent/.../pro-blocks/application/empty-sections/`
- `steady-parent/.../pro-blocks/application/page-headers/`

Rules:
- use fake data and static page-level mocks
- no backend contracts yet

Success condition:
- status UI feels coherent and readable across all main states

### Phase 5. Fake report overview page

Purpose:
- prove the audit/report visual language before any backend wiring

Deliverables:
- mocked report overview page
- cards/sections for:
  - overall score
  - category breakdown
  - issue summary
  - evidence preview
  - report list row / table

Good accelerators:
- `steady-parent/.../pro-blocks/application/application-examples/dashboard-1.tsx`
- `steady-parent/.../pro-blocks/application/table-headers/`
- `steady-parent/.../pro-blocks/application/cards/`

Rules:
- prioritize layout, typography, density, and hierarchy
- not validator correctness

Success condition:
- the report UI demonstrates that the token system supports dense dashboard views, not just marketing blocks

### Phase 6. Design review and cleanup

Purpose:
- stop before real backend wiring and fix the design system where it failed

Deliverables:
- identify missing component tokens
- identify missing semantic tokens
- identify primitives that need wrapper conventions
- identify shell/layout issues on mobile and desktop

Output:
- a short gap list before data wiring starts

## Parallel split

### Track A - Token bridge

Owner:
- one agent

Scope:
- global CSS
- token imports
- Tailwind `@theme` bridge

Done when:
- core semantic tokens are exposed as utilities and used by at least one mock component

### Track B - Primitive layer

Owner:
- one agent

Scope:
- shadcn generation/setup
- primitive styling baseline
- wrapper conventions

Done when:
- the base component set exists and visually matches the token system direction

### Track C - Dashboard shell

Owner:
- one agent

Scope:
- sidebar
- header
- container/layout system
- pro-block extraction and simplification for shell pieces

Done when:
- fake pages can be dropped into the shell without layout changes

### Track D - Mock product screens

Owner:
- one agent

Scope:
- fake crawl status page
- fake report overview page
- adapt selected pro blocks into Websmasher-specific feature components

Done when:
- both pages read as coherent product UI using only mocks

## Dependency order

- Track A and Track B can run first in parallel
- Track C should begin once Track A has enough bridge tokens and Track B has base primitives
- Track D should begin once Track C exists and Track B has the needed primitives

## File direction

Target shape should follow our apparch:

```text
src/
  types/          # mock view-model types allowed here
  logic/          # pure formatting helpers if needed
  components/
    ui/           # shadcn primitives
    dashboard/    # shell pieces
    crawl/        # fake crawl status components
    report/       # fake report components
    shared/       # shared dashboard pieces
  app/
    (dashboard)/  # thin routes using mocked data
```

## Why this order is correct

- `steady-parent` proves the token-first bridge pattern
- `autosmm` proves the dashboard composition pattern
- our own Websmasher plans already say:
  - tokens first
  - shadcn primitives, not Primer React
  - feature-based components
  - thin `app/`

So the correct first coding move is not auth or DB work.
It is:
- make tokens usable
- skin primitives
- build shell
- build two fake product pages

## Exit condition

This phase ends when:
- the token bridge works
- shadcn is proven usable under Websmasher styling
- the shell is stable
- the fake crawl/report screens look like the intended product

Only after that should the project start wiring real auth, real reads, and real workflows into the UI.
