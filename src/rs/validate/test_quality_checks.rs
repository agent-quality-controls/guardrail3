use std::path::Path;

use walkdir::WalkDir;

use super::source_scan::is_excluded_dir;
use crate::report::types::{CheckResult, Severity};

/// Run test quality checks (R-TEST-05 through R-TEST-08).
pub fn check(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    check_test_coverage_inventory(workspace_root, results);
    check_integration_tests(workspace_root, results);
    check_ignore_without_reason(workspace_root, results);
    check_mutation_hook(workspace_root, results);
}

// ---------------------------------------------------------------------------
// R-TEST-05: Test coverage inventory
// ---------------------------------------------------------------------------

fn check_test_coverage_inventory(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let mut pub_fn_count: usize = 0;
    let mut test_fn_count: usize = 0;

    // Count pub fn in src/ files
    let src_dir = workspace_root.join("src");
    if src_dir.exists() {
        for entry in WalkDir::new(&src_dir)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                name != "target" && name != "node_modules" && name != ".git"
            })
            .flatten()
        {
            if !entry.file_type().is_file() {
                continue;
            }
            if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            if let Some(content) = crate::fs::read_file(entry.path()) {
                pub_fn_count = pub_fn_count.saturating_add(count_pub_fns(&content));
            }
        }
    }

    // Count #[test] in all .rs files
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
        if let Some(content) = crate::fs::read_file(entry.path()) {
            test_fn_count = test_fn_count.saturating_add(count_test_fns(&content));
        }
    }

    #[allow(clippy::arithmetic_side_effects)] // reason: division by zero guarded by if check above
    let ratio = if pub_fn_count == 0 {
        0
    } else {
        test_fn_count.saturating_mul(100) / pub_fn_count
    };

    results.push(CheckResult {
        id: "R-TEST-05".to_owned(),
        severity: Severity::Info,
        title: "Test coverage inventory".to_owned(),
        message: format!(
            "{pub_fn_count} public functions, {test_fn_count} test functions (ratio: {ratio}%)"
        ),
        file: None,
        line: None,
    });
}

/// Count `pub fn` declarations in content (AST-based).
fn count_pub_fns(content: &str) -> usize {
    let Some(file) = super::ast_helpers::parse_file(content) else {
        return 0;
    };
    super::ast_helpers::count_pub_fn_decls(&file)
}

/// Count `#[test]` and `#[tokio::test]` attributes in content (AST-based).
fn count_test_fns(content: &str) -> usize {
    let Some(file) = super::ast_helpers::parse_file(content) else {
        return 0;
    };
    super::ast_helpers::count_test_attrs(&file)
}

// ---------------------------------------------------------------------------
// R-TEST-06: Integration tests exist
// ---------------------------------------------------------------------------

fn check_integration_tests(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let tests_dir = workspace_root.join("tests");
    if has_rs_files_in_dir(&tests_dir) {
        results.push(CheckResult {
            id: "R-TEST-06".to_owned(),
            severity: Severity::Info,
            title: "Integration tests exist".to_owned(),
            message: "tests/ directory with .rs files found".to_owned(),
            file: Some(tests_dir.display().to_string()),
            line: None,
        });
        return;
    }

    // Also check workspace members for tests/ dirs
    for entry in WalkDir::new(workspace_root)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e))
        .flatten()
    {
        if entry.file_type().is_dir()
            && entry.file_name() == "tests"
            && has_rs_files_in_dir(entry.path())
        {
            results.push(CheckResult {
                id: "R-TEST-06".to_owned(),
                severity: Severity::Info,
                title: "Integration tests exist".to_owned(),
                message: format!(
                    "tests/ directory with .rs files found at {}",
                    entry.path().display()
                ),
                file: Some(entry.path().display().to_string()),
                line: None,
            });
            return;
        }
    }

    results.push(CheckResult {
        id: "R-TEST-06".to_owned(),
        severity: Severity::Info,
        title: "No integration tests".to_owned(),
        message: "No tests/ directory with .rs files found".to_owned(),
        file: None,
        line: None,
    });
}

