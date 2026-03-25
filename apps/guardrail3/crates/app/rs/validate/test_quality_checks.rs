use std::path::Path;

use walkdir::WalkDir;

use super::source_scan::is_excluded_dir;
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

/// Run test quality checks (R-TEST-05 through R-TEST-08).
pub fn check(fs: &dyn FileSystem, workspace_root: &Path, results: &mut Vec<CheckResult>) {
    check_test_coverage_inventory(fs, workspace_root, results);
    check_integration_tests(fs, workspace_root, results);
    check_ignore_without_reason(fs, workspace_root, results);
    check_mutation_hook(fs, workspace_root, results);
}

// ---------------------------------------------------------------------------
// R-TEST-05: Test coverage inventory
// ---------------------------------------------------------------------------

pub fn check_test_coverage_inventory(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
    let mut pub_fn_count: usize = 0;
    let mut test_fn_count: usize = 0;

    // Walk ALL .rs files under workspace for both pub fn and test fn counts
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
        // Skip test fixture files
        let path_str = entry.path().display().to_string();
        if path_str.contains("tests/fixtures/") {
            continue;
        }
        let Some(content) = fs.read_file(entry.path()) else {
            continue;
        };

        // Count pub fns only in src/ directories (production code)
        if path_str.contains("/src/") && !path_str.contains("/tests/") {
            pub_fn_count = pub_fn_count.saturating_add(count_pub_fns(&content));
        }

        // Count test fns in all files
        test_fn_count = test_fn_count.saturating_add(count_test_fns(&content));
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
            "{pub_fn_count} public functions in production code, {test_fn_count} test functions across all files (test-to-function ratio: {ratio}%). This is an informational inventory — not a pass/fail check. A ratio below 100% may indicate untested public API surface."
        ),
        file: None,
        line: None,
        inventory: false,
    }.as_inventory());
}

/// Count `pub fn` declarations in content (AST-based).
pub fn count_pub_fns(content: &str) -> usize {
    let Some(file) = super::ast_helpers::parse_file(content) else {
        return 0;
    };
    super::ast_helpers::count_pub_fn_decls(&file)
}

/// Count `#[test]` and `#[tokio::test]` attributes in content (AST-based).
pub fn count_test_fns(content: &str) -> usize {
    let Some(file) = super::ast_helpers::parse_file(content) else {
        return 0;
    };
    super::ast_helpers::count_test_attrs(&file)
}

// ---------------------------------------------------------------------------
// R-TEST-06: Integration tests exist
// ---------------------------------------------------------------------------

pub fn check_integration_tests(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
    let tests_dir = workspace_root.join("tests");
    if has_rs_files_in_dir(fs, &tests_dir) {
        results.push(
            CheckResult {
                id: "R-TEST-06".to_owned(),
                severity: Severity::Info,
                title: "Integration tests exist".to_owned(),
                message: "tests/ directory with .rs files found".to_owned(),
                file: Some(tests_dir.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
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
            && has_rs_files_in_dir(fs, entry.path())
        {
            results.push(
                CheckResult {
                    id: "R-TEST-06".to_owned(),
                    severity: Severity::Info,
                    title: "Integration tests exist".to_owned(),
                    message: format!(
                        "tests/ directory with .rs files found at {}",
                        entry.path().display()
                    ),
                    file: Some(entry.path().display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
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
        inventory: false,
    });
}

/// Check if a directory exists and contains at least one .rs file.
fn has_rs_files_in_dir(fs: &dyn FileSystem, dir: &Path) -> bool {
    if !dir.exists() {
        return false;
    }
    for entry in fs.list_dir(dir) {
        if let Some(name) = entry.file_name().to_str() {
            if Path::new(name).extension().is_some_and(|e| e == "rs") {
                return true;
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// R-TEST-07: No #[ignore] without reason
// ---------------------------------------------------------------------------

pub fn check_ignore_without_reason(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
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
        let Some(content) = fs.read_file(entry.path()) else {
            continue;
        };

        let violations = find_ignore_without_reason(&content);
        for line_num in &violations {
            found_violation = true;
            results.push(CheckResult {
                id: "R-TEST-07".to_owned(),
                severity: Severity::Warn,
                title: "#[ignore] without reason".to_owned(),
                message: "`#[ignore]` attribute without a documented reason. Ignored tests without reasons become permanently forgotten — no one knows if the issue was fixed or if the test is still relevant. Add `// reason: <why>` on the same line. Example: `#[ignore] // reason: requires external service`"
                    .to_owned(),
                file: Some(entry.path().display().to_string()),
                line: Some(*line_num),
                inventory: false,
            });
        }
    }

    if !found_violation {
        results.push(CheckResult {
            id: "R-TEST-07".to_owned(),
            severity: Severity::Info,
            title: "All #[ignore] have reasons".to_owned(),
            message: "No bare `#[ignore]` attributes found — all ignored tests have documented reasons. This ensures ignored tests are tracked and revisited. No action needed.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }.as_inventory());
    }
}

/// Find lines with #[ignore] that lack a reason comment (AST-based).
/// Returns 1-based line numbers of violations.
pub fn find_ignore_without_reason(content: &str) -> Vec<usize> {
    let Some(file) = super::ast_helpers::parse_file(content) else {
        return Vec::new();
    };
    super::ast_helpers::find_ignore_without_reason(&file, content)
}

// ---------------------------------------------------------------------------
// R-TEST-08: Mutation test hook configured
// ---------------------------------------------------------------------------

pub fn check_mutation_hook(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) {
    // Check .claude/ directory for hook configs mentioning "mutant"
    let claude_dir = workspace_root.join(".claude");
    if claude_dir.exists() {
        for entry in fs.list_dir(&claude_dir) {
            let path = entry.path();
            if let Some(content) = fs.read_file(&path) {
                if content.contains("mutant") || content.contains("cargo-mutants") {
                    results.push(
                        CheckResult {
                            id: "R-TEST-08".to_owned(),
                            severity: Severity::Info,
                            title: "Mutation test hook configured".to_owned(),
                            message: format!("Mutation testing hook found in {}", path.display()),
                            file: Some(path.display().to_string()),
                            line: None,
                            inventory: false,
                        }
                        .as_inventory(),
                    );
                    return;
                }
            }
        }
    }

    // Check .git/hooks/pre-commit
    let pre_commit = workspace_root.join(".git").join("hooks").join("pre-commit");
    if let Some(content) = fs.read_file(&pre_commit) {
        if content.contains("cargo mutants")
            || content.contains("cargo-mutants")
            || content.contains("stryker")
        {
            results.push(
                CheckResult {
                    id: "R-TEST-08".to_owned(),
                    severity: Severity::Info,
                    title: "Mutation test hook configured".to_owned(),
                    message: "Mutation testing found in pre-commit hook".to_owned(),
                    file: Some(pre_commit.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
            return;
        }
    }

    results.push(CheckResult {
        id: "R-TEST-08".to_owned(),
        severity: Severity::Warn,
        title: "No mutation test hook".to_owned(),
        message: "No mutation testing hook found in `.claude/` or `.git/hooks/pre-commit`. Mutation testing (cargo-mutants) injects bugs to verify tests catch them — without a hook, it won't run automatically. Add a `.claude/` hook config or a pre-commit hook step that runs `cargo mutants`.".to_owned(),
        file: None,
        line: None,
        inventory: false,
    });
}
