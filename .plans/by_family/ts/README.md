# TypeScript Family Plan Surface

This directory is the start of the TypeScript family cutover.

It does not replace the older TypeScript ledgers yet.
Right now, detailed TypeScript family material is still split across:

- `.plans/todo/checks/ts/`
- `.plans/by_file/ts/`
- `.plans/by_file/tools/`
- `.plans/per-app-config-design/`

Current authority order for TypeScript in this transition state:

1. live code under `apps/guardrail3/crates/app/ts/`
2. this directory for family-level indexing and reconciliation
3. `.plans/todo/checks/ts/*.md` as the detailed family ledgers
4. `.plans/by_file/**` and `.plans/per-app-config-design/**` as supporting research/design material

Canonical TypeScript families:

- `arch`
- `eslint`
- `tsconfig`
- `npmrc`
- `package`
- `fmt`
- `spelling`
- `typecov`
- `size`
- `jscpd`
- `css`
- `code`
- `hexarch`
- `libarch`
- `content`
- `i18n`
- `seo`
- `tests`

The intended end state is the same shape as Rust:

- one family-indexed current planning surface here
- old plan/research docs re-labeled as tactical or historical

For this first TS pass, each family gets a placeholder status file here.
Those files now carry family-level rule inventories and current code mapping, while still pointing back to the older detailed ledgers for legacy detail.

Current reconciliation state:

- every canonical TS family now has a by-family file with a rule inventory
- the old files under `.plans/todo/checks/ts/*.md` still remain the detailed ledgers
- the next cleanup step is to add superseded banners to those old TS ledgers one family at a time, after each family summary is considered stable enough
- the first Rust-vs-TS design comparison pass is complete for:
  - `arch`
  - `eslint`
  - `tsconfig`
  - `npmrc`
  - `package`
- current trust ordering for those first five families is:
  - `npmrc`: closest to a clean narrow family
  - `package`: useful but still boundary-mixed
  - `eslint`: strong rule surface but too much mixed ownership
  - `tsconfig`: materially under-split versus Rust `toolchain` plus `cargo`
  - `arch`: clearly behind Rust and still missing the shared root/placement contract