/// Check if a directory exists and contains at least one .rs file.
#[allow(clippy::case_sensitive_file_extension_comparisons)] // reason: only checking .rs files
fn has_rs_files_in_dir(dir: &Path) -> bool {
    if !dir.exists() {
        return false;
    }
    for entry in crate::fs::list_dir(dir) {
        if let Some(name) = entry.file_name().to_str() {
            if name.ends_with(".rs") {
                return true;
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// R-TEST-07: No #[ignore] without reason
// ---------------------------------------------------------------------------

fn check_ignore_without_reason(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let mut found_violation = false;

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
        let Some(content) = crate::fs::read_file(entry.path()) else {
            continue;
        };

        let violations = find_ignore_without_reason(&content);
        for line_num in &violations {
            found_violation = true;
            results.push(CheckResult {
                id: "R-TEST-07".to_owned(),
                severity: Severity::Warn,
                title: "#[ignore] without reason".to_owned(),
                message: "#[ignore] should have a // reason: comment on same or previous line"
                    .to_owned(),
                file: Some(entry.path().display().to_string()),
                line: Some(*line_num),
            });
        }
    }

    if !found_violation {
        results.push(CheckResult {
            id: "R-TEST-07".to_owned(),
            severity: Severity::Info,
            title: "All #[ignore] have reasons".to_owned(),
            message: "No bare #[ignore] attributes found".to_owned(),
            file: None,
            line: None,
        });
    }
}

/// Find lines with #[ignore] that lack a reason comment (AST-based).
/// Returns 1-based line numbers of violations.
fn find_ignore_without_reason(content: &str) -> Vec<usize> {
    let Some(file) = super::ast_helpers::parse_file(content) else {
        return Vec::new();
    };
    super::ast_helpers::find_ignore_without_reason(&file, content)
}

// ---------------------------------------------------------------------------
// R-TEST-08: Mutation test hook configured
// ---------------------------------------------------------------------------

fn check_mutation_hook(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    // Check .claude/ directory for hook configs mentioning "mutant"
    let claude_dir = workspace_root.join(".claude");
    if claude_dir.exists() {
        for entry in crate::fs::list_dir(&claude_dir) {
            let path = entry.path();
            if let Some(content) = crate::fs::read_file(&path) {
                if content.contains("mutant") || content.contains("cargo-mutants") {
                    results.push(CheckResult {
                        id: "R-TEST-08".to_owned(),
                        severity: Severity::Info,
                        title: "Mutation test hook configured".to_owned(),
                        message: format!("Mutation testing hook found in {}", path.display()),
                        file: Some(path.display().to_string()),
                        line: None,
                    });
                    return;
                }
            }
        }
    }

    // Check .git/hooks/pre-commit
    let pre_commit = workspace_root.join(".git").join("hooks").join("pre-commit");
    if let Some(content) = crate::fs::read_file(&pre_commit) {
        if content.contains("cargo mutants")
            || content.contains("cargo-mutants")
            || content.contains("stryker")
        {
            results.push(CheckResult {
                id: "R-TEST-08".to_owned(),
                severity: Severity::Info,
                title: "Mutation test hook configured".to_owned(),
                message: "Mutation testing found in pre-commit hook".to_owned(),
                file: Some(pre_commit.display().to_string()),
                line: None,
            });
            return;
        }
    }

    results.push(CheckResult {
        id: "R-TEST-08".to_owned(),
        severity: Severity::Info,
        title: "No mutation test hook".to_owned(),
        message: "No mutation testing hook found in .claude/ or .git/hooks/pre-commit".to_owned(),
        file: None,
        line: None,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs as stdfs; // only in tests — not production code

    #[allow(clippy::expect_used)] // reason: test infra — panic on temp dir failure is fine
    fn make_temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("failed to create temp dir")
    }

    // ---- R-TEST-05: Test coverage inventory ----

    #[test]
    fn r_test_05_counts_pub_fns() {
        let content = "pub fn foo() {}\nfn bar() {}\npub fn baz() {}";
        assert_eq!(count_pub_fns(content), 2);
    }

    #[test]
    fn r_test_05_counts_test_fns() {
        let content = "#[test]\nfn a() {}\n#[test]\nfn b() {}\nfn c() {}";
        assert_eq!(count_test_fns(content), 2);
    }

    #[test]
    #[allow(clippy::expect_used, clippy::disallowed_methods)] // reason: test setup and assertions
    fn r_test_05_emits_inventory() {
        let tmp = make_temp_dir();
        let src_dir = tmp.path().join("src");
        stdfs::create_dir_all(&src_dir).expect("mkdir");
        stdfs::write(
            src_dir.join("lib.rs"),
            "pub fn foo() {}\npub fn bar() {}\n#[test]\nfn test_foo() {}",
        )
        .expect("write");
        let mut results = Vec::new();
        check_test_coverage_inventory(tmp.path(), &mut results);
        assert_eq!(results.len(), 1);
        let r = results.first().expect("should have result");
        assert_eq!(r.id, "R-TEST-05");
        assert_eq!(r.severity, Severity::Info);
        assert!(r.message.contains("2 public functions"));
        assert!(r.message.contains("1 test functions"));
    }

    // ---- R-TEST-06: Integration tests exist ----

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn r_test_06_neg_no_tests_dir() {
        let tmp = make_temp_dir();
        let mut results = Vec::new();
        check_integration_tests(tmp.path(), &mut results);
        assert_eq!(results.len(), 1);
        let r = results.first().expect("should have result");
        assert_eq!(r.id, "R-TEST-06");
        assert!(r.title.contains("No integration"));
    }

    #[test]
    #[allow(clippy::expect_used, clippy::disallowed_methods)] // reason: test setup and assertions
    fn r_test_06_pos_tests_dir_with_rs() {
        let tmp = make_temp_dir();
        let tests_dir = tmp.path().join("tests");
        stdfs::create_dir_all(&tests_dir).expect("mkdir");
        stdfs::write(tests_dir.join("integration.rs"), "#[test]\nfn it() {}").expect("write");
        let mut results = Vec::new();
        check_integration_tests(tmp.path(), &mut results);
        assert_eq!(results.len(), 1);
        let r = results.first().expect("should have result");
        assert_eq!(r.id, "R-TEST-06");
        assert!(r.title.contains("Integration tests exist"));
    }

    // ---- R-TEST-07: No #[ignore] without reason ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r_test_07_neg_bare_ignore() {
        let content = "#[test]\n#[ignore]\nfn slow_test() {}";
        let violations = find_ignore_without_reason(content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0], 2); // line 2, 1-based
    }

    #[test]
    fn r_test_07_pos_ignore_with_reason_same_line() {
        let content = "#[test]\n#[ignore] // reason: requires network\nfn slow_test() {}";
        let violations = find_ignore_without_reason(content);
        assert!(violations.is_empty(), "Should accept reason on same line");
    }

    #[test]
    fn r_test_07_pos_ignore_with_reason_prev_line() {
        let content = "#[test]\n// reason: requires database\n#[ignore]\nfn slow_test() {}";
        let violations = find_ignore_without_reason(content);
        assert!(
            violations.is_empty(),
            "Should accept reason on previous line"
        );
    }

    #[test]
    fn r_test_07_pos_ignore_with_name_value_reason() {
        let content = "#[test]\n#[ignore = \"requires network\"]\nfn slow_test() {}";
        let violations = find_ignore_without_reason(content);
        assert!(
            violations.is_empty(),
            "ignore with = reason should not be flagged"
        );
    }

    // ---- R-TEST-08: Mutation test hook configured ----

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn r_test_08_neg_no_hook() {
        let tmp = make_temp_dir();
        let mut results = Vec::new();
        check_mutation_hook(tmp.path(), &mut results);
        assert_eq!(results.len(), 1);
        let r = results.first().expect("should have result");
        assert_eq!(r.id, "R-TEST-08");
        assert!(r.title.contains("No mutation"));
    }

    #[test]
    #[allow(clippy::expect_used, clippy::disallowed_methods)] // reason: test setup and assertions
    fn r_test_08_pos_claude_hook() {
        let tmp = make_temp_dir();
        let claude_dir = tmp.path().join(".claude");
        stdfs::create_dir_all(&claude_dir).expect("mkdir");
        stdfs::write(
            claude_dir.join("hooks.json"),
            r#"{"hooks": [{"command": "cargo-mutants --in-diff"}]}"#,
        )
        .expect("write");
        let mut results = Vec::new();
        check_mutation_hook(tmp.path(), &mut results);
        assert_eq!(results.len(), 1);
        let r = results.first().expect("should have result");
        assert_eq!(r.id, "R-TEST-08");
        assert!(r.title.contains("Mutation test hook configured"));
    }

    #[test]
    #[allow(clippy::expect_used, clippy::disallowed_methods)] // reason: test setup and assertions
    fn r_test_08_pos_pre_commit_hook() {
        let tmp = make_temp_dir();
        let hooks_dir = tmp.path().join(".git").join("hooks");
        stdfs::create_dir_all(&hooks_dir).expect("mkdir");
        stdfs::write(
            hooks_dir.join("pre-commit"),
            "#!/bin/bash\ncargo mutants --in-diff -\n",
        )
        .expect("write");
        let mut results = Vec::new();
        check_mutation_hook(tmp.path(), &mut results);
        assert_eq!(results.len(), 1);
        let r = results.first().expect("should have result");
        assert_eq!(r.id, "R-TEST-08");
        assert!(r.title.contains("Mutation test hook configured"));
    }
}
