use std::collections::BTreeSet;

use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement,
};
use g3rs_hooks_types::G3RsHooksSelectedHookConfigFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-hooks/contract-required-tools-installed";

pub(crate) fn check(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    installed_tools: &[String],
    requirements: &[G3HookRequirement],
    results: &mut Vec<G3CheckResult>,
) {
    for tool in required_tools(requirements) {
        let installed = crate::support::tool_installed(installed_tools, &tool)
            || crate::support::hook_uses_path_qualified_required_tool(selected_hook, &tool);
        if installed {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    format!("{tool} installed for hook contract"),
                    format!("{tool} is available for contract-owned Rust hook execution."),
                    Some(selected_hook.rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        } else {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                format!("{tool} missing for hook contract"),
                format!(
                    "{tool} is required by a family hook contract but is not available on PATH or via a path-qualified command."
                ),
                Some(selected_hook.rel_path.clone()),
                None,
            ));
        }
    }
}

fn required_tools(requirements: &[G3HookRequirement]) -> Vec<String> {
    let mut tools = BTreeSet::from(["g3rs".to_owned(), "gitleaks".to_owned()]);
    for requirement in requirements {
        for command in &requirement.required_commands {
            add_required_command_tools(&mut tools, *command);
        }
        for command in &requirement.critical_commands {
            add_critical_command_tools(&mut tools, command);
        }
    }
    tools.into_iter().collect()
}

fn add_required_command_tools(tools: &mut BTreeSet<String>, command: G3HookCommandRequirement) {
    match command {
        G3HookCommandRequirement::CargoDenyCheck => {
            let _ = tools.insert("cargo-deny".to_owned());
        }
        G3HookCommandRequirement::CargoMachete => {
            let _ = tools.insert("cargo-machete".to_owned());
        }
        G3HookCommandRequirement::CargoDupes | G3HookCommandRequirement::CargoDupesExcludeTests => {
            let _ = tools.insert("cargo-dupes".to_owned());
        }
        G3HookCommandRequirement::Gitleaks => {
            let _ = tools.insert("gitleaks".to_owned());
        }
        G3HookCommandRequirement::G3RsValidatePath => {
            let _ = tools.insert("g3rs".to_owned());
        }
        G3HookCommandRequirement::CargoFmtCheck
        | G3HookCommandRequirement::CargoClippyDenyWarnings
        | G3HookCommandRequirement::ConcreteLockfileCommand
        | G3HookCommandRequirement::CargoTest => {}
    }
}

fn add_critical_command_tools(tools: &mut BTreeSet<String>, command: &G3HookCriticalCommand) {
    match command {
        G3HookCriticalCommand::Binary(binary) => {
            if binary != "cargo" {
                let _ = tools.insert(binary.clone());
            }
        }
        G3HookCriticalCommand::CargoSubcommand(subcommand) => match subcommand.as_str() {
            "deny" => {
                let _ = tools.insert("cargo-deny".to_owned());
            }
            "machete" => {
                let _ = tools.insert("cargo-machete".to_owned());
            }
            "dupes" => {
                let _ = tools.insert("cargo-dupes".to_owned());
            }
            _ => {}
        },
    }
}

#[cfg(test)]
fn run_case(
    content: &str,
    installed_tools: Vec<String>,
    requirements: Vec<G3HookRequirement>,
) -> Vec<G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let selected_hook = G3RsHooksSelectedHookConfigFact {
        rel_path: ".githooks/pre-commit".to_owned(),
        parsed,
    };
    let mut results = Vec::new();
    check(
        &selected_hook,
        &installed_tools,
        &requirements,
        &mut results,
    );
    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
