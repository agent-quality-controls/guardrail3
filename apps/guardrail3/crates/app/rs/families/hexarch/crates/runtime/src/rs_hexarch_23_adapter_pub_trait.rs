use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::Layer;
use super::inputs::SourceCrateHexarchInput;
#[cfg(test)]
use super::source_facts::SourceCrateFacts;

const ID: &str = "RS-HEXARCH-23";

pub fn check(input: &SourceCrateHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let source = input.source;
    if source.layer != Some(Layer::Adapters) {
        return;
    }

    if let Some(source_error_message) = &source.source_error_message {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "adapter crate `{}` source analysis failed",
                source.crate_name
            ),
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

    if source.pub_trait_count == 0 {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("adapter crate `{}` defines public traits", source.crate_name),
        message: format!(
            "Adapter crate `{}` defines {} public trait(s). Adapters should implement port traits, not define their own public trait surface.",
            source.crate_name, source.pub_trait_count
        ),
        file: Some(source.rel_dir.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub(crate) fn run_source_case(
    crate_name: &str,
    rel_dir: &str,
    pub_trait_count: usize,
    source_error_rel_path: Option<&str>,
    source_error_message: Option<&str>,
) -> Vec<CheckResult> {
    let source = SourceCrateFacts {
        crate_name: crate_name.to_owned(),
        rel_dir: rel_dir.to_owned(),
        layer: Some(Layer::Adapters),
        pub_trait_count,
        public_free_fn_count: 0,
        public_inherent_method_count: 0,
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
pub(super) fn results_for_test_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_23_adapter_pub_trait_tests/mod.rs"]
mod rs_hexarch_23_adapter_pub_trait_tests;
