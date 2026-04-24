use package_script_command_parser_types::document::{
    EslintInvocation, PackageScriptCommand, PackageScriptCommandDocument,
    PackageScriptCommandSeparator, PackageScriptParseFact, PackageScriptParseState,
};

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized package script command parser"
)]
pub fn parse(
    script_name: &str,
    input: &str,
) -> Result<PackageScriptParseFact, crate::error::Error> {
    Ok(normalize_fact(script_name, input))
}

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized package script command parser"
)]
pub fn parse_document(
    script_name: &str,
    input: &str,
) -> Result<PackageScriptCommandDocument, crate::error::Error> {
    Ok(PackageScriptCommandDocument {
        raw: input.to_owned(),
        typed: normalize_fact(script_name, input),
    })
}

pub fn from_path(
    path: impl AsRef<std::path::Path>,
    script_name: &str,
) -> Result<PackageScriptParseFact, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(script_name, &content)
}

pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
    script_name: &str,
) -> Result<PackageScriptCommandDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(script_name, &content)
}

fn normalize_fact(script_name: &str, input: &str) -> PackageScriptParseFact {
    let state = match parse_commands(input) {
        Ok(commands) => command_state(script_name, input, commands),
        Err(reason) if script_name_is_lint_related(script_name) || raw_has_eslint_token(input) => {
            PackageScriptParseState::ParseError { reason }
        }
        Err(_reason) => PackageScriptParseState::NoEslintInvocation,
    };

    PackageScriptParseFact {
        script_name: script_name.to_owned(),
        state,
    }
}

fn command_state(
    script_name: &str,
    input: &str,
    commands: Vec<PackageScriptCommand>,
) -> PackageScriptParseState {
    let lint_related = script_name_is_lint_related(script_name)
        || commands.iter().any(command_has_eslint)
        || raw_has_eslint_token(input);

    if lint_related && has_unsupported_lint_related_syntax(input) {
        return PackageScriptParseState::Unsupported {
            reason: "lint-related script uses unsupported shell syntax".to_owned(),
        };
    }

    let mut eslint_invocations = Vec::new();
    for (command_index, command) in commands.iter().enumerate() {
        match eslint_invocation(script_name, command_index, command) {
            Ok(Some(invocation)) => eslint_invocations.push(invocation),
            Ok(None) => {}
            Err(reason) => return PackageScriptParseState::ParseError { reason },
        }
    }

    if eslint_invocations.is_empty() {
        PackageScriptParseState::NoEslintInvocation
    } else {
        PackageScriptParseState::Parsed {
            commands,
            eslint_invocations,
        }
    }
}

fn parse_commands(input: &str) -> Result<Vec<PackageScriptCommand>, String> {
    let mut commands = Vec::new();
    for segment in split_segments(input)? {
        let tokens = split_tokens(&segment.invocation)?;
        if tokens.is_empty() {
            continue;
        }
        let tokens = strip_env_assignments(tokens);
        if tokens.is_empty() {
            continue;
        }
        let mut tokens = tokens.into_iter();
        let Some(executable) = tokens.next() else {
            continue;
        };
        let args = tokens.collect::<Vec<_>>();
        commands.push(PackageScriptCommand {
            invocation: segment.invocation,
            executable,
            args,
            preceded_by: segment.preceded_by,
        });
    }
    Ok(commands)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Segment {
    invocation: String,
    preceded_by: Option<PackageScriptCommandSeparator>,
}

fn split_segments(input: &str) -> Result<Vec<Segment>, String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut preceded_by = None;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                current.push(ch);
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                current.push(ch);
            }
            '&' if !in_single_quote && !in_double_quote && chars.peek() == Some(&'&') => {
                let _ = chars.next();
                push_segment(&mut segments, &mut current, preceded_by);
                preceded_by = Some(PackageScriptCommandSeparator::And);
            }
            '|' if !in_single_quote && !in_double_quote && chars.peek() == Some(&'|') => {
                let _ = chars.next();
                push_segment(&mut segments, &mut current, preceded_by);
                preceded_by = Some(PackageScriptCommandSeparator::Or);
            }
            _ => current.push(ch),
        }
    }

    if in_single_quote || in_double_quote {
        return Err("script command contains an unterminated quote".to_owned());
    }

    push_segment(&mut segments, &mut current, preceded_by);
    Ok(segments)
}

fn push_segment(
    segments: &mut Vec<Segment>,
    current: &mut String,
    preceded_by: Option<PackageScriptCommandSeparator>,
) {
    let trimmed = current.trim();
    if !trimmed.is_empty() {
        segments.push(Segment {
            invocation: trimmed.to_owned(),
            preceded_by,
        });
    }
    current.clear();
}

