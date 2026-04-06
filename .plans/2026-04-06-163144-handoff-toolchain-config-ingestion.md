# Handoff: Build g3rs-toolchain-config-ingestion

## What this package does

Takes a `G3RsWorkspaceCrawl`, finds `rust-toolchain.toml` at the workspace root, reads and parses it with `rust-toolchain-toml-parser`, and returns TWO check input types — one for channel/components checks and one for MSRV consistency checks (which also needs Cargo.toml).

## IMPORTANT: This package has TWO output types

The toolchain checks package has two separate check functions with two different input types:

### Output type 1: Channel & Components

```rust
// Defined in packages/g3rs-toolchain-config-checks/crates/types/src/lib.rs
pub struct G3RsToolchainConfigChannelComponentsInput {
    pub toolchain_rel_path: String,
    pub toolchain_toml: RustToolchainToml,
}
```

### Output type 2: MSRV Consistency

```rust
// Defined in packages/g3rs-toolchain-config-checks/crates/types/src/lib.rs
pub struct G3RsToolchainConfigMsrvConsistencyInput {
    pub toolchain_rel_path: String,
    pub toolchain_toml: RustToolchainToml,
    pub cargo_rel_path: String,
    pub cargo_toml: CargoToml,
}
```

The MSRV check needs BOTH `rust-toolchain.toml` AND `Cargo.toml` because it compares the toolchain channel version against the workspace's `rust-version` / MSRV.

## Input type (the contract you consume)

```rust
G3RsWorkspaceCrawl — call crawl.root_file("rust-toolchain.toml") and crawl.root_file("Cargo.toml")
```

## Parsers

### rust-toolchain.toml parser
- **Crate:** `rust-toolchain-toml-parser` at `packages/rust-toolchain-toml-parser`
- **Function:** `rust_toolchain_toml_parser::parse(input: &str) -> Result<RustToolchainToml, Error>`
- **Type:** `RustToolchainToml` with `toolchain: Option<ToolchainSection>` and `extra: BTreeMap`

### Cargo.toml parser (for MSRV check)
- **Crate:** `cargo-toml-parser` at `packages/cargo-toml-parser`
- **Function:** `cargo_toml_parser::parse(input: &str) -> Result<CargoToml, Error>`
- Already marked `shared = true`

## File selection logic

