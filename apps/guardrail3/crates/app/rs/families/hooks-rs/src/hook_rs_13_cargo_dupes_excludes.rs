use guardrail3_app_rs_family_hooks_shared::hook_shell::{ParsedShellScript, parse_script};
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-13";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = script_contains_cargo_dupes_with_exclude_tests(input.parsed);

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo-dupes excludes tests".to_owned(),
                message: "Hook runs cargo-dupes with `--exclude-tests`.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "cargo-dupes exclude-tests flag missing".to_owned(),
            message: "Hook does not execute cargo-dupes with `--exclude-tests`.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

fn script_contains_cargo_dupes_with_exclude_tests(parsed: &ParsedShellScript<'_>) -> bool {
    script_contains_cargo_dupes(parsed, true) && !script_contains_cargo_dupes(parsed, false)
}

fn script_contains_cargo_dupes(parsed: &ParsedShellScript<'_>, want_exclude_tests: bool) -> bool {
    parsed.executable_lines.iter().any(|line| {
        let mut visiting = Vec::new();
        segment_contains_cargo_dupes(
            line.command_text,
            parsed,
            parsed,
            &mut visiting,
            want_exclude_tests,
            line.line_no,
            line.line_no,
        ) || line_contains_cargo_dupes(
            line.raw,
            parsed,
            parsed,
            &mut visiting,
            want_exclude_tests,
            line.line_no,
            line.line_no,
        )
    })
}

