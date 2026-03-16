use std::path::Path;

use crate::report::types::{CheckResult, Severity};

use super::ast_helpers;
use super::source_scan::filter_non_comment_lines;

/// Compute which 0-based line numbers are inside multi-line string literals.
/// Tracks quote parity across lines, accounting for escaped quotes,
/// line continuations (`\` at end of line inside a string), and raw strings
/// (`r"..."`, `r#"..."#`, `r##"..."##`, etc.).
fn compute_multiline_string_lines(content: &str) -> std::collections::BTreeSet<usize> {
    let mut in_string = false;
    let mut string_lines = std::collections::BTreeSet::new();
    // When inside a raw string, this holds the closing delimiter: `"` + N `#` chars.
    // When empty, we are not inside a raw string (but may be inside a regular string).
    let mut raw_close: Option<String> = None;

    for (line_num, line) in content.lines().enumerate() {
        if in_string {
            let _ = string_lines.insert(line_num);
        }

        let bytes = line.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            // --- Inside a raw string: scan for the closing delimiter ---
            if let Some(ref closer) = raw_close {
                if let Some(pos) = line[i..].find(closer.as_str()) {
                    i = i.saturating_add(pos).saturating_add(closer.len());
                    raw_close = None;
                    in_string = false;
                } else {
                    // Closing delimiter not on this line — whole rest is string
                    break;
                }
                continue;
            }

            // --- Inside a regular (non-raw) string ---
            if in_string {
                if bytes[i] == b'\\' {
                    // skip escaped char
                    i = i.saturating_add(2);
                    continue;
                }
                if bytes[i] == b'"' {
                    in_string = false;
                    i = i.saturating_add(1);
                    continue;
                }
                i = i.saturating_add(1);
                continue;
            }

            // --- Not inside any string ---
            if bytes[i] == b'"' {
                in_string = true;
                i = i.saturating_add(1);
                continue;
            }
            // Detect raw string opener: r followed by optional #s then "
            if bytes[i] == b'r' {
                let mut hashes: usize = 0;
                let mut j = i.saturating_add(1);
                while j < bytes.len() && bytes[j] == b'#' {
                    hashes = hashes.saturating_add(1);
                    j = j.saturating_add(1);
                }
                if j < bytes.len() && bytes[j] == b'"' && (hashes > 0 || j == i.saturating_add(1))
                {
                    // Verify `r` is not part of an identifier: check char before
                    let is_ident_char = i > 0
                        && (bytes[i.saturating_sub(1)].is_ascii_alphanumeric()
                            || bytes[i.saturating_sub(1)] == b'_');
                    if !is_ident_char {
                        // Build the closing delimiter: `"` followed by `hashes` `#` chars
                        let mut closer = String::with_capacity(hashes.saturating_add(1));
                        closer.push('"');
                        for _ in 0..hashes {
                            closer.push('#');
                        }
                        raw_close = Some(closer);
                        in_string = true;
                        i = j.saturating_add(1); // skip past the opening `"`
                        continue;
                    }
                }
            }
            i = i.saturating_add(1);
        }
    }

    string_lines
}

/// Prefix constant used in R30-R31 messages.
const CRATE_ALLOW_PREFIX: &str = "#![allow(";

// R30-R31: crate-level allow attributes (syn-based with grep fallback)
pub fn check_crate_level_allow(
    path: &Path,
    content: &str,
    is_bin_entry: bool,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    if let Some(file) = ast_helpers::parse_file(content) {
        // syn parse succeeded — use AST-based detection
        let source_lines: Vec<&str> = content.lines().collect();
        for (line, lint) in &ast_helpers::find_crate_level_allows(&file) {
            emit_crate_allow_result(path, lint, *line, is_test_file, results);
        }
        // source_lines available for future comment inspection (e.g. // reason: checks)
        let _ = &source_lines;
    } else {
        // Parse failed — fall back to grep-based scanning
        check_crate_level_allow_grep(path, content, is_bin_entry, is_test_file, results);
    }
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

// Grep-based fallback for R30-R31 when syn parsing fails.
fn check_crate_level_allow_grep(
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
            emit_crate_allow_result(path, lint, line_number, is_test_file, results);
        }
    }
}

// R32-R33: #[allow(...)] — item-level
pub fn check_item_level_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    if let Some(file) = ast_helpers::parse_file(content) {
        // AST path: accurate detection for items syn can see.
        // Then grep catches allows inside macro bodies that syn skips.
        let ast_lines = check_item_level_allow_ast(path, content, &file, results);
        check_item_level_allow_grep_supplement(path, content, &ast_lines, results);
    } else {
        check_item_level_allow_grep(path, content, results);
    }
}

