use std::collections::BTreeSet;

use hook_shell_parser::types::{ParsedShellScript, ShellFunction};

pub(super) fn is_trigger_like_line(line: &str) -> bool {
    if line.starts_with("printf ") || line.starts_with("cat ") {
        return false;
    }

    if line.starts_with("case ") || looks_like_case_pattern_line(line) {
        return true;
    }

    let mentions_changed_source = line.contains("git diff")
        || line.contains("$STAGED_FILES")
        || line.contains("$changed")
        || line.contains("$changed_path");

    mentions_changed_source
        && (line.contains("grep")
            || line.starts_with("[[ ")
            || line.starts_with("[ ")
            || line.starts_with("test ")
            || line.starts_with("if ")
            || line.starts_with("elif ")
            || line.contains("==")
            || line.contains(" = "))
}

pub(super) fn mentions_config_exact(line: &str, needle: &str) -> bool {
    contains_bounded_config_reference(line, needle)
        || contains_bounded_config_reference(line, &regex_escaped_literal(needle))
}

pub(super) fn line_reaches_config_trigger(
    parsed: &ParsedShellScript,
    raw: &str,
    line_no: usize,
    needle: &str,
) -> bool {
    line_reaches_config_trigger_inner(
        parsed,
        parsed,
        raw,
        line_no,
        line_no,
        needle,
        &mut BTreeSet::new(),
    )
}

fn line_reaches_config_trigger_inner(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    raw: &str,
    line_no: usize,
    root_line_no: usize,
    needle: &str,
    visited_functions: &mut BTreeSet<String>,
) -> bool {
    if direct_trigger_line(raw, needle) {
        return true;
    }

    if looks_like_case_pattern_line(raw.trim()) && mentions_config_exact(raw, needle) {
        return true;
    }

    local
        .executable_lines
        .iter()
        .filter(|line| line.line_no == line_no && line.raw == raw)
        .any(|line| {
            let Some(function) = resolve_visible_function(
                local,
                root,
                &line.command_name,
                line_no,
                root_line_no,
            ) else {
                return false;
            };

            if !visited_functions.insert(function.name.clone()) {
                return false;
            }

            let found = function.parsed_body.executable_lines.iter().any(|nested_line| {
                line_reaches_config_trigger_inner(
                    &function.parsed_body,
                    root,
                    &nested_line.raw,
                    nested_line.line_no,
                    root_line_no,
                    needle,
                    visited_functions,
                )
            });
            let _ = visited_functions.remove(&function.name);
            found
        })
}

fn resolve_visible_function<'a>(
    local: &'a ParsedShellScript,
    root: &'a ParsedShellScript,
    function_name: &str,
    line_no: usize,
    root_line_no: usize,
) -> Option<&'a ShellFunction> {
    local
        .functions
        .iter()
        .rev()
        .find(|function| function.name == function_name && function.line_no <= line_no)
        .or_else(|| {
            if std::ptr::eq(local, root) {
                None
            } else {
                root.functions
                    .iter()
                    .rev()
                    .find(|function| function.name == function_name && function.line_no <= root_line_no)
            }
        })
}

fn direct_trigger_line(line: &str, needle: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.starts_with('#')
        && !trimmed.is_empty()
        && mentions_config_exact(line, needle)
        && is_trigger_like_line(trimmed)
}