fn line_contains_cargo_dupes(
    raw: &str,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool {
    let segments = split_command_segments(raw);
    if segments.is_empty() {
        return segment_contains_cargo_dupes(
            raw,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        );
    }

    let mut prefix_status = None;
    for segment in segments {
        let reachable = match (segment.operator_before, prefix_status) {
            (Some("&&"), Some(true)) => true,
            (Some("&&"), Some(false)) => false,
            (Some("||"), Some(true)) => false,
            (Some("||"), Some(false)) => true,
            _ => true,
        };

        if reachable {
            if segment_contains_cargo_dupes(
                &segment.text,
                current,
                root,
                visiting,
                want_exclude_tests,
                current_cutoff,
                root_cutoff,
            ) {
                return true;
            }
            for substitution in extract_command_substitutions(&segment.text) {
                if line_contains_cargo_dupes(
                    &substitution,
                    current,
                    root,
                    visiting,
                    want_exclude_tests,
                    current_cutoff,
                    root_cutoff,
                ) {
                    return true;
                }
            }
        }

        if reachable {
            if is_terminal_exit(&segment.text) {
                break;
            }
            prefix_status = constant_exit_status(&segment.text);
        }
    }

    false
}

fn segment_contains_cargo_dupes(
    segment: &str,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool {
    let tokens = shell_words(segment);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    let command_name = normalize_command_token(first);
    if token_is_shadowed_function(
        first,
        command_name,
        current,
        root,
        current_cutoff,
        root_cutoff,
    ) {
        return called_function_contains_cargo_dupes(
            command_name,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        );
    }

    match command_name {
        "env" => env_wrapper_contains_cargo_dupes(
            parts,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "sh" | "bash" => shell_wrapper_contains_cargo_dupes(
            parts,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "command" => command_wrapper_contains_cargo_dupes(
            parts,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "exec" => exec_wrapper_contains_cargo_dupes(
            parts,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "cargo" => cargo_dupes_subcommand_invocation(&mut parts, want_exclude_tests),
        "cargo-dupes" => cargo_dupes_binary_invocation(&mut parts, want_exclude_tests),
        command_name => called_function_contains_cargo_dupes(
            command_name,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
    }
}

fn called_function_contains_cargo_dupes(
    command_name: &str,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool {
    let Some(function) = current
        .functions
        .iter()
        .find(|function| function.name == command_name && function.line_no <= current_cutoff)
        .or_else(|| {
            root.functions
                .iter()
                .find(|function| function.name == command_name && function.line_no <= root_cutoff)
        })
    else {
        return false;
    };
    if visiting.iter().any(|name| name == &function.name) {
        return false;
    }

    visiting.push(function.name.clone());
    let body_parsed = parse_script(&function.body);
    let found = body_parsed.executable_lines.iter().any(|line| {
        line_contains_cargo_dupes(
            line.raw,
            &body_parsed,
            root,
            visiting,
            want_exclude_tests,
            line.line_no,
            root_cutoff,
        )
    });
    let _ = visiting.pop();
    found
}

fn env_wrapper_contains_cargo_dupes<'a, I>(
    mut parts: std::iter::Peekable<I>,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut split_string = None;

    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
        if let Some((flag_name, value)) = flag.split_once('=')
            && env_flag_takes_value(flag_name)
        {
            if matches!(flag_name, "-S" | "--split-string") {
                split_string = Some(value.to_owned());
            }
            continue;
        }
        if env_flag_allowed_without_value(flag) {
            continue;
        }
        if env_flag_takes_value(flag) {
            let value = parts.next().unwrap_or_default();
            if matches!(flag, "-S" | "--split-string") {
                split_string = Some(value.to_owned());
            }
            continue;
        }
        return false;
    }

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    if let Some(script) = split_string {
        let tail: Vec<_> = parts.map(str::to_owned).collect();
        if split_string_is_assignment_only(&script) {
            if tail.is_empty() {
                return false;
            }
            return line_contains_cargo_dupes(
                &tail.join(" "),
                current,
                root,
                visiting,
                want_exclude_tests,
                current_cutoff,
                root_cutoff,
            );
        }

        let mut nested = script;
        if !tail.is_empty() {
            nested.push(' ');
            nested.push_str(&tail.join(" "));
        }
        return line_contains_cargo_dupes(
            &nested,
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        );
    }

    let Some(next) = parts.next() else {
        return false;
    };

    wrapper_or_command_contains_cargo_dupes(
        next,
        &mut parts,
        current,
        root,
        visiting,
        want_exclude_tests,
        current_cutoff,
        root_cutoff,
    )
}

fn shell_wrapper_contains_cargo_dupes<'a, I>(
    parts: std::iter::Peekable<I>,
    _current: &ParsedShellScript<'_>,
    _root: &ParsedShellScript<'_>,
    _visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    _current_cutoff: usize,
    _root_cutoff: usize,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut parts = parts;
    let mut script = None;

    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if let Some(cluster) = parse_shell_short_flag_cluster(flag, &mut parts) {
            match cluster {
                ShellShortFlagCluster::Valid {
                    script: Some(script_value),
                } => {
                    script = Some(script_value);
                    break;
                }
                ShellShortFlagCluster::Valid { script: None } => continue,
                ShellShortFlagCluster::Invalid => return false,
            }
        }
        if let Some((flag_name, _)) = flag.split_once('=')
            && shell_flag_takes_value(flag_name)
        {
            if flag_name == "-c" {
                script = flag.split_once('=').map(|(_, value)| value.to_owned());
                break;
            }
            continue;
        }
        if shell_flag_allowed_without_value(flag) {
            continue;
        }
        if shell_flag_takes_value(flag) {
            let value = parts.next().unwrap_or_default();
            if flag == "-c" {
                script = Some(value.to_owned());
                break;
            }
            continue;
        }
        return false;
    }

    let script = script.or_else(|| parts.next().map(str::to_owned));
    let Some(script) = script else {
        return false;
    };

    let parsed = parse_script(&script);
    script_contains_cargo_dupes(&parsed, want_exclude_tests)
}

fn command_wrapper_contains_cargo_dupes<'a, I>(
    parts: std::iter::Peekable<I>,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut parts = parts;
    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) || matches!(flag, "-v" | "-V") {
            return false;
        }
        if flag == "--" {
            break;
        }
        if flag != "-p" {
            return false;
        }
    }

    let Some(next) = parts.next() else {
        return false;
    };

    wrapper_or_command_contains_cargo_dupes(
        next,
        &mut parts,
        current,
        root,
        visiting,
        want_exclude_tests,
        current_cutoff,
        root_cutoff,
    )
}

fn exec_wrapper_contains_cargo_dupes<'a, I>(
    parts: std::iter::Peekable<I>,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut parts = parts;
    while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            break;
        }
        if let Some((flag_name, _)) = flag.split_once('=')
            && exec_flag_takes_value(flag_name)
        {
            continue;
        }
        if exec_flag_allowed_without_value(flag) {
            continue;
        }
        if exec_flag_takes_value(flag) {
            let _ = parts.next();
            continue;
        }
        return false;
    }

    let Some(next) = parts.next() else {
        return false;
    };

    wrapper_or_command_contains_cargo_dupes(
        next,
        &mut parts,
        current,
        root,
        visiting,
        want_exclude_tests,
        current_cutoff,
        root_cutoff,
    )
}

