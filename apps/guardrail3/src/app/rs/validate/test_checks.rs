use std::path::Path;

use walkdir::WalkDir;

use super::source_scan::is_excluded_dir;
use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::{FileSystem, ToolChecker};

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

fn check_cargo_mutants_installed(tc: &dyn ToolChecker, results: &mut Vec<CheckResult>) {
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

fn check_mutants_toml(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let mutants_path = workspace_root.join(".cargo").join("mutants.toml");
    if mutants_path.exists() {
        results.push(CheckResult {
            id: "R-TEST-02".to_owned(),
            severity: Severity::Info,
            title: ".cargo/mutants.toml exists".to_owned(),
            message: "Mutation testing configuration found".to_owned(),
            file: Some(mutants_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R-TEST-02".to_owned(),
            severity: Severity::Warn,
            title: ".cargo/mutants.toml missing".to_owned(),
            message: "Create .cargo/mutants.toml to configure mutation testing".to_owned(),
            file: Some(mutants_path.display().to_string()),
            line: None,
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
        });
        return;
    };

    if has_mutants_profile(&content) {
        results.push(CheckResult {
            id: "R-TEST-03".to_owned(),
            severity: Severity::Info,
            title: "[profile.mutants] configured".to_owned(),
            message: "Optimized build profile for mutation testing found in Cargo.toml".to_owned(),
            file: Some(cargo_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R-TEST-03".to_owned(),
            severity: Severity::Info,
            title: "[profile.mutants] missing".to_owned(),
            message: "Add [profile.mutants] to Cargo.toml for faster mutation testing".to_owned(),
            file: Some(cargo_path.display().to_string()),
            line: None,
        });
    }
}

/// Check if content contains a [profile.mutants] section.
fn has_mutants_profile(content: &str) -> bool {
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
            message: "At least one #[test] or #[tokio::test] found".to_owned(),
            file: None,
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R-TEST-04".to_owned(),
            severity: Severity::Error,
            title: "No tests found".to_owned(),
            message: "No .rs files contain #[test] or #[tokio::test]".to_owned(),
            file: None,
            line: None,
        });
    }
}

/// Check if content contains a `#[test]` or `#[tokio::test]` attribute (AST-based).
fn content_has_test(content: &str) -> bool {
    let Some(file) = super::ast_helpers::parse_file(content) else {
        return false;
    };
    super::ast_helpers::has_test_attribute(&file)
}

// ---------------------------------------------------------------------------
// R-TEST-09: No test code in production source files
// ---------------------------------------------------------------------------

/// Walk all `.rs` files under `src/` and flag any that contain `#[test]`/`#[tokio::test]`
/// attributes or `#[cfg(test)]` modules. Test code belongs in `tests/` directories.
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

        // Skip files in tests/ paths (e.g. src/tests/)
        if path_str.contains("/tests/") {
            continue;
        }

        // Skip test fixture files
        if path_str.contains("tests/fixtures/") {
            continue;
        }

        let Some(content) = fs.read_file(path) else {
            continue;
        };

        let Some(parsed) = super::ast_helpers::parse_file(&content) else {
            continue;
        };

        // Check for #[test] or #[tokio::test] attributes
        let has_test_attr = super::ast_helpers::has_test_attribute(&parsed);

        // Check for #[cfg(test)] modules
        let has_cfg_test = file_has_cfg_test_module(&parsed);

        if has_test_attr || has_cfg_test {
            let relative = path
                .strip_prefix(workspace_root)
                .unwrap_or(path)
                .display()
                .to_string();
            results.push(CheckResult {
                id: "R-TEST-09".to_owned(),
                severity: Severity::Error,
                title: "Test code in production source".to_owned(),
                message: format!(
                    "Test code found in production source: {relative}. Move tests to tests/ directory."
                ),
                file: Some(path_str),
                line: None,
            });
        }
    }
}

