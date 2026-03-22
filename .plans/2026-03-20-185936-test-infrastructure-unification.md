# Test Infrastructure Unification + TS-ARCH-01/02 Tests

**Date:** 2026-03-20 18:59
**Depends on:** `.plans/2026-03-20-184500-parallel-workstreams.md` (Workstream 1)

## Goal

Extract duplicated test utilities into a shared module, then build TS-ARCH-01/02 tests using that shared infrastructure. The RS-ARCH-01 tests also get refactored to use the shared module.

## Current Problem

The same functions are copy-pasted across 6+ test files:

| Function | Copies found |
|---|---|
| `write_file(root, rel, content)` | 6 (rs_arch_01, 2x legacy, 2x adversarial, planned ts_arch_01) |
| `copy_dir_recursive(src, dst)` | 4 (rs_arch_01, 2x legacy, planned ts_arch_01) |
| `remove_dir(root, rel)` | 4 |
| `remove_file(root, rel)` | 4 |
| `assert_single_error(errors, fragment)` | 3 |
| `assert_file_field(errors)` | 3 (rule_02, rule_03, rule_04 — all in rs_arch_01) |

## Phase 1: Shared Test Support Module

### New files

```
tests/unit/test_support/
├── mod.rs              # re-exports
├── fixture.rs          # copy_golden(fixture_path) -> TempDir, copy_dir_recursive
├── fs_ops.rs           # write_file, remove_dir, remove_file
└── assertions.rs       # errors_by_id, assert_single_error, assert_file_field
```

### `fixture.rs`

```rust
use std::path::Path;

/// Copy a fixture directory into a fresh tempdir.
/// `fixture_rel` is relative to the crate root, e.g. "tests/fixtures/r_arch_01/golden".
pub fn copy_golden(fixture_rel: &str) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(Path::new(fixture_rel), tmp.path());
    tmp
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("read fixture dir") {
        let entry = entry.expect("read entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path).expect("create dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            std::fs::copy(&src_path, &dst_path).expect("copy file");
        }
    }
}
```

### `fs_ops.rs`

```rust
use std::path::Path;

pub fn write_file(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(&path, content).expect("write file");
}

pub fn remove_dir(root: &Path, rel: &str) {
    std::fs::remove_dir_all(root.join(rel)).expect("remove dir");
}

pub fn remove_file(root: &Path, rel: &str) {
    std::fs::remove_file(root.join(rel)).expect("remove file");
}
```

### `assertions.rs`

```rust
use guardrail3::domain::report::{CheckResult, Severity};

/// Filter results to errors matching a specific check ID.
pub fn errors_by_id<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|r| r.id == id && r.severity == Severity::Error)
        .collect()
}

/// Assert exactly 1 error with title containing the given fragment.
pub fn assert_single_error(errors: &[&CheckResult], expected_title_fragment: &str) {
    assert_eq!(errors.len(), 1, "expected exactly 1 error, got {}: {errors:#?}", errors.len());
    assert!(
        errors[0].title.contains(expected_title_fragment),
        "expected title containing '{expected_title_fragment}', got: '{}'",
        errors[0].title
    );
}

/// Assert every error has the `file` field set.
pub fn assert_file_field(errors: &[&CheckResult]) {
    for err in errors {
        assert!(
            err.file.is_some(),
            "expected file field to be set, got None for: {err:#?}"
        );
    }
}
```

### `mod.rs`

```rust
pub mod assertions;
pub mod fixture;
pub mod fs_ops;

pub use assertions::{assert_file_field, assert_single_error, errors_by_id};
pub use fixture::copy_golden;
pub use fs_ops::{remove_dir, remove_file, write_file};
```

### Wire into `unit.rs`

Add one line:
```rust
#[path = "unit/test_support/mod.rs"]
mod test_support;
```

## Phase 2: Refactor RS-ARCH-01 helpers

`rs_arch_01/helpers.rs` becomes thin:

```rust
use std::path::Path;

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::app::rs::validate::arch::rs_arch_01::check_hex_arch_structure;
use guardrail3::domain::report::CheckResult;

// Re-export shared utilities so rule files import from super::helpers
pub use crate::test_support::{
    assert_file_field, assert_single_error, copy_golden, errors_by_id,
    remove_dir, remove_file, write_file,
};

const GOLDEN: &str = "tests/fixtures/r_arch_01/golden";

pub fn copy_fixture() -> tempfile::TempDir {
    copy_golden(GOLDEN)
}

pub fn run_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    let mut results = Vec::new();
    check_hex_arch_structure(&fs, root, &mut results);
    results
}

pub fn arch_errors(results: &[CheckResult]) -> Vec<&CheckResult> {
    errors_by_id(results, "R-ARCH-01")
}
```

