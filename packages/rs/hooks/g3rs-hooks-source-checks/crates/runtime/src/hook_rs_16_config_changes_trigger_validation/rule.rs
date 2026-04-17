use crate::compat::{G3CheckResult, G3Severity};

use crate::hook_rs_08_guardrail_validate_staged_present::{
    script_contains_guardrail_step, script_contains_path_qualified_guardrail_step,
};
use crate::inputs::RustHookCommandInput;

const ID: &str = "RS-HOOKS-SOURCE-15";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let config_needles = [
        "guardrail3-rs.toml",
        "clippy.toml",
        ".clippy.toml",
        "deny.toml",
        ".deny.toml",
        "rustfmt.toml",
        ".rustfmt.toml",
        "rust-toolchain.toml",
    ];

    let content = script_text(input.parsed);
    let blocks = conditional_blocks(content.as_str());
    let covered = config_needles.iter().all(|needle| {
        blocks.iter().any(|block| {
            block_branches(block)
                .into_iter()
                .any(|branch| branch_covers_needle(&branch, needle))
        }) || content_has_direct_trigger_line_for_needle(content.as_str(), needle)
    });

    if covered {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "Rust config changes trigger hook validation".to_owned(),
                "Hook trigger logic covers Rust guardrail config files.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "Rust config-change trigger coverage incomplete".to_owned(),
            "Hook trigger logic does not clearly include all Rust guardrail config files."
                .to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn conditional_blocks(content: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current = Vec::new();
    let mut depth = 0usize;

    for line in content.lines() {
        let trimmed = line.trim();
        if depth == 0 && starts_conditional_block(trimmed) {
            current.push(line);
            if ends_conditional_block(trimmed) {
                blocks.push(current.join("\n"));
                current.clear();
                continue;
            }

            depth = 1;
            continue;
        }

        if depth > 0 {
            current.push(line);
            if starts_conditional_block(trimmed) {
                depth += 1;
            } else if ends_conditional_block(trimmed) {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    blocks.push(current.join("\n"));
                    current.clear();
                }
            }
        }
    }

    blocks
}

fn starts_conditional_block(line: &str) -> bool {
    line.starts_with("if ") || line.starts_with("case ")
}

fn ends_conditional_block(line: &str) -> bool {
    line == "fi"
        || line.ends_with("; fi")
        || line.ends_with(";fi")
        || line == "esac"
        || line.ends_with("; esac")
        || line.ends_with(";esac")
}

fn block_contains_validation(block: &str) -> bool {
    let parsed = hook_shell_parser::parse_script(block);
    script_contains_guardrail_step(&parsed)
        || script_contains_path_qualified_guardrail_step(&parsed)
}

fn branch_covers_needle(branch: &str, needle: &str) -> bool {
    let (direct_branch, nested_blocks) = partition_branch(branch);
    (block_contains_validation(&direct_branch)
        && block_mentions_config_trigger(&direct_branch, needle))
        || nested_blocks.into_iter().any(|nested_block| {
            block_branches(&nested_block)
                .into_iter()
                .any(|nested_branch| branch_covers_needle(&nested_branch, needle))
        })
}

