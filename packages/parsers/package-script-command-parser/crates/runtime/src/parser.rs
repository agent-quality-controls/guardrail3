use package_script_command_parser_types::document::{
    EslintInvocation, PackageScriptCommand, PackageScriptCommandDocument,
    PackageScriptCommandSeparator, PackageScriptParseFact, PackageScriptParseState,
    PackageScriptToolInvocation,
};
use tree_sitter::{Node, Parser};

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

#[must_use]
pub fn has_safe_tool_invocation(
    facts: &[PackageScriptParseFact],
    executable: &str,
    first_arg: &str,
) -> bool {
    facts
        .iter()
        .all(|fact| !fact_has_unsupported_target_invocation(fact, executable, first_arg))
        && facts
            .iter()
            .all(|fact| !fact_has_unsafe_tool_invocation(fact, executable, first_arg))
        && facts.iter().any(|fact| {
            !fact_has_or_separator(fact)
                && fact.tool_invocations.iter().any(|invocation| {
                    invocation_targets_tool(invocation, executable, first_arg)
                        && safe_tool_invocation_position(
                            invocation.preceded_by,
                            invocation.followed_by,
                        )
                })
        })
}

fn fact_has_unsupported_target_invocation(
    fact: &PackageScriptParseFact,
    executable: &str,
    first_arg: &str,
) -> bool {
    matches!(
        fact.state,
        PackageScriptParseState::Unsupported { .. } | PackageScriptParseState::ParseError { .. }
    ) && fact
        .all_tool_invocations
        .iter()
        .any(|invocation| invocation_targets_tool(invocation, executable, first_arg))
}

fn fact_has_or_separator(fact: &PackageScriptParseFact) -> bool {
    fact.commands
        .iter()
        .any(|command| command.preceded_by == Some(PackageScriptCommandSeparator::Or))
}

fn fact_has_unsafe_tool_invocation(
    fact: &PackageScriptParseFact,
    executable: &str,
    first_arg: &str,
) -> bool {
    fact.tool_invocations
        .iter()
        .any(|invocation| invocation_targets_tool(invocation, executable, first_arg))
        && (fact_has_or_separator(fact)
            || fact.tool_invocations.iter().any(|invocation| {
                invocation_targets_tool(invocation, executable, first_arg)
                    && !safe_tool_invocation_position(
                        invocation.preceded_by,
                        invocation.followed_by,
                    )
            }))
}

fn invocation_targets_tool(
    invocation: &PackageScriptToolInvocation,
    executable: &str,
    first_arg: &str,
) -> bool {
    invocation.executable == executable
        && invocation.args.first().is_some_and(|arg| arg == first_arg)
}

fn normalize_fact(script_name: &str, input: &str) -> PackageScriptParseFact {
    let parsed = parse_script_commands(input);
    let (commands, all_commands, state) = match parsed {
        Ok(parsed) => {
            let state = command_state(script_name, &parsed);
            (parsed.commands, parsed.all_commands, state)
        }
        Err(reason) if script_name_is_guardrail_related(script_name) => (
            Vec::new(),
            Vec::new(),
            PackageScriptParseState::ParseError { reason },
        ),
        Err(_reason) => (
            Vec::new(),
            Vec::new(),
            PackageScriptParseState::NoEslintInvocation,
        ),
    };
    let visible_tool_invocations = tool_invocations(script_name, &commands);
    let all_tool_invocations = tool_invocations(script_name, &all_commands);

    PackageScriptParseFact {
        script_name: script_name.to_owned(),
        commands,
        tool_invocations: visible_tool_invocations,
        all_tool_invocations,
        state,
    }
}

