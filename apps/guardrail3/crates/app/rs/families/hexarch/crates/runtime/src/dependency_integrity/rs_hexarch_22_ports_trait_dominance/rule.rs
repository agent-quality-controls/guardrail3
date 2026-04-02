use guardrail3_domain_report::{CheckResult, Severity};

use crate::dependency_facts::Layer;
use crate::inputs::SourceCrateHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-22";

pub fn check(input: &SourceCrateHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let source = input.source;
    if source.layer != Some(Layer::Ports) {
        return;
    }

    if let Some(source_error_message) = &source.source_error_message {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            format!("ports crate `{}` source analysis failed", source.crate_name),
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

    if source.public_free_fn_count > 0 {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            format!(
                "ports crate `{}` exposes public free functions",
                source.crate_name
            ),
            format!(
                "Ports crate `{}` exposes {} public free function(s) outside trait definitions. Ports should keep public behavior in traits or passive types, not free functions.",
                source.crate_name, source.public_free_fn_count
            ),
            Some(source.rel_dir.clone()),
            None,
            false,
        ));
    }

    if source.public_inherent_method_count > 0 {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            format!(
                "ports crate `{}` exposes public inherent methods",
                source.crate_name
            ),
            format!(
                "Ports crate `{}` exposes {} public inherent method(s) on concrete types. Ports should keep public behavior in traits or passive types, not on concrete types.",
                source.crate_name, source.public_inherent_method_count
            ),
            Some(source.rel_dir.clone()),
            None,
            false,
        ));
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

