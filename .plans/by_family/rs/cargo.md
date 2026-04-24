# RS-CARGO

Status: current, implemented, self-hosted.

Implementation roots:

- `packages/rs/cargo/g3rs-cargo-types`
- `packages/rs/cargo/g3rs-cargo-ingestion`
- `packages/rs/cargo/g3rs-cargo-config-checks`
- `packages/rs/cargo/g3rs-cargo-filetree-checks`

Current source of truth:

- this file for current family status
- `packages/rs/cargo/g3rs-cargo-config-checks/README.md`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/README.md`
- `packages/rs/cargo/g3rs-cargo-ingestion/README.md`

Current implementation shape:

- root-scoped cargo family
- owns the root `Cargo.toml` selected from the crawl root
- that root manifest may be:
  - a workspace root
  - a standalone package root
  - another Cargo manifest shape
- if the root manifest is a workspace root, ingestion also collects workspace member manifests by:
  - expanding `[workspace].members` literals and globs
  - applying `[workspace].exclude`
  - normalizing `./` and slash noise
  - deduplicating matched members
  - surfacing invalid member or exclude patterns as ingestion failures
- if the root manifest is not a workspace root, member ingestion stands down
- source lane is not implemented

Current status:

- config checks implemented
- filetree checks implemented
- self-hosted package validation is green
- source checks intentionally return `SourceIngestionNotImplemented`

Current risk:

- docs were previously stale and pointed at non-existent `apps/guardrail3/.../families/cargo` paths
- older historical cargo notes still describe broader multi-root routing models that do not match the extracted package code
- ingestion TODO still calls out one open hardening area:
  - deeper adversarial coverage for malformed workspace member-pattern edges

Done means:

- family docs describe the extracted `packages/rs/cargo` implementation exactly
- package READMEs agree on the root-scoped policy model
- no active doc claims cargo lives under a non-existent app-local family path

Historical/supplemental references:

- `.plans/todo/checks/rs/cargo.md`
  - keep as detailed historical rule ledger and migration reference

Next planning focus:

- keep the docs aligned if cargo moves back to a wider routed family model later
- if routing broadens again, update this file only after the extracted package code changes
- separately harden malformed workspace-member-pattern coverage in ingestion tests
