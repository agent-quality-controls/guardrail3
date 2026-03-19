use std::path::Path;

use walkdir::WalkDir;

use crate::domain::report::CheckResult;

use super::allow_checks;
use super::code_quality_checks;
use super::structure_checks;
use crate::ports::outbound::FileSystem;

/// A line number (0-based) paired with its trimmed content.
pub type NumberedLine = (usize, String);

pub fn check(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    scoped_files: Option<&[String]>,
    garde_enabled: bool,
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    let rs_files: Vec<String> = match scoped_files {
        Some(files) => files
            .iter()
            .filter(|f| Path::new(f).extension().is_some_and(|e| e == "rs"))
            .cloned()
            .collect(),
        None => collect_rs_files(fs, workspace_root),
    };

    for file_path in &rs_files {
        let path = Path::new(file_path);
        let Some(content) = fs.read_file(path) else {
            continue;
        };

        let is_bin_entry = is_bin_crate_entry(path);
        let is_test_file = is_test(path);

        // Allow/attribute checks (R30-R37) — R30-R35, R37 excluded for test files
        if !is_test_file {
            allow_checks::check_crate_level_allow(
                path,
                &content,
                is_bin_entry,
                is_test_file,
                &mut results,
            );
            allow_checks::check_item_level_allow(path, &content, &mut results);
            if garde_enabled {
                allow_checks::check_garde_skip(path, &content, &mut results);
            }
            allow_checks::check_cfg_attr_allow(path, &content, &mut results);
        }

        // Structure checks (R38, R40-R41)
        structure_checks::check_file_length(path, &content, is_test_file, &mut results);
        structure_checks::check_use_count(path, &content, is_test_file, &mut results);

        // R58: std::fs — catches clippy's aliased-import hole
        code_quality_checks::check_direct_fs_usage(path, &content, is_test_file, &mut results);
    }

    // R36: EXCEPTION comments in config files
    allow_checks::check_exception_comments(fs, workspace_root, &mut results);

    // R49: CLAUDE.md exists
    code_quality_checks::check_claude_md(workspace_root, &mut results);

    // R53: unsafe_code = "forbid" in workspace lints
    structure_checks::check_unsafe_code_forbid(fs, workspace_root, &mut results);

    results
}

/// Check if a walkdir entry is a directory that should be excluded from Rust source scanning.
pub fn is_excluded_dir(entry: &walkdir::DirEntry) -> bool {
    is_excluded_dir_with_gitignore(entry, &std::collections::BTreeSet::new())
}

pub fn is_excluded_dir_with_gitignore(
    entry: &walkdir::DirEntry,
    gitignored: &std::collections::BTreeSet<String>,
) -> bool {
    let name = entry.file_name().to_string_lossy();
    name == "target"
        || name == "node_modules"
        || name == ".git"
        || name == ".claude"
        || gitignored.contains(name.as_ref())
}

pub fn collect_rs_files(fs: &dyn FileSystem, root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    let gitignored = crate::app::gitignore::load_gitignore_dirs(fs, root);
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir_with_gitignore(e, &gitignored))
        .flatten()
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let path_str = path.display().to_string();
                // Skip test fixture files — adversarial test data designed to have violations
                if path_str.contains("tests/fixtures/") {
                    continue;
                }
                files.push(path_str);
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
    is_test_path(&path_str)
}

/// Check if a file path belongs to a test file.
pub fn is_test_path(path_str: &str) -> bool {
    path_str.contains("/tests/")
        || path_str.contains("/test/")
        || path_str.contains("__tests__")
        || path_str.contains("_test.rs")
        || path_str.ends_with("_tests.rs")
        || path_str.ends_with("/tests.rs")
}

/// Track whether we are inside a block comment (`/* ... */`).
/// Returns a filtered list of (`line_num`, `trimmed_line`) pairs that are NOT inside
/// block comments and NOT single-line comments.
pub fn filter_non_comment_lines(content: &str) -> Vec<NumberedLine> {
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
pub fn strip_inline_block_comments(line: &str) -> String {
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
///
/// This prevents `"/*"` inside strings from being treated as comment delimiters.
/// Handles both regular strings (`"..."`) and raw strings (`r"..."`, `r#"..."#`, etc.).
#[allow(clippy::indexing_slicing)] // reason: all char indices are bounds-checked against len before access
#[allow(clippy::string_slice)] // reason: pos from str::find is guaranteed to be a valid UTF-8 boundary
pub fn strip_string_literals(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Detect raw string opener: r followed by optional #s then "
        if chars[i] == 'r' {
            let mut hashes: usize = 0;
            let mut j = i.saturating_add(1);
            while j < len && chars[j] == '#' {
                hashes = hashes.saturating_add(1);
                j = j.saturating_add(1);
            }
            if j < len && chars[j] == '"' && (hashes > 0 || j == i.saturating_add(1)) {
                // Verify `r` is not part of an identifier
                let is_ident_char = i > 0
                    && (chars[i.saturating_sub(1)].is_ascii_alphanumeric()
                        || chars[i.saturating_sub(1)] == '_');
                if !is_ident_char {
                    // Build closing delimiter: `"` followed by `hashes` `#` chars
                    let mut closer = String::with_capacity(hashes.saturating_add(1));
                    closer.push('"');
                    for _ in 0..hashes {
                        closer.push('#');
                    }
                    // Skip past opening delimiter
                    i = j.saturating_add(1);
                    // Find closing delimiter in remaining chars
                    let remaining: String = chars[i..].iter().collect();
                    if let Some(pos) = remaining.find(closer.as_str()) {
                        // pos is byte offset; convert to char count
                        let char_offset = remaining[..pos].chars().count();
                        i = i.saturating_add(char_offset).saturating_add(closer.len());
                    } else {
                        // Unclosed raw string — skip rest of line
                        break;
                    }
                    continue;
                }
            }
        }

        // Regular string
        if chars[i] == '"' {
            i = i.saturating_add(1);
            // Skip until closing quote
            while i < len {
                if chars[i] == '\\' {
                    i = i.saturating_add(2); // skip escape sequence
                    continue;
                }
                if chars[i] == '"' {
                    i = i.saturating_add(1);
                    break;
                }
                i = i.saturating_add(1);
            }
            continue;
        }

        // Not in a string — keep the character
        result.push(chars[i]);
        i = i.saturating_add(1);
    }
    result
}
