use std::path::Path;

use walkdir::WalkDir;

use super::source_scan::is_excluded_dir;
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::{FileSystem, ToolChecker};

/// Run all test quality checks (R-TEST-01 through R-TEST-09).
pub fn check(fs: &dyn FileSystem, tc: &dyn ToolChecker, workspace_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // R-TEST-01: cargo-mutants installed
    check_cargo_mutants_installed(tc, &mut results);

    // R-TEST-02: .cargo/mutants.toml exists
    check_mutants_toml(workspace_root, &mut results);

    // R-TEST-03: [profile.mutants] in Cargo.toml
    check_mutants_profile(fs, workspace_root, &mut results);

    // R-TEST-04: At least one #[test] exists
    check_tests_exist(fs, workspace_root, &mut results);

    // R-TEST-05 through R-TEST-08: quality checks
    super::test_quality_checks::check(fs, workspace_root, &mut results);

    // R-TEST-09: No test code in production source
    check_no_tests_in_src(fs, workspace_root, &mut results);

    results
}

// ---------------------------------------------------------------------------
// R-TEST-01: cargo-mutants installed
// ---------------------------------------------------------------------------

pub fn check_cargo_mutants_installed(tc: &dyn ToolChecker, results: &mut Vec<CheckResult>) {
    super::release_repo_checks::check_tool_installed(
        tc,
        "cargo-mutants",
        "R-TEST-01",
        "cargo install cargo-mutants",
        results,
    );
}

// ---------------------------------------------------------------------------
// R-TEST-02: .cargo/mutants.toml exists
// ---------------------------------------------------------------------------

