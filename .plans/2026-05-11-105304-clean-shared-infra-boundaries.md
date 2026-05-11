# Clean shared infrastructure boundaries

## Goal

Make G3RS and G3TS independent products while keeping genuinely shared infrastructure in neutral shared packages.

Current state is not clean:

- `packages/shared/g3-workspace-crawl` exists and is tracked.
- Active manifests still depend on `packages/rs/g3rs-workspace-crawl`.
- TS code imports that RS crawl package through a `g3-workspace-crawl` alias.
- TS policy ingestion parses `guardrail3-ts.toml` through `g3rs-toml-parser`.
- `g3ts-toml-parser` exists but only models `[checks]`, so TS policy data is in the wrong parser.

End state:

- Shared crawl code lives only under `packages/shared/g3-workspace-crawl`.
- No active code depends on `packages/rs/g3rs-workspace-crawl`.
- `packages/rs/g3rs-workspace-crawl` is deleted.
- `g3rs-toml-parser` parses only `guardrail3-rs.toml`.
- `g3ts-toml-parser` parses all `guardrail3-ts.toml` data used by TS families.
- TS packages do not depend on RS-branded parser packages.
- RS packages do not depend on TS-branded parser packages.

## Approach

### 1. Workspace crawl package

Use the existing neutral package:

- `packages/shared/g3-workspace-crawl`

Do not create a new package. Do not keep a compatibility facade under `packages/rs/g3rs-workspace-crawl`.

Required changes:

- Change `apps/guardrail3-rs/Cargo.toml` workspace dependency from:
  - `g3rs-workspace-crawl = { path = "../../packages/rs/g3rs-workspace-crawl", ... }`
  - to `g3-workspace-crawl = { path = "../../packages/shared/g3-workspace-crawl", ... }`
- Change `apps/guardrail3-ts/Cargo.toml` workspace dependency from:
  - `g3-workspace-crawl = { path = "../../packages/rs/g3rs-workspace-crawl", package = "g3rs-workspace-crawl", ... }`
  - to `g3-workspace-crawl = { path = "../../packages/shared/g3-workspace-crawl", ... }`
- Change every `packages/rs/**/Cargo.toml` dependency on `g3rs-workspace-crawl` to `g3-workspace-crawl`.
- Change every `packages/ts/**/Cargo.toml` dependency on `g3-workspace-crawl` that still uses `package = "g3rs-workspace-crawl"` or a `../../rs/g3rs-workspace-crawl` path to the shared package.
- Change imports from `g3rs_workspace_crawl::*` to `g3_workspace_crawl::*`.
- Change neutral crawl type names from `G3RsWorkspace*` to `G3Workspace*`.
- Delete `packages/rs/g3rs-workspace-crawl`.

Allowed local naming:

- Family code may name variables `crawl`, `entry`, `workspace`.
- Family code must not type-alias neutral crawl types back to `G3RsWorkspace*` or `G3TsWorkspace*`.

Rejected alternatives:

- Do not keep `packages/rs/g3rs-workspace-crawl` as a facade.
- Do not keep Cargo aliasing from `g3-workspace-crawl` to `package = "g3rs-workspace-crawl"`.
- Do not duplicate crawl code into separate RS and TS packages.

### 2. TOML parser ownership

Keep two parser packages because the config files are different public contracts:

- `packages/parsers/g3rs-toml-parser`
- `packages/parsers/g3ts-toml-parser`

Required changes:

- Move TS policy structs out of `g3rs-toml-parser`:
  - `TsPolicyConfig`
  - `TsStylePolicyConfig`
  - `TsAstroPolicyConfig`
  - `TsAstroRoutesPolicyConfig`
  - `TsAstroContentPolicyConfig`
  - `TsAstroMdxPolicyConfig`
  - `TsAstroSeoPolicyConfig`
  - `TsAstroStatePolicyConfig`
  - `TsAstroI18nPolicyConfig`
  - `TsAstroMediaPolicyConfig`
  - `CollectionFieldsMap`
- Add those TS policy structs to `g3ts-toml-parser`.
- Add `style: Option<TsStylePolicyConfig>` and `astro: Option<TsAstroPolicyConfig>` to `Guardrail3TsToml`.
- Keep `checks: Option<TsChecksConfig>` in `Guardrail3TsToml`.
- Remove `ts: Option<TsPolicyConfig>` from `Guardrail3RsToml`.
- Update TS ingestion packages that currently call `g3rs_toml_parser::from_path` to call `g3ts_toml_parser::from_path`.
- Update TS ingestion packages to read policy directly from the parsed `Guardrail3TsToml`, not from a nested `config.ts`.
- Keep RS ingestion packages on `g3rs-toml-parser`.

