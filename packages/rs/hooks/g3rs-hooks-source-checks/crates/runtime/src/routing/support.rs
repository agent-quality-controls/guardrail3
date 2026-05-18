#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::indexing_slicing,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::needless_continue,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::string_slice,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use hook_shell_parser::command_query::{
    CommandQueryOptions, CommandVisit, ResolvedCommand, ShellEnvState,
    visit_resolved_commands_with_env,
};
use hook_shell_parser::types::ParsedShellScript;

/// Returns true if `command` invokes `g3rs validate` (the per-workspace validator).
pub(crate) fn is_g3rs_validate_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "g3rs"
        && command.args().first().map(String::as_str) == Some("validate")
}

/// Iterates over every resolved `g3rs validate` call and returns the `--path` value if present.
pub(crate) fn collect_precommit_scope_values(parsed: &ParsedShellScript) -> Vec<String> {
    let mut values = Vec::new();
    visit_resolved_commands_with_env(
        parsed,
        EmptyEnv,
        CommandQueryOptions::default(),
        |command, _| {
            if is_g3rs_validate_command(command) {
                if let Some(value) = scope_arg_value(command.args()) {
                    values.push(value);
                }
            }
            CommandVisit::Continue
        },
    );
    values
}

/// `EmptyEnv` struct.
#[derive(Clone)]
struct EmptyEnv;

impl ShellEnvState for EmptyEnv {
    fn apply_assignment(&mut self, _name: &str, _value: &str) {}
    fn unset(&mut self, _name: &str) {}
    fn clear(&mut self) {}
}

/// `scope_arg_value` function.
pub(crate) fn scope_arg_value(args: &[String]) -> Option<String> {
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if arg == "--path" {
            return args.get(index + 1).cloned();
        }
        if let Some(rest) = arg.strip_prefix("--path=") {
            return Some(rest.to_owned());
        }
        index += 1;
    }
    None
}

/// Returns the unquoted scope token. Strips surrounding quotes once.
pub(crate) fn unquote_scope(value: &str) -> &str {
    let trimmed = value.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2)
        || (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2)
    {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    }
}

/// Returns true when the scope token resolves wholly to a single shell variable expansion (e.g. `$unit`, `${unit}`).
/// A token like `$REPO_ROOT/apps/guardrail3-rs` is not a wholly-variable token.
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

/// `is_shell_identifier` function.
fn is_shell_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

/// Returns the set of bare shell variable names referenced inside any `--path` token
/// of a `g3rs validate` invocation. For `--path "$RUST_WORKSPACE"` returns
/// `["RUST_WORKSPACE"]`; for `--path "$REPO_ROOT/$RUST_WORKSPACE"` returns both.
pub(crate) fn precommit_scope_variable_names(parsed: &ParsedShellScript) -> Vec<String> {
    let mut names = Vec::new();
    for value in collect_precommit_scope_values(parsed) {
        let body = unquote_scope(&value);
        extract_variable_names(body, &mut names);
    }
    names
}

