use std::path::Path;

use walkdir::WalkDir;

use super::ast_helpers::{self, CommentInfo};
use crate::report::types::{CheckResult, Severity};

pub fn check(path: &Path, scoped_files: Option<&[String]>) -> Vec<CheckResult> {
    let mut results = Vec::new();

    let ts_files: Vec<String> = match scoped_files {
        Some(files) => files.iter().filter(|f| is_ts_file(f)).cloned().collect(),
        None => collect_ts_files(path),
    };

    for file_path in &ts_files {
        let fp = Path::new(file_path);
        let Some(content) = crate::fs::read_file(fp) else {
            continue;
        };

        check_eslint_disable(fp, &content, &mut results);
        check_ts_ignore(fp, &content, &mut results);
        check_process_env(fp, &content, &mut results);
        check_any_types(fp, &content, &mut results);
        check_file_length(fp, &content, &mut results);
        // T34: // noinspection
        check_comment_pattern(
            fp,
            &content,
            &["// noinspection", "/* noinspection"],
            "T34",
            "noinspection comment",
            &mut results,
        );
        // T35: istanbul ignore / c8 ignore
        check_comment_pattern(
            fp,
            &content,
            &["istanbul ignore", "c8 ignore"],
            "T35",
            "Coverage ignore comment",
            &mut results,
        );
    }

    // T59: Banned packages in node_modules
    check_banned_in_node_modules(path, &mut results);

    results
}

#[allow(clippy::case_sensitive_file_extension_comparisons)] // reason: only checking .ts/.tsx/.mjs files
fn is_ts_file(path: &str) -> bool {
    path.ends_with(".ts") || path.ends_with(".tsx") || path.ends_with(".mjs")
}

/// Check if a walkdir entry is a directory that should be excluded from TypeScript source scanning.
pub fn is_excluded_ts_dir(entry: &walkdir::DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    name == "node_modules"
        || name == ".next"
        || name == "dist"
        || name == "target"
        || name == "coverage"
        || name == ".git"
        || name == ".claude"
}

fn collect_ts_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_excluded_ts_dir(e))
        .flatten()
    {
        if entry.file_type().is_file() {
            let path_str = entry.path().display().to_string();
            if is_ts_file(&path_str) {
                files.push(path_str);
            }
        }
    }
    files
}

/// Determine whether a file path refers to a TSX file.
#[allow(clippy::case_sensitive_file_extension_comparisons)] // reason: only checking .tsx extension
fn is_tsx_path(path: &Path) -> bool {
    path.to_string_lossy().ends_with(".tsx")
}

// T23-T26: eslint-disable checks (tree-sitter with grep fallback)
fn check_eslint_disable(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    match ast_helpers::parse_ts_file(content, is_tsx_path(path)) {
        Some(tree) => {
            let comments = ast_helpers::find_comments(&tree, content);
            check_eslint_disable_from_comments(path, &comments, results);
        }
        None => check_eslint_disable_grep(path, content, results),
    }
}

