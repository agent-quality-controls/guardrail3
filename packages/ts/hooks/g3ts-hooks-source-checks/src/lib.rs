use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookCriticalCommand, G3TsHookTriggerPattern,
};
use g3ts_hooks_types::{G3TsHookScriptKind, G3TsHooksSourceChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{
    ResolvedCommand, any_resolved_command, any_resolved_command_on_line_in_context,
};
use hook_shell_parser::types::{ParsedShellScript, ShellFunction};

#[must_use]
pub fn check(input: &G3TsHooksSourceChecksInput) -> Vec<G3CheckResult> {
    check_effective(std::slice::from_ref(input))
}

#[must_use]
pub fn check_effective(inputs: &[G3TsHooksSourceChecksInput]) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let Some(primary) = inputs.first() else {
        return results;
    };
    g3ts_validate_staged_present(inputs, primary, &mut results);
    app_validate_step_present(inputs, primary, &mut results);
    guardrail_config_changes_trigger_validation(inputs, primary, &mut results);
    contract_trigger_patterns_covered(inputs, primary, &mut results);
    for input in inputs {
        no_fail_open_wrappers(input, &mut results);
        dispatcher_inventory(input, &mut results);
    }
    results
}

fn g3ts_validate_staged_present(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if app_roots(primary).iter().all(|app_root| {
        any_script_command(inputs, |command| {
            is_g3ts_validate_path_command(command, app_root)
        })
    }) {
        return;
    }
    results.push(error(
        "g3ts-hooks/g3ts-validate-staged-present",
        "pre-commit hook does not run g3ts validate",
        "The selected pre-commit hook must execute `g3ts validate --path ...` so TypeScript guardrails run before commits. Echoed text, comments, and aliases are not enough.",
        primary.rel_path.as_str(),
        None,
    ));
}

fn app_validate_step_present(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if primary
        .requirements
        .iter()
        .flat_map(|requirement| &requirement.required_commands)
        .any(|requirement| requirement == &G3TsHookCommandRequirement::AppValidateScript)
        && !app_roots(primary).iter().all(|app_root| {
            any_script_command(inputs, |command| is_app_validate_command(command, app_root))
        })
    {
        results.push(error(
            "g3ts-hooks/ts-app-validate-step-present",
            "hook does not run the app validate script",
            "A TypeScript hook contract requires the app-level `validate` script to run before commits. Add a real package-manager command such as `pnpm --filter <app> run validate`; comments and echoed text do not satisfy this rule.",
            primary.rel_path.as_str(),
            None,
        ));
    }
}

fn guardrail_config_changes_trigger_validation(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if any_script_command(inputs, command_mentions_guardrail_ts_config)
        && app_roots(primary).iter().all(|app_root| {
            any_script_command(inputs, |command| {
                is_g3ts_validate_path_command(command, app_root)
            })
        })
    {
        return;
    }
    results.push(error(
        "g3ts-hooks/ts-guardrail-config-changes-trigger-validation",
        "guardrail3-ts.toml changes do not trigger g3ts",
        "The pre-commit hook must explicitly include `guardrail3-ts.toml` in its changed-file routing and run `g3ts validate --path ...` when that file changes.",
        primary.rel_path.as_str(),
        None,
    ));
}

fn contract_trigger_patterns_covered(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let missing = primary
        .requirements
        .iter()
        .flat_map(|requirement| &requirement.trigger_patterns)
        .filter_map(|pattern| match pattern {
            G3TsHookTriggerPattern::Glob(pattern) => Some(pattern.as_str()),
        })
        .filter(|pattern| {
            !any_script_command(inputs, |command| command_mentions_pattern(command, pattern))
        })
        .collect::<Vec<_>>();

    if missing.is_empty() {
        return;
    }

    results.push(error(
        "g3ts-hooks/contract-trigger-patterns-covered",
        "hook does not route declared TypeScript trigger patterns",
        format!(
            "The hook contract declares trigger patterns that are not mentioned by executable hook routing commands: {}. Add staged-file routing for these patterns and run the required validation commands from that route.",
            missing.join(", ")
        )
        .as_str(),
        primary.rel_path.as_str(),
        None,
    ));
}