#[expect(
    clippy::excessive_nesting,
    reason = "shell script parser is inherently iterative with byte-walking loops"
)]
/// Returns variables that feed the per-workspace validate scope, including those passed
/// indirectly via a `done <<< "$VAR"` heredoc or `for X in $VAR` driving a loop body
/// that calls `g3rs validate`. A default-fallback assignment to any of these variables
/// produces a default scope just as much as a direct assignment.
pub(crate) fn precommit_scope_feeder_variable_names(parsed: &ParsedShellScript) -> Vec<String> {
    let mut names = precommit_scope_variable_names(parsed);

    let lines: Vec<&str> = parsed.source_lines.iter().map(|l| l.raw.as_str()).collect();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        // `done <<< "$VAR"` or `done <<< $VAR` -- the var feeds the closing loop block above.
        if let Some(after) = trimmed.strip_prefix("done") {
            let after = after.trim_start();
            if let Some(rest) = after.strip_prefix("<<<") {
                let rhs = rest.trim();
                if loop_block_above_calls_validate(&lines, idx) {
                    let mut buf = Vec::new();
                    extract_variable_names(rhs, &mut buf);
                    names.extend(buf);
                }
            }
        }
        // `for X in $VAR; do ... done` -- the iterated `$VAR` feeds the loop body.
        if trimmed.starts_with("for ") {
            if let Some(in_idx) = trimmed.find(" in ") {
                let after_in = trimmed[in_idx + 4..].trim();
                let payload = after_in.trim_end_matches("do").trim_end_matches(';').trim();
                if loop_block_below_calls_validate(&lines, idx) {
                    let mut buf = Vec::new();
                    extract_variable_names(payload, &mut buf);
                    names.extend(buf);
                }
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

/// `line_calls_validate` function.
fn line_calls_validate(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with('#') {
        return false;
    }
    // Accept `g3rs validate ` and `g3rs validate"` (end of line). Avoid accidental
    // matches against `g3rs validate-repo` (no `--path`).
    let words = hook_shell_parser::command_query::shell_words(trimmed);
    let mut index = 0;
    while index + 1 < words.len() {
        if words[index] == "g3rs" && words[index + 1] == "validate" {
            return true;
        }
        index += 1;
    }
    false
}

/// `loop_block_above_calls_validate` function.
fn loop_block_above_calls_validate(lines: &[&str], done_idx: usize) -> bool {
    let mut depth = 1;
    let mut i = done_idx;
    while i > 0 {
        i -= 1;
        let trimmed = lines[i].trim_start();
        if trimmed.starts_with("done") {
            depth += 1;
            continue;
        }
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            depth -= 1;
            if depth == 0 {
                break;
            }
        }
        if line_calls_validate(lines[i]) {
            return true;
        }
    }
    false
}

/// `loop_block_below_calls_validate` function.
fn loop_block_below_calls_validate(lines: &[&str], for_idx: usize) -> bool {
    let mut depth = 1;
    let mut i = for_idx;
    while i + 1 < lines.len() {
        i += 1;
        let trimmed = lines[i].trim_start();
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            depth += 1;
            continue;
        }
        if trimmed.starts_with("done") {
            depth -= 1;
            if depth == 0 {
                break;
            }
        }
        if line_calls_validate(lines[i]) {
            return true;
        }
    }
    false
}

#[expect(
    clippy::excessive_nesting,
    reason = "shell script parser is inherently iterative with byte-walking loops"
)]
/// `extract_variable_names` function.
fn extract_variable_names(value: &str, out: &mut Vec<String>) {
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' {
            if bytes.get(i + 1) == Some(&b'{') {
                let mut j = i + 2;
                let start = j;
                while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                    j += 1;
                }
                if j > start {
                    out.push(value[start..j].to_owned());
                }
                while j < bytes.len() && bytes[j] != b'}' {
                    j += 1;
                }
                i = if j < bytes.len() { j + 1 } else { j };
                continue;
            }
            let mut j = i + 1;
            let start = j;
            while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            if j > start {
                out.push(value[start..j].to_owned());
            }
            i = j;
            continue;
        }
        i += 1;
    }
}

