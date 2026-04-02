use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-04";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "gitleaks" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "gitleaks installed".to_owned(),
                "`gitleaks` is available on PATH.".to_owned(),
                None,
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "gitleaks missing".to_owned(),
            "`gitleaks` is required for Rust dependency guardrails but was not found on PATH."
                .to_owned(),
            None,
            None,
            false,
        ));
    }
}

#[cfg(test)]
fn family_route(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Deps,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, &scope, None, &selected, None)
        .map_rs_deps()
}

#[cfg(test)]
pub(super) fn collected_facts(
    tree: &guardrail3_app_rs_family_view::FamilyView,
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
        direct_dependency_caps: Vec::new(),
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
#[path = "rs_deps_04_gitleaks_installed_tests/mod.rs"]
mod rs_deps_04_gitleaks_installed_tests;