fn split_tokens(segment: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = segment.chars();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\\' if !in_single_quote => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            ch if ch.is_whitespace() && !in_single_quote && !in_double_quote => {
                push_token(&mut tokens, &mut current);
            }
            _ => current.push(ch),
        }
    }

    if in_single_quote || in_double_quote {
        return Err("script command contains an unterminated quote".to_owned());
    }

    push_token(&mut tokens, &mut current);
    Ok(tokens)
}

fn push_token(tokens: &mut Vec<String>, current: &mut String) {
    if !current.is_empty() {
        tokens.push(current.clone());
        current.clear();
    }
}

fn strip_env_assignments(tokens: Vec<String>) -> Vec<String> {
    tokens
        .into_iter()
        .skip_while(|token| is_env_assignment(token))
        .collect()
}

fn is_env_assignment(token: &str) -> bool {
    let Some((key, _value)) = token.split_once('=') else {
        return false;
    };
    !key.is_empty()
        && key.chars().enumerate().all(|(idx, ch)| {
            ch == '_' || ch.is_ascii_alphabetic() || (idx > 0 && ch.is_ascii_digit())
        })
}

fn eslint_invocation(
    script_name: &str,
    command_index: usize,
    command: &PackageScriptCommand,
) -> Result<Option<EslintInvocation>, String> {
    let Some(eslint_args) = eslint_args_from_command(command) else {
        return Ok(None);
    };

    let (ignore_patterns, ignore_path, config_path) = eslint_flags(&eslint_args)?;
    Ok(Some(EslintInvocation {
        script_name: script_name.to_owned(),
        command_index,
        invocation: command.invocation.clone(),
        args: eslint_args,
        ignore_patterns,
        ignore_path,
        config_path,
    }))
}

fn command_has_eslint(command: &PackageScriptCommand) -> bool {
    eslint_args_from_command(command).is_some()
}

fn eslint_args_from_command(command: &PackageScriptCommand) -> Option<Vec<String>> {
    match executable_name(&command.executable).as_str() {
        "eslint" => Some(command.args.clone()),
        "pnpm" => package_manager_eslint_args(&command.args, PnpmMode::Pnpm),
        "npm" => package_manager_eslint_args(&command.args, PnpmMode::Npm),
        "yarn" => package_manager_eslint_args(&command.args, PnpmMode::Yarn),
        "bun" => package_manager_eslint_args(&command.args, PnpmMode::Bun),
        "npx" | "bunx" => package_runner_eslint_args(&command.args),
        "env" | "cross-env" => env_wrapper_eslint_args(&command.args),
        _ => None,
    }
}

fn executable_name(executable: &str) -> String {
    executable
        .rsplit('/')
        .next()
        .unwrap_or(executable)
        .to_owned()
}

fn eslint_flags(args: &[String]) -> Result<(Vec<String>, Option<String>, Option<String>), String> {
    let mut ignore_patterns = Vec::new();
    let mut ignore_path = None;
    let mut config_path = None;
    let mut idx = 0usize;

    while idx < args.len() {
        let Some(arg) = args.get(idx) else {
            break;
        };
        if arg == "--" {
            break;
        }
        if let Some(value) = arg.strip_prefix("--ignore-pattern=") {
            reject_empty_flag_value(value, "--ignore-pattern")?;
            ignore_patterns.push(value.to_owned());
            idx += 1;
            continue;
        }
        if arg == "--ignore-pattern" {
            ignore_patterns.push(required_flag_value(args, idx, "--ignore-pattern")?);
            idx += 2;
            continue;
        }
        if let Some(value) = arg.strip_prefix("--ignore-path=") {
            reject_empty_flag_value(value, "--ignore-path")?;
            ignore_path = Some(value.to_owned());
            idx += 1;
            continue;
        }
        if arg == "--ignore-path" {
            ignore_path = Some(required_flag_value(args, idx, "--ignore-path")?);
            idx += 2;
            continue;
        }
        if let Some(value) = arg.strip_prefix("--config=") {
            reject_empty_flag_value(value, "--config")?;
            config_path = Some(value.to_owned());
            idx += 1;
            continue;
        }
        if arg == "--config" || arg == "-c" {
            config_path = Some(required_flag_value(args, idx, arg)?);
            idx += 2;
            continue;
        }
        idx += 1;
    }

    Ok((ignore_patterns, ignore_path, config_path))
}

fn reject_empty_flag_value(value: &str, flag: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("ESLint flag {flag} is missing a value"));
    }
    Ok(())
}

