use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::Layer;
use super::inputs::SourceCrateHexarchInput;

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
#[path = "rs_hexarch_22_ports_trait_dominance_tests/mod.rs"]
mod tests;
