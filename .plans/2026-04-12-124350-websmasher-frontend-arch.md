# Websmasher frontend architecture decisions

## 1. State management

Three state layers, each with a dedicated tool. No overlap, no conflicts. This combination is production-validated (used by Sentry, Supabase, Vercel, Clerk).

### Server state: TanStack Query v5

Data that lives in the database (reports, crawl status, user profile). TanStack Query provides:
- Client-side cache with automatic refetching
- Polling (crawl status while waiting for results)
- Optimistic updates (delete report, resubmit)
- Mutation lifecycle (loading/error/success states)
- Pagination/infinite scroll (report list as it grows)
- DevTools for debugging

RSC handles initial page load (SSR). TanStack Query takes over on the client for interactivity. Not a replacement for each other - complementary.

### Client UI state: Zustand

UI state that dies on navigation (sidebar open, active tab, filter panel expanded, modal visibility). Zustand provides:
- Selective subscriptions (only re-render components that read changed slice)
- Works outside React (can use in logic/ layer)
- No provider wrapper needed
- ~1KB bundle

Gotcha: stores must be scoped per-request on the server to prevent state leaking between users. Use the "store per request" pattern with React context wrapping.

### URL state: nuqs

State that survives refresh and sharing (report filters, sort order, active category, pagination page). nuqs provides:
- Type-safe search params with Zod integration
- Debouncing and batching
- Server component compatible
- Shareable URLs (someone shares a report link with filters applied, recipient sees same view)

nuqs is the renamed next-usequerystate. No competitor of comparable quality.

## 2. Data fetching & mutations

### Rendering model

App Router merges SSR into server components. No separate SSR strategy to decide.

- **Static** (default) - rendered at build time, cached. Login page, marketing pages.
- **Dynamic** - rendered per-request on the server when using cookies/headers/uncached data. Dashboard, report detail, anything user-specific.
- **Streaming** - dynamic but sends HTML in chunks via Suspense boundaries as data becomes available.

Server components never run on the client. No hydration for them - just HTML. Client components hydrate on top for interactivity.

### Data fetching: RSC + TanStack Query handoff

Server components fetch initial page data directly (call adapters/ on the server). No loading spinners for initial render - HTML arrives complete.

TanStack Query takes over on the client after hydration: polling, refetching, pagination. Server component passes `initialData` to TanStack Query - no double-fetch, seamless handoff.

### Mutations: server actions + TanStack Query

Two mutation mechanisms, used for different complexity levels:

**Server actions via next-safe-action** - simple mutations. Delete report, update settings, toggle preference. next-safe-action wraps server actions with Zod input validation and type-safe error handling. Progressive enhancement (forms work without JS).

**TanStack Query useMutation** - complex mutations. Submit crawl with optimistic UI, retry on failure, rollback on error, multi-step workflows with loading/error/success lifecycle.

Both go through adapters/ - neither server actions nor query functions call the database directly.

### Revalidation

Server actions call `revalidatePath()` or `revalidateTag()` after mutations. TanStack Query invalidates its cache (`queryClient.invalidateQueries()`). Both mechanisms coexist - server action revalidates the RSC cache, TanStack Query refetches client-side queries.

## 3. Component organization

### Structure: feature-based

```
components/
  ui/                # shadcn managed. Never edit. Wrap if customization needed.
  report/            # report viewer components
  dashboard/         # dashboard components
  crawl/             # crawl submission + status
  shared/            # truly shared across 3+ features (page-header, empty-state, loading-skeleton)
```

New product features get a new directory. Components never co-locate in app/ route directories - components/ is the single location for all React components.

### Rules

- Feature components receive props and render. No database calls, no server actions. Can import from types/, logic/, stores/. Cannot import from adapters/.
- shared/ is for components used by 3+ features. Don't preemptively extract - start in the feature directory, move to shared when a third consumer appears.
- Complex components use compound pattern (Report.Header, Report.CheckList) - parent manages shared state via context, children handle specific responsibilities.
- Server components at top of tree, client components pushed down as leaves. Pass server data to client components via props.

