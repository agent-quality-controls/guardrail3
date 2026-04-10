# Reorganize packages into folders + extract shared family types

## Problem

`packages/` is flat with 30+ packages. Hard to navigate, hard to see which packages belong to which family. Also, every family has the same input type defined in the checks package's types crate, which means ingestion depends on checks just to get the type. The dependency should flow: types ← ingestion, types ← checks. Not: ingestion → checks → types.

## Current state (flat)

```
packages/
  cargo-toml-parser/
  cargo-config-toml-parser/
  cliff-toml-parser/
  clippy-toml-parser/
  deny-toml-parser/
  mutants-toml-parser/
  nextest-toml-parser/
  release-plz-toml-parser/
  rust-toolchain-toml-parser/
  rustfmt-toml-parser/
  guardrail3-rs-toml-parser/
  guardrail3-check-types/
  reason-policy/
  g3rs-workspace-crawl/
  g3rs-cargo-config-checks/
  g3rs-cargo-config-ingestion/
  g3rs-clippy-config-checks/
  g3rs-clippy-config-ingestion/
  g3rs-deny-config-checks/
  g3rs-deny-config-ingestion/
  g3rs-deps-config-checks/
  g3rs-fmt-config-checks/
  g3rs-fmt-config-ingestion/
  g3rs-garde-source-checks/
  g3rs-garde-config-checks/
  g3rs-garde-config-ingestion/
  g3rs-release-config-checks/
  g3rs-release-config-ingestion/
  g3rs-toolchain-config-checks/
  g3rs-toolchain-config-ingestion/
```

## Target state (grouped)

```
packages/
  rs/
    g3rs-workspace-crawl/                    # shared crawl layer
    cargo/
      g3rs-cargo-types/                      # shared input type (CargoConfigChecksInput)
      g3rs-cargo-config-checks/              # checks (depends on types)
      g3rs-cargo-config-ingestion/           # ingestion (depends on types, NOT checks)
    clippy/
      g3rs-clippy-types/
      g3rs-clippy-config-checks/
      g3rs-clippy-config-ingestion/
    deny/
      g3rs-deny-types/
      g3rs-deny-config-checks/
      g3rs-deny-config-ingestion/
    deps/
      g3rs-deps-types/
      g3rs-deps-config-checks/
      # g3rs-deps-config-ingestion/ (TODO)
    fmt/
      g3rs-fmt-types/
      g3rs-fmt-config-checks/
      g3rs-fmt-config-ingestion/
    garde/
      g3rs-garde-types/
      g3rs-garde-config-checks/
      g3rs-garde-config-ingestion/
      g3rs-garde-source-checks/
      # g3rs-garde-ast-ingestion/ (TODO)
    release/
      g3rs-release-types/
      g3rs-release-config-checks/
      g3rs-release-config-ingestion/
    toolchain/
      g3rs-toolchain-types/
      g3rs-toolchain-config-checks/
      g3rs-toolchain-config-ingestion/
  parsers/
    cargo-toml-parser/
    cargo-config-toml-parser/
    cliff-toml-parser/
    clippy-toml-parser/
    deny-toml-parser/
    mutants-toml-parser/
    nextest-toml-parser/
    release-plz-toml-parser/
    rust-toolchain-toml-parser/
    rustfmt-toml-parser/
    guardrail3-rs-toml-parser/
  shared/
    guardrail3-check-types/
    reason-policy/
```

## Shared types extraction

Currently the input type lives in the checks package:
```
g3rs-cargo-config-checks/crates/types/src/lib.rs → G3RsCargoConfigChecksInput
```

Ingestion depends on the checks facade to access this type:
```
g3rs-cargo-config-ingestion → g3rs-cargo-config-checks (just for the type)
```

This is backwards. The type should live in a shared package:
```
packages/rs/cargo/g3rs-cargo-types/
  Cargo.toml
  src/lib.rs → G3RsCargoConfigChecksInput (re-exported as G3RsCargoInput or similar)
```

Then:
- checks depends on g3rs-cargo-types (for input type) + guardrail3-check-types (for result type)
- ingestion depends on g3rs-cargo-types (for input type) + g3rs-workspace-crawl (for crawl) + parser
- ingestion does NOT depend on checks at all

### Per-family shared types package contents

Each `g3rs-{family}-types` package is tiny — just the input struct:

**g3rs-cargo-types:**
```rust
pub struct G3RsCargoConfigChecksInput {
    pub cargo_rel_path: String,
    pub cargo: CargoToml,
}
```

**g3rs-clippy-types:**
```rust
pub struct G3RsClippyConfigChecksInput {
    pub clippy_rel_path: String,
    pub clippy: ClippyToml,
}
```

And so on for each family. The struct moves from `g3rs-{family}-config-checks/crates/types/` to `g3rs-{family}-types/`. The checks types crate becomes empty (or deleted) and the checks runtime depends on the shared types package instead.

## Execution plan

### Phase 1: Move folders (just `git mv`, no code changes)

Move packages into the folder structure. No Cargo.toml path changes yet — Cargo resolves by package name, not directory path. But internal `path = "..."` deps need updating.

Actually, Cargo.toml `path` dependencies use relative paths, so moving directories DOES break them. Every `path = "../../../cargo-toml-parser"` becomes wrong.

**Alternative approach:** Move directories AND update all path deps in one commit per group. Use `sed` or a script to rewrite paths.

### Phase 2: Extract shared types (per family)

For each family:
1. Create `packages/rs/{family}/g3rs-{family}-types/` with the input struct
2. Update `g3rs-{family}-config-checks/crates/types/` to re-export from the shared package
3. Update `g3rs-{family}-config-ingestion/crates/runtime/Cargo.toml` to depend on shared types instead of checks facade
4. Update ingestion code to import from shared types

### Phase 3: Verify

- All tests pass
- All arch/code validation clean
- App compiles
- No circular dependencies

## Risk

This is a large mechanical refactor. Every `path = "..."` in every Cargo.toml needs updating. One wrong path = compilation failure. Should be done carefully, probably one family at a time, with verification after each.

## Naming

The shared types packages:
- `g3rs-cargo-types` (not `g3rs-cargo-config-types` — the family is "cargo", not "cargo-config")
- `g3rs-clippy-types`
- `g3rs-deny-types`
- `g3rs-deps-types`
- `g3rs-fmt-types`
- `g3rs-garde-types` (shared between config-checks and ast-checks)
- `g3rs-release-types`
- `g3rs-toolchain-types`

## Order of operations

1. Extract shared types first (less risky, smaller changes)
2. Then move folders (mechanical but large)

Or:

1. Move folders first (one big commit, then everything is in the right place)
2. Then extract shared types (cleaner because paths are already correct)

I'd recommend option 2: move first, then extract. The move is mechanical and the path rewrites can be scripted. Once everything is in the right folders, the shared types extraction has shorter relative paths.
