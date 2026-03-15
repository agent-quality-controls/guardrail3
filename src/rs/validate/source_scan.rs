use std::path::Path;

use walkdir::WalkDir;

use crate::discover::ProjectInfo;
use crate::report::types::CheckResult;

use super::allow_checks;
use super::code_quality_checks;
use super::dependency_direction;
use super::structure_checks;

#[allow(clippy::case_sensitive_file_extension_comparisons)] // reason: only checking .rs files
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
        let Some(content) = crate::fs::read_file(path) else {
            continue;
        };

        let is_bin_entry = is_bin_crate_entry(path);
        let is_test_file = is_test(path);

        // Allow/attribute checks (R30-R37)
        allow_checks::check_crate_level_allow(
            path,
            &content,
            is_bin_entry,
            is_test_file,
            &mut results,
        );
        allow_checks::check_item_level_allow(path, &content, &mut results);
        allow_checks::check_garde_skip(path, &content, &mut results);
        allow_checks::check_cfg_attr_allow(path, &content, &mut results);

        // Structure checks (R38-R42)
        structure_checks::check_file_length(path, &content, is_test_file, &mut results);
        structure_checks::check_use_count(path, &content, is_test_file, &mut results);
        structure_checks::check_unsafe(path, &content, &mut results);

        // Code quality checks (R43-R44, R58)
        code_quality_checks::check_todo_macros(path, &content, is_test_file, &mut results);
        code_quality_checks::check_direct_fs_usage(path, &content, is_test_file, &mut results);

        if !is_test_file {
            code_quality_checks::check_unwrap_expect(path, &content, &mut results);
        }
    }

    // R51: Dependency direction — check each workspace member's Cargo.toml
    dependency_direction::check_all_dependency_directions(workspace_root, project, &mut results);

    // R52: Dependency graph inventory
    dependency_direction::check_dependency_graph(workspace_root, project, &mut results);

    // R36: EXCEPTION comments in config files
    allow_checks::check_exception_comments(workspace_root, &mut results);

    // R49: CLAUDE.md exists
    code_quality_checks::check_claude_md(workspace_root, &mut results);

    // R53: unsafe_code = "forbid" specifically
    structure_checks::check_unsafe_code_forbid(workspace_root, &mut results);

    results
}

