use g3rs_hooks_types::G3RsHooksSelectedHookConfigFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

/// Predicate over one selected hook.
pub(crate) type SelectedHookPredicate = fn(&G3RsHooksSelectedHookConfigFact) -> bool;

/// Messages used by one tool availability result.
#[derive(Clone, Copy)]
pub(crate) struct ToolAvailabilityMessages<'a> {
    /// Rule identifier.
    pub(crate) id: &'a str,
    /// Inventory title when the tool is available.
    pub(crate) available_title: &'a str,
    /// Inventory message when the tool is available.
    pub(crate) available_message: &'a str,
    /// Error title when the tool is missing.
    pub(crate) missing_title: &'a str,
    /// Error message when the tool is missing.
    pub(crate) missing_message: &'a str,
}

/// Inputs used to check one required tool.
#[derive(Clone, Copy)]
pub(crate) struct RequiredToolAvailabilityCheck<'a> {
    /// Predicate that decides whether the selected hook requires the tool.
    pub(crate) required: SelectedHookPredicate,
    /// Predicate that decides whether the selected hook runs a path-qualified tool command.
    pub(crate) path_qualified: SelectedHookPredicate,
    /// Tool name expected on PATH.
    pub(crate) tool: &'a str,
    /// Findings emitted by this rule.
    pub(crate) messages: ToolAvailabilityMessages<'a>,
}

/// Checks one required tool and appends one finding when the selected hook requires it.
pub(crate) fn check_required_tool_availability(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    installed_tools: &[String],
    check: RequiredToolAvailabilityCheck<'_>,
    results: &mut Vec<G3CheckResult>,
) {
    if !(check.required)(selected_hook) {
        return;
    }

    let available =
        (check.path_qualified)(selected_hook) || tool_installed(installed_tools, check.tool);
    push_tool_availability_result(selected_hook, available, check.messages, results);
}

/// Push one tool availability finding.
pub(crate) fn push_tool_availability_result(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    available: bool,
    messages: ToolAvailabilityMessages<'_>,
    results: &mut Vec<G3CheckResult>,
) {
    let mut finding = G3CheckResult::new(
        messages.id.to_owned(),
        if available {
            G3Severity::Info
        } else {
            G3Severity::Error
        },
        if available {
            messages.available_title.to_owned()
        } else {
            messages.missing_title.to_owned()
        },
        if available {
            messages.available_message.to_owned()
        } else {
            messages.missing_message.to_owned()
        },
        Some(selected_hook.rel_path.clone()),
        None,
    );
    if available {
        finding = finding.into_inventory();
    }
    results.push(finding);
}

/// Implements `tool installed`.
pub(crate) fn tool_installed(installed_tools: &[String], tool: &str) -> bool {
    installed_tools.iter().any(|installed| installed == tool)
}

/// Implements `hook uses path qualified required tool`.
pub(crate) fn hook_uses_path_qualified_required_tool(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    tool: &str,
) -> bool {
    match tool {
        "gitleaks" => {
            any_resolved_command(&selected_hook.parsed, is_path_qualified_gitleaks_command)
        }
        "cargo-deny" => {
            any_resolved_command(&selected_hook.parsed, is_path_qualified_cargo_deny_command)
        }
        "cargo-machete" => any_resolved_command(
            &selected_hook.parsed,
            is_path_qualified_cargo_machete_command,
        ),
        "cargo-dupes" => {
            any_resolved_command(&selected_hook.parsed, is_path_qualified_cargo_dupes_command)
        }
        "g3rs" => any_resolved_command(&selected_hook.parsed, |command| {
            command.path_qualified() && is_g3rs_validate_staged_command(command)
        }),
        _ => false,
    }
}

/// Implements `hook requires g3rs validation`.
pub(crate) fn hook_requires_g3rs_validation(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
) -> bool {
    any_resolved_command(&selected_hook.parsed, is_g3rs_validate_staged_command)
}