#[expect(
    clippy::excessive_nesting,
    reason = "shell script parser is inherently iterative with byte-walking loops"
)]
/// Returns true if `text` contains a command-substitution default such as
/// `$(... || echo <literal>)` or `$(cat ... 2>/dev/null || echo <literal>)`.
/// The right-hand side of the `||` must be a bare or quoted literal (no `$`).
pub(crate) fn contains_command_substitution_default(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'$' && bytes[i + 1] == b'(' {
            let mut depth = 1;
            let mut j = i + 2;
            while j < bytes.len() && depth > 0 {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => depth -= 1,
                    _ => {}
                }
                if depth > 0 {
                    j += 1;
                }
            }
            if depth == 0 && j <= bytes.len() {
                let inner = &text[i + 2..j];
                if let Some(idx) = inner.find("||") {
                    let rhs = inner[idx + 2..].trim();
                    if rhs_is_literal_default(rhs) {
                        return true;
                    }
                }
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    false
}

/// `rhs_is_literal_default` function.
fn rhs_is_literal_default(rhs: &str) -> bool {
    // Strip a leading `echo`/`printf` keyword if present.
    let trimmed = rhs.trim();
    let body = trimmed
        .strip_prefix("echo")
        .or_else(|| trimmed.strip_prefix("printf"))
        .map_or(trimmed, str::trim);
    if body.is_empty() {
        return false;
    }
    let unquoted = body.trim_matches(|c| c == '"' || c == '\'');
    // Pure literal (no shell variable) is a default. A variable-prefixed value
    // like `$REPO_ROOT/apps/guardrail3-rs` is also a fixed hardcoded path and
    // counts as a default for this rule.
    !unquoted.contains('$') || rhs_contains_literal_path_segment(unquoted)
}

#[expect(
    clippy::excessive_nesting,
    reason = "shell script parser is inherently iterative with byte-walking loops"
)]
/// `rhs_contains_literal_path_segment` function.
fn rhs_contains_literal_path_segment(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'$' => {
                if bytes.get(i + 1) == Some(&b'{') {
                    let mut depth = 1;
                    let mut j = i + 2;
                    while j < bytes.len() && depth > 0 {
                        match bytes[j] {
                            b'{' => depth += 1,
                            b'}' => depth -= 1,
                            _ => {}
                        }
                        if depth > 0 {
                            j += 1;
                        }
                    }
                    i = j + 1;
                    continue;
                }
                let mut j = i + 1;
                while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                    j += 1;
                }
                i = j;
                continue;
            }
            b'/' => {
                let after = &value[i + 1..];
                if after
                    .bytes()
                    .any(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
                {
                    return true;
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    false
}

/// Returns true if the parsed script contains a default-fallback assignment of the form
/// `if [ -z "$VAR" ]; then VAR=<literal>; fi` or its `[[ ... ]]` variant, where `VAR` is
/// in `target_names`. The literal must not reference another shell variable.
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

/// `line_is_default_fallback_assignment` function.
fn line_is_default_fallback_assignment(line: &str, name: &str) -> bool {
    // Look for the `[ -z "$NAME" ]` (or `[[ -z "$NAME" ]]`) test followed by the
    // `; then NAME=<literal>; fi` body, all on a single trimmed line.
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

    let assign = format!("{name}=");
    let Some(idx) = line.find("then") else {
        return false;
    };
    let after_then = &line[idx + 4..];
    let Some(assign_idx) = after_then.find(&assign) else {
        return false;
    };
    let after_assign = &after_then[assign_idx + assign.len()..];
    // Pull the assigned value up to the first `;` or `fi`.
    let value_end = after_assign
        .find(';')
        .or_else(|| after_assign.find("fi"))
        .unwrap_or(after_assign.len());
    let value = after_assign[..value_end].trim();
    rhs_is_literal_default(value)
}

#[expect(
    clippy::excessive_nesting,
    reason = "shell script parser is inherently iterative with byte-walking loops"
)]
/// Returns true if `text` contains an env-default substitution (`${VAR:-default}`).
pub(crate) fn contains_env_default_substitution(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'$' && bytes[i + 1] == b'{' {
            // Find matching closing brace.
            let mut depth = 1;
            let mut j = i + 2;
            while j < bytes.len() && depth > 0 {
                match bytes[j] {
                    b'{' => depth += 1,
                    b'}' => depth -= 1,
                    _ => {}
                }
                if depth > 0 {
                    j += 1;
                }
            }
            if depth == 0 && j < bytes.len() {
                let inner = &text[i + 2..j];
                if inner.contains(":-") || inner.contains(":=") {
                    return true;
                }
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    false
}