### Icons

lucide-react only. No other icon libraries.

## 4. Design system

### Approach: Primer token taxonomy + shadcn components + Tailwind v4

Use GitHub Primer's design token NAMING SCHEME and layer structure. Do NOT use Primer's React components. Use shadcn/ui for components. Bridge tokens to Tailwind v4 via `@theme`.

Primer's taxonomy provides the systematic naming and constraint system. shadcn provides the component library. Tailwind provides the utility generation. We define our own VALUES within Primer's structure.

### Token architecture: three layers + themes

Each layer only references the one below it. No layer contains raw values except base.

```
tokens/
  base/                 # raw scales - the ONLY place raw values exist
    scale.css           # size steps: --base-size-4 through --base-size-128
    palette.css         # color scales: --base-color-blue-0 through --base-color-blue-9, per hue
    typography.css      # font families, font size scale, font weight scale, line-height scale
  semantic/             # purpose-based tokens -> var(--base-*)
    spacing.css         # --stack-gap-*, --control-*-padding*, --overlay-padding-*
    color.css           # --bgColor-*, --fgColor-*, --borderColor-* (Primer roles)
    typography.css      # --text-title-*, --text-body-*, --text-caption-* (bundled size+weight+lh)
    radius.css          # --borderRadius-small/medium/large/full
    shadow.css          # --shadow-resting-*, --shadow-floating-*
    z-index.css         # --zIndex-sticky/dropdown/overlay/modal/popover
  component/            # per-component tokens -> var(--semantic-*)
    button.css          # --button-padding, --button-radius, --button-bgColor-rest/hover/active
    card.css            # --card-padding, --card-radius, --card-bgColor, --card-borderColor
    input.css           # --input-padding, --input-height, --input-bgColor, --input-borderColor
    overlay.css         # --overlay-padding, --overlay-radius, --overlay-bgColor, --overlay-shadow
    (more as needed)
  themes/
    light.css           # overrides semantic color + shadow tokens for light mode
    dark.css            # overrides semantic color + shadow tokens for dark mode
```

### Constraint system

All values are constrained to the base scales via `var()` references:

```css
/* base/scale.css - ~15 legal size values */
:root {
  --base-size-4: 0.25rem;
  --base-size-8: 0.5rem;
  --base-size-12: 0.75rem;
  --base-size-16: 1rem;
  --base-size-24: 1.5rem;
  --base-size-32: 2rem;
  --base-size-40: 2.5rem;
  --base-size-48: 3rem;
}

/* semantic/spacing.css - references base, never raw values */
:root {
  --stack-gap-condensed: var(--base-size-8);
  --stack-gap-normal: var(--base-size-16);
  --stack-gap-spacious: var(--base-size-24);
  --control-medium-paddingBlock: var(--base-size-8);
  --control-medium-paddingInline-normal: var(--base-size-16);
}

/* component/button.css - references semantic, never base */
:root {
  --button-paddingBlock: var(--control-medium-paddingBlock);
  --button-paddingInline: var(--control-medium-paddingInline-normal);
  --button-gap: var(--control-medium-gap);
  --button-radius: var(--borderRadius-medium);
}
```

Nobody can nudge a value by 2px. Every value snaps to the base scale. Changing `--base-size-8` shifts everything referencing it uniformly.

### Color token taxonomy

Merged from Primer (status/interactive roles) + shadcn (surface concepts):