fn no_fail_open_wrappers(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let critical_commands = critical_command_names(input);
    if let Some((line_no, command_text)) = first_fail_open_critical_command(
        &input.parsed,
        &input.parsed,
        1,
        0,
        &mut Vec::new(),
        &critical_commands,
    ) {
        results.push(error(
            "g3ts-hooks/no-fail-open-wrappers",
            "critical hook command is fail-open",
            format!("Critical hook command `{command_text}` is softened by a fail-open wrapper. Remove `|| true`, `|| return 0`, soft command substitutions, or non-failing availability guards so the hook fails closed.")
                .as_str(),
            input.rel_path.as_str(),
            Some(line_no),
        ));
    }
}

fn dispatcher_inventory(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.kind == G3TsHookScriptKind::PreCommit && input.has_modular_dir {
        results.push(info(
            "g3ts-hooks/pre-commit-dispatcher-inventory",
            "pre-commit dispatcher inventory",
            "`.githooks/pre-commit.d` exists; command-presence checks inspect direct modular scripts as well as the dispatcher.".to_owned(),
            input.rel_path.as_str(),
            None,
        ));
    }
}

fn any_script_command(
    inputs: &[G3TsHooksSourceChecksInput],
    predicate: impl Fn(&ResolvedCommand) -> bool + Copy,
) -> bool {
    inputs
        .iter()
        .any(|input| any_resolved_command(&input.parsed, predicate))
}

fn app_roots(input: &G3TsHooksSourceChecksInput) -> Vec<String> {
    if input.app_package_roots.is_empty() {
        return vec![".".to_owned()];
    }
    input.app_package_roots.clone()
}

fn is_g3ts_validate_path_command(command: &ResolvedCommand, app_root: &str) -> bool {
    if command.command_name() != "g3ts" {
        return false;
    }
    let args = command.args();
    args.first().is_some_and(|arg| arg == "validate")
        && !args.iter().any(|arg| arg == "--family")
        && path_arg_matches(args, app_root)
}

fn path_arg_matches(args: &[String], app_root: &str) -> bool {
    args.iter().enumerate().any(|(index, arg)| {
        let path = if arg == "--path" {
            args.get(index.saturating_add(1)).map(String::as_str)
        } else {
            arg.strip_prefix("--path=")
        };
        path.is_some_and(|path| path_matches_app_root(path, app_root))
    })
}

fn path_matches_app_root(path: &str, app_root: &str) -> bool {
    path == app_root
        || (app_root == "." && path == ".")
        || path.strip_prefix("./").is_some_and(|path| path == app_root)
        || path.ends_with(format!("/{app_root}").as_str())
}

fn is_app_validate_command(command: &ResolvedCommand, app_root: &str) -> bool {
    match command.command_name() {
        "pnpm" => pnpm_validate(command.args(), app_root),
        "npm" | "bun" => app_root == "." && run_validate(command.args()),
        "yarn" => app_root == "." && yarn_validate(command.args()),
        _ => false,
    }
}