pub fn check_mutants_toml(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let mutants_path = workspace_root.join(".cargo").join("mutants.toml");
    if mutants_path.exists() {
        results.push(
            CheckResult {
                id: "R-TEST-02".to_owned(),
                severity: Severity::Info,
                title: ".cargo/mutants.toml exists".to_owned(),
                message: "Mutation testing configuration found".to_owned(),
                file: Some(mutants_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "R-TEST-02".to_owned(),
            severity: Severity::Warn,
            title: ".cargo/mutants.toml missing".to_owned(),
            message: "Mutation testing config `.cargo/mutants.toml` not found. Mutation testing (cargo-mutants) injects bugs into code to verify tests catch them — without config, it uses defaults that may be too slow or skip important targets. Create `.cargo/mutants.toml` with `timeout_multiplier = 2.0`.".to_owned(),
            file: Some(mutants_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

// ---------------------------------------------------------------------------
// R-TEST-03: [profile.mutants] in Cargo.toml
// ---------------------------------------------------------------------------

fn check_mutants_profile(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
    let cargo_path = workspace_root.join("Cargo.toml");
    let Some(content) = fs.read_file(&cargo_path) else {
        results.push(CheckResult {
            id: "R-TEST-03".to_owned(),
            severity: Severity::Info,
            title: "Cargo.toml not found".to_owned(),
            message: "Cannot check for [profile.mutants] without Cargo.toml".to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
        return;
    };

    if has_mutants_profile(&content) {
        results.push(
            CheckResult {
                id: "R-TEST-03".to_owned(),
                severity: Severity::Info,
                title: "[profile.mutants] configured".to_owned(),
                message: "Optimized build profile for mutation testing found in Cargo.toml"
                    .to_owned(),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "R-TEST-03".to_owned(),
            severity: Severity::Warn,
            title: "[profile.mutants] missing".to_owned(),
            message: "No [profile.mutants] section in Cargo.toml. This custom build profile optimizes mutation testing speed (e.g., opt-level=0, no LTO). Without it, cargo-mutants uses the `dev` profile which may be slower. Add `[profile.mutants]` with `inherits = \"dev\"` and `opt-level = 0`.".to_owned(),
            file: Some(cargo_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// Check if content contains a [profile.mutants] section.
pub fn has_mutants_profile(content: &str) -> bool {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[profile.mutants]" {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// R-TEST-04: At least one #[test] exists
// ---------------------------------------------------------------------------

fn check_tests_exist(fs: &dyn FileSystem, workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let mut found_test = false;

    for entry in WalkDir::new(workspace_root)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e))
        .flatten()
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let Some(content) = fs.read_file(entry.path()) else {
            continue;
        };
        if content_has_test(&content) {
            found_test = true;
            break;
        }
    }

    if found_test {
        results.push(CheckResult {
            id: "R-TEST-04".to_owned(),
            severity: Severity::Info,
            title: "Tests exist".to_owned(),
            message: "At least one `#[test]` or `#[tokio::test]` function found in the workspace. Test presence confirmed, no action needed.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "R-TEST-04".to_owned(),
            severity: Severity::Error,
            title: "No tests found".to_owned(),
            message: "No `#[test]` or `#[tokio::test]` functions found anywhere in the workspace. Without tests, bugs go undetected and refactoring is unsafe. Add test functions in a `tests/` directory.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

/// Check if content contains a `#[test]` or `#[tokio::test]` attribute (AST-based).
pub fn content_has_test(content: &str) -> bool {
    let Some(file) = super::ast_helpers::parse_file(content) else {
        return false;
    };
    super::ast_helpers::has_test_attribute(&file)
}

// ---------------------------------------------------------------------------
// R-TEST-09: No test code in production source files
// ---------------------------------------------------------------------------

/// Walk all `.rs` files under `src/` and flag any that contain INLINE test code.
///
/// Acceptable pattern (NOT flagged):
///   `#[cfg(test)] #[path = "foo_tests.rs"] mod tests;`  ← pointer to separate file
///   `_tests.rs` files                                    ← the separate test files
///
/// Flagged (inline test code):
///   `#[cfg(test)] mod tests { #[test] fn ... }`          ← tests inside production file
///   `#[test] fn ...` directly in a non-test file         ← tests mixed with production
pub fn check_no_tests_in_src(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
    for entry in WalkDir::new(workspace_root)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e))
        .flatten()
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }

        let path_str = path.display().to_string();

        // Only check files in src/ directories
        if !path_str.contains("/src/") {
            continue;
        }

        // Skip files in tests/ paths
        if path_str.contains("/tests/") {
            continue;
        }

        // Skip test fixture files
        if path_str.contains("tests/fixtures/") {
            continue;
        }

        // Skip _tests.rs files — these ARE the separate test files (target of #[path])
        if path_str.ends_with("_tests.rs") || path_str.ends_with("/tests.rs") {
            continue;
        }

        let Some(content) = fs.read_file(path) else {
            continue;
        };

        let Some(parsed) = super::ast_helpers::parse_file(&content) else {
            continue;
        };

        // Check for #[test] functions directly in the file (not in a mod)
        let has_test_attr = super::ast_helpers::has_test_attribute(&parsed);

        // Check for #[cfg(test)] modules WITH a body (inline tests).
        // #[cfg(test)] mod tests; (no body, just declaration) is OK — it's a #[path] pointer.
        let has_inline_test_module = file_has_inline_cfg_test_module(&parsed);

        if has_test_attr || has_inline_test_module {
            let relative = path
                .strip_prefix(workspace_root)
                .unwrap_or(path)
                .display()
                .to_string();
            results.push(CheckResult {
                id: "R-TEST-09".to_owned(),
                severity: Severity::Error,
                title: "Inline test code in production source".to_owned(),
                message: format!(
                    "Inline test code found in `{relative}`. \
                     Extract tests to a separate file using the #[path] pattern:\n\
                     \n\
                     In {relative}:\n  \
                       #[cfg(test)]\n  \
                       #[path = \"<name>_tests.rs\"]\n  \
                       mod tests;\n\
                     \n\
                     In <name>_tests.rs:\n  \
                       use super::*;\n  \
                       #[test]\n  \
                       fn test_something() {{ ... }}\n\
                     \n\
                     This keeps unit test access to private items via `use super::*` \
                     while separating test code from production code. \
                     For integration tests that only need the public API, use `tests/` at crate root."
                ),
                file: Some(path_str),
                line: None,
                inventory: false,
            });
        }
    }
}

/// Check if a parsed file has `#[cfg(test)] mod tests { ... }` with an INLINE body.
/// Returns false for `#[cfg(test)] mod tests;` (no body — just a declaration pointing
/// to a separate file via #[path]).
pub fn file_has_cfg_test_module(file: &syn::File) -> bool {
    file_has_inline_cfg_test_module(file)
}

fn file_has_inline_cfg_test_module(file: &syn::File) -> bool {
    for item in &file.items {
        if let syn::Item::Mod(m) = item {
            if m.attrs.iter().any(super::ast_helpers::is_cfg_test_attr) {
                // m.content is Some(...) when the module has a body { ... }
                // m.content is None when it's just `mod tests;` (declaration only)
                if m.content.is_some() {
                    return true;
                }
            }
        }
    }
    false
}