fn command_state(script_name: &str, parsed: &ParsedScriptCommands) -> PackageScriptParseState {
    let guardrail_related = script_name_is_guardrail_related(script_name)
        || parsed.commands.iter().any(command_has_guardrail_tool)
        || parsed.any_command_has_guardrail_tool;

    if guardrail_related && parsed.has_parse_error {
        return PackageScriptParseState::ParseError {
            reason: "script command contains invalid shell syntax".to_owned(),
        };
    }

    if guardrail_related && parsed.has_unsupported_guardrail_syntax {
        return PackageScriptParseState::Unsupported {
            reason: "guardrail-related script uses unsupported shell syntax".to_owned(),
        };
    }

    let mut eslint_invocations = Vec::new();
    for (command_index, command) in parsed.commands.iter().enumerate() {
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
            commands: parsed.commands.clone(),
            eslint_invocations,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedScriptCommands {
    commands: Vec<PackageScriptCommand>,
    all_commands: Vec<PackageScriptCommand>,
    any_command_has_guardrail_tool: bool,
    has_parse_error: bool,
    has_unsupported_guardrail_syntax: bool,
}

fn parse_script_commands(input: &str) -> Result<ParsedScriptCommands, String> {
    let tree = parse_bash_tree(input)?;
    let root = tree.root_node();
    let mut top_level_commands = Vec::new();
    let mut all_commands = Vec::new();
    collect_command_nodes(root, &mut top_level_commands, &mut all_commands);

    let commands = top_level_commands
        .iter()
        .enumerate()
        .filter_map(|(index, command_node)| {
            let preceded_by = index
                .checked_sub(1)
                .and_then(|previous_index| top_level_commands.get(previous_index))
                .and_then(|previous| separator_between(input, *previous, *command_node));
            package_command_from_node(input, *command_node, preceded_by)
        })
        .collect::<Vec<_>>();
    let all_commands = all_commands
        .iter()
        .filter_map(|command_node| package_command_from_node(input, *command_node, None))
        .collect::<Vec<_>>();

    let any_command_has_guardrail_tool = all_commands.iter().any(|command| {
        normalized_tool(command).is_some_and(|(executable, _args)| {
            matches!(
                executable.as_str(),
                "eslint" | "astro" | "syncpack" | "only-allow" | "cspell" | "type-coverage"
            )
        })
    });

    Ok(ParsedScriptCommands {
        commands,
        all_commands,
        any_command_has_guardrail_tool,
        has_parse_error: root.has_error(),
        has_unsupported_guardrail_syntax: has_unsupported_ast_shape(root)
            || has_unsupported_command_separators(input, &top_level_commands)
            || has_unsupported_trailing_operator(input, &top_level_commands),
    })
}

fn parse_bash_tree(input: &str) -> Result<tree_sitter::Tree, String> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_bash::LANGUAGE.into())
        .map_err(|err| format!("failed to load Bash parser: {err}"))?;
    parser
        .parse(input, None)
        .ok_or_else(|| "Bash parser did not return a parse tree".to_owned())
}

fn collect_command_nodes<'tree>(
    node: Node<'tree>,
    top_level_commands: &mut Vec<Node<'tree>>,
    all_commands: &mut Vec<Node<'tree>>,
) {
    if node.kind() == "command" {
        all_commands.push(node);
        if !has_non_top_level_command_ancestor(node) {
            top_level_commands.push(node);
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_command_nodes(child, top_level_commands, all_commands);
    }
}

fn has_non_top_level_command_ancestor(node: Node<'_>) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if matches!(
            parent.kind(),
            "command"
                | "command_substitution"
                | "process_substitution"
                | "subshell"
                | "function_definition"
                | "if_statement"
                | "while_statement"
                | "for_statement"
                | "c_style_for_statement"
                | "case_statement"
        ) {
            return true;
        }
        current = parent.parent();
    }
    false
}

fn package_command_from_node(
    input: &str,
    command_node: Node<'_>,
    preceded_by: Option<PackageScriptCommandSeparator>,
) -> Option<PackageScriptCommand> {
    let name_node = command_node.child_by_field_name("name")?;
    let invocation = node_text(input, command_node)?.trim().to_owned();
    let executable = shell_word_text(input, name_node)?;
    let args = command_arguments(input, command_node);
    let args = strip_env_assignments(args);

    Some(PackageScriptCommand {
        invocation,
        executable,
        args,
        preceded_by,
    })
}