fn pnpm_validate(args: &[String], app_root: &str) -> bool {
    let mut index = 0usize;
    let mut filter_matches = app_root == ".";
    while let Some(arg) = args.get(index).map(String::as_str) {
        match arg {
            "--filter" | "-F" => {
                filter_matches = args
                    .get(index.saturating_add(1))
                    .is_some_and(|filter| filter_matches_app_root(filter, app_root));
                index = index.saturating_add(2);
            }
            "--dir" | "-C" => index = index.saturating_add(2),
            "run" => {
                return filter_matches
                    && args
                        .get(index.saturating_add(1))
                        .is_some_and(|arg| arg == "validate");
            }
            "validate" => return filter_matches,
            _ if arg.starts_with("--filter=") => {
                filter_matches = arg
                    .strip_prefix("--filter=")
                    .is_some_and(|filter| filter_matches_app_root(filter, app_root));
                index = index.saturating_add(1);
            }
            _ if arg.starts_with("-F=") => {
                filter_matches = arg
                    .strip_prefix("-F=")
                    .is_some_and(|filter| filter_matches_app_root(filter, app_root));
                index = index.saturating_add(1);
            }
            _ => index = index.saturating_add(1),
        }
    }
    false
}

fn filter_matches_app_root(filter: &str, app_root: &str) -> bool {
    filter == app_root
        || app_root
            .rsplit('/')
            .next()
            .is_some_and(|name| filter == name)
        || filter.ends_with(format!("/{app_root}").as_str())
}

fn command_mentions_guardrail_ts_config(command: &ResolvedCommand) -> bool {
    !matches!(command.command_name(), "echo" | "printf")
        && command
            .tokens()
            .iter()
            .any(|token| token.contains("guardrail3-ts.toml"))
}

fn command_mentions_pattern(command: &ResolvedCommand, pattern: &str) -> bool {
    !matches!(command.command_name(), "echo" | "printf")
        && command.tokens().iter().any(|token| token.contains(pattern))
}

fn run_validate(args: &[String]) -> bool {
    args.windows(2).any(|window| {
        window.first() == Some(&"run".to_owned()) && window.get(1) == Some(&"validate".to_owned())
    })
}

fn yarn_validate(args: &[String]) -> bool {
    args.first().is_some_and(|arg| arg == "validate") || run_validate(args)
}

fn first_fail_open_critical_command(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    absolute_base: usize,
    root_line_no: usize,
    visiting: &mut Vec<String>,
    critical_commands: &[String],
) -> Option<(usize, String)> {
    for line in &local.executable_lines {
        let absolute_line_no = absolute_line_no(absolute_base, line.line_no);
        let visible_root_line_no = if std::ptr::eq(local, root) {
            absolute_line_no
        } else {
            root_line_no
        };

        if line.softened_by.is_some()
            && any_resolved_command_on_line_in_context(
                local,
                root,
                &line.raw,
                line.line_no,
                visible_root_line_no,
                |command| is_critical_command(command, critical_commands),
            )
        {
            return Some((absolute_line_no, line.command_text.to_owned()));
        }
        if negated_if_failure_branch_is_softened(local, line.line_no)
            && any_resolved_command_on_line_in_context(
                local,
                root,
                &line.raw,
                line.line_no,
                visible_root_line_no,
                |command| is_critical_command(command, critical_commands),
            )
        {
            return Some((absolute_line_no, line.command_text.to_owned()));
        }
        if positive_availability_guard_is_softened(local, line.line_no, critical_commands) {
            return Some((absolute_line_no, line.command_text.to_owned()));
        }
        if let Some(found) = called_function_fail_open(
            local,
            root,
            &line.command_name,
            line.line_no,
            absolute_base,
            visible_root_line_no,
            visiting,
            critical_commands,
        ) {
            return Some(found);
        }
    }
    None
}

fn positive_availability_guard_is_softened(
    script: &ParsedShellScript,
    line_no: usize,
    critical_commands: &[String],
) -> bool {
    let Some(tool) = positive_availability_guard_tool(script, line_no) else {
        return false;
    };
    if !critical_commands.iter().any(|critical| critical == &tool) {
        return false;
    }
    let else_branch = positive_if_else_branch(script, line_no).unwrap_or_default();
    !branch_has_failure_terminator(&else_branch, script, line_no)
}

