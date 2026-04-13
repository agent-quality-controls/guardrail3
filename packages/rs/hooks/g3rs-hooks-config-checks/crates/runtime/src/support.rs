use g3rs_hooks_config_checks_types::G3RsHooksSelectedHookConfigFact;
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

pub(crate) fn tool_installed(installed_tools: &[String], tool: &str) -> bool {
    installed_tools.iter().any(|installed| installed == tool)
}

pub(crate) fn hook_uses_path_qualified_required_tool(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    tool: &str,
) -> bool {
    match tool {
        "gitleaks" => any_resolved_command(&selected_hook.parsed, is_path_qualified_gitleaks_command),
        "cargo-deny" => {
            any_resolved_command(&selected_hook.parsed, is_path_qualified_cargo_deny_command)
        }
        "cargo-machete" => {
            any_resolved_command(&selected_hook.parsed, is_path_qualified_cargo_machete_command)
        }
        _ => false,
    }
}

pub(crate) fn hook_requires_g3rs_validation(selected_hook: &G3RsHooksSelectedHookConfigFact) -> bool {
    any_resolved_command(&selected_hook.parsed, is_g3rs_validate_staged_command)
}

pub(crate) fn hook_uses_path_qualified_g3rs(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
) -> bool {
    any_resolved_command(&selected_hook.parsed, |command| {
        command.path_qualified() && is_g3rs_validate_staged_command(command)
    })
}

pub(crate) fn hook_requires_cargo_dupes(selected_hook: &G3RsHooksSelectedHookConfigFact) -> bool {
    any_resolved_command(&selected_hook.parsed, is_cargo_dupes_command)
}

pub(crate) fn hook_uses_path_qualified_cargo_dupes(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
) -> bool {
    any_resolved_command(&selected_hook.parsed, |command| {
        command.path_qualified() && is_cargo_dupes_command(command)
    })
}

fn is_g3rs_validate_staged_command(command: &ResolvedCommand) -> bool {
    if command.command_name() != "g3rs" {
        return false;
    }

    let args = command.args();
    let mut index = 0usize;

    while let Some(token) = args.get(index).map(String::as_str) {
        if !token.starts_with('-') {
            break;
        }

        if is_help_or_version_flag(token) {
            return false;
        }
        if let Some((flag_name, _)) = token.split_once('=')
            && g3rs_global_flag_takes_value(flag_name)
        {
            index += 1;
            continue;
        }
        if g3rs_global_flag_takes_value(token) {
            index += 2;
            continue;
        }

        index += 1;
    }

    let saw_validate = match args.get(index).map(String::as_str) {
        Some("rs") => args.get(index + 1).map(String::as_str) == Some("validate"),
        Some("validate") => true,
        _ => false,
    };

    saw_validate
        && args.iter().skip(index).all(|arg| !is_help_or_version_flag(arg))
        && args.iter().skip(index).any(|arg| arg == "--staged")
}

fn is_cargo_dupes_command(command: &ResolvedCommand) -> bool {
    match command.command_name() {
        "cargo" => command.args().first().is_some_and(|arg| arg == "dupes"),
        "cargo-dupes" => !command.args().iter().any(|arg| is_help_or_version_flag(arg)),
        _ => false,
    }
}

fn is_path_qualified_gitleaks_command(command: &ResolvedCommand) -> bool {
    command.path_qualified()
        && command.command_name() == "gitleaks"
        && !command.args().iter().any(|arg| is_help_or_version_flag(arg))
}

fn is_path_qualified_cargo_deny_command(command: &ResolvedCommand) -> bool {
    command.path_qualified()
        && command.command_name() == "cargo-deny"
        && !command.args().iter().any(|arg| is_help_or_version_flag(arg))
}

fn is_path_qualified_cargo_machete_command(command: &ResolvedCommand) -> bool {
    command.path_qualified()
        && command.command_name() == "cargo-machete"
        && !command.args().iter().any(|arg| is_help_or_version_flag(arg))
}

fn g3rs_global_flag_takes_value(flag: &str) -> bool {
    matches!(flag, "--config" | "--format" | "--root" | "--cache-dir")
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}