fn block_branches(block: &str) -> Vec<String> {
    let first_non_empty = block
        .lines()
        .find_map(|line| {
            let trimmed = line.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .unwrap_or_default();

    if first_non_empty.starts_with("if ") {
        return if_branches(block);
    }

    if first_non_empty.starts_with("case ") {
        return case_branches(block);
    }

    vec![block.to_owned()]
}

fn partition_branch(branch: &str) -> (String, Vec<String>) {
    let first_non_empty = branch
        .lines()
        .find_map(|line| {
            let trimmed = line.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .unwrap_or_default();
    let mut direct_lines = Vec::new();
    let mut nested_blocks = Vec::new();
    let mut current_nested = Vec::new();
    let mut nested_depth = 0usize;
    let mut in_if_condition =
        first_non_empty.starts_with("if ") || first_non_empty.starts_with("elif ");
    let mut saw_first_non_empty = false;

    for line in branch.lines() {
        let trimmed = line.trim();

        if !saw_first_non_empty && !trimmed.is_empty() {
            saw_first_non_empty = true;
            direct_lines.push(line);
            if in_if_condition && line_contains_then(trimmed) {
                in_if_condition = false;
            }
            continue;
        }

        if in_if_condition {
            direct_lines.push(line);
            if line_contains_then(trimmed) {
                in_if_condition = false;
            }
            continue;
        }

        if nested_depth == 0 && starts_conditional_block(trimmed) {
            current_nested.push(line);
            if ends_conditional_block(trimmed) {
                nested_blocks.push(current_nested.join("\n"));
                current_nested.clear();
            } else {
                nested_depth = 1;
            }
            continue;
        }

        if nested_depth > 0 {
            current_nested.push(line);
            if starts_conditional_block(trimmed) && !ends_conditional_block(trimmed) {
                nested_depth += 1;
            } else if ends_conditional_block(trimmed) {
                nested_depth = nested_depth.saturating_sub(1);
                if nested_depth == 0 {
                    nested_blocks.push(current_nested.join("\n"));
                    current_nested.clear();
                }
            }
            continue;
        }

        direct_lines.push(line);
    }

    (direct_lines.join("\n"), nested_blocks)
}

fn if_branches(block: &str) -> Vec<String> {
    if block.lines().count() == 1 {
        let expanded = expand_inline_if_block(block);
        if expanded != block {
            return if_branches(&expanded);
        }
    }

    let mut lines = block.lines();
    let Some(first_line) = lines.next() else {
        return Vec::new();
    };

    let mut branches = Vec::new();
    let mut current = vec![first_line];
    let mut depth = if ends_conditional_block(first_line.trim()) {
        0usize
    } else {
        1usize
    };

    if depth == 0 {
        return vec![current.join("\n")];
    }

    for line in lines {
        let trimmed = line.trim();
        if depth == 1 && (trimmed.starts_with("elif ") || trimmed == "else") {
            branches.push(current.join("\n"));
            current.clear();
        }

        current.push(line);
        depth = adjust_depth(depth, trimmed);

        if depth == 0 {
            branches.push(current.join("\n"));
            current.clear();
            break;
        }
    }

    if !current.is_empty() {
        branches.push(current.join("\n"));
    }

    branches
}

fn case_branches(block: &str) -> Vec<String> {
    if block.lines().count() == 1 {
        let expanded = expand_inline_case_block(block);
        if expanded != block {
            return case_branches(&expanded);
        }
    }

    let mut lines = block.lines();
    let Some(first_line) = lines.next() else {
        return Vec::new();
    };

    let mut branches = Vec::new();
    let mut current = Vec::new();
    let mut depth = if ends_conditional_block(first_line.trim()) {
        0usize
    } else {
        1usize
    };

    if depth == 0 {
        return vec![first_line.to_owned()];
    }

    for line in lines {
        let trimmed = line.trim();

        if depth == 1 && looks_like_case_pattern_line(trimmed) && !current.is_empty() {
            branches.push(current.join("\n"));
            current.clear();
        }

        if !current.is_empty() || looks_like_case_pattern_line(trimmed) {
            current.push(line);
        }

        depth = adjust_depth(depth, trimmed);

        if depth == 1 && trimmed == ";;" {
            branches.push(current.join("\n"));
            current.clear();
            continue;
        }

        if depth == 0 {
            if !current.is_empty() {
                branches.push(current.join("\n"));
                current.clear();
            }
            break;
        }
    }

    branches
}

fn adjust_depth(depth: usize, trimmed: &str) -> usize {
    let mut next = depth;
    if starts_conditional_block(trimmed) && !ends_conditional_block(trimmed) {
        next += 1;
    } else if ends_conditional_block(trimmed) && next > 0 {
        next -= 1;
    }
    next
}

fn block_mentions_config_trigger(block: &str, needle: &str) -> bool {
    let first_non_empty = block
        .lines()
        .find_map(|line| {
            let trimmed = line.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .unwrap_or_default();
    let case_block = block
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .any(|line| line.starts_with("case ") || looks_like_case_pattern_line(line));
    let if_like_branch = first_non_empty.starts_with("if ") || first_non_empty.starts_with("elif ");
    let mut in_if_condition = if_like_branch;

    block.lines().any(|line| {
        let trimmed = line.trim();
        let in_condition_now = in_if_condition;
        let is_match = !trimmed.starts_with('#')
            && !trimmed.is_empty()
            && mentions_config_exact(line, needle)
            && (is_trigger_like_line(trimmed)
                || (in_condition_now && is_trigger_like_line(trimmed))
                || (case_block && looks_like_case_pattern_line(trimmed)));

        if in_if_condition && line_contains_then(trimmed) {
            in_if_condition = false;
        }

        is_match
    })
}

fn content_has_direct_trigger_line_for_needle(content: &str, needle: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        !trimmed.is_empty()
            && !trimmed.starts_with('#')
            && mentions_config_exact(line, needle)
            && is_trigger_like_line(trimmed)
            && block_contains_validation(line)
    })
}

fn script_text(parsed: &hook_shell_parser::types::ParsedShellScript) -> String {
    parsed
        .source_lines
        .iter()
        .map(|line| line.raw.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_trigger_like_line(line: &str) -> bool {
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

fn mentions_config_exact(line: &str, needle: &str) -> bool {
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

fn looks_like_case_pattern_line(line: &str) -> bool {
    line.ends_with(')') && !line.starts_with("echo ") && !line.starts_with('(')
}

fn line_contains_then(line: &str) -> bool {
    line == "then" || line.ends_with("; then") || line.ends_with(";then") || line.contains(" then ")
}

fn expand_inline_if_block(line: &str) -> String {
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

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}

fn expand_inline_case_block(line: &str) -> String {
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

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