fn wrapper_or_command_contains_cargo_dupes<'a, I>(
    token: &'a str,
    parts: &mut std::iter::Peekable<I>,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    visiting: &mut Vec<String>,
    want_exclude_tests: bool,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let command_name = normalize_command_token(token);
    match command_name {
        "cargo" => cargo_dupes_subcommand_invocation(parts, want_exclude_tests),
        "cargo-dupes" => cargo_dupes_binary_invocation(parts, want_exclude_tests),
        "sh" | "bash" => shell_wrapper_contains_cargo_dupes(
            parts.by_ref().peekable(),
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "command" => command_wrapper_contains_cargo_dupes(
            parts.by_ref().peekable(),
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "exec" => exec_wrapper_contains_cargo_dupes(
            parts.by_ref().peekable(),
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        "env" => env_wrapper_contains_cargo_dupes(
            parts.by_ref().peekable(),
            current,
            root,
            visiting,
            want_exclude_tests,
            current_cutoff,
            root_cutoff,
        ),
        _ => false,
    }
}

fn cargo_dupes_subcommand_invocation<'a, I>(
    parts: &mut std::iter::Peekable<I>,
    want_exclude_tests: bool,
) -> bool
where
    I: Iterator<Item = &'a str>,
{
    if matches!(parts.peek(), Some(token) if token.starts_with('+')) {
        let _ = parts.next();
    }

    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }

        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if flag == "--" {
            return false;
        }
        if let Some((flag_name, _)) = flag.split_once('=')
            && cargo_global_flag_takes_value(flag_name)
        {
            continue;
        }
        if matches!(flag.strip_prefix("-j"), Some(value) if !value.is_empty()) {
            continue;
        }
        if cargo_global_flag_allowed_without_value(flag) {
            continue;
        }
        if cargo_global_flag_takes_value(flag) {
            let _ = parts.next();
            continue;
        }
        return false;
    }

    if parts.next() != Some("dupes") {
        return false;
    }

    command_has_exact_exclude_tests_flag(parts) == want_exclude_tests
}

fn cargo_dupes_binary_invocation<'a, I>(parts: &mut I, want_exclude_tests: bool) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut saw_subcommand = false;
    let mut saw_exclude_tests = false;

    while let Some(token) = parts.next() {
        if is_help_or_version_flag(token) {
            return false;
        }
        if token == "--" {
            break;
        }
        if !saw_subcommand {
            if token.starts_with('-') {
                return false;
            }
            saw_subcommand = true;
            continue;
        }
        if let Some((flag_name, _)) = token.split_once('=')
            && cargo_dupes_flag_takes_value(flag_name)
        {
            continue;
        }
        if cargo_dupes_flag_takes_value(token) {
            let _ = parts.next();
            continue;
        }
        if token.starts_with('-') && token != "--exclude-tests" {
            return false;
        }
        if token == "--exclude-tests" {
            saw_exclude_tests = true;
        }
    }

    saw_subcommand && saw_exclude_tests == want_exclude_tests
}

