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
