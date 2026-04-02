use guardrail3_domain_report::{CheckResult, Severity};

use crate::dependency_facts::Layer;
use crate::inputs::SourceCrateHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-23";

pub fn check(input: &SourceCrateHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let source = input.source;
    if source.layer != Some(Layer::Adapters) {
        return;
    }

    if let Some(source_error_message) = &source.source_error_message {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "adapter crate `{}` source analysis failed",
                source.crate_name
            ),
            source_error_message.clone(),
            Some(
                source
                    .source_error_rel_path
                    .clone()
                    .unwrap_or_else(|| source.rel_dir.clone()),
            ),
            None,
            false,
        ));
        return;
    }

    if source.pub_trait_count == 0 {
        push_success(
            results,
            ID,
            format!(
                "adapter crate `{}` defines no public traits",
                source.crate_name
            ),
            format!(
                "Adapter crate `{}` keeps its public surface free of adapter-owned public traits.",
                source.crate_name
            ),
            Some(source.rel_dir.clone()),
        );
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("adapter crate `{}` defines public traits", source.crate_name),
    format!(
            "Adapter crate `{}` defines {} public trait(s). Adapters should implement port traits, not define their own public trait surface.",
            source.crate_name, source.pub_trait_count
        ),
    Some(source.rel_dir.clone()),
    None,
    false,
    ));
}

