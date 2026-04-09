# Test config and AST packages

## Goal

Extract the `test` family into package lanes without forcing the mixed
structure/file-boundary rules into the wrong package.

Target package set:

- `packages/rs/test/g3rs-test-types`
- `packages/rs/test/g3rs-test-config-checks`
- `packages/rs/test/g3rs-test-ast-checks`
- `packages/rs/test/g3rs-test-ingestion`

This plan covers the config and AST lanes only.

## Rule split

### Config lane

These rules belong in `g3rs-test-config-checks`:

- `RS-TEST-09`
  - nextest timeout config
- `RS-TEST-11`
  - `cargo-mutants` installed
- `RS-TEST-12`
  - `.cargo/mutants.toml` exists
- `RS-TEST-13`
  - `[profile.mutants]` exists
- `RS-TEST-14`
  - active hooks run `cargo mutants`
- `RS-TEST-15`
  - mutants config is sane

### AST lane

These rules belong in `g3rs-test-ast-checks`:

- `RS-TEST-01`
  - inline `#[cfg(test)] mod ... { ... }`
- `RS-TEST-04`
  - `#[ignore]` reason
- `RS-TEST-05`
  - `#[should_panic(expected = ...)]`
- `RS-TEST-06`
  - tautological assertions
- `RS-TEST-07`
  - real proof site
- `RS-TEST-08`
  - weak wildcard `matches!`
- `RS-TEST-16`
  - assertions modules prove
- `RS-TEST-17`
  - external harnesses use assertions

### Deferred mixed lane

Do not force these into the first config/AST extraction:

- `RS-TEST-02`
  - owned sidecar shape
- `RS-TEST-03`
  - runtime/assertions split and import boundaries
- `RS-TEST-10`
  - input failures
- `RS-TEST-18`
  - `test_support` stays generic

Reason:
- they depend on component layout, import boundaries, or family-wide
  fail-closed ownership more than on pure config or pure AST

## Scope model

### Config scope

One `config` input per owned test root.

Root scope is required because:

- mutation activation is per root
- nextest config is per root
- hook activation is per root
- cargo profile and tool presence are per root

### AST scope

One `AST` input per owned test root.

`test` AST is not a one-file lane.

It needs root-scoped bundles because:

- proof-bearing assertions are discovered across assertions modules
- external harness checks need the root-local assertions proof catalog
- file kind (`source`, `assertions`, `external harness`, sidecar support) is
  part of rule meaning

## Public input contracts

### Config checks input

Use one root-scoped input with parsed config files plus derived activation
facts:

```rust
pub struct G3RsTestConfigChecksInput {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub mutants_rel_path: String,
    pub nextest_rel_path: String,

    pub cargo: CargoToml,
    pub nextest: Option<NextestToml>,
    pub mutants: Option<MutantsToml>,

    pub has_tests: bool,
    pub has_tokio_tests: bool,
    pub tokio_dependency_present: bool,

    pub cargo_mutants_installed: bool,
    pub mutation_hook_active: bool,
    pub mutation_hook_files: Vec<String>,
    pub mutants_exists: bool,
}
```

Rules/runtime derive small local facts from those parsed file types.
Do not expose raw `toml::Value` across the package boundary.

Keep only the truly orchestration-level derived facts normalized:

- test activation
- tokio-test activation
- tool presence
- hook activation and matched hook files
- file existence flags where the rule is about presence

### AST checks input

Use one root-scoped source bundle:

```rust
pub struct G3RsTestAstChecksInput {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub files: Vec<G3RsTestSourceFile>,
    pub components: Vec<G3RsTestComponentAstFacts>,
}
```

```rust
pub struct G3RsTestSourceFile {
    pub rel_path: String,
    pub kind: G3RsTestFileKind,
    pub owner_module_name: Option<String>,
    pub component_rel_dir: Option<String>,
    pub assertions_package_name: Option<String>,
    pub content: String,
}
```

```rust
pub enum G3RsTestFileKind {
    Source,
    InternalSidecarMod,
    InternalSidecarSupport,
    ExternalHarness,
    AssertionsModule,
    Other,
}
```

