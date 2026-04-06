# Handoff: Build g3rs-clippy-config-ingestion

## What this package does

Takes a `G3RsWorkspaceCrawl` (neutral filesystem snapshot), finds `clippy.toml` or `.clippy.toml` at the workspace root, reads and parses it with `clippy-toml-parser`, and returns `G3RsClippyConfigChecksInput` for the checks package.

## Output type (the contract you must produce)

```rust
// Defined in packages/g3rs-clippy-config-checks/crates/types/src/lib.rs
pub struct G3RsClippyConfigChecksInput {
    pub clippy_rel_path: String,    // e.g. ".clippy.toml" or "clippy.toml"
    pub clippy: ClippyToml,         // Parsed clippy config
}
```

## Input type (the contract you consume)

```rust
// From packages/g3rs-workspace-crawl/crates/types/
G3RsWorkspaceCrawl — call crawl.root_file("clippy.toml") and crawl.root_file(".clippy.toml")
```

## Parser

- **Crate:** `clippy-toml-parser` at `packages/clippy-toml-parser`
- **Function:** `clippy_toml_parser::parse(input: &str) -> Result<ClippyToml, Error>`
- **Type:** `ClippyToml` — 100+ optional fields, `#[serde(deny_unknown_fields)]`
- **Error:** `clippy_toml_parser::Error` with variants `Toml(String)` and `Io(String)`

## File selection logic

1. Try `crawl.root_file("clippy.toml")` first
2. If not found, try `crawl.root_file(".clippy.toml")`
3. If neither found, return `Err(ClippyTomlNotFound)`
4. If the found entry has `readable: false`, return `Err(Unreadable { path, reason })`

This is the ONLY difference from g3rs-cargo-config-ingestion — two possible file names instead of one.

## Package structure

Copy EXACTLY from `packages/g3rs-cargo-config-ingestion/`. Same crate layout:

```
packages/g3rs-clippy-config-ingestion/
├── Cargo.toml                    # facade — see template below
├── Cargo.lock                    # auto-generated
├── README.md
├── TODO.md
├── src/lib.rs                    # re-exports ingest() + error behind "api" feature
├── crates/
│   ├── types/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # facade: mod error; pub use error::...
│   │       └── error.rs          # G3RsClippyConfigIngestionError
│   ├── runtime/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # mod fs; mod ingest; mod parse; mod run; mod select;
│   │       ├── fs.rs             # pub(crate) fn read_to_string(path) -> io::Result<String>
│   │       ├── select.rs         # select clippy config from crawl
│   │       ├── parse.rs          # read + clippy_toml_parser::parse()
│   │       ├── ingest.rs         # assemble G3RsClippyConfigChecksInput
│   │       ├── run.rs            # pub fn ingest(&G3RsWorkspaceCrawl) -> Result<Input, Error>
│   │       └── ingest_tests/
│   │           ├── mod.rs
│   │           ├── deps.rs       # use g3rs_clippy_config_ingestion_assertions as _;
│   │           └── basic.rs      # tests
│   └── assertions/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── common.rs
```

## Error type

```rust
// In crates/types/src/error.rs
use std::path::PathBuf;

#[derive(Debug)]
pub enum G3RsClippyConfigIngestionError {
    ClippyTomlNotFound,
    Unreadable {
        path: PathBuf,
        reason: String,
    },
    ParseFailed {
        path: PathBuf,
        reason: String,
    },
}

impl std::fmt::Display for G3RsClippyConfigIngestionError { ... }
impl std::error::Error for G3RsClippyConfigIngestionError {}
```

## Runtime logic

### select.rs
```rust
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

pub(crate) fn select_clippy_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("clippy.toml")
        .or_else(|| crawl.root_file(".clippy.toml"))
}
```

### parse.rs
```rust
use clippy_toml_parser::ClippyToml;
use crate::run::IngestionError;

pub(crate) fn parse_clippy_toml(abs_path: &std::path::Path) -> Result<ClippyToml, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|err| {
        IngestionError::Unreadable { path: abs_path.to_path_buf(), reason: err.to_string() }
    })?;
    clippy_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(), reason: err.to_string(),
    })
}
```

### ingest.rs
```rust
use clippy_toml_parser::ClippyToml;
use g3rs_clippy_config_checks::G3RsClippyConfigChecksInput;

pub(crate) fn assemble(clippy_rel_path: String, clippy: ClippyToml) -> G3RsClippyConfigChecksInput {
    G3RsClippyConfigChecksInput { clippy_rel_path, clippy }
}
```

