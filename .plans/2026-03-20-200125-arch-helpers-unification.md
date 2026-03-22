# Arch Helpers Unification — RS + TS shared hex arch utilities

**Date:** 2026-03-20 20:01
**Depends on:** Rule 01 tests complete. Do this BEFORE rule 02 tests.

## Problem

The RS and TS hex arch checks have identical helper functions with different implementations:

| RS (correct) | TS (broken) | Difference |
|---|---|---|
| `list_dir_names(fs, dir)` → `fs.list_dir()` | `list_ts_dir_names(dir)` → `std::fs::read_dir()` | TS bypasses FileSystem trait |
| `list_file_names(fs, dir)` → `fs.list_dir()` | (inlined in `check_ts_loose_files`) | TS bypasses FileSystem trait |
| `has_gitkeep(fs, dir)` → `fs.read_file()` | (inlined: `fs.read_file(&dir.join(".gitkeep"))`) | TS gets this right (uses fs) |
| `check_loose_files(fs, name, dir, label, results)` | `check_ts_loose_files(fs, name, dir, label, results)` | Identical logic, different ID |

Additionally `has_ts_source_files(dir)` uses `WalkDir::new(dir)` directly (bypasses FileSystem). The RS equivalent doesn't exist — RS uses `Cargo.toml` presence for leaf validation, not recursive file scanning.

## What's identical (extractable)

These functions have **zero language-specific logic** — they operate on directories and paths:

```
list_dir_names(fs, dir) → Vec<String>        # subdirectory names
list_file_names(fs, dir) → Vec<String>        # non-directory entry names
has_gitkeep(fs, dir) → bool                   # .gitkeep exists
check_loose_files(fs, name, dir, label, id, results)  # report non-.gitkeep files
```

The only difference in `check_loose_files` is the check ID (`"R-ARCH-01"` vs `"T-ARCH-01"`) and the entity label (`"Service"` vs `"TS app"`). Both are already parameters or can be.

## What's language-specific (stays separate)

| Function | Why language-specific |
|---|---|
| `check_crates_dir` / `check_ts_modules_dir` | Different structural root, different expected dirs (`app` vs `application`), different recursion marker (`crates/` vs `modules/`) |
| `check_01_crates_exists` / modules existence | Different detection (Cargo.toml vs package.json), different severity (Error vs Warn) |
| `check_06_leaf_valid` / `validate_ts_container` | RS: leaf = has Cargo.toml. TS: leaf = has .ts/.tsx files. Fundamentally different. |
| `has_ts_source_files` | TS-only — RS uses Cargo.toml presence instead |
| `discover_ts_apps` / app discovery in RS | Completely different detection logic |

## Target structure

```
crates/app/
├── arch_helpers.rs                    # NEW — shared hex arch utilities
├── rs/validate/arch/rs_arch_01/
│   ├── mod.rs                         # uses arch_helpers::*
│   ├── helpers.rs                     # DELETE — contents moved to arch_helpers
│   ├── check_01..12                   # use arch_helpers::* instead of super::helpers
│   └── ...
├── ts/validate/
│   ├── ts_arch_checks.rs             # uses arch_helpers::*, deletes list_ts_dir_names etc.
│   └── ...
```

### `arch_helpers.rs`

