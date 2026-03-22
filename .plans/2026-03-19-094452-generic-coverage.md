# Generic coverage map engine

**Date:** 2026-03-19 09:44
**Task:** Extract common walk-up + tree logic from clippy/deny/rustfmt into a shared engine. Each tool plugs in its specifics.

## Design

### Generic inputs
- `config_files: Vec<PathBuf>` — all instances of this config type found by crawler
- `targets: Vec<Target>` — all things that need coverage (crates or app dirs)
- `resolution: Resolution` — WalkUp or Fixed (for jscpd/gitleaks)
- `detail_parser: fn(&Path) -> Option<Details>` — tool-specific config summary

### Target
```rust
struct Target {
    name: String,
    path: PathBuf,            // directory path relative to root
    structure_file: PathBuf,  // Cargo.toml or package.json that defines this target
}
```

### Generic output
```rust
struct CoverageMap {
    tool: String,
    resolution: String,
    project: String,
    configs: Vec<ConfigInstance>,  // every config file found, with parsed details
    targets: Vec<TargetCoverage>, // every target with its resolved coverage
    summary: Summary,
}

struct ConfigInstance {
    path: String,
    details: serde_json::Value,  // tool-specific (methods/types, bans, settings, etc.)
    covers: Vec<String>,         // which target paths this config covers
}

struct TargetCoverage {
    name: String,
    path: String,
    structure_file: String,
    covered_by: Option<String>,  // path to the config that covers this target
    is_shadow: bool,             // true if covered by a non-root config that intercepts walk-up
}
```

### Walk-up function
```rust
fn resolve_coverage(
    target_dir: &Path,
    project_root: &Path,
    config_files: &[PathBuf],
) -> Option<PathBuf> {
    let mut current = target_dir;
    loop {
        if config_files.iter().any(|p| p.parent() == Some(current)) {
            return Some(current.to_path_buf());
        }
        if current == project_root {
            return None;
        }
        current = current.parent()?;
    }
}
```
Same for every walk-up tool. For non-walk-up tools (jscpd, gitleaks): just check CWD/target path directly.

### Tree renderer
One shared tree renderer that takes CoverageMap and prints the hierarchical output. Tool modules don't need to implement their own tree rendering.

### JSON serializer
CoverageMap is Serialize — one `serde_json::to_string_pretty` for all tools.

## What each tool module provides

```rust
trait CoverageTool {
    fn name(&self) -> &str;
    fn resolution_description(&self) -> &str;
    fn config_files(&self, crawl: &CrawlResult) -> Vec<PathBuf>;
    fn targets(&self, crawl: &CrawlResult, root: &Path) -> Vec<Target>;
    fn parse_details(&self, config_path: &Path) -> Option<serde_json::Value>;
    fn walks_up(&self) -> bool;
}
```

Each tool implements this trait. The engine calls it.

## Files
- `src/commands/coverage/engine.rs` — generic CoverageMap, walk-up, tree renderer, JSON
- `src/commands/coverage/clippy.rs` — implements CoverageTool for clippy
- `src/commands/coverage/deny.rs` — implements CoverageTool for deny
- `src/commands/coverage/rustfmt.rs` — implements CoverageTool for rustfmt
- `src/commands/coverage/eslint.rs` — NEW: implements CoverageTool for eslint
- etc.
