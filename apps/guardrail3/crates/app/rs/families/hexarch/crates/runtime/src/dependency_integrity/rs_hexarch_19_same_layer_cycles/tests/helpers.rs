use super::super::{check, check_inventory};
use guardrail3_domain_report::CheckResult;
use crate::dependency_facts::CycleFacts;
use crate::inputs::CycleHexarchInput;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use crate::dependency_facts::Layer;
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
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
