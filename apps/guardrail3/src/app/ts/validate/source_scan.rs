use std::path::Path;

use walkdir::WalkDir;

use super::ast_helpers;
use super::ts_comment_checks;
use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

pub fn check(
    fs: &dyn FileSystem,
    path: &Path,
    scoped_files: Option<&[String]>,
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    let ts_files: Vec<String> = match scoped_files {
        Some(files) => files.iter().filter(|f| is_ts_file(f)).cloned().collect(),
        None => collect_ts_files(path),
    };

    for file_path in &ts_files {
        let fp = Path::new(file_path);
        let Some(content) = fs.read_file(fp) else {
            continue;
        };

        ts_comment_checks::check_eslint_disable(fp, &content, &mut results);
        ts_comment_checks::check_ts_ignore(fp, &content, &mut results);
        check_process_env(fp, &content, &mut results);
        check_any_types(fp, &content, &mut results);
        check_file_length(fp, &content, &mut results);
        // T34: // noinspection
        check_comment_pattern(
            fp,
            &content,
            &["// noinspection", "/* noinspection"],
            "T34",
            "IDE noinspection suppression",
            &mut results,
        );
        // T35: istanbul ignore / c8 ignore
        check_comment_pattern(
            fp,
            &content,
            &["istanbul ignore", "c8 ignore"],
            "T35",
            "Coverage ignore directive",
            &mut results,
        );
    }

    // T59: Banned packages in node_modules
    check_banned_in_node_modules(fs, path, &mut results);

    results
}

fn is_ts_file(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|e| e == "ts" || e == "tsx" || e == "mjs")
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
            // Skip test fixture files — adversarial test data designed to have violations
            if path_str.contains("tests/fixtures/") {
                continue;
            }
            if is_ts_file(&path_str) {
                files.push(path_str);
            }
        }
    }
    files
}

// T30: process.env direct access (AST-only)
pub fn check_process_env(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    // Skip env.ts, env.mjs files, and all .mjs files (config files that legitimately use process.env)
    if file_name == "env.ts" || file_name == "env.mjs" {
        return;
    }
    if path.extension().and_then(|e| e.to_str()) == Some("mjs") {
        return;
    }

    let is_tsx = ts_comment_checks::is_tsx_path(path);
    let Some(tree) = ast_helpers::parse_ts_file(content, is_tsx) else {
        return;
    };
    check_process_env_ast(path, content, &tree, results);
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
            format!(
                "ESLint-suppressed `process.env` access (acknowledged via eslint-disable): `{trimmed}`. \
                 Direct env access scatters configuration reads across the codebase, making it hard to audit \
                 what config a service needs. This instance is suppressed but still tracked."
            )
        } else {
            format!(
                "Direct `process.env` access found: `{trimmed}`. Reading environment variables directly \
                 scatters configuration across the codebase, making it impossible to see all config in one place \
                 and easy to misspell variable names. Create a centralized `env.ts` module that reads all env vars \
                 once with validation, then import from it."
            )
        };

        results.push(CheckResult {
            id: "T30".to_owned(),
            severity,
            title: "Direct process.env access".to_owned(),
            message,
            file: Some(path.display().to_string()),
            line: Some(line_number),
            inventory: false,
        });
    }
}

// T31: `as any` / `: any` type assertions (AST-only)
pub fn check_any_types(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let is_tsx = ts_comment_checks::is_tsx_path(path);
    let Some(tree) = ast_helpers::parse_ts_file(content, is_tsx) else {
        return;
    };
    check_any_types_ast(path, content, &tree, results);
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
            title: "`any` type usage".to_owned(),
            message: format!(
                "`any` type found: `{trimmed}`. The `any` type disables TypeScript's type checker for this value, \
                 allowing type errors to propagate silently at runtime. Replace with a specific type, `unknown` \
                 (forces runtime checks), or a generic type parameter."
            ),
            file: Some(path.display().to_string()),
            line: Some(line_number),
            inventory: false,
        });
    }
}

// T32: File line count
pub fn check_file_length(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
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
            title: "File exceeds 300 effective lines".to_owned(),
            message: format!(
                "{effective_lines} effective lines (blank/comment lines excluded). \
                 Large files are harder for agents and humans to reason about, increasing bug risk. \
                 Split into focused modules — extract related functions into separate files with clear boundaries."
            ),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// Scan lines for comment patterns and emit an info-level `CheckResult` for each match.
pub fn check_comment_pattern(
    path: &Path,
    content: &str,
    patterns: &[&str],
    check_id: &str,
    title: &str,
    results: &mut Vec<CheckResult>,
) {
    let explanation = match check_id {
        "T34" => " `noinspection` is a JetBrains IDE directive that suppresses inspections. \
                   These are IDE-specific and should not be in source control. \
                   Remove the comment and fix the underlying issue instead.",
        "T35" => " Coverage ignore directives (`istanbul ignore`/`c8 ignore`) hide untested code \
                   from coverage reports, masking gaps. Remove the directive and write tests for the code, \
                   or if truly untestable, document why in a code comment.",
        _ => "",
    };
    for (line_num, line) in content.lines().enumerate() {
        if patterns.iter().any(|p| line.contains(p)) {
            let line_number = line_num.saturating_add(1);
            let trimmed = line.trim();
            results.push(CheckResult {
                id: check_id.to_owned(),
                severity: Severity::Info,
                title: title.to_owned(),
                message: format!("{trimmed}.{explanation}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
                inventory: false,
            }.as_inventory());
        }
    }
}

// T59: Banned packages in node_modules
fn check_banned_in_node_modules(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
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
                title: format!("Banned package `{dep}` in node_modules"),
                message: format!(
                    "`{dep}` found in node_modules. Banned packages have preferred alternatives \
                     (e.g., native fetch over axios, date-fns over moment, crypto.randomUUID over uuid). \
                     If this is a transitive dependency, add a pnpm override to replace it. \
                     If direct, remove from package.json and switch to the approved alternative."
                ),
                file: Some(dep_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // Check embla-carousel prefix
    for entry in fs.list_dir(&nm_path) {
        let name = entry.file_name().to_string_lossy().into_owned();
        if banned_prefixes.iter().any(|p| name.starts_with(p)) {
            results.push(CheckResult {
                id: "T59".to_owned(),
                severity: Severity::Error,
                title: format!("Banned package `{name}` in node_modules"),
                message: format!(
                    "`{name}` found in node_modules. Banned packages have preferred alternatives. \
                     If this is a transitive dependency, add a pnpm override to replace it. \
                     If direct, remove from package.json and switch to the approved alternative."
                ),
                file: Some(entry.path().display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