fn positive_availability_guard_tool(script: &ParsedShellScript, line_no: usize) -> Option<String> {
    let raw = script
        .source_lines
        .iter()
        .find(|line| line.line_no == line_no)?
        .raw
        .as_str();
    let trimmed = raw.trim_start();
    if !trimmed.starts_with("if ") || !trimmed.contains("then") {
        return None;
    }
    let words = hook_shell_parser::command_query::shell_words(trimmed.strip_prefix("if ")?.trim());
    if words.first().is_some_and(|word| word == "command")
        && words.get(1).is_some_and(|word| word == "-v")
    {
        return words.get(2).cloned();
    }
    None
}

fn positive_if_else_branch(script: &ParsedShellScript, line_no: usize) -> Option<String> {
    let mut found_then = false;
    let mut found_else = false;
    let mut branch = String::new();
    for line in script
        .source_lines
        .iter()
        .filter(|line| line.line_no >= line_no)
    {
        let mut text = line.raw.as_str();
        if !found_then {
            let Some((_, after_then)) = text.split_once("then") else {
                continue;
            };
            found_then = true;
            text = after_then;
        }
        if !found_else {
            if let Some((_, after_else)) = split_control_token(text, "else") {
                found_else = true;
                text = after_else;
            } else if split_control_token(text, "fi").is_some() {
                return None;
            } else {
                continue;
            }
        }
        if let Some((before_fi, _)) = split_control_token(text, "fi") {
            branch.push_str(before_fi);
            return Some(branch);
        }
        branch.push_str(text);
        branch.push('\n');
    }
    found_else.then_some(branch)
}

fn negated_if_failure_branch_is_softened(script: &ParsedShellScript, line_no: usize) -> bool {
    let Some(branch) = negated_if_failure_branch(script, line_no) else {
        return false;
    };
    !branch_has_failure_terminator(&branch, script, line_no)
}

fn negated_if_failure_branch(script: &ParsedShellScript, line_no: usize) -> Option<String> {
    let start = script
        .source_lines
        .iter()
        .find(|line| line.line_no == line_no)?;
    if !starts_negated_if(start.raw.as_str()) {
        return None;
    }

    let mut branch = String::new();
    let mut found_then = false;
    for line in script
        .source_lines
        .iter()
        .filter(|line| line.line_no >= line_no)
    {
        let mut text = line.raw.as_str();
        if !found_then {
            let Some((_, after_then)) = text.split_once("then") else {
                continue;
            };
            found_then = true;
            text = after_then;
        }

        if let Some((before_else, _)) = split_control_token(text, "else") {
            branch.push_str(before_else);
            break;
        }
        if let Some((before_elif, _)) = split_control_token(text, "elif") {
            branch.push_str(before_elif);
            break;
        }
        if let Some((before_fi, _)) = split_control_token(text, "fi") {
            branch.push_str(before_fi);
            break;
        }

        branch.push_str(text);
        branch.push('\n');
    }

    found_then.then_some(branch)
}

fn starts_negated_if(raw: &str) -> bool {
    let trimmed = raw.trim_start();
    trimmed
        .strip_prefix("if")
        .is_some_and(|tail| tail.trim_start().starts_with('!'))
}

fn split_control_token<'a>(text: &'a str, token: &str) -> Option<(&'a str, &'a str)> {
    for (index, _) in text.match_indices(token) {
        let before = &text[..index];
        let after = &text[index + token.len()..];
        let before_ok = before
            .chars()
            .last()
            .is_none_or(|ch| ch.is_whitespace() || ch == ';');
        let after_ok = after
            .chars()
            .next()
            .is_none_or(|ch| ch.is_whitespace() || ch == ';');
        if before_ok && after_ok {
            return Some((before, after));
        }
    }
    None
}

fn branch_has_failure_terminator(
    branch: &str,
    root: &ParsedShellScript,
    visible_root_line_no: usize,
) -> bool {
    let parsed = hook_shell_parser::parse_script(branch);
    parsed.executable_lines.iter().any(|line| {
        matches!(line.command_name.as_str(), "false")
            || (matches!(line.command_name.as_str(), "exit" | "return")
                && command_second_word_is_nonzero(line.command_text.as_str()))
            || called_function_terminates_failure(root, &line.command_name, visible_root_line_no)
    })
}

