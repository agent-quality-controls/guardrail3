Goal

Design `guardrail3-ts` as the smallest possible separate CLI app for TypeScript validation:

- separate from `g3rs`
- validate only
- mirrors the live `guardrail3-rs` app shape as closely as possible
- wires only TS families
- no generation, no init, no write path, no cross-language multiplexing

Current source basis

- `apps/guardrail3-rs/Cargo.toml`
- `apps/guardrail3-rs/crates/types/app-types/**`
- `apps/guardrail3-rs/crates/logic/family-runner-*/**`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/{runtime,assertions}/**`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/{runtime,assertions}/**`
- `apps/guardrail3-rs/crates/io/outbound/packages/**`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/{runtime,assertions}/**`

Desired end state

- `apps/guardrail3-ts` exists as a standalone Rust workspace
- binary name:
  - `g3ts`
- command surface:
  - `g3ts validate --path <PATH>`
  - optional:
    - `--family`
    - `--inventory`
- TS family packages are the only runtime dependencies
- shared infrastructure reused only where it is already language-neutral:
  - `g3-workspace-crawl`
  - `guardrail3-check-types`

App structure

Mirror the Rust app exactly unless a crate would be empty or purely redundant.

Workspace root:

- `apps/guardrail3-ts/Cargo.toml`

Types:

- `crates/types/app-types`
  - `errors.rs`
  - `report.rs`
  - `request.rs`
  - `supported_family.rs`
  - `traits.rs`

Logic:

- `crates/logic/validate-command/crates/runtime`
- `crates/logic/validate-command/crates/assertions`

Runners:

- `crates/logic/family-runner-config`
- `crates/logic/family-runner-quality`

IO:

- `crates/io/inbound/cli/crates/runtime`
- `crates/io/inbound/cli/crates/assertions`
- `crates/io/outbound/packages`
- `crates/io/outbound/report/crates/runtime`
- `crates/io/outbound/report/crates/assertions`

Why only two runner groups

`guardrail3-rs` has many runner groups because it already has many Rust families with distinct lanes.

For the minimal TS app we do not need to pre-copy all of that grouping.

Initial TS grouping should be:

- `family-runner-config`
  - `eslint`
  - `tsconfig`
  - `npmrc`
  - `package`
- `family-runner-quality`
  - reserved for later TS families such as:
    - `code`
    - `tests`
    - `arch`
    - `libarch`
    - `hexarch`
    - `content`
    - `i18n`
    - `seo`

This still mirrors the Rust app style:

- typed family enum
- bounded runner groups
- shared validate command
- CLI adapter

It just does not create empty runner crates for unused lanes.

Supported families v1

Only families that exist should be wired.

Phase 1:

- `eslint`

Phase 2:

- `tsconfig`
- `npmrc`
- `package`

The app should not expose CLI family values for families that are not implemented yet.

So `SupportedFamily` for the first cut should be:

- `Eslint`

Then grow to:

- `Eslint`
- `Tsconfig`
- `Npmrc`
- `Package`

Do not add placeholder enum variants for future families.

CLI shape

Mirror `g3rs`:

- `Cli`
- `Command::Validate`
- `FamilyArg`
- `run_command`

Concrete command target:

- `g3ts validate --path /path/to/app`

No additional subcommands.

Crawler boundary

Reuse the same neutral crawl adapter pattern:

- `apps/guardrail3-ts/crates/io/outbound/packages`
  - wraps `g3-workspace-crawl::crawl`
  - returns the neutral crawl into app traits

Do not create a TS-specific crawler package.

Report boundary

Mirror the same report path as Rust:

- `ValidateReport`
- `FamilyRun`
- plain-text renderer

Keep the rendered format aligned with `g3rs`:

- same severity ordering
- same `--inventory` behavior

This keeps both CLIs cognitively aligned.

Family wiring v1

For the first real app cut:

- `g3ts-eslint-ingestion::ingest_for_config_checks(crawl)`
- `g3ts-eslint-config-checks::check(&input)`

That is enough to make `g3ts validate --path <ts-app-root>` meaningful immediately.

No app-local ESLint logic should exist in `guardrail3-ts`.

All semantics stay in `packages/ts/eslint/**`.

What not to copy from Rust

Do not copy Rust-specific family groups:

- no `style`
- no `policy`
- no `process`
- no `structure`
- no `test`

unless TS actually needs those groups later.

Do not copy Rust family names into TS.

Do not wire `g3rs-*` packages into `guardrail3-ts`.

Do not make `guardrail3-ts` validate Rust package roots.

Implementation order

1. create `apps/guardrail3-ts/Cargo.toml`
2. scaffold `crates/types/app-types`
3. scaffold `crates/logic/validate-command`
4. scaffold `crates/io/outbound/packages`
5. scaffold `crates/io/outbound/report`
6. scaffold `crates/io/inbound/cli`
7. scaffold `crates/logic/family-runner-config`
8. wire only `eslint`
9. make `g3ts validate --path <root>` work
10. validate the app itself under `g3rs`

Key decisions

- Separate product, not a mode of `g3rs`
  - Reason: the user explicitly rejected a combined Rust/TS app.
- Minimal mirror, not literal duplication
  - Reason: Rust app crate shape should be copied where it serves a real purpose, not as empty ceremony.
- Only implemented families in the enum and CLI
  - Reason: placeholders rot and create fake surface area.
- Reuse neutral shared crates only
  - Reason: `g3-workspace-crawl` and `guardrail3-check-types` are already cross-language infrastructure.

Files to create

- `apps/guardrail3-ts/Cargo.toml`
- `apps/guardrail3-ts/crates/types/app-types/**`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/**`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/assertions/**`
- `apps/guardrail3-ts/crates/logic/family-runner-config/**`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/**`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/assertions/**`
- `apps/guardrail3-ts/crates/io/outbound/packages/**`
- `apps/guardrail3-ts/crates/io/outbound/report/crates/runtime/**`
- `apps/guardrail3-ts/crates/io/outbound/report/crates/assertions/**`

Files to reuse as patterns

- `apps/guardrail3-rs/Cargo.toml`
- `apps/guardrail3-rs/crates/types/app-types/**`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/**`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/**`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/**`
- `apps/guardrail3-rs/crates/io/outbound/packages/src/runtime.rs`