/// Check if a parsed file has any `#[cfg(test)] mod ...` at the top level.
fn file_has_cfg_test_module(file: &syn::File) -> bool {
    for item in &file.items {
        if let syn::Item::Mod(m) = item {
            if m.attrs.iter().any(super::ast_helpers::is_cfg_test_attr) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs as stdfs; // only in tests — not production code

    #[allow(clippy::expect_used)] // reason: test infra — panic on temp dir failure is fine
    fn make_temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("failed to create temp dir")
    }

    // ---- R-TEST-01: cargo-mutants installed ----

    #[test]
    fn r_test_01_detects_installed_tool() {
        let mut results = Vec::new();
        let tc = crate::adapters::outbound::tool_runner::RealToolChecker;
        check_cargo_mutants_installed(&tc, &mut results);
        assert_eq!(results.len(), 1);
        assert_eq!(results.first().map(|r| r.id.as_str()), Some("R-TEST-01"));
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn r_test_01_severity_matches_installation() {
        let mut results = Vec::new();
        let tc = crate::adapters::outbound::tool_runner::RealToolChecker;
        check_cargo_mutants_installed(&tc, &mut results);
        let result = results.first().expect("should have one result");
        assert!(
            result.severity == Severity::Info || result.severity == Severity::Warn,
            "Severity must be Info (installed) or Warn (missing)"
        );
    }

    // ---- R-TEST-02: .cargo/mutants.toml exists ----

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn r_test_02_neg_no_mutants_toml() {
        let tmp = make_temp_dir();
        let mut results = Vec::new();
        check_mutants_toml(tmp.path(), &mut results);
        assert_eq!(results.len(), 1);
        let r = results.first().expect("should have result");
        assert_eq!(r.id, "R-TEST-02");
        assert_eq!(r.severity, Severity::Warn);
    }

    #[test]
    #[allow(clippy::expect_used, clippy::disallowed_methods)] // reason: test setup and assertions
    fn r_test_02_pos_mutants_toml_exists() {
        let tmp = make_temp_dir();
        let cargo_dir = tmp.path().join(".cargo");
        stdfs::create_dir_all(&cargo_dir).expect("mkdir");
        stdfs::write(cargo_dir.join("mutants.toml"), "profile = \"mutants\"").expect("write");
        let mut results = Vec::new();
        check_mutants_toml(tmp.path(), &mut results);
        assert_eq!(results.len(), 1);
        let r = results.first().expect("should have result");
        assert_eq!(r.id, "R-TEST-02");
        assert_eq!(r.severity, Severity::Info);
    }

    // ---- R-TEST-03: [profile.mutants] in Cargo.toml ----

    #[test]
    fn r_test_03_neg_no_profile() {
        let content = "[package]\nname = \"foo\"\nversion = \"0.1.0\"";
        assert!(!has_mutants_profile(content));
    }

    #[test]
    fn r_test_03_pos_has_profile() {
        let content = "[package]\nname = \"foo\"\n\n[profile.mutants]\ninherits = \"test\"";
        assert!(has_mutants_profile(content));
    }

    // ---- R-TEST-04: At least one #[test] exists ----

    #[test]
    fn r_test_04_neg_no_test_attr() {
        let content = "fn main() {}\npub fn helper() {}";
        assert!(!content_has_test(content));
    }

    #[test]
    fn r_test_04_pos_has_test_attr() {
        let content = "#[test]\nfn it_works() { assert!(true); }";
        assert!(content_has_test(content));
    }

    #[test]
    fn r_test_04_pos_has_tokio_test() {
        let content = "#[tokio::test]\nasync fn it_works() {}";
        assert!(content_has_test(content));
    }

    // ---- R-TEST-09: No test code in production source ----

    #[test]
    fn r_test_09_detects_cfg_test_module() {
        let content = "fn production() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn it_works() {}\n}";
        let parsed = super::super::ast_helpers::parse_file(content);
        assert!(parsed.is_some());
        assert!(file_has_cfg_test_module(&parsed.unwrap()));
    }

    #[test]
    fn r_test_09_no_cfg_test_is_clean() {
        let content = "fn production() {}";
        let parsed = super::super::ast_helpers::parse_file(content);
        assert!(parsed.is_some());
        assert!(!file_has_cfg_test_module(&parsed.unwrap()));
    }
}
