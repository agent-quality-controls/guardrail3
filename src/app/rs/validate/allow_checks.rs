use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

use super::ast_helpers;
use crate::ports::outbound::FileSystem;

/// Prefix constant used in R30-R31 messages.
const CRATE_ALLOW_PREFIX: &str = "#![allow(";

// R30-R31: crate-level allow attributes (syn-based)
pub fn check_crate_level_allow(
    path: &Path,
    content: &str,
    _is_bin_entry: bool,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    let source_lines: Vec<&str> = content.lines().collect();
    for (line, lint) in &ast_helpers::find_crate_level_allows(&file) {
        emit_crate_allow_result(path, lint, *line, is_test_file, results);
    }
    // source_lines available for future comment inspection (e.g. // reason: checks)
    let _ = &source_lines;
}

/// Emit a single R30 or R31 result for one lint in a crate-level allow.
fn emit_crate_allow_result(
    path: &Path,
    lint: &str,
    line_number: usize,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    if lint == "unused_crate_dependencies" {
        // Always Info — pre-commit hook exempts this lint universally
        // (it produces false positives in bin crates, integration tests,
        // lib crates with proc macros, etc.)
        results.push(CheckResult {
            id: "R31".to_owned(),
            severity: Severity::Info,
            title: format!("Justified {CRATE_ALLOW_PREFIX}...)"),
            message: "unused_crate_dependencies — universally exempted".to_owned(),
            file: Some(path.display().to_string()),
            line: Some(line_number),
        });
    } else {
        // Test files are exempt from R30 (matches pre-commit hook behavior
        // which excludes /tests/ from source scanning)
        let severity = if is_test_file {
            Severity::Info
        } else {
            Severity::Error
        };
        results.push(CheckResult {
            id: "R30".to_owned(),
            severity,
            title: format!("Crate-level {CRATE_ALLOW_PREFIX}...)"),
            message: format!(
                "{CRATE_ALLOW_PREFIX}{lint})] — crate-wide lint suppression banned"
            ),
            file: Some(path.display().to_string()),
            line: Some(line_number),
        });
    }
}

// R32-R33: #[allow(...)] — item-level
pub fn check_item_level_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    check_item_level_allow_ast(path, content, &file, results);
}

/// Run AST-based detection for item-level #[allow(...)] attributes.
fn check_item_level_allow_ast(
    path: &Path,
    content: &str,
    file: &syn::File,
    results: &mut Vec<CheckResult>,
) {
    let raw_lines: Vec<&str> = content.lines().collect();
    for (line_1based, lint) in ast_helpers::find_item_allows(file) {
        let has_comment = raw_lines
            .get(line_1based.wrapping_sub(1))
            .is_some_and(|l| l.contains("//"));
        if has_comment {
            let reason = raw_lines
                .get(line_1based.wrapping_sub(1))
                .and_then(|l| l.split("//").nth(1))
                .map_or("no reason given", str::trim);
            results.push(CheckResult {
                id: "R33".to_owned(),
                severity: Severity::Info,
                title: "Justified #[allow]".to_owned(),
                message: format!("{lint} — {reason}"),
                file: Some(path.display().to_string()),
                line: Some(line_1based),
            });
        } else {
            results.push(CheckResult {
                id: "R32".to_owned(),
                severity: Severity::Error,
                title: "#[allow] without reason".to_owned(),
                message: format!("#[allow({lint})] has no // comment justification"),
                file: Some(path.display().to_string()),
                line: Some(line_1based),
            });
        }
    }
}

// R34-R35: #[garde(skip)]
pub fn check_garde_skip(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    check_garde_skip_ast(path, content, &file, results);
}

fn check_garde_skip_ast(
    path: &Path,
    content: &str,
    file: &syn::File,
    results: &mut Vec<CheckResult>,
) {
    let raw_lines: Vec<&str> = content.lines().collect();
    for line_1based in ast_helpers::find_garde_skips(file) {
        let has_comment = raw_lines
            .get(line_1based.wrapping_sub(1))
            .is_some_and(|l| l.contains("//"));
        if has_comment {
            let reason = raw_lines
                .get(line_1based.wrapping_sub(1))
                .and_then(|l| l.split("//").nth(1))
                .map_or("no reason given", str::trim);
            results.push(CheckResult {
                id: "R35".to_owned(),
                severity: Severity::Info,
                title: "Justified garde(skip)".to_owned(),
                message: format!("garde(skip) — {reason}"),
                file: Some(path.display().to_string()),
                line: Some(line_1based),
            });
        } else {
            results.push(CheckResult {
                id: "R34".to_owned(),
                severity: Severity::Error,
                title: "garde(skip) without reason".to_owned(),
                message: "garde(skip) has no // comment justification".to_owned(),
                file: Some(path.display().to_string()),
                line: Some(line_1based),
            });
        }
    }
}