fn command_arguments(input: &str, command_node: Node<'_>) -> Vec<String> {
    let mut args = Vec::new();
    let mut cursor = command_node.walk();
    for child in command_node.children(&mut cursor) {
        if child.kind() == "command_name"
            || child.kind() == "variable_assignment"
            || child.kind().contains("redirect")
        {
            continue;
        }
        if let Some(text) = shell_word_text(input, child) {
            args.push(text);
        }
    }
    args
}

fn shell_word_text(input: &str, node: Node<'_>) -> Option<String> {
    let text = node_text(input, node)?.trim();
    if text.is_empty() {
        return None;
    }
    Some(strip_balanced_shell_quotes(text))
}

fn node_text<'input>(input: &'input str, node: Node<'_>) -> Option<&'input str> {
    node.utf8_text(input.as_bytes()).ok()
}

fn strip_balanced_shell_quotes(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let Some(last) = value.chars().last() else {
        return String::new();
    };
    let char_count = value.chars().count();
    if char_count >= 2 && matches!((first, last), ('\'', '\'') | ('"', '"')) {
        return value.chars().skip(1).take(char_count - 2).collect();
    }
    value.to_owned()
}

fn separator_between(
    input: &str,
    previous: Node<'_>,
    current: Node<'_>,
) -> Option<PackageScriptCommandSeparator> {
    let between = bytes_between(input, previous.end_byte(), current.start_byte())?;
    let compact = between
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>();
    match compact.as_str() {
        "&&" => Some(PackageScriptCommandSeparator::And),
        "||" => Some(PackageScriptCommandSeparator::Or),
        _ => None,
    }
}

fn bytes_between(input: &str, start: usize, end: usize) -> Option<&str> {
    if start > end {
        return None;
    }
    input.get(start..end)
}

