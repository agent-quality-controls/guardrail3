# TS-CONTENT

Status: planned family contract, partial legacy implementation only.

Implementation roots:

- content-specific portions of `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs`
- content gating in `apps/guardrail3/crates/app/ts/validate/mod.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/content.md` as the detailed family ledger until the cutover is complete

Current state:

- content-specific ideas exist in planning and some mixed runtime logic
- no cohesive family runtime exists yet

Rule inventory:

- `TS-CONTENT-01` — content apps are identified explicitly.
  - What it should do: detect content-app roots from config or explicit root typing.
  - What it is for: content-specific rules should not fire on unrelated service/library roots.
- `TS-CONTENT-02` — a content pipeline is configured.
  - What it should do: require a configured content system such as Velite or Contentlayer when the root is a content app.
  - What it is for: content apps should not have ad hoc untyped content ingestion.
- `TS-CONTENT-03` — content directories and generated artifacts live in canonical places.
  - What it should do: enforce canonical content roots and generated artifact ownership.
  - What it is for: this prevents content data and generated outputs from scattering across the app.
- `TS-CONTENT-04` — content schema/model ownership is explicit.
  - What it should do: require a clear content-model/schema surface for content roots.
  - What it is for: content should not be “just blobs of markdown/json” without owned structure.
- `TS-CONTENT-05` — content import/use boundaries are respected.
  - What it should do: enforce content-only structural restrictions, such as safe import boundaries for content apps.
  - What it is for: content apps should not quietly accumulate service/database behavior through unowned imports.

Current mixed code mapping:

- content-specific portions of `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs`
  - currently carry `T60` content import restriction
  - currently carry `T61` velite config existence
- content gating in `apps/guardrail3/crates/app/ts/validate/mod.rs`
  - currently influences whether certain mixed checks run at all

Current doc/code reconciliation notes:

- this family is still mostly planning-led
- the clearest live content checks are currently stuck in the wrong runtime file (`jscpd_check.rs`)
- content ownership still overlaps with `ts/i18n` and `ts/seo`, so the family boundary should be kept explicit while reconciling

Historical/supplemental references:

- `.plans/todo/checks/ts/content.md`

Next planning focus:

- separate content-pipeline rules from duplication and generic site checks
- decide whether content app discovery belongs here or in the future shared TS arch layer