fn collect_rs_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != "target" && name != "node_modules" && name != ".git" && name != ".claude"
        })
        .flatten()
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                files.push(path.display().to_string());
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
/// Returns a filtered list of (`line_num`, `trimmed_line`) pairs that are NOT inside
/// block comments and NOT single-line comments.
#[allow(clippy::type_complexity)] // reason: legitimate complex type
pub fn filter_non_comment_lines(content: &str) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let mut in_block_comment = false;
    #[allow(clippy::string_slice)] // reason: block comment parsing needs ASCII slicing
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim().to_owned();

        // Strip string literals for comment boundary detection only
        let for_comment_check = strip_string_literals(&trimmed);

        if in_block_comment {
            if let Some(end_pos) = for_comment_check.find("*/") {
                // Find corresponding position in original trimmed line
                let after = trimmed[end_pos.saturating_add(2)..].trim().to_owned();
                let after_for_check = strip_string_literals(&after);
                // Check if remaining content opens a new block comment
                if after_for_check.contains("/*") {
                    in_block_comment = true;
                    if let Some(new_open) = after_for_check.find("/*") {
                        let before_new = after[..new_open].trim().to_owned();
                        if !before_new.is_empty() && !before_new.starts_with("//") {
                            result.push((line_num, before_new));
                        }
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
        let processed_for_check = strip_string_literals(&processed);

        // Check if line opens a block comment that doesn't close
        if let Some(open_pos) = processed_for_check.find("/*") {
            let before = processed[..open_pos].trim().to_owned();
            in_block_comment = true;
            if !before.is_empty() && !before.starts_with("//") {
                result.push((line_num, before));
            }
            continue;
        }

        let final_trimmed = processed.trim().to_owned();

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
#[allow(clippy::string_slice)] // reason: inline comment stripping on known ASCII delimiters /* and */
fn strip_inline_block_comments(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut remaining = line;

    // Use string-stripped version for finding comment delimiters
    loop {
        let remaining_for_check = strip_string_literals(remaining);
        match remaining_for_check.find("/*") {
            Some(start) => {
                result.push_str(&remaining[..start]);
                let check_rest = strip_string_literals(&remaining[start..]);
                match check_rest.find("*/") {
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

/// Strip string literals from a line for comment-detection purposes.
/// This prevents `"/*"` inside strings from being treated as comment delimiters.
fn strip_string_literals(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut in_string = false;
    let mut prev_was_escape = false;

    for c in line.chars() {
        if prev_was_escape {
            prev_was_escape = false;
            if !in_string {
                result.push(c);
            }
            continue;
        }
        if c == '\\' && in_string {
            prev_was_escape = true;
            continue;
        }
        if c == '"' {
            in_string = !in_string;
            continue;
        }
        if !in_string {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn filter_preserves_lines_with_string_containing_block_comment() {
        let input = r#"let x = remaining.find("/*");"#;
        let result = filter_non_comment_lines(input);
        assert_eq!(
            result.len(),
            1,
            "Line with /* in string should not be dropped"
        );
        // Assert FULL line preserved — a partial match like contains("remaining.find")
        // would pass even if the line was truncated at the string-embedded "/*"
        assert_eq!(
            result[0].1, r#"let x = remaining.find("/*");"#,
            "Full original line must be preserved exactly including the string literal"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn filter_removes_actual_block_comments() {
        let input = "code\n/* comment */\nmore code";
        let result = filter_non_comment_lines(input);
        assert_eq!(result.len(), 2);
        assert!(result[0].1.contains("code"));
        assert!(result[1].1.contains("more code"));
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn filter_handles_multiline_block_comment() {
        let input = "before\n/* start\nmiddle\nend */\nafter";
        let result = filter_non_comment_lines(input);
        assert_eq!(result.len(), 2);
        assert!(result[0].1.contains("before"));
        assert!(result[1].1.contains("after"));
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn filter_handles_block_comment_end_with_code() {
        let input = "/* comment */ let x = 1;";
        let result = filter_non_comment_lines(input);
        assert_eq!(result.len(), 1);
        assert!(result[0].1.contains("let x = 1"));
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    fn filter_skips_line_comments() {
        let input = "code\n// comment\nmore";
        let result = filter_non_comment_lines(input);
        assert_eq!(result.len(), 2);
        assert!(result[0].1.contains("code"));
        assert!(result[1].1.contains("more"));
    }

    #[test]
    fn filter_empty_input() {
        let result = filter_non_comment_lines("");
        assert!(result.is_empty());
    }

    #[test]
    fn filter_only_comments() {
        let input = "// line comment\n/* block */\n/// doc comment";
        let result = filter_non_comment_lines(input);
        assert!(result.is_empty(), "Only comments should produce no output");
    }

    #[test]
    fn filter_only_code() {
        let input = "let x = 1;\nlet y = 2;";
        let result = filter_non_comment_lines(input);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn filter_nested_block_comment_delimiters_in_string() {
        let input = r#"let re = Regex::new("/* unterminated */");"#;
        let result = filter_non_comment_lines(input);
        assert_eq!(
            result.len(),
            1,
            "/* */ inside string should not affect comment parsing"
        );
    }

    #[test]
    fn strip_string_literals_removes_content() {
        let result = strip_string_literals(r#"let x = "hello /* world */";"#);
        assert!(!result.contains("hello"));
        assert!(
            !result.contains("/*"),
            "Comment delimiters inside string should be stripped"
        );
    }

    #[test]
    fn strip_string_literals_handles_escaped_quotes() {
        let result = strip_string_literals(r#"let x = "she said \"hi\"";"#);
        assert!(!result.contains("she"));
    }

    #[test]
    fn strip_string_literals_preserves_non_string_content() {
        let result = strip_string_literals("let x = 42; /* comment */");
        assert!(result.contains("let x = 42"));
        assert!(result.contains("/* comment */"));
    }

    #[test]
    fn strip_string_literals_empty_string() {
        let result = strip_string_literals(r#"let x = "";"#);
        assert!(result.contains("let x = "));
    }
}
