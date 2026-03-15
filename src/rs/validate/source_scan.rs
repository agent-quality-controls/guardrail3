use std::path::Path;

use walkdir::WalkDir;

use crate::discover::ProjectInfo;
use crate::report::types::{CheckResult, Severity};

pub fn check(
    workspace_root: &Path,
    scoped_files: Option<&[String]>,
    project: &ProjectInfo,
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    let rs_files: Vec<String> = match scoped_files {
        Some(files) => files
            .iter()
            .filter(|f| f.ends_with(".rs"))
            .cloned()
            .collect(),
        None => collect_rs_files(workspace_root),
    };

    for file_path in &rs_files {
        let path = Path::new(file_path);
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let is_bin_entry = is_bin_crate_entry(path);
        let is_test_file = is_test(path);

        check_crate_level_allow(path, &content, is_bin_entry, &mut results);
        check_item_level_allow(path, &content, &mut results);
        check_garde_skip(path, &content, &mut results);
        check_cfg_attr_allow(path, &content, &mut results);
        check_file_length(path, &content, &mut results);
        check_use_count(path, &content, &mut results);
        check_unsafe(path, &content, &mut results);
        check_todo_macros(path, &content, &mut results);

        if !is_test_file {
            check_unwrap_expect(path, &content, &mut results);
        }
    }

    // R51: Dependency direction — check each workspace member's Cargo.toml
    check_all_dependency_directions(workspace_root, project, &mut results);

    // R52: Dependency graph inventory
    check_dependency_graph(workspace_root, project, &mut results);

    // R36: EXCEPTION comments in config files
    check_exception_comments(workspace_root, &mut results);

    // R49: CLAUDE.md exists
    check_claude_md(workspace_root, &mut results);

    // R53: unsafe_code = "forbid" specifically
    check_unsafe_code_forbid(workspace_root, &mut results);

    results
}

fn collect_rs_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != "target" && name != "node_modules" && name != ".git"
        })
    {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    files.push(path.display().to_string());
                }
            }
        }
    }
    files
}

fn is_bin_crate_entry(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    name == "main.rs"
}

fn is_test(path: &Path) -> bool {
    let path_str = path.display().to_string();
    path_str.contains("/tests/")
        || path_str.contains("/test/")
        || path_str.contains("__tests__")
        || path_str.contains("_test.rs")
        || path_str.ends_with("_tests.rs")
}

/// Track whether we are inside a block comment (`/* ... */`).
/// Returns a filtered list of (line_num, trimmed_line) pairs that are NOT inside
/// block comments and NOT single-line comments.
fn filter_non_comment_lines(content: &str) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let mut in_block_comment = false;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim().to_string();

        if in_block_comment {
            if let Some(end_pos) = trimmed.find("*/") {
                // Extract the part after */
                let after = trimmed[end_pos.saturating_add(2)..].trim().to_string();
                // Check if remaining content opens a new block comment
                if let Some(new_open) = after.find("/*") {
                    in_block_comment = true;
                    let before_new = after[..new_open].trim().to_string();
                    if !before_new.is_empty() && !before_new.starts_with("//") {
                        result.push((line_num, before_new));
                    }
                } else {
                    in_block_comment = false;
                    if !after.is_empty() && !after.starts_with("//") {
                        result.push((line_num, after));
                    }
                }
            }
            continue;
        }

        // Strip inline /* ... */ pairs from the line
        let processed = strip_inline_block_comments(&trimmed);

        // Check if line opens a block comment that doesn't close
        if let Some(open_pos) = processed.find("/*") {
            let before = processed[..open_pos].trim().to_string();
            in_block_comment = true;
            if !before.is_empty() && !before.starts_with("//") {
                result.push((line_num, before));
            }
            continue;
        }

        let final_trimmed = processed.trim().to_string();

        if final_trimmed.is_empty()
            || final_trimmed.starts_with("//")
            || final_trimmed.starts_with("///")
        {
            continue;
        }

        result.push((line_num, final_trimmed));
    }
    result
}

