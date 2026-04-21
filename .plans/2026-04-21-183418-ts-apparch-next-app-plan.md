## Goal

Build `ts/apparch` as an app-internal architecture family for one explicit Next.js app root.

It should mirror the live Rust `apparch` intent:

- dependency direction by layer
- purity rules by layer
- selected public-surface ownership rules

But it should adapt the unit of analysis from Rust crates to TypeScript app source roots and source-file import edges.

End state:

- `g3ts` supports `--family apparch`
- package group exists:
  - `packages/ts/apparch/g3ts-apparch-types`
  - `packages/ts/apparch/g3ts-apparch-ingestion`
  - `packages/ts/apparch/g3ts-apparch-config-checks`
  - `packages/ts/apparch/g3ts-apparch-source-checks`
- no `file-tree-checks` package in wave 1
- the family validates one Next app root using app-internal layer roots

## Approach

### 1. Mirror the live Rust `apparch` seam, not the old TS package-graph idea

Rust reference:

- `g3rs-apparch-types`
- `g3rs-apparch-ingestion`
- `g3rs-apparch-config-checks`
- `g3rs-apparch-source-checks`

Key carry-over:

- no file-tree lane
- config lane owns dependency-direction rules
- source lane owns selected public-surface rules

Adaptation:

- Rust crate dependency edges become TS source-file import edges between app-internal layer roots

### 2. Use app-internal layer roots as the owned architecture surface

Wave 1 owned roots:

- `src/types/**`
- `src/logic/**`
- `src/io/inbound/**`
- `src/io/outbound/**`
- `src/app/**`

Optional roots not enforced in wave 1:

- `src/ui/**`
- `src/lib/**`

Reason:

- `src/app` is framework-owned shell in Next
- `src/io/inbound` is the owned inbound adapter layer behind thin Next entry files
- `src/io/outbound` is the owned outbound adapter layer
- `src/types` and `src/logic` are the direct Rust intent analogues

### 3. Define TS family types around layer facts, not package manifests

Planned types:

- `G3TsApparchLayer`
  - `App`
  - `Types`
  - `Logic`
  - `IoInbound`
  - `IoOutbound`
- `G3TsApparchImportKind`
  - `Import`
  - `Reexport`
  - `DynamicImport`
- `G3TsApparchSourceFile`
  - `rel_path`
  - `layer`
- `G3TsApparchInternalEdge`
  - `from_rel_path`
  - `from_layer`
  - `to_rel_path`
  - `to_layer`
  - `kind`
- `G3TsApparchExternalImport`
  - `from_rel_path`
  - `from_layer`
  - `module_name`
  - `kind`
- `G3TsApparchPublicItemKind`
  - `Interface`
  - `TypeAlias`
  - `Function`
  - `Class`
- `G3TsApparchPublicItem`
  - `rel_path`
  - `layer`
  - `item_name`
  - `kind`
  - `line`
- `G3TsApparchConfigChecksInput`
  - `files`
  - `internal_edges`
  - `external_imports`
- `G3TsApparchSourceChecksInput`
  - `files`
  - `public_items`

Key decision:

- no package manifest dependency graph in wave 1
- app-internal source imports are the real dependency graph for a single Next app

### 4. Build ingestion from source files and imports

`g3ts-apparch-ingestion` should:

- discover source files under the owned layer roots
- classify each file into one layer from its rel path
- parse TS/TSX files once using tree-sitter
- extract:
  - static `import ... from`
  - re-export `export ... from`
  - dynamic `import("...")`
- resolve internal targets for:
  - relative paths
  - `@/` alias into `src/`
- keep non-resolved bare module specifiers as external imports
- extract exported declarations for source-lane public-surface checks

Wave 1 resolution rules:

- support:
  - `./`
  - `../`
  - `@/`
- ignore unresolved custom aliases for now instead of guessing

Reason:

- this is enough to enforce the intended app-internal architecture cleanly
- it avoids inventing fake manifest dependency edges from one app package

### 5. Config checks: dependency direction by layer

