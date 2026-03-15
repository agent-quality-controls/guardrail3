use std::path::Path;

use walkdir::WalkDir;

use super::source_scan::is_excluded_ts_dir;
use crate::report::types::{CheckResult, Severity};

/// Run all TypeScript test quality checks.
pub fn check(path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    check_stryker_config(path, &mut results);
    check_test_files_exist(path, &mut results);
    check_test_runner_config(path, &mut results);

    // Source scanning checks: walk TS/TSX files
    let ts_files = collect_ts_tsx_files(path);
    for file_path in &ts_files {
        let fp = Path::new(file_path);
        let Some(content) = crate::fs::read_file(fp) else {
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
fn check_stryker_config(path: &Path, results: &mut Vec<CheckResult>) {
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
        });
    } else {
        results.push(CheckResult {
            id: "T-TEST-01".to_owned(),
            severity: Severity::Warn,
            title: "No Stryker config".to_owned(),
            message: "No stryker.config.json or stryker.config.mjs found — mutation testing not configured".to_owned(),
            file: None,
            line: None,
        });
    }
}

/// T-TEST-02: At least one test file exists.
fn check_test_files_exist(path: &Path, results: &mut Vec<CheckResult>) {
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
        });
    } else {
        results.push(CheckResult {
            id: "T-TEST-02".to_owned(),
            severity: Severity::Error,
            title: "No test files".to_owned(),
            message: "No .test.ts, .spec.ts, .test.tsx, or .spec.tsx files found".to_owned(),
            file: None,
            line: None,
        });
    }
}

#[allow(clippy::case_sensitive_file_extension_comparisons)] // reason: test file extensions are always lowercase ASCII
fn is_test_file(name: &str) -> bool {
    name.ends_with(".test.ts")
        || name.ends_with(".spec.ts")
        || name.ends_with(".test.tsx")
        || name.ends_with(".spec.tsx")
}

/// T-TEST-03: Test runner configured.
fn check_test_runner_config(path: &Path, results: &mut Vec<CheckResult>) {
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
        });
    } else {
        results.push(CheckResult {
            id: "T-TEST-03".to_owned(),
            severity: Severity::Warn,
            title: "No test runner config".to_owned(),
            message: "No vitest.config.ts/mts or jest.config.ts/js/mjs found".to_owned(),
            file: None,
            line: None,
        });
    }
}

/// Collect .ts and .tsx files, skipping common non-source directories.
#[allow(clippy::case_sensitive_file_extension_comparisons)] // reason: only checking .ts/.tsx files
fn collect_ts_tsx_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_excluded_ts_dir(e))
        .flatten()
    {
        if entry.file_type().is_file() {
            let path_str = entry.path().display().to_string();
            if path_str.ends_with(".ts") || path_str.ends_with(".tsx") {
                files.push(path_str);
            }
        }
    }
    files
}

/// T-TEST-04: No `.skip()` without reason comment on same line.
///
/// Detects `test.skip(`, `describe.skip(`, `it.skip(` without `// reason` on the same line.
/// Returns results for a single file's content. Testable without filesystem.
pub fn check_skip_without_reason_content(content: &str, filename: &str) -> Vec<CheckResult> {
    let skip_patterns = ["test.skip(", "describe.skip(", "it.skip("];
    let mut results = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Skip comment-only lines
        if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
            continue;
        }

        let has_skip = skip_patterns.iter().any(|p| trimmed.contains(p));
        if !has_skip {
            continue;
        }

        let line_number = line_num.saturating_add(1);

        if trimmed.contains("// reason") {
            results.push(CheckResult {
                id: "T-TEST-04".to_owned(),
                severity: Severity::Info,
                title: "test.skip with reason".to_owned(),
                message: trimmed.to_owned(),
                file: Some(filename.to_owned()),
                line: Some(line_number),
            });
        } else {
            results.push(CheckResult {
                id: "T-TEST-04".to_owned(),
                severity: Severity::Warn,
                title: "test.skip without reason".to_owned(),
                message: format!("Add `// reason: <why>` comment: {trimmed}"),
                file: Some(filename.to_owned()),
                line: Some(line_number),
            });
        }
    }

    results
}