fn command_has_exact_exclude_tests_flag<'a, I>(parts: &mut I) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut saw_exclude_tests = false;

    while let Some(token) = parts.next() {
        if is_help_or_version_flag(token) {
            return false;
        }
        if token == "--" {
            break;
        }
        if let Some((flag_name, _)) = token.split_once('=')
            && cargo_dupes_flag_takes_value(flag_name)
        {
            continue;
        }
        if cargo_dupes_flag_takes_value(token) {
            let _ = parts.next();
            continue;
        }
        if token.starts_with('-') && token != "--exclude-tests" {
            return false;
        }
        if token == "--exclude-tests" {
            saw_exclude_tests = true;
        }
    }

    saw_exclude_tests
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandSegment {
    text: String,
    operator_before: Option<&'static str>,
    operator_after: Option<&'static str>,
}

fn split_command_segments(raw: &str) -> Vec<CommandSegment> {
    let mut line = strip_inline_comment(raw).trim();

    if let Some(stripped) = line.strip_prefix("if ") {
        line = stripped.trim();
    }
    line = line.strip_suffix("; then").unwrap_or(line).trim();
    line = line.strip_suffix("then").unwrap_or(line).trim();

    let pieces = split_unquoted_commands(line);
    let trailing_operator = trailing_control_operator(line);
    pieces
        .iter()
        .enumerate()
        .map(|(index, (segment, operator_before))| {
            let mut segment = segment.trim();
            if let Some(stripped) = segment.strip_prefix("then ") {
                segment = stripped.trim();
            }
            segment = segment
                .trim_end_matches(|c: char| c == ';' || c == ' ')
                .trim();
            if let Some(stripped) = segment.strip_suffix(" fi") {
                segment = stripped.trim();
            }

            CommandSegment {
                text: normalize_segment_text(segment),
                operator_before: *operator_before,
                operator_after: pieces.get(index + 1).and_then(|(_, op)| *op).or_else(|| {
                    (index + 1 == pieces.len())
                        .then_some(trailing_operator)
                        .flatten()
                }),
            }
        })
        .filter(|segment| !segment.text.is_empty())
        .collect()
}

fn normalize_segment_text(segment: &str) -> String {
    let mut segment = segment
        .trim_matches(|c: char| c == '{' || c == '}' || c == ';' || c == '&' || c == '|')
        .trim();

    if segment.starts_with('(') && segment.ends_with(')') && !segment.contains("$(") {
        segment = segment.trim_start_matches('(').trim_end_matches(')').trim();
    }

    segment.to_owned()
}

