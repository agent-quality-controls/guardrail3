use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-01";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "cargo-deny" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo-deny installed".to_owned(),
                message: "`cargo-deny` is available on PATH.".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "cargo-deny missing".to_owned(),
            message:
                "`cargo-deny` is required for Rust dependency guardrails but was not found on PATH."
                    .to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
fn family_route(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Deps,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, &scope, None, &selected, None)
        .map_rs_deps()
}

#[cfg(test)]
pub(super) fn collected_facts(
    tree: &guardrail3_domain_project_tree::ProjectTree,
    installed: &[&str],
) -> super::facts::DepsFacts {
    super::facts::collect(
        tree,
        &family_route(tree),
        &test_support::StubToolChecker::new(installed),
    )
}

#[cfg(test)]
pub(super) fn tool_input<'a>(
    facts: &'a super::facts::DepsFacts,
    tool_name: &str,
) -> super::inputs::ToolDepsInput<'a> {
    let tool = facts
        .tools
        .iter()
        .find(|tool| tool.tool_name == tool_name)
        .expect("expected tool facts");
    super::inputs::ToolDepsInput::new(tool)
}

#[cfg(test)]
pub(super) fn tool_facts(tool_name: &str, installed: bool) -> super::facts::DepsFacts {
    super::facts::DepsFacts {
        tools: vec![super::facts::ToolFacts {
            tool_name: tool_name.to_owned(),
            installed,
        }],
        lockfiles: vec![super::facts::LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        input_failures: Vec::new(),
    }
}

#[cfg(test)]
pub(super) fn run_with_facts(
    facts: &super::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}

#[cfg(test)]
#[path = "rs_deps_01_cargo_deny_installed_tests/mod.rs"]
mod rs_deps_01_cargo_deny_installed_tests;