/// T-TEST-05: No `.only()` in committed code.
///
/// Detects `test.only(`, `describe.only(`, `it.only(`.
/// These should never be committed — they cause other tests to be silently skipped.
/// Returns results for a single file's content. Testable without filesystem.
pub fn check_only_in_source_content(content: &str, filename: &str) -> Vec<CheckResult> {
    let only_patterns = ["test.only(", "describe.only(", "it.only("];
    let mut results = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Skip comment-only lines
        if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
            continue;
        }

        let has_only = only_patterns.iter().any(|p| trimmed.contains(p));
        if !has_only {
            continue;
        }

        let line_number = line_num.saturating_add(1);
        results.push(CheckResult {
            id: "T-TEST-05".to_owned(),
            severity: Severity::Error,
            title: ".only() in committed code".to_owned(),
            message: format!("Remove .only() before committing: {trimmed}"),
            file: Some(filename.to_owned()),
            line: Some(line_number),
        });
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs as stdfs; // only in tests — not production code

    #[allow(clippy::expect_used)] // reason: test infra — panic on temp dir failure is fine
    fn make_temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("failed to create temp dir")
    }

    // ---- T-TEST-01: Stryker config ----

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    fn t_test_01_no_stryker_config() {
        let dir = make_temp_dir();
        let mut results = Vec::new();
        check_stryker_config(dir.path(), &mut results);
        assert_eq!(results.len(), 1, "Should produce exactly one result");
        assert_eq!(results[0].id, "T-TEST-01");
        assert_eq!(results[0].severity, Severity::Warn);
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    #[allow(clippy::disallowed_methods)] // reason: test creates temp files
    fn t_test_01_stryker_config_exists() {
        let dir = make_temp_dir();
        stdfs::write(dir.path().join("stryker.config.json"), "{}").expect("write config");
        let mut results = Vec::new();
        check_stryker_config(dir.path(), &mut results);
        assert_eq!(results.len(), 1, "Should produce exactly one result");
        assert_eq!(results[0].id, "T-TEST-01");
        assert_eq!(results[0].severity, Severity::Info);
    }

    // ---- T-TEST-02: Test files exist ----

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    fn t_test_02_no_test_files() {
        let dir = make_temp_dir();
        let mut results = Vec::new();
        check_test_files_exist(dir.path(), &mut results);
        assert_eq!(results.len(), 1, "Should produce exactly one result");
        assert_eq!(results[0].id, "T-TEST-02");
        assert_eq!(results[0].severity, Severity::Error);
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    #[allow(clippy::disallowed_methods)] // reason: test creates temp files
    fn t_test_02_test_file_exists() {
        let dir = make_temp_dir();
        stdfs::write(dir.path().join("foo.test.ts"), "test('a', () => {})")
            .expect("write test file");
        let mut results = Vec::new();
        check_test_files_exist(dir.path(), &mut results);
        assert_eq!(results.len(), 1, "Should produce exactly one result");
        assert_eq!(results[0].id, "T-TEST-02");
        assert_eq!(results[0].severity, Severity::Info);
    }

    // ---- T-TEST-03: Test runner config ----

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    fn t_test_03_no_runner_config() {
        let dir = make_temp_dir();
        let mut results = Vec::new();
        check_test_runner_config(dir.path(), &mut results);
        assert_eq!(results.len(), 1, "Should produce exactly one result");
        assert_eq!(results[0].id, "T-TEST-03");
        assert_eq!(results[0].severity, Severity::Warn);
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    #[allow(clippy::disallowed_methods)] // reason: test creates temp files
    fn t_test_03_vitest_config_exists() {
        let dir = make_temp_dir();
        stdfs::write(dir.path().join("vitest.config.ts"), "export default {}")
            .expect("write config");
        let mut results = Vec::new();
        check_test_runner_config(dir.path(), &mut results);
        assert_eq!(results.len(), 1, "Should produce exactly one result");
        assert_eq!(results[0].id, "T-TEST-03");
        assert_eq!(results[0].severity, Severity::Info);
    }

    // ---- T-TEST-04: .skip() without reason ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    fn t_test_04_bare_skip_flagged() {
        let content = "test.skip('broken test', () => {\n  expect(1).toBe(1);\n});";
        let results = check_skip_without_reason_content(content, "app.test.ts");
        assert_eq!(results.len(), 1, "Should flag one skip");
        assert_eq!(results[0].id, "T-TEST-04");
        assert_eq!(results[0].severity, Severity::Warn);
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    fn t_test_04_skip_with_reason_ok() {
        let content = "test.skip('broken test', () => {}); // reason: flaky on CI, tracked in #123";
        let results = check_skip_without_reason_content(content, "app.test.ts");
        assert_eq!(results.len(), 1, "Should produce info for skip with reason");
        assert_eq!(results[0].id, "T-TEST-04");
        assert_eq!(results[0].severity, Severity::Info);
    }

    // ---- T-TEST-05: .only() in committed code ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vec
    fn t_test_05_only_flagged() {
        let content = "describe.only('my suite', () => {\n  it('works', () => {});\n});";
        let results = check_only_in_source_content(content, "app.test.ts");
        assert_eq!(results.len(), 1, "Should flag one .only()");
        assert_eq!(results[0].id, "T-TEST-05");
        assert_eq!(results[0].severity, Severity::Error);
    }

    #[test]
    fn t_test_05_clean_source() {
        let content = "describe('my suite', () => {\n  it('works', () => {});\n});";
        let results = check_only_in_source_content(content, "app.test.ts");
        assert!(
            results.is_empty(),
            "Clean source should produce no T-TEST-05 results"
        );
    }
}
