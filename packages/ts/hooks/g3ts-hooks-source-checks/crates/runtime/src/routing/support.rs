use hook_shell_parser::command_query::{
    CommandQueryOptions, CommandVisit, ResolvedCommand, ShellEnvState,
    visit_resolved_commands_with_env,
};
use hook_shell_parser::types::ParsedShellScript;

/// Returns true if `command` invokes `g3ts validate` (the per-package validator).
pub(crate) fn is_g3ts_validate_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "g3ts"
        && command.args().first().map(String::as_str) == Some("validate")
}

/// Iterates over every resolved `g3ts validate` call and returns the `--path` value if present.
pub(crate) fn collect_precommit_scope_values(parsed: &ParsedShellScript) -> Vec<String> {
    let mut values = Vec::new();
    visit_resolved_commands_with_env(
        parsed,
        EmptyEnv,
        CommandQueryOptions::default(),
        |command, _| {
            if is_g3ts_validate_command(command) {
                if let Some(value) = scope_arg_value(command.args()) {
                    values.push(value);
                }
            }
            CommandVisit::Continue
        },
    );
    values
}

/// Empty `ShellEnvState` used when we do not need to track variable bindings.
#[derive(Clone, Debug)]
struct EmptyEnv;

impl ShellEnvState for EmptyEnv {
    fn apply_assignment(&mut self, _name: &str, _value: &str) {}
    fn unset(&mut self, _name: &str) {}
    fn clear(&mut self) {}
}

/// Returns the value of the `--path <value>` or `--path=<value>` argument, if present.
pub(crate) fn scope_arg_value(args: &[String]) -> Option<String> {
    for (index, arg) in args.iter().enumerate() {
        if arg == "--path" {
            return args.get(index.saturating_add(1)).cloned();
        }
        if let Some(rest) = arg.strip_prefix("--path=") {
            return Some(rest.to_owned());
        }
    }
    None
}

/// Returns the unquoted scope token. Strips surrounding quotes once.
pub(crate) fn unquote_scope(value: &str) -> &str {
    let trimmed = value.trim();
    if let Some(inner) = trimmed.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return inner;
    }
    if let Some(inner) = trimmed
        .strip_prefix('\'')
        .and_then(|s| s.strip_suffix('\''))
    {
        return inner;
    }
    trimmed
}

/// Returns true when the scope token resolves wholly to a single shell variable expansion.
pub(crate) fn scope_is_wholly_variable(value: &str) -> bool {
    let body = unquote_scope(value);
    if let Some(rest) = body.strip_prefix("${") {
        if let Some(name) = rest.strip_suffix('}') {
            return is_shell_identifier(name);
        }
    }
    if let Some(name) = body.strip_prefix('$') {
        return is_shell_identifier(name);
    }
    false
}

/// Returns true when `name` is a valid POSIX shell identifier (`[A-Za-z_][A-Za-z0-9_]*`).
fn is_shell_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

/// Returns the set of bare shell variable names referenced inside any `--path` token
/// of a `g3ts validate` invocation.
pub(crate) fn precommit_scope_variable_names(parsed: &ParsedShellScript) -> Vec<String> {
    let mut names = Vec::new();
    for value in collect_precommit_scope_values(parsed) {
        let body = unquote_scope(&value);
        extract_variable_names(body, &mut names);
    }
    names
}

/// Returns variables that feed the per-package validate scope, including those passed
/// indirectly via a `done <<< "$VAR"` heredoc or `for X in $VAR` driving a loop body
/// that calls `g3ts validate`.
pub(crate) fn precommit_scope_feeder_variable_names(parsed: &ParsedShellScript) -> Vec<String> {
    let mut names = precommit_scope_variable_names(parsed);

    let lines: Vec<&str> = parsed.source_lines.iter().map(|l| l.raw.as_str()).collect();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        collect_done_heredoc_feeders(&lines, idx, trimmed, &mut names);
        collect_for_in_feeders(&lines, idx, trimmed, &mut names);
    }
    names.sort();
    names.dedup();
    names
}