fn called_function_terminates_failure(
    parsed: &ParsedShellScript,
    command_name: &str,
    line_no: usize,
) -> bool {
    parsed
        .functions
        .iter()
        .rev()
        .find(|function| function.name == command_name && function.line_no <= line_no)
        .is_some_and(|function| {
            branch_has_failure_terminator(&function.body, parsed, function.line_no)
        })
}

fn command_second_word_is_nonzero(command_text: &str) -> bool {
    command_text
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.trim_end_matches(';').parse::<u8>().ok())
        .is_some_and(|value| value != 0)
}

fn called_function_fail_open(
    local: &ParsedShellScript,
    root: &ParsedShellScript,
    command_name: &str,
    call_line_no: usize,
    absolute_base: usize,
    root_line_no: usize,
    visiting: &mut Vec<String>,
    critical_commands: &[String],
) -> Option<(usize, String)> {
    let (function, function_absolute_base) = resolve_visible_function(
        local,
        root,
        command_name,
        call_line_no,
        absolute_base,
        root_line_no,
    )?;
    if visiting.iter().any(|name| name == &function.name) {
        return None;
    }
    visiting.push(function.name.to_owned());
    let found = first_fail_open_critical_command(
        &function.parsed_body,
        root,
        function_absolute_base,
        root_line_no,
        visiting,
        critical_commands,
    );
    let _ = visiting.pop();
    found
}

fn is_critical_command(command: &ResolvedCommand, critical_commands: &[String]) -> bool {
    critical_commands
        .iter()
        .any(|critical| critical == command.command_name())
}

fn critical_command_names(input: &G3TsHooksSourceChecksInput) -> Vec<String> {
    let mut names = input
        .requirements
        .iter()
        .flat_map(|requirement| &requirement.critical_commands)
        .map(|command| match command {
            G3TsHookCriticalCommand::Binary(name) => name.clone(),
        })
        .collect::<Vec<_>>();
    names.extend(["g3ts", "pnpm", "npm", "yarn", "bun"].map(str::to_owned));
    names.sort();
    names.dedup();
    names
}

fn absolute_line_no(absolute_base: usize, local_line_no: usize) -> usize {
    absolute_base.saturating_add(local_line_no.saturating_sub(1))
}

fn function_body_absolute_base(absolute_base: usize, function: &ShellFunction) -> usize {
    absolute_base.saturating_add(if function.body_starts_on_definition_line {
        function.line_no.saturating_sub(1)
    } else {
        function.line_no
    })
}

fn resolve_visible_function<'a>(
    local: &'a ParsedShellScript,
    root: &'a ParsedShellScript,
    command_name: &str,
    local_line_no: usize,
    absolute_base: usize,
    root_line_no: usize,
) -> Option<(&'a ShellFunction, usize)> {
    if let Some(function) = local
        .functions
        .iter()
        .rev()
        .find(|function| function.name == command_name && function.line_no <= local_line_no)
    {
        return Some((
            function,
            function_body_absolute_base(absolute_base, function),
        ));
    }

    if std::ptr::eq(local, root) {
        return None;
    }

    root.functions
        .iter()
        .rev()
        .find(|function| function.name == command_name && function.line_no <= root_line_no)
        .map(|function| (function, function_body_absolute_base(1, function)))
}

fn error(id: &str, title: &str, message: &str, file: &str, line: Option<usize>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message.to_owned(),
        Some(file.to_owned()),
        line,
    )
}

fn info(id: &str, title: &str, message: String, file: &str, line: Option<usize>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        line,
    )
    .into_inventory()
}

#[cfg(test)]
#[path = "lib_tests/mod.rs"]
mod lib_tests;
