# Extract g3rs-release-config-checks

## Goal

Extract the config-only rules from the release family into `g3rs-release-config-checks`. Unify the RS-BIN/RS-PUB/RS-RELEASE naming into one `RS-RELEASE-CONFIG-XX` sequence. Follow the same pattern as every other checks package: one input type, one check function.

## Rules to extract

### Per-crate checks (need only CargoToml)

All of these read parsed `[package]` fields from Cargo.toml. They currently use a `PublishableCrateFacts` struct full of pre-computed booleans. In the extracted package, the checks should compute these directly from CargoToml.

| Old ID | New ID | What it checks |
|--------|--------|----------------|
| RS-PUB-01 | RS-RELEASE-CONFIG-01 | `[package].description` present |
| RS-PUB-02 | RS-RELEASE-CONFIG-02 | `[package].license` or `license-file` present |
| RS-PUB-03 | RS-RELEASE-CONFIG-03 | `[package].repository` present |
| RS-PUB-06 | RS-RELEASE-CONFIG-04 | `[package].keywords` present (1-5 entries) |
| RS-PUB-07 | RS-RELEASE-CONFIG-05 | `[package].categories` present |
| RS-PUB-08 | RS-RELEASE-CONFIG-06 | `[package].version` is valid semver |
| RS-PUB-13 | RS-RELEASE-CONFIG-07 | `[package.metadata.docs.rs]` present (libraries only) |
| RS-BIN-03 | RS-RELEASE-CONFIG-08 | `[package.metadata.binstall]` present (binaries only) |
| RS-RELEASE-11 | RS-RELEASE-CONFIG-09 | Accidentally publishable (no `publish = false` but missing description+license+repo) |

### Per-repo checks (need release-plz.toml and cliff.toml)

| Old ID | New ID | What it checks |
|--------|--------|----------------|
| RS-RELEASE-03 | RS-RELEASE-CONFIG-10 | release-plz.toml baseline (workspace section, changelog_config, git_release_enable) |
| RS-RELEASE-04 | RS-RELEASE-CONFIG-11 | cliff.toml baseline (git section, conventional_commits, commit_parsers) |

### NOT extracting (mixed with filetree/tool)

- RS-PUB-04, RS-PUB-05: Need README file existence and content — filetree, stays in app
- RS-PUB-09, RS-PUB-10, RS-PUB-11, RS-PUB-12, RS-PUB-14: Mixed checks — stay in app
- RS-BIN-01, RS-BIN-02: Workflow presence — filetree, stays in app
- RS-RELEASE-01, RS-RELEASE-02: File existence — filetree, stays in app
- RS-RELEASE-05 through RS-RELEASE-10, RS-RELEASE-12: Mixed/tool — stay in app
- RS-RELEASE-08: Tool installation — stays in app

## Input type

```rust
pub struct G3RsReleaseConfigChecksInput {
    /// Repo-relative path to the crate `Cargo.toml` being checked.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest.
    pub cargo: CargoToml,
    /// Parsed `release-plz.toml` content, if present at the workspace root.
    pub release_plz: Option<toml::Value>,
    /// Repo-relative path to `release-plz.toml`, if present.
    pub release_plz_rel_path: Option<String>,
    /// Parsed `cliff.toml` content, if present at the workspace root.
    pub cliff: Option<toml::Value>,
    /// Repo-relative path to `cliff.toml`, if present.
    pub cliff_rel_path: Option<String>,
}
```

No new parser crates needed. release-plz.toml and cliff.toml are simple enough to stay as `toml::Value`. The checks package depends on the `toml` crate directly for the `Value` type.

Per-crate checks (01-09) read only `cargo_rel_path` + `cargo`.
Per-repo checks (10-11) read the Optional release_plz/cliff fields.

The caller iterates per publishable crate, passing the same release_plz/cliff with each. Per-repo checks produce the same result each time — the caller or the app deduplicates.

Actually, better: per-repo checks should only fire when the Optional fields are Some. The caller passes Some for the first crate and None for subsequent ones, or deduplicates after.

## Package structure

Same as every other checks package:

```
packages/g3rs-release-config-checks/
├── Cargo.toml
├── src/lib.rs
├── crates/
│   ├── types/
│   │   ├── Cargo.toml     # deps: cargo-toml-parser, toml
│   │   └── src/lib.rs     # G3RsReleaseConfigChecksInput
│   ├── runtime/
│   │   ├── Cargo.toml     # deps: types, cargo-toml-parser, toml, guardrail3-check-types
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── run.rs     # pub fn check()
│   │       ├── support.rs # helper functions
│   │       ├── rs_release_config_01_description_present/
│   │       ├── rs_release_config_02_license_present/
│   │       ├── ... (one module per check)
│   │       ├── rs_release_config_11_cliff_baseline/
│   └── assertions/
│       └── ...
```