fn required_flag_value(args: &[String], idx: usize, flag: &str) -> Result<String, String> {
    let Some(value) = args.get(idx + 1) else {
        return Err(format!("ESLint flag {flag} is missing a value"));
    };
    if value.starts_with('-') {
        return Err(format!("ESLint flag {flag} is missing a value"));
    }
    Ok(value.clone())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PnpmMode {
    Pnpm,
    Npm,
    Yarn,
    Bun,
}

fn package_manager_eslint_args(args: &[String], mode: PnpmMode) -> Option<Vec<String>> {
    let mut idx = 0usize;
    while idx < args.len() {
        let arg = args.get(idx)?;
        if mode.allows_direct_eslint() && arg == "eslint" {
            return Some(args.iter().skip(idx + 1).cloned().collect());
        }
        if mode.allows_exec() && (arg == "exec" || arg == "x" || arg == "dlx") {
            let command_idx = if args.get(idx + 1).is_some_and(|next| next == "--") {
                idx + 2
            } else {
                idx + 1
            };
            if args.get(command_idx).is_some_and(|next| next == "eslint") {
                return Some(args.iter().skip(command_idx + 1).cloned().collect());
            }
        }
        let width = package_manager_wrapper_arg_width(args, idx, mode);
        if width == 1 {
            return None;
        }
        idx += width;
    }
    None
}

impl PnpmMode {
    fn allows_direct_eslint(self) -> bool {
        matches!(self, Self::Pnpm | Self::Yarn | Self::Bun)
    }

    fn allows_exec(self) -> bool {
        matches!(self, Self::Pnpm | Self::Npm | Self::Bun)
    }
}

fn package_manager_wrapper_arg_width(args: &[String], idx: usize, mode: PnpmMode) -> usize {
    let Some(arg) = args.get(idx) else {
        return 1;
    };
    let pnpm_flag = arg == "--filter" || arg == "-F" || arg == "--dir" || arg == "-C";
    let npm_flag = arg == "--workspace" || arg == "-w" || arg == "--prefix";
    let yarn_flag = arg == "--cwd";
    if matches!(mode, PnpmMode::Pnpm) && pnpm_flag
        || matches!(mode, PnpmMode::Npm) && npm_flag
        || matches!(mode, PnpmMode::Yarn) && yarn_flag
        || matches!(mode, PnpmMode::Bun) && (pnpm_flag || yarn_flag)
    {
        2
    } else {
        1
    }
}

fn package_runner_eslint_args(args: &[String]) -> Option<Vec<String>> {
    let mut idx = 0usize;
    while idx < args.len() {
        let arg = args.get(idx)?;
        if arg == "eslint" {
            return Some(args.iter().skip(idx + 1).cloned().collect());
        }
        idx += package_runner_arg_width(args, idx);
    }
    None
}

fn package_runner_arg_width(args: &[String], idx: usize) -> usize {
    let Some(arg) = args.get(idx) else {
        return 1;
    };
    if arg == "--package" || arg == "-p" {
        2
    } else {
        1
    }
}

fn env_wrapper_eslint_args(args: &[String]) -> Option<Vec<String>> {
    let tokens = strip_env_assignments(args.to_vec());
    let command = PackageScriptCommand {
        invocation: tokens.join(" "),
        executable: tokens.first()?.clone(),
        args: tokens.iter().skip(1).cloned().collect(),
        preceded_by: None,
    };
    eslint_args_from_command(&command)
}

fn has_unsupported_lint_related_syntax(input: &str) -> bool {
    let mut chars = input.chars().peekable();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            ';' | '<' | '>' | '`' if !in_single_quote && !in_double_quote => return true,
            '$' if !in_single_quote => return true,
            '&' if !in_single_quote && !in_double_quote => {
                if chars.peek() == Some(&'&') {
                    let _ = chars.next();
                } else {
                    return true;
                }
            }
            '|' if !in_single_quote && !in_double_quote => {
                if chars.peek() == Some(&'|') {
                    let _ = chars.next();
                } else {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn script_name_is_lint_related(script_name: &str) -> bool {
    let normalized = script_name.to_ascii_lowercase();
    if normalized == "prelint" || normalized == "postlint" {
        return true;
    }
    normalized
        .split([':', '-', '_', '.', '/'])
        .any(|token| token == "lint" || token == "eslint")
}

fn raw_has_eslint_token(input: &str) -> bool {
    input
        .split(|ch: char| {
            ch.is_whitespace()
                || matches!(
                    ch,
                    '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | '&' | '|' | ';' | '<' | '>'
                )
        })
        .any(|token| executable_name(token) == "eslint")
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