/// Run AST-based detection. Returns the set of 1-based line numbers already reported.
fn check_item_level_allow_ast(
    path: &Path,
    content: &str,
    file: &syn::File,
    results: &mut Vec<CheckResult>,
) -> std::collections::BTreeSet<usize> {
    let raw_lines: Vec<&str> = content.lines().collect();
    let mut covered = std::collections::BTreeSet::new();
    for (line_1based, lint) in ast_helpers::find_item_allows(file) {
        let _ = covered.insert(line_1based);
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
    covered
}

/// Grep supplement: catch #[allow(..)] inside macro bodies that syn cannot visit.
/// Skips lines already reported by the AST pass.
fn check_item_level_allow_grep_supplement(
    path: &Path,
    content: &str,
    ast_covered: &std::collections::BTreeSet<usize>,
    results: &mut Vec<CheckResult>,
) {
    let non_comment_lines = filter_non_comment_lines(content);
    let string_lines = compute_multiline_string_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        let allow_prefix = "#[allow("; // pattern we scan for
        if !trimmed.starts_with(allow_prefix) {
            continue;
        }
        let line_number = line_num.saturating_add(1);
        // Skip lines already reported by AST pass
        if ast_covered.contains(&line_number) {
            continue;
        }
        // Skip lines inside multi-line string literals
        if string_lines.contains(line_num) {
            continue;
        }
        let lint = if trimmed.contains(')') {
            trimmed
                .strip_prefix(allow_prefix)
                .and_then(|s| s.split(')').next())
                .unwrap_or(trimmed)
                .to_owned()
        } else {
            trimmed
                .strip_prefix(allow_prefix)
                .unwrap_or(trimmed)
                .trim()
                .to_owned()
                + "..."
        };
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

fn check_item_level_allow_grep(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
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
pub fn check_garde_skip(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    if let Some(file) = ast_helpers::parse_file(content) {
        check_garde_skip_ast(path, content, &file, results);
    } else {
        check_garde_skip_grep(path, content, results);
    }
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

#[allow(clippy::string_slice)] // reason: garde attribute parsing on known ASCII content
fn check_garde_skip_grep(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        if !trimmed.contains("garde(skip)") {
            continue;
        }
        if let Some(pos) = trimmed.find("garde(skip)") {
            let before = &trimmed[..pos];
            let quote_count = before.chars().filter(|c| *c == '"').count();
            if quote_count % 2 != 0 {
                continue;
            }
        }
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
pub fn check_cfg_attr_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    if let Some(file) = ast_helpers::parse_file(content) {
        check_cfg_attr_allow_ast(path, content, &file, results);
    } else {
        check_cfg_attr_allow_grep(path, content, results);
    }
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

#[allow(clippy::string_slice)] // reason: cfg_attr parsing on known ASCII content
fn check_cfg_attr_allow_grep(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        if !trimmed.contains("#[cfg_attr(") && !trimmed.contains("#![cfg_attr(") {
            continue;
        }
        if !trimmed.contains("allow(") {
            continue;
        }
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

    // ---- Raw string handling in compute_multiline_string_lines ----

    #[test]
    fn multiline_string_lines_handles_raw_strings() {
        // r#"..."# containing a quote and an allow-like pattern must be treated as string
        let content = "let s = r#\"#[allow(unused)]\n  inner line\n\"#;\nreal code";
        let string_lines = compute_multiline_string_lines(content);
        // Line 0 starts the raw string (the opening r#" is on it — rest is string content)
        // Line 1 is inside the raw string
        // Line 2 closes the raw string — the "# is consumed, so this line started in-string
        assert!(
            string_lines.contains(&1),
            "Line 1 should be inside raw string, got: {string_lines:?}"
        );
        assert!(
            string_lines.contains(&2),
            "Line 2 should be inside raw string (started in-string), got: {string_lines:?}"
        );
        assert!(
            !string_lines.contains(&3),
            "Line 3 should NOT be inside raw string, got: {string_lines:?}"
        );
    }

    // ---- R36: EXCEPTION comments ----

    #[test]
    #[allow(clippy::expect_used, clippy::disallowed_methods)] // reason: test setup needs temp dir and direct fs
    fn r36_exception_comment_in_config_file() {
        use std::fs as stdfs;
        let tmp = tempfile::tempdir().expect("failed to create temp dir");
        let clippy_path = tmp.path().join("clippy.toml");
        stdfs::write(
            &clippy_path,
            "# EXCEPTION: allowing foo because bar\nsome_setting = true\n",
        )
        .expect("write clippy.toml");
        let mut results = Vec::new();
        check_exception_comments(tmp.path(), &mut results);
        assert!(!results.is_empty(), "Should detect EXCEPTION comment");
        assert_eq!(results[0].id, "R36");
        assert_eq!(results[0].severity, Severity::Info);
    }

    // ---- R37: cfg_attr allow ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn r37_cfg_attr_allow_detected() {
        let content = "#[cfg_attr(test, allow(unused))]\nfn foo() {}";
        let path = Path::new("test.rs");
        let mut results = Vec::new();
        check_cfg_attr_allow(path, content, &mut results);
        assert!(!results.is_empty(), "Should detect cfg_attr allow");
        assert_eq!(results[0].id, "R37");
        assert_eq!(results[0].severity, Severity::Info);
    }
}
