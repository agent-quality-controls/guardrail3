Goal
- Compare Astro against Next.js + Velite for a complex content website in exact technical terms.
- Focus on what Astro can do that the current stack cannot do directly or cannot do as cleanly.
- Make the decision criteria explicit for your own apps.

Current comparison target
- Astro:
  - Astro framework
  - Astro content collections
  - Astro loaders
  - Astro islands
  - Astro SSR/actions/endpoints/sessions/i18n
- Next stack:
  - Next.js App Router
  - Velite for typed content collections
  - custom route helpers and loader utilities
  - React components shared across landing/app/report surfaces

What Astro gives exactly

1. One content system for both local and remote content
- Astro:
  - local content can be loaded through collection loaders such as `glob()` and `file()`
  - remote or custom sources can be loaded through the Content Loader API
  - fresh request-time content can use live collections
  - all of these still live behind the content collection API
- Next.js + Velite:
  - Velite covers local content collections well
  - for non-Velite sources, you usually write separate app-specific loaders
  - those loaders are not automatically part of the same collection/query/render abstraction
- Exact difference:
  - Astro has one framework content abstraction for:
    - local files
    - remote content
    - custom loaders
    - live request-time entries
  - Next + Velite has:
    - Velite for local typed content
    - separate custom code for alternate sources
- Why that matters:
  - it reduces side-channel content utilities like the current `kit-taxonomy.ts`
  - alternate content sources are less likely to become random libs with ad hoc parsing

2. Framework-owned content query API
- Astro:
  - `getCollection()`
  - `getEntry()`
  - `getEntries()`
  - `getLiveCollection()`
  - `getLiveEntry()`
  - `render()`
- Next.js + Velite:
  - generated data in `.velite`
  - app code reads arrays/objects directly
  - app usually adds its own helpers such as `getVisiblePosts()`
- Exact difference:
  - Astro gives one standard query surface
  - Next + Velite gives generated data, then your app invents the rest
- Why that matters:
  - fewer app-specific content access patterns
  - easier to enforce one standard content pipeline across sites

3. Framework-owned content render path
- Astro:
  - content entries can be rendered through `render(entry)`
  - that returns `<Content />`
  - rendered content stays inside the framework content API
- Next.js + Velite:
  - `s.mdx()` compiles MDX to a function-body string
  - app code renders it by executing that code manually
  - current landing app does this in:
    - `src/lib/mdx-content.tsx`
    - `src/lib/blog-mdx-content.tsx`
- Exact difference:
  - Astro owns the render primitive
  - Next + Velite makes the app own the render bridge
- Why that matters:
  - fewer custom MDX runtime wrappers
  - less room for per-app divergence in how MDX is rendered

4. Static-first rendering model with explicit interactivity
- Astro:
  - pages render to HTML by default
  - interactivity is opt-in through `client:*` directives
  - server-delayed dynamic regions can use server islands
- Next.js:
  - can render static content fine
  - but the framework model is not "no client JS unless explicit island"
  - it is a React application model first
- Exact difference:
  - Astro is stricter about accidental client-JS growth on content pages
  - Next gives you more freedom, but less automatic discipline
- Why that matters:
  - content sites tend to rot into over-hydrated React apps
  - Astro fights that by default

5. SSR support
- Astro:
  - yes, via on-demand rendering with adapters
  - pages and endpoints can render per request
- Next.js:
  - yes, natively
- Exact difference:
  - neither stack wins on "can it SSR"
  - Astro does not lose here

6. API route support
- Astro:
  - endpoints can serve JSON, images, RSS, API responses
  - in SSR mode they behave as live server endpoints
- Next.js:
  - route handlers / API routes
- Exact difference:
  - both can implement API surfaces
  - no meaningful winner for this use case

7. Typed mutation/form handling
- Astro:
  - Actions API
  - Zod-validated inputs at the action boundary
  - callable from forms or client-side code
- Next.js:
  - Server Actions and route handlers
  - validation is up to your app
- Exact difference:
  - Astro gives a clearer schema-first mutation boundary out of the box
  - Next gives tighter React integration with the App Router model
- Why that matters:
  - if you want content-site forms with typed inputs and light server logic, Astro is clean
  - if you want app-style mutation flows tightly inside React server/client composition, Next is stronger

8. Sessions
- Astro:
  - framework sessions API exists
  - available in pages, endpoints, middleware, and actions
- Next.js:
  - sessions are usually solved through app code and libraries
- Exact difference:
  - Astro ships a more explicit framework-level session surface
  - Next relies more on ecosystem patterns

