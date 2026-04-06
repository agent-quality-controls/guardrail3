# G3RS Workspace Crawl And Family Ingestion Plan

## Goal

Build the new Rust-only validation stack for `g3rs` without rebuilding the
old `guardrail3` app topology / legality / family-mapper system.

The new stack must:

- validate **one explicit workspace root at a time**
- support real validation, not just unit tests
- centralize shared filesystem crawling semantics so they do not drift
- keep per-family file selection and parsing logic isolated
- keep checks packages pure

This is the package graph we are targeting:

```text
workspace root
  -> g3rs-workspace-crawl
  -> g3rs-<family>-<surface>-ingestion
  -> g3rs-<family>-<surface>-checks
```

Examples:

- `g3rs-workspace-crawl`
- `g3rs-fmt-config-ingestion`
- `g3rs-fmt-config-checks`
- `g3rs-garde-ast-ingestion`
- `g3rs-garde-ast-checks`

## Core Decision

We **do** crawl once, but only per explicit workspace, and only into a neutral
filesystem snapshot.

We **do not**:

- crawl the whole repo
- resolve overlapping roots
- decide legality
- decide family ownership across sibling/nested roots
- build a giant project-wide semantic model

The key difference from the old app is not that crawl/slice disappears. The
difference is that the problem is reduced to:

> "validate this one chosen workspace"

instead of:

> "understand all candidate Rust roots in the whole repository and decide who
> owns what"

## Packages

### 1. `g3rs-workspace-crawl`

Purpose:
- crawl one explicit workspace root
- own shared path / ignore / recoverability semantics
- expose neutral filesystem facts

It should know:
- workspace root path
- discovered paths under that root
- which paths are ignored
- which paths are readable/unreadable
- basic file metadata

It should **not** know:
- which files a family needs
- which config covers which subtree
- legality / precedence / ownership
- rule semantics

This package exists specifically to avoid duplicating:
- `.gitignore` behavior
- hidden file policy
- unreadable-file handling
- directory walking semantics

across every ingestion package.

### 2. `g3rs-<family>-<surface>-ingestion`

Purpose:
- take the neutral workspace crawl output
- decide what files that family/surface needs
- read them
- parse them
- build the exact input type for the checks package

Examples:

- `g3rs-fmt-config-ingestion`
- `g3rs-toolchain-config-ingestion`
- `g3rs-clippy-config-ingestion`
- `g3rs-cargo-config-ingestion`
- `g3rs-deny-config-ingestion`
- `g3rs-deps-config-ingestion`
- `g3rs-garde-config-ingestion`
- `g3rs-garde-ast-ingestion`

This layer owns:
- family-local file selection
- family-local parsing
- family-local input assembly

This layer does **not** own:
- rule evaluation
- global workspace legality
- cross-workspace precedence

### 3. `g3rs-<family>-<surface>-checks`

Purpose:
- pure validation of already assembled typed input

These packages already exist for the extracted surfaces and must stay pure.

## Terminology

Do **not** call the new middle layer a mapper.

Old "mapper" implied:
- taking already discovered / already parsed / already normalized data
- slicing it for a family

The new layer is doing more:
- file selection
- file reading
- parsing
- input assembly

So the correct name is:
- `ingestion`

## Pipeline

The concrete runtime flow is:

```text
workspace root
  -> crawl workspace once
  -> family ingestion selects files from crawl
  -> family ingestion reads/parses selected files
  -> family ingestion builds checks input
  -> checks package runs rules
```

Expanded:

```text
crawl
  -> select
  -> parse
  -> assemble
  -> check
```

There is no need to keep a separate `mapping` term unless we later discover a
real need for a post-parse pure transformation layer.

## Dependency Rules

The packages should not call each other directly. The orchestrator wires them
together.

Dependency shape:

- orchestrator
  - depends on crawl logic
  - depends on ingestion logic
  - depends on checks logic

- crawl package
  - standalone
  - exports crawl types + crawl logic

- ingestion package
  - depends on crawl **types**
  - depends on parser crates
  - depends on checks-input **types**
  - exports ingestion types + ingestion logic
  - does **not** call checks logic

- checks package
  - depends on its own input types
  - depends on parser model types as needed
  - knows nothing about crawl or ingestion

This is the feature rule:

- each facade exports `types` and `logic` separately
- packages depend on each other through `types`
- only the orchestrator depends on the `logic` features of the stages it runs

## Input / Output Boundaries

### Crawl package

The crawl package should expose a neutral snapshot such as:

```rust
pub struct G3RsWorkspaceCrawl {
    // root path
    // discovered paths
    // ignore state
    // metadata / readability
}
```

It may also expose helper value types such as:

```rust
pub struct G3RsWorkspacePath {
    pub rel_path: String,
    pub abs_path: std::path::PathBuf,
}
```

The exact type names are less important than the rule:

- crawl output is filesystem-neutral
- no family semantics in the type

### Ingestion package

Each ingestion package takes the crawl output and returns the exact checks input
for its corresponding checks package.

Examples:

- `g3rs-fmt-config-ingestion`
  - input: `&G3RsWorkspaceCrawl`
  - output: `G3RsFmtConfigChecksInput`

