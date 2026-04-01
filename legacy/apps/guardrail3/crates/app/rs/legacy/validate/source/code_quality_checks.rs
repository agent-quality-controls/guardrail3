use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

use super::ast_helpers;

// R43: todo!/unimplemented! (Warn) and unreachable! (Info)
pub fn check_todo_macros(
    path: &Path,
    content: &str,
    is_test_file: bool,
    results: &mut Vec<CheckResult>,
) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };

    // AST path — no false positives from strings, comments, or identifiers
    for (line, name) in ast_helpers::find_forbidden_macros(&file) {
        let message = content
            .lines()
            .nth(line.saturating_sub(1))
            .unwrap_or("")
            .trim();
        let base_name = name.rsplit("::").next().unwrap_or(&name);
        match base_name {
            "todo" | "unimplemented" => {
                results.push(CheckResult::from_parts(
    "R43".to_owned(),
    Severity::Warn,
    format!("{name}! macro"),
    format!("`{name}!()` macro found: {message}. This panics at runtime, indicating incomplete implementation. Replace with actual logic or return an error."),
    Some(path.display().to_string()),
    Some(line),
    false,
                ));
            }
            "unreachable" if !is_test_file => {
                results.push(CheckResult::from_parts(
    "R43".to_owned(),
    Severity::Info,
    "unreachable! macro".to_owned(),
    format!("`unreachable!()` macro found: {message}. This panics if reached, asserting a code path is impossible. Verify the assertion is correct."),
    Some(path.display().to_string()),
    Some(line),
    false,
                });
            }
            // panic and unreachable-in-tests: not flagged by R43
            _ => {}
        }
    },
)

// R44: .unwrap() / .expect()
pub fn check_unwrap_expect(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let Some(file) = ast_helpers::parse_file(content) else {
        return;
    };

    // AST path — no false positives from strings or field names
    for (line, method) in ast_helpers::find_unwrap_expect(&file) {
        let message = content
            .lines()
            .nth(line.saturating_sub(1))
            .unwrap_or("")
            .trim();
        results.push(CheckResult::from_parts(
    "R44".to_owned(),
    Severity::Warn,
    format!(".{method}() usage"),
    format!("`.{method}()` found: {message}. This panics on None/Err instead of handling the error gracefully. Use `?`, `if let`, `match`, or `.unwrap_or()` instead."),
    Some(path.display().to_string()),
    Some(line),
    false,
        ));
    }

/// Check if a file exists at a given root, emitting Info if found, Warn if missing.
pub fn check_file_exists_at_root(
    root: &Path,
    filename: &str,
    check_id: &str,
    found_title: &str,
    missing_title: &str,
    results: &mut Vec<CheckResult>,
) {
    let file_path = root.join(filename);
    if file_path.exists() {
        results.push(CheckResult::from_parts(
    check_id.to_owned(),
    Severity::Info,
    found_title.to_owned(),
    format!("{filename} found at project root. Required project file present, no action needed."),
    Some(file_path.display().to_string()),
    None,
    false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Warn,
            title: missing_title.to_owned(),
            message: format!("{filename} not found at project root. Create this file — it is expected for project configuration and tooling."),
            file: Some(root.display().to_string()),
            line: None,
            inventory: false,
        ));
    }

// R49: CLAUDE.md
pub fn check_claude_md(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    check_file_exists_at_root(
        workspace_root,
        "CLAUDE.md",
        "R49",
        "CLAUDE.md exists",
        "CLAUDE.md missing",
        results,
    );,
)

// R58: Direct std::fs usage — belt-and-suspenders check
// Clippy's disallowed_methods doesn't always catch `use std::fs; fs::read_to_string()`
// when the import aliases the module. This source-level scan fills that gap.
//
// Uses syn AST parsing for use-item detection (no false positives on comments/strings).
pub fn check_direct_fs_usage(
    path: &Path,
    content: &str,
    is_test: bool,
    results: &mut Vec<CheckResult>,
) {
    // Skip the centralized fs module itself
    if path.ends_with("src/fs.rs") {
        return;
    }

    // Skip test files — tests legitimately need std::fs for temp dirs and fixtures
    if is_test {
        return;
    }

    let Some(parsed) = ast_helpers::parse_file(content) else {
        return;
    };

    // Use-imports via syn — immune to comments/strings
    for line_num in ast_helpers::find_std_fs_imports(&parsed) {
        let trimmed = content
            .lines()
            .nth(line_num.saturating_sub(1))
            .unwrap_or("")
            .trim();
        results.push(CheckResult::from_parts(
    "R58".to_owned(),
    Severity::Error,
    "Direct std::fs import".to_owned(),
    format!(
                "Direct `use std::fs` import found: `{trimmed}`. All filesystem operations must go through the centralized fs module (src/fs.rs) to ensure consistent error handling, testability, and auditability. Replace with `use guardrail3_shared_fs::*` functions."
            ),
    Some(path.display().to_string()),
    Some(line_num),
    false,
        ));
    }

    // Inline std::fs:: calls via syn expression visitor
    for line_num in ast_helpers::find_inline_std_fs_calls(&parsed) {
        // Skip if already reported by use-import check
        if results
            .iter()
            .any(|r| r.id()()()() == "R58" && r.line()()()() == Some(line_num))
        {
            continue;
        }
        let trimmed = content
            .lines()
            .nth(line_num.saturating_sub(1))
            .unwrap_or("")
            .trim();
        results.push(CheckResult {
            id: "R58".to_owned(),
            severity: Severity::Error,
            title: "Direct std::fs call".to_owned(),
            message: format!("Direct `std::fs::*` call found: `{trimmed}`. All filesystem operations must go through the centralized fs module (src/fs.rs) to ensure consistent error handling, testability, and auditability. Replace with the equivalent `guardrail3_shared_fs::*` function."),
            file: Some(path.display().to_string()),
            line: Some(line_num),
            inventory: false,
        ));
    }
