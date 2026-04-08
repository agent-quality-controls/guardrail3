# AST Checks Package Architecture

**Status:** working design

This file defines the AST lane template for the new `g3rs` package pipeline.

It is intentionally independent of current `garde` implementation details.

## Goal

Define how AST checks should work in the package world:

- what ingestion owns
- what AST checks runtime owns
- what rules own
- what the public AST input contract should look like

## Core Decision

AST checks packages should consume **bounded source contents**, not filesystem paths.

That means:

- ingestion reads files
- ingestion chooses scope
- checks runtime parses and maps within that bounded input
- rules stay tiny and pure

AST checks packages should not do workspace discovery.

They should also avoid direct filesystem reads as part of the intended final boundary.

## Pipeline

```text
workspace crawl
  -> AST ingestion
  -> AST checks input
  -> AST checks runtime/support
  -> tiny rule inputs
  -> pure rules
```

## Responsibilities

### AST ingestion owns

- choosing AST scope
- selecting files in that scope
- reading those files
- attaching family-local non-AST supporting inputs when needed
- constructing the public AST checks input

AST ingestion does **not** need to fan out to rule-level facts.

### AST checks runtime owns

- parsing source contents once
- building family-local maps and indexes
- cross-file resolution inside the given scope
- emitting tiny rule-local facts
- calling pure rule functions

This is where the heavy AST logic belongs.

### Rules own

- one local assertion
- one tiny typed input
- no parsing
- no I/O
- no sibling discovery

## Scope Model

AST is not one-file-only.

The right rule is:

**Use the smallest bounded AST scope that matches the family rule.**

Allowed scopes:

- one file
- one crate
- one root
- one package

Not allowed by default:

- whole repo AST scope as the normal package contract

## Public Input Shape

AST checks input should carry source contents, not `abs_path`.

Base source item:

```rust
pub struct G3RsSourceFile {
    pub rel_path: String,
    pub content: String,
}
```

Optional non-source supporting file:

```rust
pub struct G3RsTextFile {
    pub rel_path: String,
    pub content: String,
}
```

The exact family input then wraps those in the correct scope.

## Runtime-local parsed shape

AST checks runtimes may keep one small parsed container local to the runtime/support layer.

For the single-file `code` specimen, use:

```rust
pub struct G3RsCodeSourceFileAst {
    pub source_file: G3RsSourceFile,
    pub ast: syn::File,
}
```

This type is:

- runtime-local
- produced after parse
- not the public package input
- not the default rule input

Its job is only to keep the parsed AST attached to the exact source input it came from.

Rules should still receive smaller derived inputs where that adds clarity.

## Two AST Package Shapes

### 1. Single-file AST package

Use this when every rule is local to one source file.

Example shape:

```rust
pub struct G3RsCodeAstChecksInput {
    pub source_file: G3RsSourceFile,
}
```

Runtime behavior:

- parse one file once
- keep one `G3RsCodeSourceFileAst` per checks call
- derive local facts
- run per-file rules

Best first specimen:

- `code`

Why:

- simplest AST lane
- proves the package split with the least complexity
- does not require cross-file maps

### 2. Scoped multi-file AST package

Use this when rules need cross-file facts inside a bounded scope.

Example shape:

```rust
pub struct G3RsGardeAstChecksInput {
    pub root_rel_dir: String,
    pub source_files: Vec<G3RsSourceFile>,
    pub policy_file: Option<G3RsTextFile>,
}
```

Runtime behavior:

- parse all source files once
- build shared type/helper/usage maps
- resolve cross-file relationships
- emit tiny rule facts

Use this shape for families like:

- `garde`
- `test`
- `hexarch`

## Family Guidance

### `code`

Target shape:

- one input per source file
- runtime parses one file
- rules run directly on local facts

This should be the first AST package specimen.

### `garde`

Target shape:

- one input per garde root
- includes all governed source files for that root
- may include one policy/config text file for escape-hatch lookups

Runtime owns:

- parse-once across files
- shared validation-state maps
- cross-file nested-type resolution

### `test`

Target shape:

- one input per test root or one input per owned assertions package, depending on the rule group

Important:

- not every `test` rule belongs in AST
- AST package should only own source-semantic rules
- config/tool/hook checks stay outside the AST lane

### `hexarch`

Target shape:

- one input per crate
- source files bounded to that crate

Runtime owns:

- module graph walking inside the given crate scope
- public API aggregation

## Package Structure

AST checks packages should follow the same package scaffold as other checks packages:

```text
g3rs-{family}-ast-checks/
  Cargo.toml
  README.md
  TODO.md
  src/lib.rs
  crates/
    types/
    assertions/
    runtime/
```

Runtime layout should keep:

- one rule per file
- `run.rs` as orchestrator
- `support.rs` for shared AST mapping
- `parse.rs` only for AST parsing helpers if needed

Good runtime shape:

```text
crates/runtime/src/
  lib.rs
  run.rs
  support.rs
  parse.rs
  rs_family_ast_01_.../
  rs_family_ast_02_.../
```

## Boundary Rules

### Allowed in AST ingestion

- file selection
- file reads
- family-scope grouping
- passing source text into AST input

### Allowed in AST checks runtime

- syn parsing
- parse-once caching local to one checks call
- small runtime-local parsed containers such as `G3RsCodeSourceFileAst`
- cross-file maps inside one bounded scope
- rule fan-out

### Forbidden in rules

- reading files
- parsing source text
- walking the whole scope
- building their own cross-file maps

## First Implementation Order

### Phase 1

Build the first clean AST package specimen as `code`.

Reason:

- one-file AST is the simplest case
- proves the AST lane without cross-file complexity

### Phase 2

Build the first clean multi-file AST specimen as `garde`.

Reason:

- proves bounded cross-file AST scope
- proves that checks runtime can own heavy semantic mapping

### Phase 3

Use lessons from `code` and `garde` to design:

- `test` AST package split
- `hexarch` crate-scoped AST package

## Short Version

- ingestion chooses AST scope and reads files
- AST checks input carries source contents, not file paths
- AST checks runtime does parse-once and heavy semantic mapping
- small runtime-local parsed containers are allowed
- rules stay tiny and pure
- first AST specimen should be `code`
- first multi-file AST specimen should be `garde`