## Check logic (what each rule reads from CargoToml)

### RS-RELEASE-CONFIG-01: description present
```rust
cargo.package.as_ref().and_then(|p| p.description.as_ref()).is_some()
```

### RS-RELEASE-CONFIG-02: license present
```rust
cargo.package.as_ref().map_or(false, |p| {
    p.license.is_some() || p.license_file.is_some()
})
```

### RS-RELEASE-CONFIG-03: repository present
```rust
cargo.package.as_ref().and_then(|p| p.repository.as_ref()).is_some()
```

### RS-RELEASE-CONFIG-04: keywords (1-5)
```rust
cargo.package.as_ref().and_then(|p| p.keywords.as_ref()).map(|k| k.len())
// Error if None or 0 or >5
```

### RS-RELEASE-CONFIG-05: categories present
```rust
cargo.package.as_ref().and_then(|p| p.categories.as_ref()).map(|c| c.len())
// Error if None or 0
```

### RS-RELEASE-CONFIG-06: valid semver
```rust
// Check [package].version is valid semver
// Handle workspace inheritance (version.workspace = true → skip, it's valid)
```

### RS-RELEASE-CONFIG-07: docs.rs metadata (libraries only)
```rust
// Only check if crate has [lib] section
cargo.package.as_ref()
    .and_then(|p| p.metadata.as_ref())
    .and_then(|m| m.get("docs").and_then(|d| d.get("rs")))
    .is_some()
```

### RS-RELEASE-CONFIG-08: binstall metadata (binaries only)
```rust
// Only check if crate has [[bin]] entries
cargo.package.as_ref()
    .and_then(|p| p.metadata.as_ref())
    .and_then(|m| m.get("binstall"))
    .is_some()
```

### RS-RELEASE-CONFIG-09: accidentally publishable
```rust
// Warn if publish != false AND missing all of: description, license, repository
let publishable = cargo.package.as_ref()
    .map_or(true, |p| !matches!(p.publish, Some(VecStringOrBool::Bool(false))));
// If publishable && !description && !license && !repository → error
```

### RS-RELEASE-CONFIG-10: release-plz.toml baseline
```rust
// If input.release_plz is Some(value):
// Check value["workspace"] exists
// Check value["workspace"]["changelog_config"] == "cliff.toml"
// Check value["workspace"]["git_release_enable"] == true
```

### RS-RELEASE-CONFIG-11: cliff.toml baseline
```rust
// If input.cliff is Some(value):
// Check value["git"]["conventional_commits"] == true
// Check value["git"]["filter_unconventional"] == true
// Check value["git"]["commit_parsers"] covers required prefixes
```

## Publishability gate

Most per-crate checks should only fire for publishable crates. A crate is publishable when `[package].publish` is NOT `false`. The check function should compute this from CargoToml and skip non-publishable crates for checks 01-09.

## Dependencies

### types Cargo.toml
```toml
[dependencies]
cargo-toml-parser = { path = "../../../cargo-toml-parser", version = "0.1.0" }
toml = "0.8"
```

### runtime Cargo.toml
```toml
[dependencies]
cargo-toml-parser = { path = "../../../cargo-toml-parser", version = "0.1.0" }
g3rs-release-config-checks-types = { path = "../types", version = "0.1.0" }
guardrail3-check-types = { path = "../../../guardrail3-check-types/crates/guardrail3-check-types", version = "0.1.0" }
toml = "0.8"
```

## Ingestion

`g3rs-release-config-ingestion` would:
1. Select Cargo.toml from crawl (required)
2. Select release-plz.toml from crawl (optional)
3. Select cliff.toml from crawl (optional)
4. Parse all with appropriate parsers (cargo-toml-parser for Cargo.toml, toml::from_str for the other two)
5. Return G3RsReleaseConfigChecksInput

Both `release-plz.toml` and `cliff.toml` are already on the recovery list in g3rs-workspace-crawl.

## Test matrix

Golden fixture: a valid publishable Cargo.toml with all metadata + valid release-plz.toml + valid cliff.toml → all 11 checks produce Info.

Per-check tests: mutate the golden fixture to remove each field and verify the check fires.

Non-publishable fixture: Cargo.toml with `publish = false` → per-crate checks 01-09 produce nothing (skipped).