**From Primer (product-agnostic):**
- Background: `--bgColor-default`, `--bgColor-muted`, `--bgColor-inset`, `--bgColor-emphasis`, `--bgColor-inverse`, `--bgColor-disabled`
- Status (each has fg + bgColor-muted + bgColor-emphasis + borderColor): `accent`, `success`, `danger`, `attention`, `severe`, `neutral`
- Foreground: `--fgColor-default`, `--fgColor-muted`, `--fgColor-link`, `--fgColor-disabled`, `--fgColor-onEmphasis`
- Border: `--borderColor-default`, `--borderColor-muted`, `--borderColor-emphasis`, `--borderColor-disabled`
- Interactive control states: `--control-bgColor-rest/hover/active/disabled/selected`
- Focus: `--focus-outlineColor`
- Overlay: `--overlay-bgColor`, `--overlay-borderColor`, `--overlay-backdrop-bgColor`

**From shadcn (surface concepts):**
- `--card-bgColor`, `--card-fgColor`
- `--popover-bgColor`, `--popover-fgColor`
- `--sidebar-bgColor`, `--sidebar-fgColor`
- `--input-bgColor`, `--input-borderColor`

All semantic color tokens reference `var(--base-color-{hue}-{step})` from the palette. Dark mode overrides the semantic tokens, not the base palette.

### Spacing token taxonomy (from Primer)

Three density tiers: condensed / normal / spacious.

- Stack: `--stack-gap-{density}`, `--stack-padding-{density}`
- Controls: `--control-{size}-paddingBlock`, `--control-{size}-paddingInline-{density}`, `--control-{size}-size`, `--control-{size}-gap`
- Control stacks: `--controlStack-{size}-gap-{density}`
- Overlay: `--overlay-padding-{density}`, `--overlay-width-{size}`, `--overlay-height-{size}`

### Typography token taxonomy (from Primer)

Role-based tokens bundling size + weight + line-height:

- `--text-display-*` (hero/marketing)
- `--text-title-*-{large/medium/small}` (page/section headings)
- `--text-subtitle-*` (subheadings)
- `--text-body-*-{large/medium/small}` (body text)
- `--text-caption-*` (small labels, timestamps)

### Tailwind v4 bridge

Map semantic tokens into `@theme` so Tailwind generates utilities:

```css
@theme {
  --spacing-stack-condensed: var(--stack-gap-condensed);
  --spacing-stack-normal: var(--stack-gap-normal);
  --spacing-stack-spacious: var(--stack-gap-spacious);
  --color-bgColor-default: var(--bgColor-default);
  --color-fgColor-default: var(--fgColor-default);
  /* etc. */
}
```

Components use semantic Tailwind utilities: `gap-stack-normal`, `bg-bgColor-muted`, `text-fgColor-default`.

### Rebranding surface

To change the product's visual identity:
- **Different brand colors**: replace `base/palette.css` (~10 hues x 10 steps). All semantic and component tokens shift via `var()`.
- **Different density/spacing**: replace `base/scale.css` (~15 values). All spacing shifts uniformly.
- **Different typography**: replace `base/typography.css` (font family, size scale).
- Semantic and component layers untouched.

### Dark mode

`next-themes` for switching (class-based). Theme files override semantic color tokens only. Spacing, typography, radius don't change between themes.

### Component styling rules

- shadcn components are the primitive layer. Never edit `components/ui/` directly.
- Wrap shadcn components to apply component-layer tokens.
- `cn()` helper (clsx + tailwind-merge) for conditional classes.
- Components reference component-layer tokens first, semantic tokens where no component token exists. Never raw Tailwind values.
- lucide-react only for icons.

## 5. Accessibility

### Foundation: shadcn/Radix

shadcn is built on Radix primitives which implement full WAI-ARIA spec: proper roles, keyboard navigation, focus management, screen reader support. This covers most components out of the box.

### Rules

- Don't break what Radix gives you - incorrect composition breaks keyboard flow
- All interactive elements must be keyboard accessible
- Focus management on route changes (Next.js built-in route announcer + useFocusOnNavigation)
- Color contrast: WCAG 2.1 AA minimum
- Skip links for navigation bypass

### Testing

axe-core + axe DevTools browser extension (Deque) for automated a11y auditing. Run as part of development workflow, not just CI.