fn regex_escaped_literal(needle: &str) -> String {
    let mut escaped = String::with_capacity(needle.len() * 2);
    for ch in needle.chars() {
        match ch {
            '.' => escaped.push_str("\\."),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn contains_bounded_config_reference(line: &str, needle: &str) -> bool {
    line.match_indices(needle).any(|(start, _)| {
        let before = line[..start].chars().next_back();
        let after = line[start + needle.len()..].chars().next();
        !before.is_some_and(is_filename_continuation)
            && !after.is_some_and(is_filename_continuation)
    })
}

fn is_filename_continuation(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-')
}

pub(super) fn looks_like_case_pattern_line(line: &str) -> bool {
    line.ends_with(')') && !line.starts_with("echo ") && !line.starts_with('(')
}

pub(super) fn line_contains_then(line: &str) -> bool {
    line == "then" || line.ends_with("; then") || line.ends_with(";then") || line.contains(" then ")
}

pub(super) fn expand_inline_if_block(line: &str) -> String {
    replace_top_level_if_marker(
        &replace_top_level_if_marker(
            &replace_top_level_if_marker(
                &replace_top_level_if_marker(
                    &replace_top_level_if_marker(line, "; then ", "; then\n"),
                    ";then ",
                    ";then\n",
                ),
                "; else ",
                "\nelse\n",
            ),
            ";elif ",
            "\nelif ",
        ),
        "; fi",
        "\nfi",
    )
    .pipe(|expanded| replace_top_level_if_marker(&expanded, "; else", "\nelse\n"))
    .pipe(|expanded| replace_top_level_if_marker(&expanded, ";else ", "\nelse\n"))
    .pipe(|expanded| replace_top_level_if_marker(&expanded, "; elif ", "\nelif "))
    .pipe(|expanded| replace_top_level_if_marker(&expanded, ";elif ", "\nelif "))
    .pipe(|expanded| replace_top_level_if_marker(&expanded, ";fi", "\nfi"))
}

pub(super) fn expand_inline_case_block(line: &str) -> String {
    replace_top_level_case_marker(
        &replace_top_level_case_marker(
            &replace_top_level_case_marker(
                &insert_newline_after_top_level_case_pattern_paren(
                    &replace_first_top_level_case_marker(line, " in ", " in\n"),
                ),
                " ;; ",
                "\n;;\n",
            ),
            ";; ",
            "\n;;\n",
        ),
        ";;",
        "\n;;\n",
    )
    .replace("; esac", "\nesac")
    .replace(";esac", "\nesac")
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}

fn insert_newline_after_top_level_case_pattern_paren(line: &str) -> String {
    let mut result = String::with_capacity(line.len() + 8);
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut index = 0usize;

    while index < line.len() {
        let rest = &line[index..];
        let ch = rest
            .chars()
            .next()
            .expect("insert_newline_after_top_level_case_pattern_paren walks utf-8 boundaries");

        match ch {
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
                result.push(ch);
                index += ch.len_utf8();
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
                result.push(ch);
                index += ch.len_utf8();
            }
            ')' if !single_quoted && !double_quoted => {
                result.push(')');
                let rest = &line[index + ch.len_utf8()..];
                let trimmed_rest = rest.trim_start();
                if !trimmed_rest.is_empty() && !trimmed_rest.starts_with(';') {
                    result.push('\n');
                }
                index += ch.len_utf8();
            }
            _ => {
                result.push(ch);
                index += ch.len_utf8();
            }
        }
    }

    result
}

fn replace_top_level_if_marker(line: &str, needle: &str, replacement: &str) -> String {
    let mut result = String::with_capacity(line.len() + 8);
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut index = 0usize;

    while index < line.len() {
        let rest = &line[index..];
        let ch = rest
            .chars()
            .next()
            .expect("replace_top_level_if_marker only walks valid utf-8 boundaries");

        match ch {
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
                result.push(ch);
                index += ch.len_utf8();
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
                result.push(ch);
                index += ch.len_utf8();
            }
            _ if !single_quoted && !double_quoted && rest.starts_with(needle) => {
                result.push_str(replacement);
                index += needle.len();
            }
            _ => {
                result.push(ch);
                index += ch.len_utf8();
            }
        }
    }

    result
}

fn replace_first_top_level_case_marker(line: &str, needle: &str, replacement: &str) -> String {
    replace_top_level_case_marker_impl(line, needle, replacement, true)
}

fn replace_top_level_case_marker(line: &str, needle: &str, replacement: &str) -> String {
    replace_top_level_case_marker_impl(line, needle, replacement, false)
}

fn replace_top_level_case_marker_impl(
    line: &str,
    needle: &str,
    replacement: &str,
    replace_once: bool,
) -> String {
    let mut result = String::with_capacity(line.len() + 8);
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut index = 0usize;
    let mut replaced = false;

    while index < line.len() {
        let rest = &line[index..];
        let ch = rest
            .chars()
            .next()
            .expect("replace_top_level_case_marker only walks valid utf-8 boundaries");

        match ch {
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
                result.push(ch);
                index += ch.len_utf8();
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
                result.push(ch);
                index += ch.len_utf8();
            }
            _ if !single_quoted
                && !double_quoted
                && (!replace_once || !replaced)
                && rest.starts_with(needle) =>
            {
                result.push_str(replacement);
                index += needle.len();
                replaced = true;
            }
            _ => {
                result.push(ch);
                index += ch.len_utf8();
            }
        }
    }

    result
}
