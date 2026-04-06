# Handoff: Build g3rs-fmt-config-ingestion

## What this package does

Takes a `G3RsWorkspaceCrawl`, finds THREE files — `rustfmt.toml`/`.rustfmt.toml`, `Cargo.toml`, and `rust-toolchain.toml` — reads and parses all three, and returns `G3RsFmtConfigChecksInput` for the checks package.

This is the MOST COMPLEX ingestion package because the checks input requires three parsed files.

## Output type (the contract you must produce)

```rust
// Defined in packages/g3rs-fmt-config-checks/crates/types/src/lib.rs
pub struct G3RsFmtConfigChecksInput {
    pub rustfmt_rel_path: String,       // e.g. "rustfmt.toml" or ".rustfmt.toml"
    pub rustfmt: RustfmtToml,           // Parsed rustfmt config
    pub cargo_rel_path: String,         // e.g. "Cargo.toml"
    pub cargo: CargoToml,               // Parsed Cargo manifest
    pub toolchain_rel_path: String,     // e.g. "rust-toolchain.toml"
    pub toolchain: RustToolchainToml,   // Parsed rust-toolchain manifest
}
```

ALL THREE files are REQUIRED. If any is missing, the ingestion fails.

## Input type (the contract you consume)

```rust
G3RsWorkspaceCrawl — call root_file() for each of the three file names
```

## Parsers

### rustfmt.toml parser
- **Crate:** `rustfmt-toml-parser` at `packages/rustfmt-toml-parser`
- **Function:** `rustfmt_toml_parser::parse(input: &str) -> Result<RustfmtToml, Error>`
- **Type:** `RustfmtToml` — 63+ optional fields, `#[serde(flatten)]` for extras

### Cargo.toml parser
- **Crate:** `cargo-toml-parser` at `packages/cargo-toml-parser`
- **Function:** `cargo_toml_parser::parse(input: &str) -> Result<CargoToml, Error>`
- Already marked `shared = true`

### rust-toolchain.toml parser
- **Crate:** `rust-toolchain-toml-parser` at `packages/rust-toolchain-toml-parser`
- **Function:** `rust_toolchain_toml_parser::parse(input: &str) -> Result<RustToolchainToml, Error>`

## File selection logic

1. Find rustfmt config: try `crawl.root_file("rustfmt.toml")`, then `crawl.root_file(".rustfmt.toml")`
2. Find Cargo.toml: `crawl.root_file("Cargo.toml")`
3. Find toolchain: `crawl.root_file("rust-toolchain.toml")`
4. ALL THREE must be found — if any is missing, return the appropriate error variant

## Package structure

```
packages/g3rs-fmt-config-ingestion/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── TODO.md
├── src/lib.rs
├── crates/
│   ├── types/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── error.rs          # G3RsFmtConfigIngestionError
│   ├── runtime/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── fs.rs
│   │       ├── select.rs         # select all 3 files from crawl
│   │       ├── parse.rs          # parse all 3 files
│   │       ├── ingest.rs         # assemble G3RsFmtConfigChecksInput
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
pub enum G3RsFmtConfigIngestionError {
    /// No `rustfmt.toml` or `.rustfmt.toml` found.
    RustfmtTomlNotFound,
    /// No `Cargo.toml` found.
    CargoTomlNotFound,
    /// No `rust-toolchain.toml` found.
    ToolchainTomlNotFound,
    /// A required file exists but cannot be read.
    Unreadable { path: PathBuf, reason: String },
    /// A required file could not be parsed.
    ParseFailed { path: PathBuf, reason: String },
}

impl std::fmt::Display for G3RsFmtConfigIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RustfmtTomlNotFound => f.write_str("no rustfmt.toml or .rustfmt.toml found at the workspace root"),
            Self::CargoTomlNotFound => f.write_str("no Cargo.toml found at the workspace root"),
            Self::ToolchainTomlNotFound => f.write_str("no rust-toolchain.toml found at the workspace root"),
            Self::Unreadable { path, reason } => write!(f, "cannot read {}: {reason}", path.display()),
            Self::ParseFailed { path, reason } => write!(f, "cannot parse {}: {reason}", path.display()),
        }
    }
}
impl std::error::Error for G3RsFmtConfigIngestionError {}
```

## Runtime logic

### select.rs
```rust
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

pub(crate) fn select_rustfmt_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("rustfmt.toml")
        .or_else(|| crawl.root_file(".rustfmt.toml"))
}

pub(crate) fn select_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}

pub(crate) fn select_toolchain_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("rust-toolchain.toml")
}
```

### parse.rs
Three parse functions, each following the same pattern:

```rust
pub(crate) fn parse_rustfmt_toml(abs_path: &Path) -> Result<RustfmtToml, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|err| {
        IngestionError::Unreadable { path: abs_path.to_path_buf(), reason: err.to_string() }
    })?;
    rustfmt_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(), reason: err.to_string(),
    })
}

pub(crate) fn parse_cargo_toml(abs_path: &Path) -> Result<CargoToml, IngestionError> { ... }
pub(crate) fn parse_toolchain_toml(abs_path: &Path) -> Result<RustToolchainToml, IngestionError> { ... }
```

