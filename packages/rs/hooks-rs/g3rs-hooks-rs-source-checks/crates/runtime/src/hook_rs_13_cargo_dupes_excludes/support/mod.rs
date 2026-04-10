pub(super) mod helpers;

use hook_shell_parser::{ParsedShellScript, parse_script};

use self::helpers::*;

pub(crate) fn env_wrapper_contains_cargo_dupes<'a, I>(
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
            return super::line_contains_cargo_dupes(
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
        return super::line_contains_cargo_dupes(
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

pub(crate) fn shell_wrapper_contains_cargo_dupes<'a, I>(
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
    super::script_contains_cargo_dupes(&parsed, want_exclude_tests)
}

pub(crate) fn command_wrapper_contains_cargo_dupes<'a, I>(
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

pub(crate) fn exec_wrapper_contains_cargo_dupes<'a, I>(
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

pub(crate) fn cargo_dupes_subcommand_invocation<'a, I>(
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

pub(crate) fn cargo_dupes_binary_invocation<'a, I>(parts: &mut I, want_exclude_tests: bool) -> bool
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
