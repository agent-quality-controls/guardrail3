use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::Layer;
use super::inputs::SourceCrateHexarchInput;
use super::inventory::push_success;
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

    if source.public_free_fn_count > 0 {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!(
                "ports crate `{}` exposes public free functions",
                source.crate_name
            ),
            message: format!(
                "Ports crate `{}` exposes {} public free function(s) outside trait definitions. Ports should keep public behavior in traits or passive types, not free functions.",
                source.crate_name, source.public_free_fn_count
            ),
            file: Some(source.rel_dir.clone()),
            line: None,
            inventory: false,
        });
    }

    if source.public_inherent_method_count > 0 {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!(
                "ports crate `{}` exposes public inherent methods",
                source.crate_name
            ),
            message: format!(
                "Ports crate `{}` exposes {} public inherent method(s) on concrete types. Ports should keep public behavior in traits or passive types, not on concrete types.",
                source.crate_name, source.public_inherent_method_count
            ),
            file: Some(source.rel_dir.clone()),
            line: None,
            inventory: false,
        });
    }

    if source.public_free_fn_count == 0 && source.public_inherent_method_count == 0 {
        push_success(
            results,
            ID,
            format!(
                "ports crate `{}` keeps public behavior in traits",
                source.crate_name
            ),
            format!(
                "Ports crate `{}` exposes no public free functions or public inherent methods on concrete types.",
                source.crate_name
            ),
            Some(source.rel_dir.clone()),
        );
    }
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
    public_free_fn_count: usize,
    public_inherent_method_count: usize,
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
        public_free_fn_count,
        public_inherent_method_count,
        source_error_rel_path: source_error_rel_path.map(|value| value.to_owned()),
        source_error_message: source_error_message.map(|value| value.to_owned()),
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);
    results
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]
#[path = "rs_hexarch_22_ports_trait_dominance_tests/mod.rs"]
mod rs_hexarch_22_ports_trait_dominance_tests;