```rust
use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// List subdirectory names in a directory.
pub fn list_dir_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    let entries = fs.list_dir(dir);
    let mut names = Vec::new();
    for entry in &entries {
        let Ok(ft) = entry.file_type() else { continue };
        if ft.is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names
}

/// List file names (non-directories) in a directory.
pub fn list_file_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    let entries = fs.list_dir(dir);
    let mut names = Vec::new();
    for entry in &entries {
        let Ok(ft) = entry.file_type() else { continue };
        if !ft.is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names
}

/// Check if a directory contains a `.gitkeep` file.
pub fn has_gitkeep(fs: &dyn FileSystem, dir: &Path) -> bool {
    fs.read_file(&dir.join(".gitkeep")).is_some()
}

/// Report loose files in a directory (only `.gitkeep` is allowed).
///
/// Parameters:
/// - `id`: Check ID (e.g., "R-ARCH-01" or "T-ARCH-01")
/// - `entity`: Entity label (e.g., "Service" or "TS app")
pub fn check_loose_files(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    id: &str,
    entity: &str,
    results: &mut Vec<CheckResult>,
) {
    let mut bad_files: Vec<String> = Vec::new();
    for entry in &fs.list_dir(dir) {
        let entry_name = entry.file_name().to_string_lossy().into_owned();
        let Ok(ft) = entry.file_type() else { continue };
        if !ft.is_dir() && entry_name != ".gitkeep" {
            bad_files.push(entry_name);
        }
    }

    if !bad_files.is_empty() {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Error,
            title: format!("{entity} `{name}` has loose files in {label}/"),
            message: format!(
                "{entity} `{name}` has files in `{label}/` that don't belong: {}. \
                 Only `.gitkeep` is allowed in structural/container directories. \
                 Move code into module subdirectories.",
                bad_files.join(", ")
            ),
            file: Some(dir.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// Check that a structural directory contains exactly the expected subdirectories.
///
/// Reports missing expected dirs and unexpected dirs.
pub fn check_exact_subdirs(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    expected: &[&str],
    id: &str,
    entity: &str,
    results: &mut Vec<CheckResult>,
) {
    let dir_names = list_dir_names(fs, dir);

    for exp in expected {
        if !dir_names.iter().any(|n| n == exp) {
            results.push(CheckResult {
                id: id.to_owned(),
                severity: Severity::Error,
                title: format!("{entity} `{name}` missing {label}/{exp}/ directory"),
                message: format!(
                    "{entity} `{name}` is missing `{label}/{exp}/`. \
                     Create it and add a `.gitkeep` if not needed yet."
                ),
                file: Some(dir.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    for dir_name in &dir_names {
        if !expected.contains(&dir_name.as_str()) {
            results.push(CheckResult {
                id: id.to_owned(),
                severity: Severity::Error,
                title: format!(
                    "{entity} `{name}` has unexpected directory {label}/{dir_name}/"
                ),
                message: format!(
                    "{entity} `{name}` has `{label}/{dir_name}/` which is not part of \
                     the hex arch template. Only `{{{}}}` directories are allowed in `{label}/`.",
                    expected.join(", ")
                ),
                file: Some(dir.join(dir_name).display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    check_loose_files(fs, name, dir, label, id, entity, results);
}

/// Check that a container is not empty (must have subdirs or .gitkeep).
/// Also calls `check_loose_files` on the container.
///
/// Design decision: when a container has files but no subdirs and no .gitkeep,
/// we report ONLY the "empty container" error (which lists the files in its
/// message). We do NOT also call check_loose_files in this case, to avoid
/// double-fire where the user gets two errors for the same files.
/// check_loose_files only runs when the container HAS subdirs (i.e., the
/// container is not empty, but has stray files alongside real crates).
pub fn check_container_not_empty(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    id: &str,
    entity: &str,
    results: &mut Vec<CheckResult>,
) {
    if fs.metadata(dir).is_none() {
        return; // missing dir already reported
    }

    let dir_names = list_dir_names(fs, dir);
    let gitkeep = has_gitkeep(fs, dir);

    if dir_names.is_empty() && !gitkeep {
        let files = list_file_names(fs, dir);
        let detail = if files.is_empty() {
            "is empty".to_owned()
        } else {
            format!(
                "contains files ({}) but no crate subdirectories",
                files.join(", ")
            )
        };
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Error,
            title: format!("{entity} `{name}` empty container {label}/"),
            message: format!(
                "{entity} `{name}` container `{label}/` {detail}. \
                 Each subdirectory must be a crate with its own `Cargo.toml`, \
                 or add a `.gitkeep` if this layer is not needed yet."
            ),
            file: Some(dir.display().to_string()),
            line: None,
            inventory: false,
        });
        // Do NOT call check_loose_files here — the empty-container error
        // already lists the offending files. Calling it would produce a
        // redundant "loose files" error for the same files (double-fire bug).
        return;
    }

    // Container has subdirs — check for loose files alongside them
    check_loose_files(fs, name, dir, label, id, entity, results);
}
```

## Design decisions made in this plan

### Double-fire fix (bug #4 from design audit)
`check_container_not_empty` now handles loose files INTERNALLY:
- If container is empty (no subdirs, no .gitkeep): reports "empty container" with file listing in message. Does NOT call `check_loose_files`. Returns early. One error, not two.
- If container has subdirs: calls `check_loose_files` to catch stray files alongside valid crates. This is the only path where loose files in containers get reported.

This means the orchestrator (`check_crates_dir` in mod.rs) does NOT need to call `check_loose_files` separately for containers. The `check_container_not_empty` function handles it.

### check_12 fix (bug #7 from design audit)
`check_12_src_banned` should use `fs.metadata()` instead of `list_dir()` to detect empty `src/` dirs. Fix during migration.

### check_01 fix (bug #3 from design audit)
`check_01_crates_exists` should use `fs.metadata()` to distinguish "missing" from "empty". Different error messages. Fix during migration.

## Migration steps