```rust
pub struct G3RsTestComponentAstFacts {
    pub rel_dir: String,
    pub runtime_rel_dir: String,
    pub runtime_package_name: Option<String>,
    pub assertions_rel_dir: String,
    pub assertions_exists: bool,
    pub assertions_package_name: Option<String>,
}
```

This keeps the public AST input bounded:

- root metadata
- source contents
- minimal component metadata

The AST runtime still owns parse-once and proof-catalog construction.

## Ingestion responsibilities

`g3rs-test-ingestion` stays family-wide and exposes:

- `ingest_for_config_checks`
- `ingest_for_ast_checks`
- `ingest_for_file_tree_checks`

### Shared discovery inside ingestion

Both lanes need the same root/component discovery:

- owned test roots
- component layout
- file kinds
- mutation activation markers

So `g3rs-test-ingestion` should have one shared discovery layer that:

- finds roots from workspace crawl
- classifies test files
- detects mutation hook activation
- derives root-local package/component metadata

Then each lane projects that shared discovery into its own checks input.

### Config ingestion

`ingest_for_config_checks(&crawl) -> Result<Vec<G3RsTestConfigChecksInput>, G3RsTestIngestionError>`

It owns:

- root discovery
- `Cargo.toml` reads and typed parsing
- `.cargo/mutants.toml` reads and typed parsing
- `.config/nextest.toml` reads and typed parsing
- hook file reads and executable-line parsing
- lightweight test activation summary

Important detail:
- `RS-TEST-09` needs `has_tokio_tests`
- config ingestion may inspect Rust source just enough to derive root activation
  and tokio-test presence
- that is acceptable because ingestion owns root-level mapping
- the config checks package should receive parsed config files plus derived
  activation facts

### AST ingestion

`ingest_for_ast_checks(&crawl) -> Result<Vec<G3RsTestAstChecksInput>, G3RsTestIngestionError>`

It owns:

- root discovery
- file-kind classification
- reading source contents
- bundling one root-scoped AST input

It does not:

- parse AST
- build proof catalogs
- run rule semantics

## Runtime responsibilities

### Config runtime

`g3rs-test-config-checks` runtime:

- fans one root config input into tiny rule calls
- does not parse files
- does not read hooks
- does not inspect source text

### AST runtime

`g3rs-test-ast-checks` runtime:

- parses all files in one root input once
- builds root-local proof-bearing assertion catalogs
- derives tiny rule inputs
- runs pure rules

This is the same pattern we already settled on:

- ingestion chooses scope and reads files
- AST runtime does heavy parse/mapping
- rules stay tiny

## Staged implementation order

### Stage 1 - types and ingestion scaffold

1. create `g3rs-test-types`
2. create `g3rs-test-config-checks`
3. create `g3rs-test-ast-checks`
4. create `g3rs-test-ingestion`
5. wire stub entrypoints for all three lanes

### Stage 2 - config lane first

Implement:

- `RS-TEST-11`
- `RS-TEST-12`
- `RS-TEST-13`
- `RS-TEST-14`
- `RS-TEST-15`
- `RS-TEST-09`

Reason:
- this gives a clean root-scoped config specimen
- it also forces hook parsing and root activation into the new ingestion package

### Stage 3 - AST lane initial slice

Implement:

- `RS-TEST-01`
- `RS-TEST-04`
- `RS-TEST-05`
- `RS-TEST-06`
- `RS-TEST-08`

Reason:
- these are the safest AST rules first
- no proof catalog needed yet

### Stage 4 - AST proof-catalog slice

Implement:

- `RS-TEST-07`
- `RS-TEST-16`
- `RS-TEST-17`

Reason:
- these prove the root-scoped multi-file AST runtime

### Stage 5 - deferred mixed lane design

After config and AST are real, design the remaining lane for:

- `RS-TEST-02`
- `RS-TEST-03`
- `RS-TEST-10`
- `RS-TEST-18`

## Files this plan implies

- `packages/rs/test/g3rs-test-types`
- `packages/rs/test/g3rs-test-config-checks`
- `packages/rs/test/g3rs-test-ast-checks`
- `packages/rs/test/g3rs-test-ingestion`

## Done means

- `test` has real extracted config and AST packages
- config lane owns `09` and `11-15`
- AST lane owns `01`, `04-08`, `16`, `17`
- mixed rules are explicitly deferred, not silently lost
- family ingestion is the only discovery/orchestration surface
