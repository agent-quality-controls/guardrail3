use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ScopedTopologyConfigInput;

const ID: &str = "RS-TOPOLOGY-05";

pub fn check(input: &ScopedTopologyConfigInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Scoped `topology` config is forbidden".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

pub fn check_success(
    has_scoped_failures: bool,
    config_input_failed_closed: bool,
    results: &mut Vec<CheckResult>,
) {
    if has_scoped_failures || config_input_failed_closed {
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "topology config remains global-only".to_owned(),
            "No forbidden scoped `topology` configuration was found under app or package sections."
                .to_owned(),
            None,
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_topology_05_scoped_topology_config_forbidden_tests/mod.rs"]
mod rs_topology_05_scoped_topology_config_forbidden_tests;