// R36: EXCEPTION comments
pub fn check_exception_comments(fs: &dyn FileSystem, workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let config_files = ["clippy.toml", "deny.toml", "Cargo.toml", "rustfmt.toml"];

    for config_file in &config_files {
        let path = workspace_root.join(config_file);
        if !path.exists() {
            continue;
        }

        let Some(content) = fs.read_file(&path) else {
            continue;
        };

        for (line_num, line) in content.lines().enumerate() {
            if line.contains("// EXCEPTION:") || line.contains("# EXCEPTION:") {
                let line_number = line_num.saturating_add(1);
                results.push(CheckResult {
                    id: "R36".to_owned(),
                    severity: Severity::Info,
                    title: "EXCEPTION comment".to_owned(),
                    message: line.trim().to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }
    }
}

// R37: cfg_attr allow — must be an actual attribute (#[cfg_attr(..., allow(...))])
pub fn check_cfg_attr_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };
    check_cfg_attr_allow_ast(path, content, &file, results);
}

fn check_cfg_attr_allow_ast(
    path: &Path,
    content: &str,
    file: &syn::File,
    results: &mut Vec<CheckResult>,
) {
    let raw_lines: Vec<&str> = content.lines().collect();
    for (line_1based, lint) in ast_helpers::find_cfg_attr_allows(file) {
        let message = raw_lines
            .get(line_1based.wrapping_sub(1))
            .map_or_else(|| format!("cfg_attr allow: {lint}"), |l| l.trim().to_owned());
        results.push(CheckResult {
            id: "R37".to_owned(),
            severity: Severity::Info,
            title: "cfg_attr allow".to_owned(),
            message,
            file: Some(path.display().to_string()),
            line: Some(line_1based),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Bug 2: Check ID mappings R30-R35 ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn crate_level_allow_without_reason_is_error_r30() {
        let attr = ["#!", "[allow(clippy::unwrap_used)]"].concat(); // pre-commit: test string
        let content = format!("{attr}\nfn main() {{}}");
        let path = Path::new("test.rs");
        let mut results = Vec::new();
        check_crate_level_allow(path, &content, false, false, &mut results);
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Error)
            .collect();
        assert!(!errors.is_empty(), "Should produce an error");
        assert_eq!(errors[0].id, "R30", "Should be R30, got {}", errors[0].id);
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn crate_level_allow_unused_crate_deps_is_info_r31() {
        let content = "#![allow(unused_crate_dependencies)]\nfn main() {}";
        let path = Path::new("main.rs");
        let mut results = Vec::new();
        check_crate_level_allow(path, content, true, false, &mut results);
        let infos: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Info)
            .collect();
        assert!(!infos.is_empty(), "Should produce Info");
        assert_eq!(infos[0].id, "R31");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn item_level_allow_without_comment_is_error_r32() {
        // Build the test input by concatenation to avoid tripping the pre-commit grep
        let attr = ["#[allow(", "clippy::unwrap_used)]"].concat(); // pre-commit: test string
        let content = format!("{attr}\nfn foo() {{}}");
        let path = Path::new("test.rs");
        let mut results = Vec::new();
        check_item_level_allow(path, &content, &mut results);
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Error)
            .collect();
        assert!(!errors.is_empty(), "Should produce an error");
        assert_eq!(errors[0].id, "R32");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn item_level_allow_with_comment_is_info_r33() {
        let content = "#[allow(clippy::unwrap_used)] // reason: test\nfn foo() {}";
        let path = Path::new("test.rs");
        let mut results = Vec::new();
        check_item_level_allow(path, content, &mut results);
        let infos: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Info)
            .collect();
        assert!(!infos.is_empty(), "Should produce Info");
        assert_eq!(infos[0].id, "R33");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn garde_skip_without_comment_is_error_r34() {
        let content = "struct Foo {\n    #[garde(skip)]\n    field: String,\n}";
        let path = Path::new("test.rs");
        let mut results = Vec::new();
        check_garde_skip(path, content, &mut results);
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Error)
            .collect();
        assert!(!errors.is_empty(), "Should produce an error");
        assert_eq!(errors[0].id, "R34");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn garde_skip_with_comment_is_info_r35() {
        let content = "struct Foo {\n    #[garde(skip)] // reason: validated elsewhere\n    field: String,\n}";
        let path = Path::new("test.rs");
        let mut results = Vec::new();
        check_garde_skip(path, content, &mut results);
        let infos: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Info)
            .collect();
        assert!(!infos.is_empty(), "Should produce Info");
        assert_eq!(infos[0].id, "R35");
    }

    // ---- Bug 7: unused_crate_dependencies universal exemption ----

    #[test]
    #[allow(clippy::needless_collect)] // reason: collect into Vec for readable test assertions
    fn unused_crate_deps_is_info_in_lib_rs() {
        let content = "#![allow(unused_crate_dependencies)]\nfn main() {}";
        let path = Path::new("src/lib.rs");
        let mut results = Vec::new();
        check_crate_level_allow(path, content, false, false, &mut results);
        // Should be Info (R31), not Error (R30)
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "unused_crate_dependencies should be Info everywhere, not Error"
        );
        let infos: Vec<_> = results
            .iter()
            .filter(|r| r.id == "R31" && r.severity == Severity::Info)
            .collect();
        assert!(
            !infos.is_empty(),
            "Should produce R31 Info for unused_crate_dependencies"
        );
    }

    #[test]
    #[allow(clippy::needless_collect)] // reason: collect into Vec for readable test assertions
    fn unused_crate_deps_is_info_in_any_file() {
        let content = "#![allow(unused_crate_dependencies)]\nmod foo;";
        let path = Path::new("src/some_module.rs");
        let mut results = Vec::new();
        check_crate_level_allow(path, content, false, false, &mut results);
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "unused_crate_dependencies should be Info everywhere"
        );
    }

    // ---- Bug 4 (partial): Test file exemption for R30 ----

    #[test]
    #[allow(clippy::needless_collect)] // reason: collect into Vec for readable test assertions
    fn crate_level_allow_in_test_file_is_info_not_error() {
        let attr = ["#!", "[allow(clippy::unwrap_used)]"].concat(); // pre-commit: test string
        let content = format!("{attr}\nfn test_stuff() {{}}");
        let path = Path::new("/project/tests/integration.rs");
        let mut results = Vec::new();
        check_crate_level_allow(path, &content, false, true, &mut results);
        let errors: Vec<_> = results
            .iter()
            .filter(|r| r.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Test files should be exempt from R30 errors"
        );
    }
}