/// Strip all `/* ... */` inline block comment pairs from a single line.
fn strip_inline_block_comments(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut remaining = line;

    loop {
        match remaining.find("/*") {
            Some(start) => {
                result.push_str(&remaining[..start]);
                match remaining[start..].find("*/") {
                    Some(end) => {
                        // end is relative to remaining[start..], so skip past */
                        remaining = &remaining[start.saturating_add(end).saturating_add(2)..];
                    }
                    None => {
                        // Unclosed block comment — return what we have so far,
                        // keep the /* so the caller can detect it
                        result.push_str(&remaining[start..]);
                        break;
                    }
                }
            }
            None => {
                result.push_str(remaining);
                break;
            }
        }
    }

    result
}

// R30-R31: #![allow(...)]
fn check_crate_level_allow(
    path: &Path,
    content: &str,
    _is_bin_entry: bool,
    results: &mut Vec<CheckResult>,
) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        if !trimmed.starts_with("#![allow(") {
            continue;
        }

        let line_number = line_num.saturating_add(1);

        // Extract the lint name — handle trailing )] and optional // comment
        let raw_lint = trimmed
            .strip_prefix("#![allow(")
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
            raw_lint.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
        } else {
            vec![raw_lint.trim()]
        };

        for lint in lints {
            if lint == "unused_crate_dependencies" {
                // Always Info — pre-commit hook exempts this lint universally
                // (it produces false positives in bin crates, integration tests,
                // lib crates with proc macros, etc.)
                results.push(CheckResult {
                    id: "R31".to_string(),
                    severity: Severity::Info,
                    title: "Justified #![allow]".to_string(),
                    message: "unused_crate_dependencies — universally exempted".to_string(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                results.push(CheckResult {
                    id: "R30".to_string(),
                    severity: Severity::Error,
                    title: "Crate-level #![allow]".to_string(),
                    message: format!("#![allow({lint})] — crate-wide lint suppression banned"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }
    }
}

// R32-R33: #[allow(...)] — item-level
fn check_item_level_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        // Match #[allow(...)] but NOT #![allow(...)]
        if !trimmed.starts_with("#[allow(") {
            continue;
        }

        let line_number = line_num.saturating_add(1);

        // Handle multi-line: if no closing ), gather the lint name from what we have
        let lint = if trimmed.contains(')') {
            trimmed
                .strip_prefix("#[allow(")
                .and_then(|s| s.split(')').next())
                .unwrap_or(trimmed)
                .to_string()
        } else {
            // Multi-line #[allow(... — take what's after the opening paren
            trimmed
                .strip_prefix("#[allow(")
                .unwrap_or(trimmed)
                .trim()
                .to_string()
                + "..."
        };

        // Check if same line has a // comment
        let has_comment = trimmed.contains("//");

        if has_comment {
            let reason = trimmed
                .split("//")
                .nth(1)
                .map(|s| s.trim())
                .unwrap_or("no reason given");
            results.push(CheckResult {
                id: "R33".to_string(),
                severity: Severity::Info,
                title: "Justified #[allow]".to_string(),
                message: format!("{lint} — {reason}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        } else {
            results.push(CheckResult {
                id: "R32".to_string(),
                severity: Severity::Error,
                title: "#[allow] without reason".to_string(),
                message: format!("#[allow({lint})] has no // comment justification"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// R34-R35: #[garde(skip)]
fn check_garde_skip(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
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
                .map(|s| s.trim())
                .unwrap_or("no reason given");
            results.push(CheckResult {
                id: "R35".to_string(),
                severity: Severity::Info,
                title: "Justified garde(skip)".to_string(),
                message: format!("garde(skip) — {reason}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        } else {
            results.push(CheckResult {
                id: "R34".to_string(),
                severity: Severity::Error,
                title: "garde(skip) without reason".to_string(),
                message: "garde(skip) has no // comment justification".to_string(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// R36: EXCEPTION comments
fn check_exception_comments(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let config_files = [
        "clippy.toml",
        "deny.toml",
        "Cargo.toml",
        "rustfmt.toml",
    ];

    for config_file in &config_files {
        let path = workspace_root.join(config_file);
        if !path.exists() {
            continue;
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (line_num, line) in content.lines().enumerate() {
            if line.contains("// EXCEPTION:") || line.contains("# EXCEPTION:") {
                let line_number = line_num.saturating_add(1);
                results.push(CheckResult {
                    id: "R36".to_string(),
                    severity: Severity::Info,
                    title: "EXCEPTION comment".to_string(),
                    message: line.trim().to_string(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }
    }
}

// R37: cfg_attr allow — must be an actual attribute (#[cfg_attr(..., allow(...))])
fn check_cfg_attr_allow(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
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
            id: "R37".to_string(),
            severity: Severity::Info,
            title: "cfg_attr allow".to_string(),
            message: trimmed.to_string(),
            file: Some(path.display().to_string()),
            line: Some(line_number),
        });
    }
}

// R38-R39: File line count
fn check_file_length(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let effective_lines = filter_non_comment_lines(content).len();

    if effective_lines > 500 {
        results.push(CheckResult {
            id: "R38".to_string(),
            severity: Severity::Error,
            title: "File too long".to_string(),
            message: format!("{effective_lines} effective lines (max 500)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    } else if effective_lines > 400 {
        results.push(CheckResult {
            id: "R39".to_string(),
            severity: Severity::Warn,
            title: "File approaching limit".to_string(),
            message: format!("{effective_lines} effective lines (warn at 400, max 500)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    }
}

// R40-R41: use statement count
fn check_use_count(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);
    let use_count = non_comment_lines
        .iter()
        .filter(|(_, trimmed)| {
            trimmed.starts_with("use ") || trimmed.starts_with("pub use ")
        })
        .count();

    if use_count > 20 {
        results.push(CheckResult {
            id: "R40".to_string(),
            severity: Severity::Error,
            title: "Too many use statements".to_string(),
            message: format!("{use_count} use statements (max 20)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    } else if use_count > 15 {
        results.push(CheckResult {
            id: "R41".to_string(),
            severity: Severity::Warn,
            title: "Many use statements".to_string(),
            message: format!("{use_count} use statements (warn at 15, max 20)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    }
}

// R42: unsafe
fn check_unsafe(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        // Check for unsafe as a keyword — must be preceded by whitespace/start-of-line
        // and followed by '{', ' fn', ' impl', ' trait', or whitespace
        let check_patterns = [
            "unsafe {",
            "unsafe{",
            "unsafe fn ",
            "unsafe impl ",
            "unsafe trait ",
        ];

        let mut found = false;
        for pattern in &check_patterns {
            if trimmed.contains(pattern) {
                // Make sure it's not inside a string literal
                // Simple heuristic: if a `"` appears before the unsafe keyword, skip
                if let Some(unsafe_pos) = trimmed.find(pattern) {
                    let before = &trimmed[..unsafe_pos];
                    // Count quotes before — if odd, we're inside a string
                    let quote_count = before.chars().filter(|c| *c == '"').count();
                    if quote_count % 2 != 0 {
                        continue;
                    }
                    found = true;
                    break;
                }
            }
        }

        // Also check if line starts with "unsafe " — at start of line, can't be in a string
        if !found && trimmed.starts_with("unsafe ") {
            found = true;
        }

        if found {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "R42".to_string(),
                severity: Severity::Error,
                title: "unsafe usage".to_string(),
                message: trimmed.to_string(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// R43: todo!/unimplemented! (Warn) and unreachable! (Info)
fn check_todo_macros(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        for macro_name in &["todo!(", "unimplemented!("] {
            if trimmed.contains(macro_name) {
                let line_number = line_num.saturating_add(1);
                results.push(CheckResult {
                    id: "R43".to_string(),
                    severity: Severity::Warn,
                    title: format!("{} macro", macro_name.trim_end_matches('(')),
                    message: trimmed.to_string(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // unreachable! is Info — legitimately used in exhaustive matches
        if trimmed.contains("unreachable!(") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "R43".to_string(),
                severity: Severity::Info,
                title: "unreachable! macro".to_string(),
                message: trimmed.to_string(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// R44: .unwrap() / .expect()
fn check_unwrap_expect(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let non_comment_lines = filter_non_comment_lines(content);

    for (line_num, trimmed) in &non_comment_lines {
        if trimmed.contains(".unwrap()") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "R44".to_string(),
                severity: Severity::Warn,
                title: ".unwrap() usage".to_string(),
                message: trimmed.to_string(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }

        if trimmed.contains(".expect(") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "R44".to_string(),
                severity: Severity::Warn,
                title: ".expect() usage".to_string(),
                message: trimmed.to_string(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// R49: CLAUDE.md
fn check_claude_md(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let claude_path = workspace_root.join("CLAUDE.md");
    if claude_path.exists() {
        results.push(CheckResult {
            id: "R49".to_string(),
            severity: Severity::Info,
            title: "CLAUDE.md exists".to_string(),
            message: "Found at project root".to_string(),
            file: Some(claude_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R49".to_string(),
            severity: Severity::Warn,
            title: "CLAUDE.md missing".to_string(),
            message: "No CLAUDE.md found at project root".to_string(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
    }
}

// R51: Dependency direction — iterate workspace member Cargo.tomls
fn check_all_dependency_directions(
    workspace_root: &Path,
    project: &ProjectInfo,
    results: &mut Vec<CheckResult>,
) {
    for member_dir in &project.workspace_member_dirs {
        let cargo_path = workspace_root.join(member_dir).join("Cargo.toml");
        if !cargo_path.exists() {
            continue;
        }
        check_dependency_direction(&cargo_path, member_dir, results);
    }
}

fn check_dependency_direction(
    cargo_path: &Path,
    member_dir: &str,
    results: &mut Vec<CheckResult>,
) {
    let content = match std::fs::read_to_string(cargo_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    // Determine crate kind from path
    let is_domain = member_dir.contains("/domain/")
        || member_dir.contains("/domain-")
        || member_dir.ends_with("/domain")
        || member_dir == "domain";
    let is_types = member_dir.contains("/types/")
        || member_dir.contains("/types-")
        || member_dir.ends_with("/types")
        || member_dir == "types";
    let is_commands = member_dir.contains("/commands/")
        || member_dir.contains("/commands-")
        || member_dir.ends_with("/commands")
        || member_dir == "commands";
    let is_repo = member_dir.contains("/repo/")
        || member_dir.contains("/repo-")
        || member_dir.ends_with("/repo")
        || member_dir == "repo"
        || member_dir.contains("/ports/")
        || member_dir.contains("/ports-")
        || member_dir.ends_with("/ports")
        || member_dir == "ports";

    if !is_domain && !is_types && !is_commands && !is_repo {
        return;
    }

    // Banned dependency names per crate kind (exact name match)
    let banned_for_domain_types: &[&str] =
        &["db", "api", "commands", "adapters", "sqlx", "axum", "reqwest"];
    let banned_for_commands: &[&str] =
        &["db", "api", "adapters", "sqlx", "axum", "reqwest"];
    let banned_for_repo: &[&str] =
        &["db", "api", "commands", "adapters", "sqlx", "axum"];

    let banned = if is_domain || is_types {
        banned_for_domain_types
    } else if is_commands {
        banned_for_commands
    } else {
        banned_for_repo
    };

    let kind = if is_domain {
        "domain"
    } else if is_types {
        "types"
    } else if is_commands {
        "commands"
    } else {
        "repo/ports"
    };

    // Suffixes that indicate architectural layer crates
    let banned_suffixes: &[&str] = &["-db", "-api", "-adapters", "-commands", "-repo", "-ports"];

    if let Some(deps) = table.get("dependencies") {
        if let Some(dep_table) = deps.as_table() {
            for dep_name in dep_table.keys() {
                // Exact crate name matching
                let exact_match = banned.contains(&dep_name.as_str());
                // Suffix matching for prefixed crate names (e.g. "myapp-db", "myapp-api")
                let suffix_match = banned_suffixes
                    .iter()
                    .any(|suffix| dep_name.ends_with(suffix));

                if exact_match || suffix_match {
                    results.push(CheckResult {
                        id: "R51".to_string(),
                        severity: Severity::Error,
                        title: "Dependency direction violation".to_string(),
                        message: format!(
                            "{kind} crate ({member_dir}) depends on \"{dep_name}\""
                        ),
                        file: Some(cargo_path.display().to_string()),
                        line: None,
                    });
                }
            }
        }
    }
}

// R52: Dependency graph inventory
fn check_dependency_graph(
    workspace_root: &Path,
    project: &ProjectInfo,
    results: &mut Vec<CheckResult>,
) {
    for (idx, member_dir) in project.workspace_member_dirs.iter().enumerate() {
        let cargo_path = workspace_root.join(member_dir).join("Cargo.toml");
        if !cargo_path.exists() {
            continue;
        }

        let content = match std::fs::read_to_string(&cargo_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let table: toml::Value = match content.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let crate_name = project
            .workspace_members
            .get(idx)
            .map(|s| s.as_str())
            .unwrap_or(member_dir.as_str());

        if let Some(deps) = table.get("dependencies") {
            if let Some(dep_table) = deps.as_table() {
                // Collect internal deps (path dependencies)
                let mut internal_deps = Vec::new();
                for (dep_name, dep_val) in dep_table {
                    let is_path = match dep_val {
                        toml::Value::Table(t) => t.get("path").is_some(),
                        _ => false,
                    };
                    if is_path {
                        internal_deps.push(dep_name.clone());
                    }
                }

                if !internal_deps.is_empty() {
                    internal_deps.sort();
                    results.push(CheckResult {
                        id: "R52".to_string(),
                        severity: Severity::Info,
                        title: format!("{crate_name} internal deps"),
                        message: format!("depends on: {}", internal_deps.join(", ")),
                        file: Some(cargo_path.display().to_string()),
                        line: None,
                    });
                }
            }
        }
    }
}

// R53: unsafe_code = "forbid"
fn check_unsafe_code_forbid(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let cargo_path = workspace_root.join("Cargo.toml");
    if !cargo_path.exists() {
        return;
    }

    let content = match std::fs::read_to_string(&cargo_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let level = table
        .get("workspace")
        .and_then(|w| w.get("lints"))
        .and_then(|l| l.get("rust"))
        .and_then(|r| r.get("unsafe_code"));

    match level {
        Some(toml::Value::String(s)) if s == "forbid" => {
            results.push(CheckResult {
                id: "R53".to_string(),
                severity: Severity::Info,
                title: "unsafe_code = forbid".to_string(),
                message: "unsafe_code is forbidden (cannot be overridden per-crate)"
                    .to_string(),
                file: Some(cargo_path.display().to_string()),
                line: None,
            });
        }
        Some(toml::Value::String(s)) if s == "deny" => {
            results.push(CheckResult {
                id: "R53".to_string(),
                severity: Severity::Error,
                title: "unsafe_code should be forbid".to_string(),
                message:
                    "unsafe_code = \"deny\" can be overridden per-crate; use \"forbid\""
                        .to_string(),
                file: Some(cargo_path.display().to_string()),
                line: None,
            });
        }
        _ => {
            // Already covered by R26 lint checks
        }
    }
}