Known TS ingestion files using the wrong parser now:

- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/policy.rs`
- `packages/ts/astro/content/g3ts-astro-content-ingestion/src/policy.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/src/policy.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/policy.rs`
- `packages/ts/astro/media/g3ts-astro-media-ingestion/crates/runtime/src/policy.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion/src/policy.rs`
- `packages/ts/astro/state/g3ts-astro-state-ingestion/src/policy.rs`

Rejected alternatives:

- Do not create one generic `guardrail3-toml-parser` that knows every product config.
- Do not leave TS policy schema in `g3rs-toml-parser`.
- Do not parse TS policy data by reading raw TOML in family ingestion code.

### 3. Verifier

Add one deterministic verifier script for this cleanup:

- `scripts/verify-shared-infra-boundaries.py`

It must read `.plans/2026-05-11-105304-clean-shared-infra-boundaries.manifest.toml`.

It must check:

- required shared package path exists
- deleted RS crawl package path does not exist
- no active `Cargo.toml` points at `packages/rs/g3rs-workspace-crawl`
- no active `Cargo.toml` uses `package = "g3rs-workspace-crawl"`
- no active Rust source imports `g3rs_workspace_crawl`
- no active Rust source refers to `G3RsWorkspaceCrawl`, `G3RsWorkspaceEntry`, `G3RsWorkspaceEntryKind`, `G3RsWorkspaceIgnoreState`, or `G3RsWorkspacePath`
- no TS-side active package depends on `g3rs-toml-parser`
- no TS-side active Rust source imports `g3rs_toml_parser`
- no RS-side active package depends on `g3ts-toml-parser`
- `g3rs-toml-parser` type schema has no `Ts*PolicyConfig` structs and no `ts` field on `Guardrail3RsToml`
- `g3ts-toml-parser` type schema has the TS policy structs and fields required by TS ingestion
- old package path is absent from `apps/guardrail3-rs/Cargo.toml`
- old package path is absent from `apps/guardrail3-ts/Cargo.toml`

The verifier must exclude historical artifacts:

- `.plans/**`
- `.worklogs/**`
- `.baselines/**`
- `target/**`
- `node_modules/**`
- `legacy/**`

Verifier output must be layer-based:

```text
layer:1-tree status:PASS
layer:2-cargo-deps status:PASS
layer:3-source-imports status:PASS
layer:4-parser-schema status:PASS
layer:5-validate status:PASS
```

### 4. Mechanical validation

Run these after implementation:

- `python3 scripts/verify-shared-infra-boundaries.py`
- `cargo test --manifest-path packages/shared/g3-workspace-crawl/Cargo.toml --workspace`
- `cargo test --manifest-path packages/parsers/g3rs-toml-parser/Cargo.toml --workspace`
- `cargo test --manifest-path packages/parsers/g3ts-toml-parser/Cargo.toml --workspace`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml --workspace`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/shared/g3-workspace-crawl`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/parsers/g3rs-toml-parser`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/parsers/g3ts-toml-parser`
- `apps/guardrail3-rs/target/release/g3rs validate --path apps/guardrail3-rs`
- `apps/guardrail3-rs/target/release/g3rs validate --path apps/guardrail3-ts`

If any command reveals existing unrelated strict-lint debt, fix that debt in the owning package before claiming completion.

## Key decisions

- Workspace crawl is neutral shared infrastructure. It belongs under `packages/shared`, not under `packages/rs`.
- TOML parser packages are not generic shared infrastructure. Each parser owns one config-file contract.
- Parser packages can live under `packages/parsers` while still being product-specific by file format.
- No backwards compatibility package remains under the old path.
- No aliasing is allowed because aliasing hides the dependency boundary and recreates this bug.

## Files to modify

Expected direct changes:

- `apps/guardrail3-rs/Cargo.toml`
- `apps/guardrail3-ts/Cargo.toml`
- `packages/rs/**/Cargo.toml`
- `packages/ts/**/Cargo.toml`
- `apps/guardrail3-rs/**/*.rs`
- `apps/guardrail3-ts/**/*.rs`
- `packages/rs/**/*.rs`
- `packages/ts/**/*.rs`
- `packages/parsers/g3rs-toml-parser/**`
- `packages/parsers/g3ts-toml-parser/**`
- `scripts/verify-shared-infra-boundaries.py`
- `.plans/2026-05-11-105304-clean-shared-infra-boundaries.manifest.toml`

Expected deletion:

- `packages/rs/g3rs-workspace-crawl/**`

Do not modify:

- `legacy/**`
- historical `.worklogs/**` content
- historical `.baselines/**` content