9. i18n
- Astro:
  - built-in i18n routing support and middleware composition
- Next.js:
  - built-in routing pieces plus common libraries like `next-intl`
  - your current landing app already uses `next-intl`
- Exact difference:
  - Astro is more directly opinionated here
  - Next is flexible, but usually more library-driven

What Astro does not give you better

1. Shared React application runtime
- Astro:
  - can run React components
  - cannot give you Next App Router semantics
  - cannot give you Next Server Components architecture
  - cannot give you Next Server Actions architecture
- Next.js:
  - native platform for those things
- Exact difference:
  - if you need shared runtime architecture across landing + app + reports, Astro is weaker

2. Shared route/runtime conventions across products
- Astro:
  - shared presentational React components are fine
  - shared platform-level patterns are not the same
- Next.js:
  - all surfaces can use the same route/runtime model
- Exact difference:
  - Astro shares UI
  - Next can share both UI and platform semantics

3. Advanced app-style rendering/caching stack
- Astro:
  - has SSR
  - has route caching, but the docs currently mark it experimental/beta
- Next.js:
  - App Router
  - Server Components
  - Server Actions
  - ISR
  - PPR
  - cache components
- Exact difference:
  - for content sites this may not matter much
  - for hybrid site-app systems it can matter a lot

What Next.js + Velite can do today that overlaps Astro

1. Local typed content collections
- Yes.
- Velite already gives:
  - local file discovery by pattern
  - Zod schema validation
  - generated typed output
  - MDX support
- So Astro is not winning on "can we have typed local content"
- It wins on how unified the whole content layer is beyond that

2. MDX content pages
- Yes.
- Current landing app already does this through Velite + `BlogMDXContent`.
- Astro is cleaner here because the framework owns the render primitive, but Next + Velite is already capable.

3. SSR pages and endpoints
- Yes.
- Current stack already has this.

4. React component reuse
- Yes.
- Current stack is stronger here because React is native, not an integration layer.

Exact decision criteria

Choose Astro if these are true
- The site is primarily a content property, not a member of the same app runtime as the rest of your product.
- You want one framework-owned content abstraction for:
  - local content
  - remote content
  - custom loaders
  - rendering
- You want the framework to push the site toward:
  - static HTML first
  - explicit interactive islands only
- You are willing to share React components but not framework/runtime semantics.

Choose Next.js + Velite if these are true
- The site is part of the same React platform as other product surfaces.
- You want to share not only UI components, but also runtime conventions.
- The main problem is enforcement, not framework capability.
- You are willing to build and enforce a stricter content architecture with guardrails.

Applied to your current landing app
- The real current problems are:
  - raw alternate content loader in `src/lib/kit-taxonomy.ts`
  - route-local content shaping and casts
  - duplicated static page wrappers
  - weak enforcement of the typed pipeline boundary
- Those are not proof that Next.js + Velite is insufficient.
- They are proof that the content architecture is not enforced hard enough.

Bottom line
- Astro is technically better at being a content-site framework because it gives:
  - one unified collection/loader/query/render model
  - stricter static-first/islands rendering discipline
- Next.js + Velite is technically better if the content site is part of a broader shared React application platform.
- For your setup, the deciding factor is not "can Astro do SSR or React"
- The deciding factor is:
  - do you want a separate content-site platform
  - or one shared React platform with stronger content guardrails

Primary sources
- Astro content collections:
  - https://docs.astro.build/en/guides/content-collections/
  - https://docs.astro.build/en/reference/content-loader-reference/
  - https://docs.astro.build/en/reference/modules/astro-content/
- Astro SSR/actions/sessions/i18n/react:
  - https://docs.astro.build/en/guides/on-demand-rendering/
  - https://docs.astro.build/en/guides/endpoints/
  - https://docs.astro.build/en/guides/actions/
  - https://docs.astro.build/en/guides/sessions/
  - https://docs.astro.build/en/guides/internationalization/
  - https://docs.astro.build/en/guides/integrations-guide/react/
  - https://v4.docs.astro.build/en/concepts/islands/
- Next.js:
  - https://nextjs.org/docs/app
  - https://nextjs.org/docs/app/guides/forms
  - https://nextjs.org/docs/app/guides/deploying-to-platforms
  - https://nextjs.org/docs/pages/guides/incremental-static-regeneration
- Velite:
  - https://velite.js.org/guide/define-collections
  - https://velite.js.org/guide/using-mdx
  - https://velite.js.org/guide/custom-loader