/// Implements `hook uses path qualified g3rs`.
pub(crate) fn hook_uses_path_qualified_g3rs(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
) -> bool {
    any_resolved_command(&selected_hook.parsed, |command| {
        command.path_qualified() && is_g3rs_validate_staged_command(command)
    })
}

/// Implements `hook requires cargo dupes`.
pub(crate) fn hook_requires_cargo_dupes(selected_hook: &G3RsHooksSelectedHookConfigFact) -> bool {
    any_resolved_command(&selected_hook.parsed, is_cargo_dupes_command)
}

/// Implements `hook uses path qualified cargo dupes`.
pub(crate) fn hook_uses_path_qualified_cargo_dupes(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
) -> bool {
    any_resolved_command(&selected_hook.parsed, |command| {
        command.path_qualified() && is_cargo_dupes_command(command)
    })
}

/// Implements `is g3rs validate staged command`.
fn is_g3rs_validate_staged_command(command: &ResolvedCommand) -> bool {
    if command.command_name() != "g3rs" {
        return false;
    }

    let args = command.args();
    if args
        .first()
        .is_some_and(|token| token.starts_with('-') || is_help_or_version_flag(token))
    {
        return false;
    }

    if args.first().map(String::as_str) != Some("validate") {
        return false;
    }

    let Some(rest) = args.get(1..) else {
        return false;
    };
    parse_validate_args(rest)
}

/// Implements `is cargo dupes command`.
fn is_cargo_dupes_command(command: &ResolvedCommand) -> bool {
    match command.command_name() {
        "cargo" => command.args().first().is_some_and(|arg| arg == "dupes"),
        "cargo-dupes" => !command
            .args()
            .iter()
            .any(|arg| is_help_or_version_flag(arg)),
        _ => false,
    }
}

/// Implements `is path qualified gitleaks command`.
fn is_path_qualified_gitleaks_command(command: &ResolvedCommand) -> bool {
    command.path_qualified()
        && command.command_name() == "gitleaks"
        && !command
            .args()
            .iter()
            .any(|arg| is_help_or_version_flag(arg))
}

/// Implements `is path qualified cargo deny command`.
fn is_path_qualified_cargo_deny_command(command: &ResolvedCommand) -> bool {
    command.path_qualified()
        && command.command_name() == "cargo-deny"
        && !command
            .args()
            .iter()
            .any(|arg| is_help_or_version_flag(arg))
}

/// Implements `is path qualified cargo machete command`.
fn is_path_qualified_cargo_machete_command(command: &ResolvedCommand) -> bool {
    command.path_qualified()
        && command.command_name() == "cargo-machete"
        && !command
            .args()
            .iter()
            .any(|arg| is_help_or_version_flag(arg))
}

/// Implements `is path qualified cargo dupes command`.
fn is_path_qualified_cargo_dupes_command(command: &ResolvedCommand) -> bool {
    command.path_qualified()
        && command.command_name() == "cargo-dupes"
        && !command
            .args()
            .iter()
            .any(|arg| is_help_or_version_flag(arg))
}

/// Implements `parse validate args`.
fn parse_validate_args(args: &[String]) -> bool {
    let mut saw_path = false;
    let mut iter = args.iter().map(String::as_str);

    while let Some(arg) = iter.next() {
        if is_help_or_version_flag(arg) {
            return false;
        }
        if let Some(path) = arg.strip_prefix("--path=") {
            if path.is_empty() || path.starts_with('-') {
                return false;
            }
            saw_path = true;
            continue;
        }
        if arg == "--path" {
            let Some(value) = iter.next() else {
                return false;
            };
            if value.is_empty() || value.starts_with('-') {
                return false;
            }
            saw_path = true;
            continue;
        }
        if let Some(value) = arg.strip_prefix("--family=") {
            if value.is_empty() {
                return false;
            }
            continue;
        }
        if arg == "--family" {
            let Some(value) = iter.next() else {
                return false;
            };
            if value.starts_with('-') {
                return false;
            }
            continue;
        }
        if arg == "--inventory" || arg == "--staged" {
            continue;
        }
        return false;
    }

    saw_path
}

/// Implements `is help or version flag`.
fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}