These are the TS adaptations of the live Rust config rules.

Wave 1 rules:

- `TS-APPARCH-CONFIG-01`
  - `types` files must not import `logic`, `io/inbound`, `io/outbound`, or `app`
- `TS-APPARCH-CONFIG-02`
  - `logic` files must not import `io/inbound`, `io/outbound`, or `app`
  - same-layer `logic -> logic` imports are allowed
- `TS-APPARCH-CONFIG-03`
  - `io/outbound` files must not import `logic`, `io/inbound`, or `app`
  - same-layer `io/outbound -> io/outbound` imports are allowed
- `TS-APPARCH-CONFIG-04`
  - `io/inbound` files must not import `io/outbound` or `app`
  - may import `logic` and `types`
- `TS-APPARCH-CONFIG-05`
  - `app` files must not import `io/outbound` directly
  - the Next shell must go through inbound adapters or logic

Explicit adaptation from Rust:

- Rust same-layer crate cycles do not translate to same-layer file imports
- do not copy the Rust cycle rule naively into file-level TS imports

### 6. Source checks: selected public-surface ownership

Wave 1 source rules:

- `TS-APPARCH-SOURCE-01`
  - `types` files must not export public functions or classes
  - keep `types` passive
- `TS-APPARCH-SOURCE-02`
  - `io/inbound` and `io/outbound` files must not export interfaces
  - transport/adapter contracts belong in `types`

Explicitly not enforced yet:

- ban on all exported type aliases in io layers
  - too risky for wave 1 because internal adapter helper types may still be useful
- entrypoint-only public surface modeling
  - can be added later if needed

### 7. Keep `src/app` thin by dependency direction, not by fake folder ownership

Next requires routes and page entry files to live under `app`.

So the family should enforce this indirectly:

- `src/app/**` may not import `src/io/outbound/**` directly
- `src/app/**` should not become the home for outbound implementations
- thin shell behavior is encouraged by permitted dependency directions

Reason:

- this matches the real Next filesystem constraint
- it does not pretend `route.ts` can live outside `app`

### 8. Wire `apparch` into the `g3ts` structure runner

Changes:

- add `SupportedFamily::Apparch`
- route it through `family-runner-structure`
- keep the runner group aligned with `arch`

### 9. Verification

Before reporting done:

- unit tests cover:
  - file classification by layer
  - import extraction
  - relative path resolution
  - `@/` resolution
  - each forbidden direction rule
  - allowed same-layer imports
  - `types` exported function/class failure
  - io exported interface failure
- `cargo test` passes for all new roots
- `g3rs validate` passes for all new roots
- `g3rs validate --path apps/guardrail3-ts` stays clean
- one adversarial review compares the implementation against:
  - live Rust `apparch` intent
  - this corrected plan

## Key decisions

- Build `ts/apparch` over app-internal layer roots, not package roots.
  - Why: this family is for app architecture.
  - Rejected: workspace package graph as the primary unit, because that would be a different family.

- Keep no file-tree lane.
  - Why: live Rust `apparch` has config and source only.

- Do not copy Rust same-layer cycle rules directly.
  - Why: same-layer file imports inside one layer root are normal and necessary.

- Use source import edges as the dependency graph.
  - Why: one Next app usually has one `package.json`, so manifest edges do not encode internal architecture.

## Files to modify

- `apps/guardrail3-ts/Cargo.toml`
- `apps/guardrail3-ts/crates/types/app-types/src/supported_family.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs`
- `packages/ts/apparch/g3ts-apparch-types/...`
- `packages/ts/apparch/g3ts-apparch-ingestion/...`
- `packages/ts/apparch/g3ts-apparch-config-checks/...`
- `packages/ts/apparch/g3ts-apparch-source-checks/...`

## Explicit non-goals for wave 1

- no `ts/topology`
- no package-workspace graph enforcement
- no source-tree structural checks already owned by `ts/arch`
- no attempt to move Next route files out of `app`
- no custom alias resolution beyond relative imports and `@/`