/// Adds variables read by `done <<< "$VAR"` into `names` when the loop above calls validate.
fn collect_done_heredoc_feeders(
    lines: &[&str],
    idx: usize,
    trimmed: &str,
    names: &mut Vec<String>,
) {
    let Some(after) = trimmed.strip_prefix("done") else {
        return;
    };
    let after = after.trim_start();
    let Some(rest) = after.strip_prefix("<<<") else {
        return;
    };
    let rhs = rest.trim();
    if !loop_block_above_calls_validate(lines, idx) {
        return;
    }
    let mut buf = Vec::new();
    extract_variable_names(rhs, &mut buf);
    names.extend(buf);
}

/// Adds variables read by `for X in $VAR` into `names` when the loop body calls validate.
fn collect_for_in_feeders(lines: &[&str], idx: usize, trimmed: &str, names: &mut Vec<String>) {
    if !trimmed.starts_with("for ") {
        return;
    }
    let Some((_, after_in)) = trimmed.split_once(" in ") else {
        return;
    };
    let payload = after_in
        .trim()
        .trim_end_matches("do")
        .trim_end_matches(';')
        .trim();
    if !loop_block_below_calls_validate(lines, idx) {
        return;
    }
    let mut buf = Vec::new();
    extract_variable_names(payload, &mut buf);
    names.extend(buf);
}

/// Returns true when `line` invokes `g3ts validate ...`.
fn line_calls_validate(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with('#') {
        return false;
    }
    let words = hook_shell_parser::command_query::shell_words(trimmed);
    words
        .windows(2)
        .any(|pair| matches!(pair, [first, second] if first == "g3ts" && second == "validate"))
}

/// Returns true when the loop block ending at `done_idx` (1-based depth from `done`) invokes validate.
fn loop_block_above_calls_validate(lines: &[&str], done_idx: usize) -> bool {
    let mut depth: usize = 1;
    let mut cursor = done_idx;
    while cursor > 0 {
        cursor = cursor.saturating_sub(1);
        let Some(line) = lines.get(cursor) else { break };
        let trimmed = line.trim_start();
        if trimmed.starts_with("done") {
            depth = depth.saturating_add(1);
            continue;
        }
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                break;
            }
        }
        if line_calls_validate(line) {
            return true;
        }
    }
    false
}

/// Returns true when the loop block opened at `for_idx` invokes validate.
fn loop_block_below_calls_validate(lines: &[&str], for_idx: usize) -> bool {
    let mut depth: usize = 1;
    let mut cursor = for_idx;
    while cursor.saturating_add(1) < lines.len() {
        cursor = cursor.saturating_add(1);
        let Some(line) = lines.get(cursor) else { break };
        let trimmed = line.trim_start();
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            depth = depth.saturating_add(1);
            continue;
        }
        if trimmed.starts_with("done") {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                break;
            }
        }
        if line_calls_validate(line) {
            return true;
        }
    }
    false
}

/// Extracts every `$VAR` and `${VAR}` identifier in `value` into `out`.
fn extract_variable_names(value: &str, out: &mut Vec<String>) {
    let mut rest = value;
    while let Some(idx) = rest.find('$') {
        let after = rest.get(idx.saturating_add(1)..).unwrap_or("");
        if let Some(braced) = after.strip_prefix('{') {
            let _ = push_identifier_prefix(braced, out);
            // Advance past the closing brace, if present.
            let close_offset = braced
                .find('}')
                .map_or(braced.len(), |i| i.saturating_add(1));
            rest = braced.get(close_offset..).unwrap_or("");
            continue;
        }
        let identifier_len = push_identifier_prefix(after, out);
        rest = after.get(identifier_len..).unwrap_or("");
    }
}

/// Pushes the leading shell-identifier (if any) from `text` into `out`, returning its length.
fn push_identifier_prefix(text: &str, out: &mut Vec<String>) -> usize {
    let identifier_len = identifier_run_len(text);
    if identifier_len > 0 {
        if let Some(name) = text.get(..identifier_len) {
            out.push(name.to_owned());
        }
    }
    identifier_len
}

/// Returns the length (in bytes) of the leading shell-identifier run in `text`.
fn identifier_run_len(text: &str) -> usize {
    text.bytes()
        .take_while(|b| b.is_ascii_alphanumeric() || *b == b'_')
        .count()
}

