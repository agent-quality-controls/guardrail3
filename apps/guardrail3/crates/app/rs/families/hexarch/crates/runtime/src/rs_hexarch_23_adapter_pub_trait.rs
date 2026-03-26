use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::Layer;
use super::inputs::SourceCrateHexarchInput;

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
#[path = "rs_hexarch_23_adapter_pub_trait_tests/mod.rs"]
mod rs_hexarch_23_adapter_pub_trait_tests;