### Step 1: Create `arch_helpers.rs`
- New file at `crates/app/arch_helpers.rs`
- Add `pub mod arch_helpers;` to `crates/app/mod.rs`
- Contains: `list_dir_names`, `list_file_names`, `has_gitkeep`, `check_loose_files`, `check_exact_subdirs`, `check_container_not_empty`
- `check_loose_files` takes `id` and `entity` params for language-specific error IDs
- `check_container_not_empty` calls `check_loose_files` internally (only when container has subdirs) — fixes double-fire bug

### Step 2: Migrate RS-ARCH-01
- `rs_arch_01/helpers.rs` → DELETE (or reduce to re-exports of arch_helpers)
- `check_02_exact_contents.rs` → use `arch_helpers::check_exact_subdirs` with `id="R-ARCH-01"`, `entity="Service"`, `expected=["adapters","app","domain","ports"]`
- `check_03_inbound_outbound.rs` → use `arch_helpers::check_exact_subdirs` with `expected=["inbound","outbound"]`
- `check_05_container_not_empty.rs` → use `arch_helpers::check_container_not_empty`. DELETE the explicit `check_loose_files` call (now internal to check_container_not_empty)
- `check_01_crates_exists.rs` → fix bug #3: use `fs.metadata()` to distinguish missing vs empty
- `check_12_src_banned.rs` → fix bug #7: use `fs.metadata()` instead of `list_dir()`
- All remaining `helpers::list_dir_names` calls → `arch_helpers::list_dir_names`

### Step 3: Migrate TS-ARCH-01
- Delete `list_ts_dir_names` (line 382-392) — FIXES the std::fs bypass bug
- Delete `check_ts_loose_files` (line 345-378)
- `check_ts_modules_dir` → use `arch_helpers::check_exact_subdirs` with `id="T-ARCH-01"`, `entity="TS app"`, `expected=["adapters","application","domain","ports"]`
- `check_ts_inbound_outbound` → use `arch_helpers::check_exact_subdirs` with `expected=["inbound","outbound"]`
- `validate_ts_container` → use `arch_helpers::check_container_not_empty` (+ leaf validation stays TS-specific using `has_ts_source_files`)

### Step 4: Fix bugs identified by design audit
- Bug #3: `check_01_crates_exists` — metadata() to distinguish missing vs empty
- Bug #7: `check_12_src_banned` — metadata() to detect empty src/
- Bug #4: double-fire — already fixed by check_container_not_empty design
- Bug #5: has_gitkeep case sensitivity — normalize comparison in arch_helpers

### Step 5: Update tests
- Tests that assert double-fire behavior (e.g., `container_with_only_loose_files_double_error` in rule_05) need to be updated to expect SINGLE error instead of two
- Tests that assert "empty container" messages with file listings remain valid
- Tests for check_01 empty-vs-missing need new assertions for the different error messages
- Tests for check_12 empty src/ should now PASS (previously failing)
- Run full `cargo test rs_arch_01` — verify count changes make sense

### Step 6: Verify
- `cargo test rs_arch_01` — all passing tests still pass, some previously-failing tests now pass
- `cargo test ts_arch_01` — all pass
- `cargo test` — full suite green
- Error messages improved (no double-fire, accurate empty-vs-missing)

## What this achieves

1. **Fixes the `std::fs::read_dir` bypass** — TS now goes through `FileSystem` trait
2. **Single source of truth** for directory listing, loose file detection, exact-subdirs checking
3. **RS and TS produce structurally identical error messages** — only ID and entity label differ
4. **Fixes double-fire bug** — containers with files but no subdirs get one error, not two
5. **Fixes empty-vs-missing bug** — check_01 distinguishes "crates/ missing" from "crates/ empty"
6. **Fixes empty src/ bug** — check_12 detects empty src/ directories
7. **Fixes .gitkeep case sensitivity** — consistent behavior across platforms
8. **Future languages** (Python? Go?) get the same helpers for free
9. **~100 lines deleted** from `ts_arch_checks.rs`, ~80 lines from RS helpers

## What this does NOT change

- App discovery (RS: Cargo.toml, TS: package.json/.ts files) — stays separate
- Leaf validation (RS: Cargo.toml, TS: .ts/.tsx files) — stays separate
- Hex-in-hex recursion marker (RS: `crates/`, TS: `modules/`) — stays separate
- Severity differences (RS missing crates = Error, TS missing modules = Warn) — stays separate
- Symlink handling (bug #1) — deferred, needs FileSystem trait changes
- Permission denied (bug #2) — deferred, needs FileSystem trait changes
- `has_ts_source_files` using WalkDir directly — separate concern, fix if needed