/// Tree-sitter path: only inspect actual comment nodes for eslint-disable patterns.
fn check_eslint_disable_from_comments(
    path: &Path,
    comments: &[CommentInfo],
    results: &mut Vec<CheckResult>,
) {
    for comment in comments {
        let text = comment.text.trim();
        let line_number = comment.line;

        // Block-level eslint-disable (T23/T24)
        if text.contains("eslint-disable")
            && !text.contains("eslint-disable-next-line")
            && !text.contains("eslint-disable-line")
        {
            if text.contains("-- ") {
                results.push(CheckResult {
                    id: "T24".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable with reason".to_owned(),
                    message: text.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T23".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable without reason".to_owned(),
                    message: format!("eslint-disable missing `-- ` reason: {text}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // eslint-disable-next-line (T25/T26)
        if text.contains("eslint-disable-next-line") {
            if text.contains("-- ") {
                results.push(CheckResult {
                    id: "T26".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable-next-line with reason".to_owned(),
                    message: text.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T25".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable-next-line without reason".to_owned(),
                    message: format!("Missing `-- ` reason: {text}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // eslint-disable-line (T25/T26 — inline suppression)
        if text.contains("eslint-disable-line") && !text.contains("eslint-disable-line-") {
            if text.contains("-- ") {
                results.push(CheckResult {
                    id: "T26".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable-line with reason".to_owned(),
                    message: text.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T25".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable-line without reason".to_owned(),
                    message: format!("Missing `-- ` reason: {text}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }
    }
}

/// Grep fallback: scan all lines (used when tree-sitter parse fails).
fn check_eslint_disable_grep(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let line_number = line_num.saturating_add(1);

        // Block-level eslint-disable (T23/T24)
        if trimmed.contains("eslint-disable")
            && !trimmed.contains("eslint-disable-next-line")
            && !trimmed.contains("eslint-disable-line")
        {
            if trimmed.contains("-- ") {
                results.push(CheckResult {
                    id: "T24".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable with reason".to_owned(),
                    message: trimmed.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T23".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable without reason".to_owned(),
                    message: format!("eslint-disable missing `-- ` reason: {trimmed}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // eslint-disable-next-line (T25/T26)
        if trimmed.contains("eslint-disable-next-line") {
            if trimmed.contains("-- ") {
                results.push(CheckResult {
                    id: "T26".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable-next-line with reason".to_owned(),
                    message: trimmed.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T25".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable-next-line without reason".to_owned(),
                    message: format!("Missing `-- ` reason: {trimmed}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // eslint-disable-line (T25/T26 — inline suppression)
        if trimmed.contains("eslint-disable-line") && !trimmed.contains("eslint-disable-line-") {
            if trimmed.contains("-- ") {
                results.push(CheckResult {
                    id: "T26".to_owned(),
                    severity: Severity::Info,
                    title: "eslint-disable-line with reason".to_owned(),
                    message: trimmed.to_owned(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "T25".to_owned(),
                    severity: Severity::Error,
                    title: "eslint-disable-line without reason".to_owned(),
                    message: format!("Missing `-- ` reason: {trimmed}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }
    }
}

// T27-T29: @ts-ignore / @ts-expect-error (tree-sitter with grep fallback)
fn check_ts_ignore(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    match ast_helpers::parse_ts_file(content, is_tsx_path(path)) {
        Some(tree) => {
            let comments = ast_helpers::find_comments(&tree, content);
            check_ts_ignore_from_comments(path, &comments, results);
        }
        None => check_ts_ignore_grep(path, content, results),
    }
}

/// Tree-sitter path: only inspect actual comment nodes for ts-ignore/ts-expect-error.
fn check_ts_ignore_from_comments(
    path: &Path,
    comments: &[CommentInfo],
    results: &mut Vec<CheckResult>,
) {
    for comment in comments {
        let text = comment.text.trim();
        let line_number = comment.line;

        // T27: @ts-ignore
        if text.contains("@ts-ignore") {
            results.push(CheckResult {
                id: "T27".to_owned(),
                severity: Severity::Error,
                title: "@ts-ignore usage".to_owned(),
                message: format!("Use @ts-expect-error instead: {text}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }

        // T28/T29: @ts-expect-error
        if text.contains("@ts-expect-error") {
            if let Some(pos) = text.find("@ts-expect-error") {
                #[allow(clippy::string_slice)] // reason: @ts-expect-error is ASCII, byte offset + 16 is safe
                let after = text.get(pos.saturating_add(16)..).unwrap_or("").trim();
                if after.is_empty() || after == "*/" {
                    results.push(CheckResult {
                        id: "T28".to_owned(),
                        severity: Severity::Warn,
                        title: "@ts-expect-error without explanation".to_owned(),
                        message: text.to_owned(),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                    });
                } else {
                    results.push(CheckResult {
                        id: "T29".to_owned(),
                        severity: Severity::Info,
                        title: "@ts-expect-error with explanation".to_owned(),
                        message: text.to_owned(),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                    });
                }
            }
        }
    }
}

/// Grep fallback: scan all lines (used when tree-sitter parse fails).
fn check_ts_ignore_grep(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let line_number = line_num.saturating_add(1);

        // T27: @ts-ignore
        if trimmed.contains("@ts-ignore") {
            results.push(CheckResult {
                id: "T27".to_owned(),
                severity: Severity::Error,
                title: "@ts-ignore usage".to_owned(),
                message: format!("Use @ts-expect-error instead: {trimmed}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }

        // T28/T29: @ts-expect-error
        if trimmed.contains("@ts-expect-error") {
            if let Some(pos) = trimmed.find("@ts-expect-error") {
                #[allow(clippy::string_slice)] // reason: @ts-expect-error is ASCII, byte offset + 16 is safe
                let after = trimmed[pos.saturating_add(16)..].trim();
                if after.is_empty() || after == "*/" {
                    results.push(CheckResult {
                        id: "T28".to_owned(),
                        severity: Severity::Warn,
                        title: "@ts-expect-error without explanation".to_owned(),
                        message: trimmed.to_owned(),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                    });
                } else {
                    results.push(CheckResult {
                        id: "T29".to_owned(),
                        severity: Severity::Info,
                        title: "@ts-expect-error with explanation".to_owned(),
                        message: trimmed.to_owned(),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                    });
                }
            }
        }
    }
}

// T30: process.env direct access (tree-sitter with grep fallback)
fn check_process_env(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    // Skip env.ts, env.mjs files, and all .mjs files (config files that legitimately use process.env)
    if file_name == "env.ts" || file_name == "env.mjs" {
        return;
    }
    if path.extension().and_then(|e| e.to_str()) == Some("mjs") {
        return;
    }

    let is_tsx = is_tsx_path(path);
    match ast_helpers::parse_ts_file(content, is_tsx) {
        Some(tree) => check_process_env_ast(path, content, &tree, results),
        None => check_process_env_grep(path, content, results),
    }
}

/// Tree-sitter path: find `process.env` via AST member-expression nodes.
fn check_process_env_ast(
    path: &Path,
    content: &str,
    tree: &tree_sitter::Tree,
    results: &mut Vec<CheckResult>,
) {
    let lines: Vec<&str> = content.lines().collect();
    let hit_lines = ast_helpers::find_process_env(tree, content);

    for line_number in hit_lines {
        let line_idx = line_number.saturating_sub(1);
        let trimmed = lines.get(line_idx).unwrap_or(&"").trim();

        // Check if the previous line contains eslint-disable-next-line
        let prev_line = if line_idx > 0 {
            lines.get(line_idx.saturating_sub(1))
        } else {
            None
        };
        let is_suppressed = prev_line.is_some_and(|pl| pl.contains("eslint-disable-next-line"));

        let severity = if is_suppressed {
            Severity::Info
        } else {
            Severity::Error
        };

        let message = if is_suppressed {
            format!("ESLint-suppressed process.env access: {trimmed}")
        } else {
            format!("Use env() import instead: {trimmed}")
        };

        results.push(CheckResult {
            id: "T30".to_owned(),
            severity,
            title: "Direct process.env access".to_owned(),
            message,
            file: Some(path.display().to_string()),
            line: Some(line_number),
        });
    }
}

/// Grep fallback for T30: scan lines for process.env (used when tree-sitter parse fails).
fn check_process_env_grep(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let lines: Vec<&str> = content.lines().collect();
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Skip comment lines
        if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
            continue;
        }

        if trimmed.contains("process.env.") || trimmed.contains("process.env[") {
            let line_number = line_num.saturating_add(1);

            // Check if the previous line contains eslint-disable-next-line
            let prev_line = if line_num > 0 {
                lines.get(line_num.saturating_sub(1))
            } else {
                None
            };
            let is_suppressed = prev_line.is_some_and(|pl| pl.contains("eslint-disable-next-line"));

            let severity = if is_suppressed {
                Severity::Info
            } else {
                Severity::Error
            };

            let message = if is_suppressed {
                format!("ESLint-suppressed process.env access: {trimmed}")
            } else {
                format!("Use env() import instead: {trimmed}")
            };

            results.push(CheckResult {
                id: "T30".to_owned(),
                severity,
                title: "Direct process.env access".to_owned(),
                message,
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// T31: `as any` / `: any` type assertions (tree-sitter with grep fallback)
fn check_any_types(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let is_tsx = is_tsx_path(path);
    match ast_helpers::parse_ts_file(content, is_tsx) {
        Some(tree) => check_any_types_ast(path, content, &tree, results),
        None => check_any_types_grep(path, content, results),
    }
}

/// Tree-sitter path: find `: any` and `as any` via AST type annotation / `as_expression` nodes.
fn check_any_types_ast(
    path: &Path,
    content: &str,
    tree: &tree_sitter::Tree,
    results: &mut Vec<CheckResult>,
) {
    let lines: Vec<&str> = content.lines().collect();
    let hit_lines = ast_helpers::find_any_types(tree, content);

    for line_number in hit_lines {
        let line_idx = line_number.saturating_sub(1);
        let trimmed = lines.get(line_idx).unwrap_or(&"").trim();
        results.push(CheckResult {
            id: "T31".to_owned(),
            severity: Severity::Info,
            title: "any type usage".to_owned(),
            message: trimmed.to_owned(),
            file: Some(path.display().to_string()),
            line: Some(line_number),
        });
    }
}

/// Grep fallback for T31: scan lines for `as any` / `: any` (used when tree-sitter parse fails).
fn check_any_types_grep(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        // Skip comment lines
        if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
            continue;
        }

        if trimmed.contains("as any") || trimmed.contains(": any") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "T31".to_owned(),
                severity: Severity::Info,
                title: "any type usage".to_owned(),
                message: trimmed.to_owned(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// T32-T33: File line count
fn check_file_length(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let effective_lines = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with('*')
        })
        .count();

    if effective_lines > 300 {
        results.push(CheckResult {
            id: "T32".to_owned(),
            severity: Severity::Warn,
            title: "File too long".to_owned(),
            message: format!("{effective_lines} effective lines (max 300)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    } else if effective_lines > 250 {
        results.push(CheckResult {
            id: "T33".to_owned(),
            severity: Severity::Info,
            title: "File approaching limit".to_owned(),
            message: format!("{effective_lines} effective lines (warn at 300)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    }
}

/// Scan lines for comment patterns and emit an info-level `CheckResult` for each match.
fn check_comment_pattern(
    path: &Path,
    content: &str,
    patterns: &[&str],
    check_id: &str,
    title: &str,
    results: &mut Vec<CheckResult>,
) {
    for (line_num, line) in content.lines().enumerate() {
        if patterns.iter().any(|p| line.contains(p)) {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: check_id.to_owned(),
                severity: Severity::Info,
                title: title.to_owned(),
                message: line.trim().to_owned(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// T59: Banned packages in node_modules
fn check_banned_in_node_modules(path: &Path, results: &mut Vec<CheckResult>) {
    let nm_path = path.join("node_modules");
    if !nm_path.exists() {
        return;
    }

    let banned: &[&str] = &[
        "axios",
        "lodash",
        "moment",
        "uuid",
        "nanoid",
        "pg",
        "express",
        "classnames",
        "winston",
        "pino",
        "request",
        "got",
        "superagent",
        "node-fetch",
        "isomorphic-fetch",
        "underscore",
        "request-promise",
        "postgres",
        "cross-fetch",
    ];
    let banned_prefixes: &[&str] = &["embla-carousel"];

    for dep in banned {
        let dep_path = nm_path.join(dep);
        if dep_path.exists() {
            results.push(CheckResult {
                id: "T59".to_owned(),
                severity: Severity::Error,
                title: format!("Banned package in node_modules: {dep}"),
                message: format!("{dep} found in node_modules (transitive dependency?)"),
                file: Some(dep_path.display().to_string()),
                line: None,
            });
        }
    }

    // Check embla-carousel prefix
    for entry in crate::fs::list_dir(&nm_path) {
        let name = entry.file_name().to_string_lossy().into_owned();
        if banned_prefixes.iter().any(|p| name.starts_with(p)) {
            results.push(CheckResult {
                id: "T59".to_owned(),
                severity: Severity::Error,
                title: format!("Banned package in node_modules: {name}"),
                message: format!("{name} found in node_modules"),
                file: Some(entry.path().display().to_string()),
                line: None,
            });
        }
    }
}
