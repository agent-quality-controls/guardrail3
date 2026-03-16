use std::path::Path;

use walkdir::WalkDir;

use super::ast_helpers;
use super::source_scan::is_excluded_ts_dir;
use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// Run all TypeScript test quality checks.
pub fn check(fs: &dyn FileSystem, path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    check_stryker_config(path, &mut results);
    check_test_files_exist(path, &mut results);
    check_test_runner_config(path, &mut results);

    // Source scanning checks: walk TS/TSX files
    let ts_files = collect_ts_tsx_files(path);
    for file_path in &ts_files {
        let fp = Path::new(file_path);
        let Some(content) = fs.read_file(fp) else {
            continue;
        };
        let display = fp.display().to_string();

        let skip_results = check_skip_without_reason_content(&content, &display);
        results.extend(skip_results);

        let only_results = check_only_in_source_content(&content, &display);
        results.extend(only_results);
    }

    results
}

/// T-TEST-01: Stryker mutation testing config exists.
pub fn check_stryker_config(path: &Path, results: &mut Vec<CheckResult>) {
    let candidates = ["stryker.config.json", "stryker.config.mjs"];
    let found = candidates.iter().any(|name| path.join(name).exists());

    if found {
        results.push(CheckResult {
            id: "T-TEST-01".to_owned(),
            severity: Severity::Info,
            title: "Stryker config found".to_owned(),
            message: "Mutation testing config present".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T-TEST-01".to_owned(),
            severity: Severity::Warn,
            title: "No Stryker config".to_owned(),
            message: "No stryker.config.json or stryker.config.mjs found — mutation testing not configured".to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

/// T-TEST-02: At least one test file exists.
pub fn check_test_files_exist(path: &Path, results: &mut Vec<CheckResult>) {
    let has_tests = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_excluded_ts_dir(e))
        .flatten()
        .any(|entry| {
            if !entry.file_type().is_file() {
                return false;
            }
            let name = entry.file_name().to_string_lossy();
            is_test_file(&name)
        });

    if has_tests {
        results.push(CheckResult {
            id: "T-TEST-02".to_owned(),
            severity: Severity::Info,
            title: "Test files found".to_owned(),
            message: "At least one .test.ts/.spec.ts/.test.tsx/.spec.tsx file exists".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T-TEST-02".to_owned(),
            severity: Severity::Error,
            title: "No test files".to_owned(),
            message: "No .test.ts, .spec.ts, .test.tsx, or .spec.tsx files found".to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

fn is_test_file(name: &str) -> bool {
    let p = Path::new(name);
    let ext_matches = p.extension().is_some_and(|e| e == "ts" || e == "tsx");
    if !ext_matches {
        return false;
    }
    let stem = p.file_stem().unwrap_or_default();
    let stem_path = Path::new(stem);
    stem_path.extension().is_some_and(|e| e == "test" || e == "spec")
}/// T-TEST-03: Test runner configured.
pub fn check_test_runner_config(path: &Path, results: &mut Vec<CheckResult>) {
    let candidates = [
        "vitest.config.ts",
        "vitest.config.mts",
        "jest.config.ts",
        "jest.config.js",
        "jest.config.mjs",
    ];
    let found = candidates.iter().any(|name| path.join(name).exists());

    if found {
        results.push(CheckResult {
            id: "T-TEST-03".to_owned(),
            severity: Severity::Info,
            title: "Test runner configured".to_owned(),
            message: "Test runner config found".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T-TEST-03".to_owned(),
            severity: Severity::Warn,
            title: "No test runner config".to_owned(),
            message: "No vitest.config.ts/mts or jest.config.ts/js/mjs found".to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

/// Collect .ts and .tsx files, skipping common non-source directories.
fn collect_ts_tsx_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_excluded_ts_dir(e))
        .flatten()
    {
        if entry.file_type().is_file() {
            let ep = entry.path();
            if ep.extension().is_some_and(|e| e == "ts" || e == "tsx") {
                files.push(ep.display().to_string());
            }
        }
    }
    files
}

/// T-TEST-04: No `.skip()` without reason comment on same line.
///
/// Detects `test.skip(`, `describe.skip(`, `it.skip(` without `// reason` on the same line.
/// Uses tree-sitter AST for accurate detection (no false positives from strings/comments).
/// Returns results for a single file's content. Testable without filesystem.
pub fn check_skip_without_reason_content(content: &str, filename: &str) -> Vec<CheckResult> {
    let is_tsx = Path::new(filename).extension().is_some_and(|e| e == "tsx");
    let Some(tree) = ast_helpers::parse_ts_file(content, is_tsx) else {
        return Vec::new();
    };
    let skip_lines = ast_helpers::find_test_method_calls(&tree, content, "skip");
    check_skip_lines_with_reason(content, filename, &skip_lines)
}

/// Given confirmed skip call line numbers (from tree-sitter), check each for `// reason`.
fn check_skip_lines_with_reason(
    content: &str,
    filename: &str,
    skip_lines: &[usize],
) -> Vec<CheckResult> {
    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();

    for &line_number in skip_lines {
        let line_text = lines
            .get(line_number.saturating_sub(1))
            .map_or("", |l| l.trim());

        if line_text.contains("// reason") {
            results.push(CheckResult {
                id: "T-TEST-04".to_owned(),
                severity: Severity::Info,
                title: "test.skip with reason".to_owned(),
                message: line_text.to_owned(),
                file: Some(filename.to_owned()),
                line: Some(line_number),
                inventory: false,
            });
        } else {
            results.push(CheckResult {
                id: "T-TEST-04".to_owned(),
                severity: Severity::Warn,
                title: "test.skip without reason".to_owned(),
                message: format!("Add `// reason: <why>` comment: {line_text}"),
                file: Some(filename.to_owned()),
                line: Some(line_number),
                inventory: false,
            });
        }
    }

    results
}

/// T-TEST-05: No `.only()` in committed code.
///
/// Detects `test.only(`, `describe.only(`, `it.only(`.
/// These should never be committed — they cause other tests to be silently skipped.
/// Uses tree-sitter AST for accurate detection (no false positives from strings/comments).
/// Returns results for a single file's content. Testable without filesystem.
pub fn check_only_in_source_content(content: &str, filename: &str) -> Vec<CheckResult> {
    let is_tsx = Path::new(filename).extension().is_some_and(|e| e == "tsx");
    let Some(tree) = ast_helpers::parse_ts_file(content, is_tsx) else {
        return Vec::new();
    };
    let only_lines = ast_helpers::find_test_method_calls(&tree, content, "only");
    check_only_lines(content, filename, &only_lines)
}

/// Given confirmed `.only()` call line numbers (from tree-sitter), emit errors.
fn check_only_lines(content: &str, filename: &str, only_lines: &[usize]) -> Vec<CheckResult> {
    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();

    for &line_number in only_lines {
        let line_text = lines
            .get(line_number.saturating_sub(1))
            .map_or("", |l| l.trim());

        results.push(CheckResult {
            id: "T-TEST-05".to_owned(),
            severity: Severity::Error,
            title: ".only() in committed code".to_owned(),
            message: format!("Remove .only() before committing: {line_text}"),
            file: Some(filename.to_owned()),
            line: Some(line_number),
            inventory: false,
        });
    }

    results
}