### ingest.rs
```rust
pub(crate) fn assemble(
    rustfmt_rel_path: String, rustfmt: RustfmtToml,
    cargo_rel_path: String, cargo: CargoToml,
    toolchain_rel_path: String, toolchain: RustToolchainToml,
) -> G3RsFmtConfigChecksInput {
    G3RsFmtConfigChecksInput {
        rustfmt_rel_path, rustfmt,
        cargo_rel_path, cargo,
        toolchain_rel_path, toolchain,
    }
}
```

### run.rs
```rust
pub fn ingest(crawl: &G3RsWorkspaceCrawl) -> Result<G3RsFmtConfigChecksInput, IngestionError> {
    // 1. Select rustfmt config (required)
    let rustfmt_entry = crate::select::select_rustfmt_toml(crawl)
        .ok_or(IngestionError::RustfmtTomlNotFound)?;
    if !rustfmt_entry.readable {
        return Err(IngestionError::Unreadable {
            path: rustfmt_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    // 2. Select Cargo.toml (required)
    let cargo_entry = crate::select::select_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;
    if !cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    // 3. Select rust-toolchain.toml (required)
    let toolchain_entry = crate::select::select_toolchain_toml(crawl)
        .ok_or(IngestionError::ToolchainTomlNotFound)?;
    if !toolchain_entry.readable {
        return Err(IngestionError::Unreadable {
            path: toolchain_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    // 4. Parse all three
    let rustfmt = crate::parse::parse_rustfmt_toml(&rustfmt_entry.path.abs_path)?;
    let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;
    let toolchain = crate::parse::parse_toolchain_toml(&toolchain_entry.path.abs_path)?;

    // 5. Assemble
    Ok(crate::ingest::assemble(
        rustfmt_entry.path.rel_path.clone(), rustfmt,
        cargo_entry.path.rel_path.clone(), cargo,
        toolchain_entry.path.rel_path.clone(), toolchain,
    ))
}
```

## Dependencies

### Runtime Cargo.toml
```toml
[dependencies]
cargo-toml-parser = { path = "../../../cargo-toml-parser", version = "0.1.0" }
g3rs-fmt-config-checks = { path = "../../../g3rs-fmt-config-checks", version = "0.1.0" }
g3rs-fmt-config-ingestion-types = { path = "../types", version = "0.1.0" }
g3rs-workspace-crawl = { path = "../../../g3rs-workspace-crawl", version = "0.1.0" }
rust-toolchain-toml-parser = { path = "../../../rust-toolchain-toml-parser", version = "0.1.0" }
rustfmt-toml-parser = { path = "../../../rustfmt-toml-parser", version = "0.1.0" }

[dev-dependencies]
g3rs-fmt-config-ingestion-assertions = { path = "../assertions" }
tempfile = "3"
```

### Types Cargo.toml
No external dependencies (error type only uses std::path::PathBuf).

## Packages that need `shared = true` added

Add `[package.metadata.guardrail3]\nshared = true` to:
- `packages/rustfmt-toml-parser/Cargo.toml`
- `packages/g3rs-fmt-config-checks/Cargo.toml`
- `packages/rust-toolchain-toml-parser/Cargo.toml` (if not already done by toolchain ingestion agent)

## Tests (in ingest_tests/basic.rs)

Every test: create tempdir, `git_init(root)`, write ALL THREE fixture files (unless testing missing-file case), `crawl(root)`, `ingest(&crawl)`, assert.

**Valid fixture content for the three files:**

```rust
// Minimal valid rustfmt.toml
"edition = \"2024\"\n"

// Minimal valid Cargo.toml
"[workspace]\nmembers = []\nresolver = \"2\"\n"

// Minimal valid rust-toolchain.toml
"[toolchain]\nchannel = \"1.85.0\"\n"
```

### Tests:

1. **`ingests_all_three_files`** — write all three. Assert ingestion succeeds, all three rel_paths correct, parsed content accessible.

2. **`prefers_rustfmt_toml_over_dot_variant`** — write both `rustfmt.toml` and `.rustfmt.toml` plus the other two. Assert `input.rustfmt_rel_path == "rustfmt.toml"`.

3. **`ingests_dot_rustfmt_toml`** — write only `.rustfmt.toml` (no `rustfmt.toml`). Assert `input.rustfmt_rel_path == ".rustfmt.toml"`.

4. **`fails_when_rustfmt_toml_missing`** — write Cargo.toml + rust-toolchain.toml but no rustfmt config. Assert `Err(RustfmtTomlNotFound)`.

5. **`fails_when_cargo_toml_missing`** — write rustfmt.toml + rust-toolchain.toml but no Cargo.toml. Assert `Err(CargoTomlNotFound)`.

6. **`fails_when_toolchain_toml_missing`** — write rustfmt.toml + Cargo.toml but no rust-toolchain.toml. Assert `Err(ToolchainTomlNotFound)`.

7. **`fails_on_malformed_rustfmt_toml`** — write invalid TOML as rustfmt.toml plus valid other two. Assert `Err(ParseFailed { .. })`.

## Strict constraints

Same as the other two handoffs. All workspace lints, facade-only lib.rs/mod.rs, fs.rs boundary, strong expect messages, feature gates (`api` for types, `ingest` for runtime, `ingest` for assertions).

## Verification

Same pattern — `cargo test` + `rs validate` with arch+code families, filter to ingestion package, zero errors/warnings.