1. Find `rust-toolchain.toml` via `crawl.root_file("rust-toolchain.toml")`
2. If not found, return `Err(ToolchainTomlNotFound)`
3. For the MSRV check, ALSO find `Cargo.toml` via `crawl.root_file("Cargo.toml")`
4. If Cargo.toml not found, the MSRV input cannot be constructed — return `None` for it (not an error — a workspace without Cargo.toml just can't do MSRV checks)

## Public API

The ingestion should expose TWO functions (or one function returning a struct with both):

**Option A (recommended — simpler):** One function returning a result struct:

```rust
pub struct G3RsToolchainConfigIngestionResult {
    pub channel_components: G3RsToolchainConfigChannelComponentsInput,
    pub msrv_consistency: Option<G3RsToolchainConfigMsrvConsistencyInput>,
}

pub fn ingest(crawl: &G3RsWorkspaceCrawl) -> Result<G3RsToolchainConfigIngestionResult, IngestionError>
```

`msrv_consistency` is `None` when Cargo.toml is missing (not an error, just means MSRV check can't run).

## Package structure

```
packages/g3rs-toolchain-config-ingestion/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── TODO.md
├── src/lib.rs
├── crates/
│   ├── types/
│   │   ├── Cargo.toml            # deps: cargo-toml-parser, rust-toolchain-toml-parser,
│   │   │                         #       g3rs-toolchain-config-checks
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs          # G3RsToolchainConfigIngestionError
│   │       └── result.rs         # G3RsToolchainConfigIngestionResult
│   ├── runtime/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── fs.rs
│   │       ├── select.rs         # select toolchain + cargo from crawl
│   │       ├── parse.rs          # parse both files
│   │       ├── ingest.rs         # assemble both input types
│   │       ├── run.rs            # pub fn ingest()
│   │       └── ingest_tests/
│   │           ├── mod.rs
│   │           ├── deps.rs
│   │           └── basic.rs
│   └── assertions/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── common.rs
```

## Error type

```rust
use std::path::PathBuf;

#[derive(Debug)]
pub enum G3RsToolchainConfigIngestionError {
    ToolchainTomlNotFound,
    Unreadable { path: PathBuf, reason: String },
    ParseFailed { path: PathBuf, reason: String },
}

impl std::fmt::Display for G3RsToolchainConfigIngestionError { ... }
impl std::error::Error for G3RsToolchainConfigIngestionError {}
```

## Result type

```rust
use g3rs_toolchain_config_checks::{
    G3RsToolchainConfigChannelComponentsInput,
    G3RsToolchainConfigMsrvConsistencyInput,
};

#[derive(Debug)]
pub struct G3RsToolchainConfigIngestionResult {
    pub channel_components: G3RsToolchainConfigChannelComponentsInput,
    pub msrv_consistency: Option<G3RsToolchainConfigMsrvConsistencyInput>,
}
```

This goes in `crates/types/src/result.rs`. The types crate needs dependencies on the checks types and parser types for this.

## Runtime logic

### select.rs
```rust
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

pub(crate) fn select_toolchain_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("rust-toolchain.toml")
}

pub(crate) fn select_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}
```

### parse.rs
```rust
pub(crate) fn parse_toolchain_toml(abs_path: &Path) -> Result<RustToolchainToml, IngestionError> { ... }
pub(crate) fn parse_cargo_toml(abs_path: &Path) -> Result<CargoToml, IngestionError> { ... }
```

### ingest.rs
```rust
pub(crate) fn assemble_channel_components(...) -> G3RsToolchainConfigChannelComponentsInput { ... }
pub(crate) fn assemble_msrv_consistency(...) -> G3RsToolchainConfigMsrvConsistencyInput { ... }
```

### run.rs
```rust
pub fn ingest(crawl: &G3RsWorkspaceCrawl) -> Result<G3RsToolchainConfigIngestionResult, IngestionError> {
    // 1. Select and parse rust-toolchain.toml (required)
    let toolchain_entry = crate::select::select_toolchain_toml(crawl)
        .ok_or(IngestionError::ToolchainTomlNotFound)?;
    if !toolchain_entry.readable { return Err(Unreadable...); }
    let toolchain = crate::parse::parse_toolchain_toml(&toolchain_entry.path.abs_path)?;
    let toolchain_rel_path = toolchain_entry.path.rel_path.clone();

    // 2. Build channel_components input (always)
    let channel_components = crate::ingest::assemble_channel_components(
        toolchain_rel_path.clone(), toolchain.clone()
    );

    // 3. Try to select and parse Cargo.toml (optional for MSRV)
    let msrv_consistency = if let Some(cargo_entry) = crate::select::select_cargo_toml(crawl) {
        if cargo_entry.readable {
            if let Ok(cargo) = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path) {
                Some(crate::ingest::assemble_msrv_consistency(
                    toolchain_rel_path, toolchain,
                    cargo_entry.path.rel_path.clone(), cargo,
                ))
            } else { None }
        } else { None }
    } else { None };

    Ok(G3RsToolchainConfigIngestionResult { channel_components, msrv_consistency })
}
```

## Dependencies

### Runtime Cargo.toml
```toml
[dependencies]
cargo-toml-parser = { path = "../../../cargo-toml-parser", version = "0.1.0" }
g3rs-toolchain-config-checks = { path = "../../../g3rs-toolchain-config-checks", version = "0.1.0" }
g3rs-toolchain-config-ingestion-types = { path = "../types", version = "0.1.0" }
g3rs-workspace-crawl = { path = "../../../g3rs-workspace-crawl", version = "0.1.0" }
rust-toolchain-toml-parser = { path = "../../../rust-toolchain-toml-parser", version = "0.1.0" }

[dev-dependencies]
g3rs-toolchain-config-ingestion-assertions = { path = "../assertions" }
tempfile = "3"
```

### Types Cargo.toml
```toml
[dependencies]
g3rs-toolchain-config-checks = { path = "../../../g3rs-toolchain-config-checks", version = "0.1.0" }
```
(Needs the checks types for the result struct fields.)

## Packages that need `shared = true` added

Add `[package.metadata.guardrail3]\nshared = true` to:
- `packages/rust-toolchain-toml-parser/Cargo.toml`
- `packages/g3rs-toolchain-config-checks/Cargo.toml`

## Root facade exports

```rust
// src/lib.rs
#[cfg(feature = "api")]
pub use g3rs_toolchain_config_ingestion_runtime::ingest;
#[cfg(feature = "api")]
pub use g3rs_toolchain_config_ingestion_types::{
    G3RsToolchainConfigIngestionError,
    G3RsToolchainConfigIngestionResult,
};
```

## Tests (in ingest_tests/basic.rs)

1. **`ingests_toolchain_toml`** — write `rust-toolchain.toml` with `[toolchain]\nchannel = "1.85.0"`. Assert `channel_components` has the right path and parsed content.

2. **`ingests_with_cargo_for_msrv`** — write both `rust-toolchain.toml` and `Cargo.toml`. Assert `msrv_consistency` is `Some(...)`.

3. **`msrv_is_none_without_cargo_toml`** — write only `rust-toolchain.toml`. Assert `msrv_consistency` is `None` and `channel_components` still succeeds.

4. **`fails_when_toolchain_toml_is_missing`** — no toolchain file. Assert `Err(ToolchainTomlNotFound)`.

5. **`fails_on_malformed_toolchain_toml`** — write invalid TOML. Assert `Err(ParseFailed { .. })`.

6. **`ignored_but_recovered_toolchain_toml_is_ingested`** — gitignore it, write it. Assert ingestion succeeds (recovered by crawl).

## Strict constraints

Same as clippy handoff — all workspace lints, facade-only lib.rs/mod.rs, fs.rs boundary, strong expect messages, feature gates.

## Verification

Same pattern — `cargo test` + `rs validate` with arch+code families, filter to ingestion package, zero errors/warnings.