### run.rs
```rust
use g3rs_clippy_config_checks::G3RsClippyConfigChecksInput;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
pub use g3rs_clippy_config_ingestion_types::G3RsClippyConfigIngestionError as IngestionError;

pub fn ingest(crawl: &G3RsWorkspaceCrawl) -> Result<G3RsClippyConfigChecksInput, IngestionError> {
    let entry = crate::select::select_clippy_toml(crawl)
        .ok_or(IngestionError::ClippyTomlNotFound)?;
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let clippy = crate::parse::parse_clippy_toml(&entry.path.abs_path)?;
    let clippy_rel_path = entry.path.rel_path.clone();
    Ok(crate::ingest::assemble(clippy_rel_path, clippy))
}
```

## Dependencies

### Root facade Cargo.toml
```toml
[dependencies]
g3rs-clippy-config-ingestion-runtime = { path = "crates/runtime", version = "0.1.0", optional = true }
g3rs-clippy-config-ingestion-types = { path = "crates/types", version = "0.1.0", optional = true }
```

### Runtime Cargo.toml
```toml
[dependencies]
clippy-toml-parser = { path = "../../../clippy-toml-parser", version = "0.1.0" }
g3rs-clippy-config-checks = { path = "../../../g3rs-clippy-config-checks", version = "0.1.0" }
g3rs-clippy-config-ingestion-types = { path = "../types", version = "0.1.0" }
g3rs-workspace-crawl = { path = "../../../g3rs-workspace-crawl", version = "0.1.0" }

[dev-dependencies]
g3rs-clippy-config-ingestion-assertions = { path = "../assertions" }
tempfile = "3"
```

### Types Cargo.toml
No external dependencies. Just `[lints] workspace = true` and `[package.metadata.guardrail3] shared = true`.

### Assertions Cargo.toml
```toml
[dependencies]
g3rs-clippy-config-ingestion-types = { path = "../types" }
```

## Packages that need `shared = true` added

Add `[package.metadata.guardrail3]\nshared = true` to these Cargo.toml files (BEFORE `[package.metadata.docs.rs]` if it exists):
- `packages/clippy-toml-parser/Cargo.toml`
- `packages/g3rs-clippy-config-checks/Cargo.toml`

## Tests (in ingest_tests/basic.rs)

Every test: create tempdir, `git_init(root)`, write fixtures, `crawl(root)`, `ingest(&crawl)`, assert.

1. **`ingests_clippy_toml`** — write `clippy.toml` with valid content (e.g. `msrv = "1.85"`). Assert ingestion succeeds and `input.clippy_rel_path == "clippy.toml"`.

2. **`ingests_dot_clippy_toml`** — write `.clippy.toml` only. Assert `input.clippy_rel_path == ".clippy.toml"`.

3. **`prefers_clippy_toml_over_dot_variant`** — write BOTH `clippy.toml` and `.clippy.toml`. Assert `input.clippy_rel_path == "clippy.toml"` (non-dot preferred).

4. **`fails_when_clippy_toml_is_missing`** — no clippy config. Assert `Err(ClippyTomlNotFound)`.

5. **`fails_on_malformed_clippy_toml`** — write invalid TOML. Assert `Err(ParseFailed { .. })`.

6. **`ignored_but_recovered_clippy_toml_is_ingested`** — gitignore `clippy.toml`, write it. Assert ingestion succeeds (recovered by crawl).

## Strict constraints

- All workspace lints apply (deny warnings, forbid unsafe, deny unused_results, deny expect_used, etc.)
- Use `let _status = Command::new("git")...` for git_init (unused result)
- Strong expect messages on all `.expect()` calls — explain the failure, don't just name the file
- Feature gates: types `api`, runtime `ingest`, assertions `ingest`
- `lib.rs` must be facade-only (no imports, only `mod` and `pub use` behind `#[cfg(feature)]`)
- `mod.rs` must be facade-only (only `mod` declarations)
- Route all `std::fs` through `fs.rs`
- Dev-dependency anchor: `use g3rs_clippy_config_ingestion_assertions as _;` in `ingest_tests/deps.rs`

## Verification

After building, run:
```
cargo test --workspace --manifest-path packages/g3rs-clippy-config-ingestion/Cargo.toml
cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate packages/g3rs-clippy-config-ingestion --family arch --family code --format json | jq '.sections[] | {name, results: [.results[] | select(.file_relative | startswith("packages/g3rs-clippy-config-ingestion/")) | select(.severity == "error" or .severity == "warn")]}'
```

Both must be clean (zero errors/warnings from the ingestion package itself).