### Update rule files

Every `rs_arch_01/rule_*.rs` that currently defines its own `assert_no_packages`, `assert_file_field`, etc.:
- Delete the local definitions
- Import from `super::helpers` (which re-exports from `test_support`)

The rule files that define `assert_per_app`, `assert_inner_hex`, `assert_no_ts_apps`, `assert_no_packages` keep those as **suite-specific** assertions — they reference RS-specific constants (RUST_APPS, INNER_HEX) and are not universal. They stay in the rule files or move to `rs_arch_01/helpers.rs`.

### Rename mapping in RS-ARCH-01

| Old name | New name | Reason |
|---|---|---|
| `copy_golden()` | `copy_fixture()` | `copy_golden` now takes a path param in test_support |
| `arch_01_errors()` | `arch_errors()` | Shorter, module path provides context |

All rule files update their imports accordingly.

## Phase 3: TS-ARCH-01/02 Test Suite

### New files

```
tests/unit/ts_arch_01/
├── mod.rs
├── helpers.rs
├── golden.rs
├── rule_01.rs          # modules/ must exist
├── rule_02.rs          # exact contents {domain, ports, application, adapters}
├── rule_03.rs          # inbound/outbound in adapters/ and ports/
├── rule_04.rs          # loose files in structural/container dirs
├── rule_05.rs          # container not empty
├── rule_06.rs          # leaf validity (.ts/.tsx, .gitkeep, or modules/ hex-in-hex)
└── rule_07.rs          # T-ARCH-02 import boundaries
```

### `ts_arch_01/helpers.rs`

```rust
use std::path::Path;

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::app::ts::validate::ts_arch_checks;
use guardrail3::domain::report::CheckResult;

pub use crate::test_support::{
    assert_file_field, assert_single_error, copy_golden, errors_by_id,
    remove_dir, remove_file, write_file,
};

const GOLDEN: &str = "tests/fixtures/r_arch_01/golden";  // SAME golden fixture

pub fn copy_fixture() -> tempfile::TempDir {
    copy_golden(GOLDEN)
}

pub fn run_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    ts_arch_checks::check_hex_arch_structure(&fs, root)
}

pub fn run_import_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    ts_arch_checks::check_import_boundaries(&fs, root)
}

pub fn arch_errors(results: &[CheckResult]) -> Vec<&CheckResult> {
    errors_by_id(results, "T-ARCH-01")
}

pub fn import_errors(results: &[CheckResult]) -> Vec<&CheckResult> {
    errors_by_id(results, "T-ARCH-02")
}
```

### Same golden fixture

Both RS and TS suites use `tests/fixtures/r_arch_01/golden/`. It already has:
- **admin** — TS service app with full hex arch in `src/modules/` (T-ARCH-01 should pass)
- **landing** — TS content site with velite in devDeps + `content/` dir (auto-detected as content, SKIPPED)
- **devctl/backend/worker** — Rust apps (TS checks should ignore them)
- **packages/shared-types, packages/ui-kit** — Should not be checked

No fixture changes needed for TS structural tests. The golden already covers the happy path.

### Test patterns

Every test follows the same pattern as RS-ARCH-01:

```rust
use super::helpers::{arch_errors, copy_fixture, run_check, write_file, remove_dir};

#[test]
fn test_name() {
    let tmp = copy_fixture();
    // mutate: remove_dir, write_file, etc.
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_eq!(errors.len(), EXPECTED, "...");
    // assert title content, file field, per-app attribution, no false positives
}
```

### TS-specific constants (in relevant rule files)

```rust
const TS_SERVICE_APPS: &[&str] = &["admin"];  // only 1 TS service app in golden

const TS_CONTAINER_SUFFIXES: &[&str] = &[
    "application",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

// All 6 container paths for admin
fn all_container_paths() -> Vec<String> {
    TS_CONTAINER_SUFFIXES
        .iter()
        .map(|s| format!("apps/admin/src/modules/{s}"))
        .collect()
}
```

### Key differences from RS test suite

| Aspect | RS-ARCH-01 | TS-ARCH-01 |
|---|---|---|
| Service apps in golden | 3 (devctl, backend, worker) | 1 (admin) |
| Inner hex-in-hex | Yes (backend/mcp/crates) | No (not in golden) |
| Total container locations | 24 (3×6 + 1×6) | 6 (1×6) |
| Content site skipping | N/A (Rust check ignores TS apps) | landing auto-skipped |
| Structural root | `crates/` | `src/modules/` |
| Layer names | `app`, `domain`, `adapters/{in,out}`, `ports/{in,out}` | `application`, `domain`, `adapters/{in,out}`, `ports/{in,out}` |
| Leaf marker | `Cargo.toml` in subdir | `.ts`/`.tsx` files in subdir |
| Missing modules/ severity | Error | **Warn** (line 117 of ts_arch_checks.rs) |