fn split_unquoted_commands(line: &str) -> Vec<(&str, Option<&'static str>)> {
    let mut segments = Vec::new();
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut command_substitution_depth = 0usize;
    let mut start = 0usize;
    let mut operator_before = None;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let (idx, ch) = chars[i];
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '$' if !single_quoted && !double_quoted => {
                if chars.get(i + 1).is_some_and(|(_, next)| *next == '(') {
                    command_substitution_depth += 1;
                    i += 1;
                }
            }
            ')' if !single_quoted && !double_quoted && command_substitution_depth > 0 => {
                command_substitution_depth -= 1;
            }
            ';' if !single_quoted && !double_quoted && command_substitution_depth == 0 => {
                if start < idx {
                    segments.push((line[start..idx].trim(), operator_before));
                }
                operator_before = Some(";");
                start = idx + ch.len_utf8();
            }
            '&' if !single_quoted && !double_quoted && command_substitution_depth == 0 => {
                let next_is_ampersand = chars.get(i + 1).is_some_and(|(_, next)| *next == '&');
                if start < idx {
                    segments.push((line[start..idx].trim(), operator_before));
                }
                operator_before = Some(if next_is_ampersand { "&&" } else { "&" });
                let next_idx = if next_is_ampersand {
                    chars[i + 1].0
                } else {
                    idx
                };
                start = next_idx + 1;
                if next_is_ampersand {
                    i += 1;
                }
            }
            '|' if !single_quoted && !double_quoted && command_substitution_depth == 0 => {
                let next_is_pipe = chars.get(i + 1).is_some_and(|(_, next)| *next == '|');
                if start < idx {
                    segments.push((line[start..idx].trim(), operator_before));
                }
                operator_before = Some(if next_is_pipe { "||" } else { "|" });
                let next_idx = if next_is_pipe { chars[i + 1].0 } else { idx };
                start = next_idx + 1;
                if next_is_pipe {
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    if start < line.len() {
        segments.push((line[start..].trim(), operator_before));
    }

    segments
}

fn trailing_control_operator(line: &str) -> Option<&'static str> {
    let trimmed = line.trim_end();
    if trimmed.ends_with("&&") || trimmed.ends_with("||") {
        return None;
    }
    if trimmed.ends_with('&') {
        return Some("&");
    }
    if trimmed.ends_with('|') {
        return Some("|");
    }
    None
}

fn constant_exit_status(segment: &str) -> Option<bool> {
    let mut segment = segment.trim().trim_end_matches(';').trim();
    let mut negated = false;

    while let Some(stripped) = segment.strip_prefix('!') {
        negated = !negated;
        segment = stripped.trim();
    }

    segment = segment.trim_matches(|c: char| c == '(' || c == ')' || c == '{' || c == '}');

    let status = match segment {
        "true" | ":" => Some(true),
        "false" => Some(false),
        value if value.starts_with("exit ") => Some(value.split_whitespace().nth(1) == Some("0")),
        _ => None,
    }?;

    Some(if negated { !status } else { status })
}

fn is_terminal_exit(segment: &str) -> bool {
    let mut segment = segment.trim().trim_end_matches(';').trim();
    while let Some(stripped) = segment.strip_prefix('!') {
        segment = stripped.trim();
    }
    segment = segment.trim_matches(|c: char| c == '(' || c == ')' || c == '{' || c == '}');
    segment.starts_with("exit ")
}

fn cargo_global_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--config"
            | "-Z"
            | "--manifest-path"
            | "--color"
            | "--target"
            | "--target-dir"
            | "--jobs"
            | "-j"
            | "-C"
    )
}

fn cargo_global_flag_allowed_without_value(flag: &str) -> bool {
    matches!(
        flag,
        "-q" | "--quiet" | "-v" | "--verbose" | "--frozen" | "--locked" | "--offline"
    )
}

fn cargo_dupes_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "--max-exact" | "--max-exact-percent")
}

fn shell_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-c" | "-o" | "-O")
}

fn shell_flag_allowed_without_value(flag: &str) -> bool {
    matches!(
        flag,
        "-e" | "-i"
            | "-l"
            | "-n"
            | "-r"
            | "-s"
            | "-u"
            | "-v"
            | "-x"
            | "--login"
            | "--noprofile"
            | "--norc"
            | "--posix"
            | "--restricted"
            | "--verbose"
    )
}

fn env_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "-C" | "--chdir" | "-S" | "--split-string" | "-u" | "--unset"
    )
}

fn env_flag_allowed_without_value(flag: &str) -> bool {
    matches!(flag, "-i" | "--ignore-environment")
}

fn exec_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "-a")
}