- `g3rs-garde-ast-ingestion`
  - input: `&G3RsWorkspaceCrawl`
  - output: `G3RsGardeAstChecksInput`

It may also expose structured ingestion failures if the new app wants to report
them independently of the checks packages.

### Checks package

Checks packages stay exactly what they are becoming:
- pure rules over typed input

## File Selection Ownership

This is the crucial boundary:

- `g3rs-workspace-crawl` does **not** decide what files a family wants
- `g3rs-*-ingestion` **does** decide what files that family wants

Examples:

- `fmt` ingestion decides:
  - root `rustfmt.toml` / `.rustfmt.toml`
  - root `Cargo.toml`
  - root `rust-toolchain.toml`

- `clippy` ingestion decides:
  - covering `clippy.toml` / `.clippy.toml`
  - relevant `Cargo.toml`

- `garde-ast` ingestion decides:
  - governed `.rs` files inside the workspace
  - required policy/config files

This keeps file-selection semantics family-local without duplicating filesystem
walk semantics.

## Shared Crawl Semantics

The crawl package is the single place that must own:

- path enumeration under the explicit workspace root
- ignore handling
- readability / recoverability handling
- common file existence queries

Without this, each ingestion package would reinvent:
- what counts as ignored
- what to do with unreadable files
- what hidden/special files to consider
- how to walk the workspace

That would drift immediately.

## What The New App Should Not Rebuild

The new `g3rs` app should **not** recreate:

- legality
- multi-root ownership resolution
- nested workspace precedence
- family mapper in the old project-wide sense
- giant normalized fact bags shared across families

The only shared global-ish stage is:
- crawl one explicit workspace

Everything else is:
- family-local ingestion
- pure checks

## Orchestrator Shape

The orchestrator becomes minimal:

1. accept explicit workspace root
2. run `g3rs-workspace-crawl`
3. choose family/surface
4. call the corresponding ingestion package
5. call the corresponding checks package
6. print/aggregate results

The orchestrator should be able to run:

- one family
- one surface
- or a small configured set

without needing the old repo-wide app machinery.

## First Implementation Order

The goal is to prove the architecture on the smallest useful path first.

### Phase 1: Shared crawl

Build:
- `g3rs-workspace-crawl`

Scope:
- one explicit workspace root
- path enumeration
- ignore/recoverability semantics
- neutral queries only

Do **not** add family semantics here.

### Phase 2: One trivial config family

Build:
- `g3rs-cargo-config-ingestion`

Why first:
- it only needs one `Cargo.toml`
- the file selection is minimal
- it proves the package wiring with the least noise

Pipeline to prove:
- crawl workspace
- ingestion locates/parses root `Cargo.toml`
- checks run via `g3rs-cargo-config-checks`

### Phase 3: One multi-file config family

Build:
- `g3rs-fmt-config-ingestion`

Why:
- proves multi-file config ingestion cleanly
- still no source-tree scanning

### Phase 4: One source-analysis family

Build:
- `g3rs-garde-ast-ingestion`

Why:
- proves source-file selection from crawl
- proves AST ingestion can live on top of the same shared crawl

### Phase 5: New minimal app

Only after the packages above are working:
- create the new Rust-only `g3rs` app
- keep it as thin orchestration only

## Proposed Package List

Shared:
- `g3rs-workspace-crawl`

Config ingestion:
- `g3rs-fmt-config-ingestion`
- `g3rs-toolchain-config-ingestion`
- `g3rs-clippy-config-ingestion`
- `g3rs-cargo-config-ingestion`
- `g3rs-deny-config-ingestion`
- `g3rs-deps-config-ingestion`
- `g3rs-garde-config-ingestion`

AST ingestion:
- `g3rs-garde-ast-ingestion`

Checks:
- `g3rs-fmt-config-checks`
- `g3rs-toolchain-config-checks`
- `g3rs-clippy-config-checks`
- `g3rs-cargo-config-checks`
- `g3rs-deny-config-checks`
- `g3rs-deps-config-checks`
- `g3rs-garde-config-checks`
- `g3rs-garde-ast-checks`

## Open Questions

### Ingestion failures

Need to decide whether ingestion packages return:

- only `Result<Input, IngestionFailure>`

or

- `Input + structured non-fatal ingestion findings`

This matters especially for families that currently have explicit malformed-input
rules in the old app.

### Surface naming for non-config/non-ast rules

Some existing app-owned rules do not fit neatly into:
- `config`
- `filetree`
- `ast`

Examples:
- deps tooling checks
- malformed-input sink rules

Those probably need a later explicit naming decision, not silent shoehorning.

### How much content caching belongs in crawl

The current plan assumes:
- crawl enumerates paths once
- ingestion reads/parses selected files

If repeated family runs make file I/O too expensive, we can add lazy content
caching to crawl later. It should stay neutral and optional.

## Short Version

The new stack is:

```text
g3rs-workspace-crawl
  -> g3rs-<family>-<surface>-ingestion
  -> g3rs-<family>-<surface>-checks
```

Rules:
- crawl owns shared filesystem semantics
- ingestion owns family-specific file selection and parsing
- checks own pure validation
- orchestrator only wires the stages together

That is the smallest version that still behaves like a real validator while
avoiding the old app’s repo-wide legality/topology mess.