/// Returns true if `text` contains a command-substitution default such as
/// `$(... || echo <literal>)`.
pub(crate) fn contains_command_substitution_default(text: &str) -> bool {
    let mut rest = text;
    while let Some(at) = rest.find("$(") {
        let after = rest.get(at.saturating_add(2)..).unwrap_or("");
        let Some(close_offset) = find_balanced_close(after, '(', ')') else {
            return false;
        };
        let inner = after.get(..close_offset).unwrap_or("");
        if let Some((_, rhs_part)) = inner.split_once("||") {
            if rhs_is_literal_default(rhs_part.trim()) {
                return true;
            }
        }
        rest = after.get(close_offset.saturating_add(1)..).unwrap_or("");
    }
    false
}

/// Returns the byte offset (within `body`) of the balanced closing delimiter, when
/// the caller has already consumed one opener of `(opener, closer)`.
fn find_balanced_close(body: &str, opener: char, closer: char) -> Option<usize> {
    let mut depth: usize = 1;
    for (index, ch) in body.char_indices() {
        if ch == opener {
            depth = depth.saturating_add(1);
        } else if ch == closer {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

/// Returns true if `rhs` is a literal default (a non-`$`-bearing string or one with
/// a literal path segment).
fn rhs_is_literal_default(rhs: &str) -> bool {
    let trimmed = rhs.trim();
    let body = trimmed
        .strip_prefix("echo")
        .or_else(|| trimmed.strip_prefix("printf"))
        .map_or(trimmed, str::trim);
    if body.is_empty() {
        return false;
    }
    let unquoted = body.trim_matches(|c| c == '"' || c == '\'');
    !unquoted.contains('$') || rhs_contains_literal_path_segment(unquoted)
}

/// Returns true if `value` contains a literal path segment (mirrors the routing rule).
fn rhs_contains_literal_path_segment(value: &str) -> bool {
    let mut bytes = value.bytes();
    while let Some(byte) = bytes.next() {
        if byte == b'/'
            && bytes
                .clone()
                .any(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
        {
            return true;
        }
    }
    false
}

/// Returns true if the parsed script contains a default-fallback assignment of the form
/// `if [ -z "$VAR" ]; then VAR=<literal>; fi` matching `target_names`.
pub(crate) fn contains_default_fallback_assignment_for(
    parsed: &ParsedShellScript,
    target_names: &[String],
) -> Option<String> {
    if target_names.is_empty() {
        return None;
    }
    for line in &parsed.source_lines {
        let trimmed = line.raw.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        for name in target_names {
            if line_is_default_fallback_assignment(trimmed, name) {
                return Some(name.clone());
            }
        }
    }
    None
}

/// Returns true when `line` is `if [ -z "$NAME" ]; then NAME=<literal>; fi` for `name`.
fn line_is_default_fallback_assignment(line: &str, name: &str) -> bool {
    let dollar_name = format!("\"${name}\"");
    let braced_name = format!("\"${{{name}}}\"");
    let bare_name = format!("${name}");
    let bare_braced = format!("${{{name}}}");

    let has_z_test = (line.contains("[ -z ") || line.contains("[[ -z "))
        && (line.contains(&dollar_name)
            || line.contains(&braced_name)
            || line.contains(&format!("[ -z {bare_name}"))
            || line.contains(&format!("[ -z {bare_braced}"))
            || line.contains(&format!("[[ -z {bare_name}"))
            || line.contains(&format!("[[ -z {bare_braced}")));
    if !has_z_test {
        return false;
    }

    let Some((_, after_then)) = line.split_once("then") else {
        return false;
    };
    let assign = format!("{name}=");
    let Some((_, after_assign)) = after_then.split_once(assign.as_str()) else {
        return false;
    };
    let value_end = after_assign
        .find(';')
        .or_else(|| after_assign.find("fi"))
        .unwrap_or(after_assign.len());
    let value = after_assign.get(..value_end).unwrap_or("").trim();
    rhs_is_literal_default(value)
}

/// Returns true if `text` contains an env-default substitution (`${VAR:-default}`).
pub(crate) fn contains_env_default_substitution(text: &str) -> bool {
    let mut rest = text;
    while let Some(at) = rest.find("${") {
        let after = rest.get(at.saturating_add(2)..).unwrap_or("");
        let Some(close_offset) = find_balanced_close(after, '{', '}') else {
            return false;
        };
        let inner = after.get(..close_offset).unwrap_or("");
        if inner.contains(":-") || inner.contains(":=") {
            return true;
        }
        rest = after.get(close_offset.saturating_add(1)..).unwrap_or("");
    }
    false
}
