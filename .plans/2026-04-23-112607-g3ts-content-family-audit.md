Goal
- Design a `g3ts` content family for landing apps that forces a typed content pipeline and prevents route/component code from becoming the real content source of truth.
- Base the family on the real implementation in `/Users/tartakovsky/Projects/steady-parent/apps/landing`, not the earlier prototype.

Actual landing shape
- The app already has a real typed content pipeline:
  - `velite.config.ts` defines collections for `posts`, `pages`, `courses`, `quizzes`, `landing`, `blocks`, index pages, and blog categories.
  - content is authored under `/content/*`.
  - routes mostly consume generated data from `@/.velite`.
  - longform page bodies use Velite-compiled MDX rendered through `BlogMDXContent`.
- The app also has real bypasses and weak spots:
  - freebies bypass Velite and load raw taxonomy JSON through `src/lib/kit-taxonomy.ts`.
  - route files still manufacture page-level content locally with casts and adapter code:
    - homepage builds `LandingContent` in `src/app/[locale]/(public)/page.tsx`
    - course page casts `blocks.find(...).data as AuthorityContent`
  - static MDX pages are duplicated as one route per slug:
    - `about/page.tsx`
    - `privacy/page.tsx`
    - `terms/page.tsx`
    instead of one generic typed page route over the `pages` collection.
  - route files still contain content-like literals for SEO and form copy:
    - blog index description string
    - quiz gate defaults
    - freebie subscribe copy
    - category freebie CTA template string
- There is also doc drift:
  - `apps/landing/CLAUDE.md` says content lives in `src/content/` dirs, but authored content actually lives in top-level `content/`.

Desired architecture to enforce
- Content authoring lives in `content/**` only.
- `velite.config.ts` is the schema source of truth for authored landing content.
- Routes consume generated typed content from `@/.velite` or from narrow typed content adapters built outside the route layer.
- Route files should compose content, not author it.
- Non-Velite content sources are allowed only when explicitly declared as a second content source with typed loader + schema + ownership boundary.
- Repeated slug-based static page wrappers should collapse to a generic typed page pipeline.

Family shape
- Family name
  - `content`
- Target
  - `g3ts` landing/content-heavy Next.js apps
- Lanes
  - `filetree`
    - required content roots and generated output wiring
  - `config`
    - Velite presence and collection/schema coverage
  - `source`
    - route/component usage of typed content
  - `pipeline`
    - non-Velite content sources, typed loader requirements, and allowed import boundaries

What the family should consider compliant
- Route imports content from `@/.velite`.
- Route imports typed adapter modules from a dedicated content adapter layer.
- MDX pages render Velite-generated code from typed page/post records.
- Non-Velite loader modules are allowed only if they:
  - own runtime validation
  - expose typed return values
  - are used through a dedicated content module boundary

What the family should consider non-compliant
- Route/component imports raw authored files directly from `content/**`.
- Route/component reads filesystem content directly.
- Route/component builds longform page copy inline instead of reading typed content.
- Route/component performs repeated ad hoc casts from heterogeneous content records.
- Content-bearing loader reads external JSON without schema validation.
- One route file per static MDX slug when the app already has a typed `pages` collection.

Initial rule inventory
- `CONTENT-FILETREE-01`
  - landing app must have `velite.config.*`
  - Why: there must be a declared typed content pipeline.
- `CONTENT-FILETREE-02`
  - landing app must have authored content root `content/`
  - Why: separate authored content from route/component code.
- `CONTENT-CONFIG-01`
  - `velite.config.*` must define collections for every authored content lane actually consumed by routes:
    - posts
    - pages
    - landing
    - courses
    - quizzes
    - index/catalog pages used by routes
  - Why: no shadow content pipeline in code.
- `CONTENT-CONFIG-02`
  - every MDX/JSON page collection must include typed page metadata fields:
    - title
    - description
    - keywords
    - ogImage
  - Why: content pages must be SEO-complete at the schema layer.
- `CONTENT-SOURCE-01`
  - public route files must not import raw content from `content/**`
  - Allowed:
    - `@/.velite`
    - dedicated typed loader modules
- `CONTENT-SOURCE-02`
  - public route files must not read content via `fs`, `path`, or raw JSON loads
  - Why: routes should consume pipeline output, not become loaders.
- `CONTENT-SOURCE-03`
  - longform page routes must render typed content bodies from collection records, not inline large prose in JSX
  - Why: route code should not become the content store.
- `CONTENT-SOURCE-04`
  - slug-keyed static page routes must not be duplicated when a typed `pages` collection exists
  - This should flag the current `about/privacy/terms` wrapper pattern and require one generic page pipeline.
- `CONTENT-SOURCE-05`
  - route files must not perform repeated ad hoc casts from heterogeneous content blobs such as `block.data as X`
  - Require typed adapter boundary outside the route layer.
- `CONTENT-PIPELINE-01`
  - non-Velite content loaders must have explicit runtime validation at the loader boundary
  - This should flag the current `src/lib/kit-taxonomy.ts` raw `JSON.parse(...) as TaxonomyData` path.
- `CONTENT-PIPELINE-02`
  - non-Velite content sources used in routes must be declared content modules, not arbitrary util libs
  - Why: make alternate content sources explicit and guardrailed.
- `CONTENT-PIPELINE-03`
  - route-level CTA/form copy with content semantics should come from typed content records, not hardcoded literals, when the page already has a content record
  - This should catch patterns like category freebie CTA strings and quiz gate default copy.

Rules to avoid
- Do not require Velite specifically for every app.
  - The actual rule is typed content pipeline, not one library brand.
- Do not ban all UI strings in code.
  - Short interaction labels and control text can stay in UI string modules.
  - The family should target content semantics, not every literal.
- Do not make ingestion understand every page design variant.
  - The family should reason about boundaries and pipeline ownership, not page aesthetics.

Implementation approach in guardrail3
- Start with a new TS family under the `g3ts` side, not in the Rust families.
- First build a narrow landing-app-oriented family rather than a generic all-websites content checker.
- Implement in this order:
  1. filetree rules for presence of content pipeline roots
  2. source rules for route import boundaries and raw-content bypasses
  3. pipeline rules for external content loaders
  4. structural rules for duplicated static page wrappers
  5. stronger semantic rules for route-local content authoring and ad hoc casts

Key design decisions
- Use the actual landing app as the reference architecture, but enforce the improved version, not the current messy state.
- Treat `@/.velite` as the happy path and alternate typed loaders as an explicit exception path.
- Scope the first version to public route files and content loader modules. Do not start by auditing every component.
- Make the first wave boundary-focused and mechanically detectable. Leave deeper semantic "this JSX prose is too content-like" heuristics for a later wave.

Files used for this audit
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/velite.config.ts`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/content/velite-schemas.ts`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/content/velite-prepare.ts`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/blog/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/blog/[category]/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/blog/[category]/[slug]/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/course/[slug]/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/quiz/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/quiz/[slug]/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/about/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/privacy/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/terms/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/freebies/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/app/[locale]/(public)/freebies/[slug]/page.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/lib/posts.ts`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/lib/blog-mdx-content.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/lib/mdx-content.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/lib/kit-taxonomy.ts`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/lib/ui-strings.ts`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/components/blog/freebie-mailing-form.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/src/components/freebie/freebie-subscribe-form.tsx`
- `/Users/tartakovsky/Projects/steady-parent/apps/landing/CLAUDE.md`

Next steps
- Freeze the first-wave rule list around boundary violations only.
- Then scaffold the `g3ts content` family from the filetree + source import rules first.
