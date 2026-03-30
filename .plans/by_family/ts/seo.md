# TS-SEO

Status: planned family contract, no cohesive family runtime yet, still strongly planning-led.

Implementation roots:

- no dedicated TS SEO validator family in code yet

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/seo.md` as the detailed family ledger until the cutover is complete

Current state:

- SEO/static-route checks remain primarily a planning lane
- this family looks like a capability family layered on top of public-web/content roots, not a standalone owner family

Rule inventory:

- `TS-SEO-01` — sitemap exists where required.
  - What it should do: require sitemap ownership for public web/content apps.
  - What it is for: sitemap generation is a concrete part of the public route surface.
- `TS-SEO-02` — robots policy exists where required.
  - What it should do: require robots ownership/configuration for public web/content apps.
  - What it is for: crawl policy should be explicit, not implicit.
- `TS-SEO-03` — route metadata ownership is explicit.
  - What it should do: require canonical metadata ownership for public page/layout surfaces.
  - What it is for: metadata should not be ad hoc or inconsistently distributed.
- `TS-SEO-04` — static-route/prerender policy is coherent.
  - What it should do: enforce route-surface completeness where static generation/prerendering is part of the product contract.
  - What it is for: public route coverage is an architecture concern, not just a deploy-time accident.
- `TS-SEO-05` — structured-data or other required SEO artifacts exist where the chosen site contract requires them.
  - What it should do: enforce whatever structured SEO artifacts the product shape requires.
  - What it is for: this keeps public SEO commitments explicit and testable.

Current code mapping:

- no dedicated runtime yet

Current doc/code reconciliation notes:

- this family is still purely planning-led
- the old ledger is the detailed source, but it remains broad and product-shape-dependent
- SEO applicability should not be discovered here; it should be routed from `TS-ARCH` and possibly narrowed by `TS-CONTENT`
- the family still needs a clearer split between universal public-web requirements and optional product-shape-specific policy
- unlike the Rust owner families, this family currently has no fail-closed or rooted-input model at all because it lacks a runtime and a routed root contract
- until those rooted inputs exist, this is better understood as a capability/subdomain family than as a peer owner family

Historical/supplemental references:

- `.plans/todo/checks/ts/seo.md`
- `.plans/by_family/rs/arch.md`
- `.plans/by_family/rs/code.md`

Next planning focus:

- define which public web/content apps are in scope and what route/metadata artifacts are authoritative
- decide which SEO surfaces are universal vs only required for content/public-web roots
- make explicit that SEO is a capability family depending on stronger owner families rather than a peer root-owner family
- do not treat this family as implementation-ready until a routed-root applicability model exists
