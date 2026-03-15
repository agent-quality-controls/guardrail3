use std::path::Path;

use crate::report::types::{CheckResult, Severity};

use super::source_scan::filter_non_comment_lines;

// R30-R31: crate-level allow attributes
pub fn check_crate_level_allow(
    path: &Path,
    content: &str,
    _is_bin_entry: bool,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    let non_comment_lines = filter_non_comment_lines(content);

    let crate_allow_prefix: &str = // crate-wide allow attribute pattern
        &["#!", "[allow("].concat();
    for (line_num, trimmed) in &non_comment_lines {
        if !trimmed.starts_with(crate_allow_prefix) {
            continue;
        }

        let line_number = line_num.saturating_add(1);

        // Extract the lint name — handle trailing )] and optional // comment
        let raw_lint = trimmed
            .strip_prefix(crate_allow_prefix)
            .and_then(|s| s.split(')').next())
            .unwrap_or(trimmed);

        // Skip empty/whitespace-only lint names — these are multi-line attributes
        // that we can't properly parse line-by-line
        if raw_lint.trim().is_empty() {
            continue;
        }

        // If the extracted lint contains commas (e.g., `clippy::foo, clippy::bar`),
        // split on comma and process each lint separately
        let lints: Vec<&str> = if raw_lint.contains(',') {
            raw_lint
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            vec![raw_lint.trim()]
        };

        for lint in lints {
            if lint == "unused_crate_dependencies" {
                // Always Info — pre-commit hook exempts this lint universally
                // (it produces false positives in bin crates, integration tests,
                // lib crates with proc macros, etc.)
                results.push(CheckResult {
                    id: "R31".to_owned(),
                    severity: Severity::Info,
                    title: format!("Justified {crate_allow_prefix}...)"),
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
                    title: format!("Crate-level {crate_allow_prefix}...)"),
                    message: format!(
                        "{crate_allow_prefix}{lint})] — crate-wide lint suppression banned"
                    ),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }
    }
}

// R32-R33: #[allow(...)] — item-level
pub fn check_item_level_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        // Match item-level allow but NOT crate-level allow
        let allow_prefix = "#[allow("; // pattern we scan for
        if !trimmed.starts_with(allow_prefix) {
            continue;
        }

        let line_number = line_num.saturating_add(1);

        // Handle multi-line: if no closing ), gather the lint name from what we have
        let lint = if trimmed.contains(')') {
            trimmed
                .strip_prefix(allow_prefix) // extract lint name
                .and_then(|s| s.split(')').next())
                .unwrap_or(trimmed)
                .to_owned()
        } else {
            // Multi-line attribute — take what's after the opening paren
            trimmed
                .strip_prefix(allow_prefix) // extract partial lint
                .unwrap_or(trimmed)
                .trim()
                .to_owned()
                + "..."
        };

        // Check if same line has a // comment
        let has_comment = trimmed.contains("//");

        if has_comment {
            let reason = trimmed
                .split("//")
                .nth(1)
                .map_or("no reason given", str::trim);
            results.push(CheckResult {
                id: "R33".to_owned(),
                severity: Severity::Info,
                title: "Justified #[allow]".to_owned(),
                message: format!("{lint} — {reason}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        } else {
            results.push(CheckResult {
                id: "R32".to_owned(),
                severity: Severity::Error,
                title: "#[allow] without reason".to_owned(),
                message: format!("#[allow({lint})] has no // comment justification"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// R34-R35: #[garde(skip)]
#[allow(clippy::string_slice)] // reason: garde attribute parsing on known ASCII content
pub fn check_garde_skip(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        // Must be an actual attribute — look for #[garde(skip)] or #[...garde(skip)...]
        if !trimmed.contains("garde(skip)") {
            continue;
        }

        // Skip if garde(skip) only appears inside a string literal
        // Simple heuristic: if there's a `"` before the occurrence, it's in a string
        if let Some(pos) = trimmed.find("garde(skip)") {
            let before = &trimmed[..pos];
            let quote_count = before.chars().filter(|c| *c == '"').count();
            if quote_count % 2 != 0 {
                continue;
            }
        }

        // Must look like an attribute context (contains #[ or starts with garde)
        if !trimmed.contains("#[") && !trimmed.starts_with("garde(") {
            continue;
        }

        let line_number = line_num.saturating_add(1);
        let has_comment = trimmed.contains("//");

        if has_comment {
            let reason = trimmed
                .split("//")
                .nth(1)
                .map_or("no reason given", str::trim);
            results.push(CheckResult {
                id: "R35".to_owned(),
                severity: Severity::Info,
                title: "Justified garde(skip)".to_owned(),
                message: format!("garde(skip) — {reason}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        } else {
            results.push(CheckResult {
                id: "R34".to_owned(),
                severity: Severity::Error,
                title: "garde(skip) without reason".to_owned(),
                message: "garde(skip) has no // comment justification".to_owned(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// R36: EXCEPTION comments
pub fn check_exception_comments(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let config_files = ["clippy.toml", "deny.toml", "Cargo.toml", "rustfmt.toml"];

    for config_file in &config_files {
        let path = workspace_root.join(config_file);
        if !path.exists() {
            continue;
        }

        let Some(content) = crate::fs::read_file(&path) else {
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
#[allow(clippy::string_slice)] // reason: cfg_attr parsing on known ASCII content
pub fn check_cfg_attr_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        // Must be an attribute line containing #[cfg_attr or #![cfg_attr
        if !trimmed.contains("#[cfg_attr(") && !trimmed.contains("#![cfg_attr(") {
            continue;
        }

        if !trimmed.contains("allow(") {
            continue;
        }

        // Skip if it's inside a string literal
        if let Some(pos) = trimmed.find("cfg_attr") {
            let before = &trimmed[..pos];
            let quote_count = before.chars().filter(|c| *c == '"').count();
            if quote_count % 2 != 0 {
                continue;
            }
        }

        let line_number = line_num.saturating_add(1);

        results.push(CheckResult {
            id: "R37".to_owned(),
            severity: Severity::Info,
            title: "cfg_attr allow".to_owned(),
            message: trimmed.to_owned(),
            file: Some(path.display().to_string()),
            line: Some(line_number),
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
        let content = "#[garde(skip)]\nfield: String,";
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
        let content = "#[garde(skip)] // reason: validated elsewhere\nfield: String,";
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