### Rule-by-rule scenarios

**rule_01.rs — modules/ must exist:**
- Remove `apps/admin/src/modules/` → 1 warning (Warn, not Error)
- Landing NOT checked (content site)
- Rust apps NOT checked
- Packages NOT checked

**rule_02.rs — exact contents {domain, ports, application, adapters}:**
- Remove each one individually → 1 error each (4 tests)
- Remove all 4 → 4 errors
- Add unexpected dir (e.g. `modules/services/`) → 1 error
- Add unexpected + remove expected → both errors

**rule_03.rs — inbound/outbound in adapters/ and ports/:**
- Remove `adapters/inbound/` → 1 error
- Remove `ports/outbound/` → 1 error
- Remove all 4 (adapters/{in,out}, ports/{in,out}) → 4 errors
- Add unexpected dir in adapters/ or ports/ → 1 error each

**rule_04.rs — loose files:**
- Add `.ts` file in `modules/` → 1 error
- Add `.ts` in `modules/adapters/` → 1 error
- Add files in all 6 containers + 3 structural dirs → 9 errors
- `.gitkeep` allowed everywhere
- `.gitignore` is NOT `.gitkeep` → error
- Multiple loose files per dir → 1 error per dir (lists all files)

**rule_05.rs — container not empty:**
- Remove all contents of `modules/domain/` (subdirs + .gitkeep) → 1 error
- Empty all 6 containers → 6 errors
- `.gitkeep` satisfies non-empty check

**rule_06.rs — leaf validity:**
- Add subdir with no `.ts`/`.tsx` files → 1 error
- Add subdir with `.ts` file → pass
- Add subdir with `.gitkeep` → pass
- Add subdir with `modules/` inside (hex-in-hex) → triggers recursion, needs full structure

**rule_07.rs — T-ARCH-02 import boundaries:**
- Write domain file importing from adapters → 1 error
- Write application file importing from domain → 0 errors (allowed)
- Write domain file with alias import `@/modules/adapters/...` → 1 error
- Multiple violations in one file → multiple errors
- Test files excluded from checking

### Wire into `unit.rs`

```rust
#[path = "unit/test_support/mod.rs"]
mod test_support;

#[path = "unit/ts_arch_01/mod.rs"]
mod ts_arch_01;
```

## Phase 4: Verify & Clean Up

1. `cargo test rs_arch_01` — all existing tests still pass after refactor
2. `cargo test ts_arch_01` — all new tests pass
3. Run adversarial review (4-agent pattern from workstreams plan)
4. The old `ts_arch_checks_test.rs` (StubFs-based) stays as unit tests for helper functions (import parsing, layer detection) — those don't need fixtures

## Execution Order

1. **Phase 1** first — create `test_support/` module, wire into `unit.rs`
2. **Phase 2** — refactor `rs_arch_01/helpers.rs` to use `test_support`, update imports in all rule files, verify `cargo test rs_arch_01` passes
3. **Phase 3** — build `ts_arch_01/` suite using `test_support`
4. **Phase 4** — verify everything, adversarial review

Phase 1+2 can run in parallel with Phase 3 if careful (Phase 3 just needs to import from `test_support` which Phase 1 creates). But safer to do 1→2→3→4 sequentially.

## Files Modified

| File | Change |
|---|---|
| `tests/unit/test_support/mod.rs` | NEW |
| `tests/unit/test_support/fixture.rs` | NEW |
| `tests/unit/test_support/fs_ops.rs` | NEW |
| `tests/unit/test_support/assertions.rs` | NEW |
| `tests/unit.rs` | Add test_support + ts_arch_01 module declarations |
| `tests/unit/rs_arch_01/helpers.rs` | Refactor to use test_support |
| `tests/unit/rs_arch_01/rule_02.rs` | Remove local assert_file_field, use helpers |
| `tests/unit/rs_arch_01/rule_03.rs` | Remove local assert_file_field, use helpers |
| `tests/unit/rs_arch_01/rule_04.rs` | Remove local assert_file_field, use helpers |
| `tests/unit/ts_arch_01/mod.rs` | NEW |
| `tests/unit/ts_arch_01/helpers.rs` | NEW |
| `tests/unit/ts_arch_01/golden.rs` | NEW |
| `tests/unit/ts_arch_01/rule_01.rs` through `rule_07.rs` | NEW |