fn exec_flag_allowed_without_value(flag: &str) -> bool {
    matches!(flag, "-c" | "-l")
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ShellShortFlagCluster {
    Valid { script: Option<String> },
    Invalid,
}

fn parse_shell_short_flag_cluster<'a, I>(
    flag: &str,
    parts: &mut std::iter::Peekable<I>,
) -> Option<ShellShortFlagCluster>
where
    I: Iterator<Item = &'a str>,
{
    if flag.starts_with("--") || !flag.starts_with('-') || flag.len() <= 2 {
        return None;
    }

    let short_flags = &flag[1..];
    let mut chars = short_flags.char_indices().peekable();

    while let Some((idx, ch)) = chars.next() {
        match ch {
            'e' | 'i' | 'l' | 'n' | 'r' | 's' | 'u' | 'v' | 'x' => {}
            'c' | 'o' | 'O' => {
                let remainder = &short_flags[idx + 1..];
                if ch == 'c' {
                    if remainder.is_empty() {
                        return Some(ShellShortFlagCluster::Valid {
                            script: Some(parts.next().unwrap_or_default().to_owned()),
                        });
                    }
                    return Some(ShellShortFlagCluster::Valid {
                        script: Some(remainder.to_owned()),
                    });
                }

                if remainder.is_empty() {
                    let _ = parts.next();
                }
                return Some(ShellShortFlagCluster::Valid { script: None });
            }
            _ => return Some(ShellShortFlagCluster::Invalid),
        }
    }

    Some(ShellShortFlagCluster::Valid { script: None })
}

fn looks_like_env_assignment(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte == b'_' || byte.is_ascii_alphanumeric())
}

fn split_string_is_assignment_only(payload: &str) -> bool {
    let tokens = shell_words(payload);
    let Some(first) = tokens.first() else {
        return false;
    };
    looks_like_env_assignment(first)
        && !tokens.iter().any(|token| {
            matches!(
                normalize_command_token(token),
                "cargo" | "cargo-dupes" | "env"
            )
        })
}

fn token_is_shadowed_function(
    token: &str,
    command_name: &str,
    current: &ParsedShellScript<'_>,
    root: &ParsedShellScript<'_>,
    current_cutoff: usize,
    root_cutoff: usize,
) -> bool {
    if token.contains('/') {
        return false;
    }

    current
        .functions
        .iter()
        .any(|function| function.name == command_name && function.line_no <= current_cutoff)
        || root
            .functions
            .iter()
            .any(|function| function.name == command_name && function.line_no <= root_cutoff)
}

fn shell_words(command_text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = command_text.chars().peekable();
    let mut single_quoted = false;
    let mut double_quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '\\' if !single_quoted => {
                if matches!(chars.peek(), Some('\n')) {
                    let _ = chars.next();
                } else if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ch if ch.is_whitespace() && !single_quoted && !double_quoted => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

fn extract_command_substitutions(line: &str) -> Vec<String> {
    let mut substitutions = Vec::new();
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut depth = 0usize;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;
    let mut start = None;
    let mut escaped = false;

    while i < chars.len() {
        let (idx, ch) = chars[i];

        if escaped {
            escaped = false;
            i += 1;
            continue;
        }

        match ch {
            '\\' if !single_quoted => {
                escaped = true;
            }
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '$' if !single_quoted => {
                if chars.get(i + 1).is_some_and(|(_, next)| *next == '(') {
                    if depth == 0 {
                        start = chars.get(i + 2).map(|(next_idx, _)| *next_idx);
                    }
                    depth += 1;
                    i += 1;
                }
            }
            ')' if !single_quoted && depth > 0 => {
                depth -= 1;
                if depth == 0 {
                    if let Some(start_idx) = start.take() {
                        substitutions.push(line[start_idx..idx].trim().to_owned());
                    }
                }
            }
            _ => {}
        }

        i += 1;
    }

    substitutions
}

fn strip_inline_comment(line: &str) -> &str {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut prev_was_whitespace = true;

    for (index, ch) in line.char_indices() {
        match ch {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            '#' if !single_quoted && !double_quoted && prev_was_whitespace => {
                return &line[..index];
            }
            _ => {}
        }
        prev_was_whitespace = ch.is_whitespace();
    }

    line
}

#[cfg(test)]
pub(super) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = test_support::parsed_hook(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "hook_rs_13_cargo_dupes_excludes_tests/mod.rs"]
mod hook_rs_13_cargo_dupes_excludes_tests;
