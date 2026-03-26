use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::Layer;
use super::inputs::SourceCrateHexarchInput;
#[cfg(test)]
use super::source_facts::SourceCrateFacts;

const ID: &str = "RS-HEXARCH-22";

pub fn check(input: &SourceCrateHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let source = input.source;
    if source.layer != Some(Layer::Ports) {
        return;
    }

    if let Some(source_error_message) = &source.source_error_message {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("ports crate `{}` source analysis failed", source.crate_name),
            message: source_error_message.clone(),
            file: Some(
                source
                    .source_error_rel_path
                    .clone()
                    .unwrap_or_else(|| source.rel_dir.clone()),
            ),
            line: None,
            inventory: false,
        });
        return;
    }

    if source.impl_count <= source.pub_trait_count {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: format!("ports crate `{}` is impl-heavy", source.crate_name),
        message: format!(
            "Ports crate `{}` has {} impl blocks and {} public traits. Ports should stay trait-dominant.",
            source.crate_name, source.impl_count, source.pub_trait_count
        ),
        file: Some(source.rel_dir.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceCrateLayerForTest {
    Ports,
    Adapters,
}

#[cfg(test)]
pub(crate) fn run_source_case(
    layer: SourceCrateLayerForTest,
    crate_name: &str,
    rel_dir: &str,
    pub_trait_count: usize,
    impl_count: usize,
    source_error_rel_path: Option<&str>,
    source_error_message: Option<&str>,
) -> Vec<CheckResult> {
    let source = SourceCrateFacts {
        crate_name: crate_name.to_owned(),
        rel_dir: rel_dir.to_owned(),
        layer: Some(match layer {
            SourceCrateLayerForTest::Ports => Layer::Ports,
            SourceCrateLayerForTest::Adapters => Layer::Adapters,
        }),
        pub_trait_count,
        impl_count,
        source_error_rel_path: source_error_rel_path.map(|value| value.to_owned()),
        source_error_message: source_error_message.map(|value| value.to_owned()),
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);
    results
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_tree(tree: &guardrail3_domain_project_tree::ProjectTree) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_22_ports_trait_dominance_tests/mod.rs"]
mod rs_hexarch_22_ports_trait_dominance_tests;
