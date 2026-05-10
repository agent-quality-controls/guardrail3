## Goal

Plan the TypeScript-side migration to the same package shape and app wiring model now used for Rust:

- one package group per family of checks
- family packages own types, ingestion, and pure checks
- the active app wires those families into validation
- nothing else is in scope for the first pass

This is a planning document.
It is not a final architecture decision record.

## Scope for this plan

In scope:

- TypeScript family package split
- TypeScript family wiring into the active validation app
- validation-only migration
- canonical family boundaries
- current-code-to-target-family mapping
- recommended migration order

Out of scope for this plan:

- TypeScript generation
- TypeScript init
- TypeScript diff / write path
- deploy family work
- hook generation changes

## Current state

- The active app is `apps/guardrail3-rs`.
- Its live CLI currently only exposes Rust `validate`.
- The old grouped TypeScript implementation lives under:
  - `legacy/apps/guardrail3-current/crates/app/ts/validate`
- The canonical TS family inventory already exists under:
  - `.plans/todo/checks/ts`

Relevant current sources:

- `.plans/todo/checks/ts/README.md`
- `.plans/todo/checks/ts/eslint.md`
- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/todo/checks/ts/package.md`
- `.plans/todo/checks/ts/content.md`
- `legacy/apps/guardrail3-current/crates/app/ts/validate`

## Guiding direction

Mirror the Rust migration strategy as closely as possible:

- family packages, not grouped validator buckets
- typed boundaries between packages
- ingestion owns discovery and parse-once work
- check packages own pure rules
- the active app owns orchestration only

For the first TS pass, validation is the only required product surface.

That means:

- do not rebuild TypeScript generation first
- do not revive the old monolithic `app/ts/validate` runtime
- do not mix validation migration with write-path design

## Canonical TypeScript family set

Use the canonical set already defined in `.plans/todo/checks/ts/README.md`:

1. `ts/arch`
2. `ts/eslint`
3. `ts/tsconfig`
4. `ts/npmrc`
5. `ts/package`
6. `ts/fmt`
7. `ts/spelling`
8. `ts/typecov`
9. `ts/size`
10. `ts/jscpd`
11. `ts/css`
12. `ts/code`
13. `ts/hexarch`
14. `ts/libarch`
15. `ts/content`
16. `ts/i18n`
17. `ts/seo`
18. `ts/tests`

These should be treated as the migration target.

## Target package shape

For each TS family, use the same general package shape used for Rust:

- `g3ts-<family>-types`
- `g3ts-<family>-ingestion`
- one or more check packages for pure rules, depending on lane split

Likely lane split:

- `-config-checks`
- `-source-checks`
- `-filetree-checks`

But only create the lanes that are actually justified by the family.

Examples:

- `ts/eslint`
  - likely `types`
  - `ingestion`
  - `config-checks`
- `ts/code`
  - likely `types`
  - `ingestion`
  - `source-checks`
- `ts/arch`
  - likely `types`
  - `ingestion`
  - `filetree-checks`
- `ts/content`
  - may need both `config-checks` and `source-checks`

Do not force every family into every lane if the lane is fake.

## App wiring target

The active app should gain TS validation support the same way Rust is wired today:

- app-level family enum includes TS families
- app-level family runner dispatches into TS family runners
- validation request can select TS families
- report renderer stays generic

The app should not own TS rule logic.

It should only own:

- family selection
- orchestration
- crawl handoff
- final rendering

## Latest ideas to carry over from Rust

These are current working ideas, not final decisions.

### 1. Validation-only first

Do exactly what worked for Rust:

- package families first
- active app wiring second
- generation later

### 2. Typed file boundaries

Use the same config-boundary preference we settled on for Rust:

- parse once
- type once
- pass whole typed config files across family boundaries
- do not centrally slice config files into per-family mini-views

Examples for TS:

- `eslint.config.*`
- `tsconfig*.json`
- `.npmrc`
- `.jscpd.json`
- stylelint config
- `package.json`

Inside a family, rules can still receive smaller local typed inputs.

### 3. Shared config ownership must be explicit

Some files are shared across multiple TS families.

Most importantly:

- `package.json`

The family plans already imply explicit key-level ownership.

That should stay the rule:

- `ts/package` owns generic package policy
- `ts/eslint` owns eslint plugin package presence
- `ts/fmt` owns formatter package presence
- `ts/spelling` owns cspell package presence
- `ts/typecov` owns type-coverage package presence
- `ts/size` owns size-budget package presence
- `ts/css` owns stylelint-related package presence

Do not let `ts/package` become the dumping ground for every tool policy.

## Legacy implementation map to normalize

The current grouped implementation under `legacy/apps/guardrail3-current/crates/app/ts/validate` should be treated as input material only.

Current grouped surfaces:

- `eslint/`
  - target mostly `ts/eslint`
  - architecture-specific pieces move to `ts/hexarch`
- `packages/config_files.rs`
  - currently mixes:
    - `ts/eslint`
    - `ts/tsconfig`
    - `ts/npmrc`
    - `ts/package`
    - `ts/jscpd`
- `packages/package_deps.rs`
  - currently mixes:
    - `ts/eslint`
    - `ts/fmt`
    - `ts/spelling`
    - `ts/typecov`
    - `ts/size`
    - `ts/css`
    - `ts/jscpd`
- `packages/tool_config_checks.rs`
  - currently mixes:
    - `ts/fmt`
    - `ts/spelling`
    - `ts/typecov`
    - `ts/size`
- `packages/stylelint_check.rs`
  - target `ts/css`
- `packages/jscpd_check.rs`
  - target `ts/jscpd`
  - content-specific logic moves to `ts/content`
- `packages/i18n_check.rs`
  - target `ts/i18n`
- `source/source_scan.rs`
  - target `ts/code`
- `source/ts_comment_checks.rs`
  - target `ts/code`
- `source/ts_code_analysis.rs`
  - target `ts/code`
- `source/test_checks.rs`
  - target `ts/tests`
- `topology/ts_topology_checks.rs`
  - split into:
    - `ts/arch`
    - `ts/hexarch`
    - `ts/libarch`
    - possibly content-root ownership pieces to `ts/content`

## Root ownership model to preserve

Carry forward the root model already established in the TS family README:

- validation root
- package-manager root
- TS package/app root
- local config root

This matters because TS is more local-config-driven than Rust.

Families that should likely use nearest-local-config ownership:

- `ts/eslint`
- `ts/tsconfig`
- `ts/fmt`
- `ts/spelling`
- `ts/typecov`
- `ts/size`
- `ts/jscpd`
- `ts/css`

Families that should not be nearest-config-wins:

- `ts/package`
- `ts/code`
- `ts/hexarch`
- `ts/libarch`
- `ts/content`
- `ts/i18n`
- `ts/seo`
- `ts/tests`

## Recommended package groups

This is a likely target, not a forced final crate map.

### Group 1 - Config-policy families

- `ts/eslint`
- `ts/tsconfig`
- `ts/npmrc`
- `ts/package`
- `ts/fmt`
- `ts/spelling`
- `ts/typecov`
- `ts/size`
- `ts/jscpd`
- `ts/css`

### Group 2 - Source / architecture families

- `ts/code`
- `ts/arch`
- `ts/hexarch`
- `ts/libarch`
- `ts/tests`

### Group 3 - Product-shape families

- `ts/content`
- `ts/i18n`
- `ts/seo`

## Recommended migration order

Do not migrate all 18 families at once.

Recommended order:

1. `ts/package`
2. `ts/npmrc`
3. `ts/tsconfig`
4. `ts/eslint`
5. `ts/fmt`
6. `ts/jscpd`
7. `ts/css`
8. `ts/code`
9. `ts/tests`
10. `ts/arch`
11. `ts/libarch`
12. `ts/hexarch`
13. `ts/content`
14. `ts/i18n`
15. `ts/seo`
16. `ts/spelling`
17. `ts/typecov`
18. `ts/size`

Rationale:

- start with config and package-policy surfaces
- prove local-config ownership and package-key ownership early
- move generic source checks before architectural TS families
- leave product-specific site families until the generic substrate exists
- leave the weaker / tool-driven families until the main package architecture is proven

## First vertical slice

Best first slice:

- `ts/package`
- `ts/npmrc`
- `ts/tsconfig`
- `ts/eslint`

Why:

- these cover most of the current config-policy substrate
- they force us to solve:
  - root ownership
  - local config ownership
  - `package.json` key ownership
  - typed file handoff
  - active app TS-family routing

If these four work cleanly, the rest of the TS family migration gets much easier.

## App changes likely needed

Not implementing these yet, but this is the likely app work:

- extend app family enum to include TS families
- add TS family runner groups alongside current Rust runner groups
- teach the workspace crawler / project snapshot how to expose the TS discovery state needed by TS ingestions
- make the CLI family filter support both Rust and TS families cleanly

Open question:

- whether TS validation lands in the existing `guardrail3-rs` binary temporarily
- or whether the app should be renamed again once it truly becomes mixed Rust + TS validation

Current practical recommendation:

- keep the active binary and app as-is for now
- wire TS families into the current app first
- revisit naming only after TS validation is actually migrated

## Non-final design questions

These are still open:

1. Do TS packages live under `packages/ts/` mirroring `packages/rs/`, or under another top-level grouping?
2. Should TS family package names mirror the Rust naming style exactly, such as `g3ts-...`?
3. Do we create one shared TS discovery/crawl package before any family migration, or let the first family ingest directly from the current app snapshot?
4. Which current TS checks should be deleted rather than migrated?
5. Should `ts/content`, `ts/i18n`, and `ts/seo` move together as one product-site wave, or separately?
6. What is the final active product name once the app validates both Rust and TS again?

## Concrete next planning steps

1. Decide the TS package namespace and top-level directory layout.
2. Write one detailed family-package migration plan for:
   - `ts/package`
3. Write one detailed family-package migration plan for:
   - `ts/tsconfig`
4. Write one detailed family-package migration plan for:
   - `ts/eslint`
5. Write the app-wiring plan for adding TS family routing into the active validation app.

## Files to modify later

Likely next anchors:

- `.plans/todo/checks/ts/README.md`
- `apps/guardrail3-rs/crates/types/app-types/...`
- `apps/guardrail3-rs/crates/io/inbound/cli/...`
- new TS family packages under a future TS package root
- old grouped TS code under `legacy/apps/guardrail3-current/crates/app/ts/validate/**` as migration input only
