# Fix g3rs-workspace-crawl: correct ignore semantics + targeted recovery

## Goal
Replace the broken single-gitignore walker with correct two-phase crawl:
1. `ignore::WalkBuilder` for correct ancestor+nested gitignore semantics
2. Targeted recovery of ignored-but-relevant files (config, manifests)

## Files

### Delete
- `runtime/src/ignore.rs` — single-root Gitignore builder, replaced by WalkBuilder

### Rewrite
- `runtime/src/crawl.rs` — two-phase: WalkBuilder walk + recovery walk
- `runtime/src/support.rs` — build_entry takes ignore state directly, no Gitignore param

### New
- `runtime/src/recovery.rs` — recovery list, banned dirs, should_recover()

### Modify
- `runtime/src/lib.rs` — mod ignore → mod recovery

### Unchanged
- `runtime/src/run.rs`
- `runtime/src/fs.rs`
- `runtime/Cargo.toml` (already has ignore + walkdir deps)
- `types/` crate entirely
- `assertions/` crate entirely

### Tests
- `crawl_tests/ignore_state.rs` — rewrite for nested/ancestor gitignore
- `crawl_tests/hidden_files.rs` — adjust if needed
- `crawl_tests/queries.rs` — unchanged

## Logic

### Phase 1: WalkBuilder
```
WalkBuilder::new(workspace_root)
    .hidden(false)        // dotfiles are normal entries
    .git_ignore(true)     // respect gitignore
    .git_global(false)    // no machine-local globals
    .git_exclude(false)   // no .git/info/exclude
    .parents(true)        // ancestor gitignores
    .ignore(false)        // no .ignore files
    .follow_links(false)  // skip symlinks
```
All emitted entries → Included. Collect paths into HashSet for phase 2.

### Phase 2: Recovery
- walkdir::WalkDir from workspace_root
- Skip banned dirs (.git, target, node_modules)
- For each file not in phase 1 set AND matching recovery list → add as Ignored
- Recovery list: config/manifest file names from old app's should_cache/should_recover_ignored

## Key decisions
- No global git excludes, no .git/info/exclude — validation must be machine-independent
- Recovery list is hardcoded data, not pluggable — keeps API simple
- Recovery walk skips banned dirs to avoid target/node_modules bloat