fn has_unsupported_command_separators(input: &str, commands: &[Node<'_>]) -> bool {
    commands.windows(2).any(|pair| {
        let Some(previous) = pair.first() else {
            return false;
        };
        let Some(current) = pair.get(1) else {
            return false;
        };
        let Some(between) = bytes_between(input, previous.end_byte(), current.start_byte()) else {
            return true;
        };
        let trimmed = between.trim();
        let compact = trimmed
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>();
        !matches!(compact.as_str(), "&&" | "||")
    })
}

fn has_unsupported_trailing_operator(input: &str, commands: &[Node<'_>]) -> bool {
    let Some(last_command) = commands.last() else {
        return false;
    };
    let Some(after) = bytes_between(input, last_command.end_byte(), input.len()) else {
        return true;
    };
    let compact = after
        .trim()
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>();

    !compact.is_empty()
}

fn has_unsupported_ast_shape(node: Node<'_>) -> bool {
    if matches!(
        node.kind(),
        "command_substitution"
            | "process_substitution"
            | "pipeline"
            | "file_redirect"
            | "herestring_redirect"
            | "heredoc_redirect"
            | "redirected_statement"
            | "subshell"
            | "function_definition"
            | "if_statement"
            | "while_statement"
            | "for_statement"
            | "c_style_for_statement"
            | "case_statement"
            | "simple_expansion"
            | "expansion"
    ) {
        return true;
    }

    let mut cursor = node.walk();
    node.children(&mut cursor).any(has_unsupported_ast_shape)
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

fn command_has_guardrail_tool(command: &PackageScriptCommand) -> bool {
    normalized_tool(command).is_some_and(|(executable, _args)| {
        matches!(
            executable.as_str(),
            "eslint" | "astro" | "syncpack" | "only-allow"
        )
    })
}

fn tool_invocations(
    script_name: &str,
    commands: &[PackageScriptCommand],
) -> Vec<PackageScriptToolInvocation> {
    commands
        .iter()
        .enumerate()
        .filter_map(|(index, command)| {
            let (executable, args) = normalized_tool(command)?;
            Some(PackageScriptToolInvocation {
                script_name: script_name.to_owned(),
                command_index: index,
                invocation: command.invocation.clone(),
                executable,
                args,
                preceded_by: command.preceded_by,
                followed_by: commands
                    .get(index.saturating_add(1))
                    .and_then(|next| next.preceded_by),
            })
        })
        .collect()
}

fn safe_tool_invocation_position(
    preceded_by: Option<PackageScriptCommandSeparator>,
    followed_by: Option<PackageScriptCommandSeparator>,
) -> bool {
    matches!(preceded_by, None | Some(PackageScriptCommandSeparator::And))
        && matches!(followed_by, None | Some(PackageScriptCommandSeparator::And))
}

fn normalized_tool(command: &PackageScriptCommand) -> Option<(String, Vec<String>)> {
    let executable = executable_name(&command.executable);
    match executable.as_str() {
        "env" | "cross-env" => normalized_env_tool(&command.args),
        "npm" => normalized_package_manager_tool(&command.args, PnpmMode::Npm),
        "pnpm" => normalized_package_manager_tool(&command.args, PnpmMode::Pnpm),
        "yarn" => normalized_package_manager_tool(&command.args, PnpmMode::Yarn),
        "bun" => normalized_package_manager_tool(&command.args, PnpmMode::Bun),
        "npx" | "bunx" => normalized_package_runner_tool(&command.args),
        _ => Some((executable, command.args.clone())),
    }
}

fn eslint_args_from_command(command: &PackageScriptCommand) -> Option<Vec<String>> {
    let (executable, args) = normalized_tool(command)?;
    if executable == "eslint" {
        Some(args)
    } else {
        None
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

fn normalized_package_manager_tool(
    args: &[String],
    mode: PnpmMode,
) -> Option<(String, Vec<String>)> {
    let mut idx = 0usize;
    while idx < args.len() {
        let arg = args.get(idx)?;
        if mode.allows_exec() && (arg == "exec" || arg == "x" || arg == "dlx") {
            let command_idx = if args.get(idx + 1).is_some_and(|next| next == "--") {
                idx + 2
            } else {
                idx + 1
            };
            let executable = executable_name(args.get(command_idx)?);
            return Some((
                executable,
                args.iter().skip(command_idx + 1).cloned().collect(),
            ));
        }
        if arg == "run" || arg == "run-script" {
            let script_idx = if args.get(idx + 1).is_some_and(|next| next == "--") {
                idx + 2
            } else {
                idx + 1
            };
            return Some((
                "package-script".to_owned(),
                args.iter().skip(script_idx).cloned().collect(),
            ));
        }
        if mode.allows_direct_tool() && !arg.starts_with('-') {
            return Some((
                executable_name(arg),
                args.iter().skip(idx + 1).cloned().collect(),
            ));
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
    fn allows_direct_tool(self) -> bool {
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

fn normalized_package_runner_tool(args: &[String]) -> Option<(String, Vec<String>)> {
    let mut idx = 0usize;
    while idx < args.len() {
        let arg = args.get(idx)?;
        if arg == "--" {
            idx = idx.saturating_add(1);
            continue;
        }
        if !arg.starts_with('-') {
            return Some((
                executable_name(arg),
                args.iter().skip(idx + 1).cloned().collect(),
            ));
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

fn normalized_env_tool(args: &[String]) -> Option<(String, Vec<String>)> {
    let tokens = strip_env_assignments(args.to_vec());
    let command = PackageScriptCommand {
        invocation: tokens.join(" "),
        executable: tokens.first()?.clone(),
        args: tokens.iter().skip(1).cloned().collect(),
        preceded_by: None,
    };
    normalized_tool(&command)
}

fn script_name_is_guardrail_related(script_name: &str) -> bool {
    let normalized = script_name.to_ascii_lowercase();
    if matches!(
        normalized.as_str(),
        "check"
            | "precheck"
            | "postcheck"
            | "lint"
            | "prelint"
            | "postlint"
            | "validate"
            | "prevalidate"
            | "postvalidate"
    ) {
        return true;
    }
    normalized.split([':', '-', '_', '.', '/']).any(|token| {
        matches!(
            token,
            "check"
                | "lint"
                | "validate"
                | "eslint"
                | "astro"
                | "syncpack"
                | "spellcheck"
                | "typecov"
        )
    })
}
