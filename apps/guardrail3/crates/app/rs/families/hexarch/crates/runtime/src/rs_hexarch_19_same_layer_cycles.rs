use guardrail3_domain_report::{CheckResult, Severity};

#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;

#[cfg(test)]
use super::dependency_facts::{CycleFacts, Layer};

use super::inputs::CycleHexarchInput;

const ID: &str = "RS-HEXARCH-19";

pub fn check(input: &CycleHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let cycle = input.cycle;
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("same-layer {} dependency cycle", cycle.layer.label()),
        message: format!(
            "Found same-layer dependency cycle in `{}` layer: {}",
            cycle.layer.label(),
            cycle.members.join(" -> ")
        ),
        file: None,
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub fn results_for_cycles_for_test(tree: &ProjectTree) -> (Vec<String>, Vec<CheckResult>) {
    let facts = crate::collect_dependency_facts_from_tree_for_tests(tree);
    let mut results = Vec::new();
    let cycle_layers = facts
        .cycles
        .iter()
        .map(|cycle| cycle.layer.label().to_owned())
        .collect::<Vec<_>>();
    for cycle in &facts.cycles {
        check(&CycleHexarchInput::new(cycle), &mut results);
    }
    (cycle_layers, results)
}

#[cfg(test)]
pub fn check_cycle_for_test(layer: &str, members: Vec<&str>) -> Vec<CheckResult> {
    let layer = match layer {
        "domain" | "Domain" => Layer::Domain,
        "ports" | "Ports" => Layer::Ports,
        "app" | "App" => Layer::App,
        "adapters" | "Adapters" => Layer::Adapters,
        other => panic!("unsupported test layer `{other}`"),
    };
    let cycle = CycleFacts {
        layer,
        members: members.into_iter().map(|value| value.to_owned()).collect(),
    };
    let mut results = Vec::new();
    check(&CycleHexarchInput::new(&cycle), &mut results);
    results
}

#[cfg(test)]
#[path = "rs_hexarch_19_same_layer_cycles_tests/mod.rs"]
mod rs_hexarch_19_same_layer_cycles_tests;
