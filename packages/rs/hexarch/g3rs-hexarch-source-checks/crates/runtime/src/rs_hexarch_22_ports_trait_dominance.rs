use g3rs_hexarch_types::{G3RsHexarchLayer, G3RsHexarchSourceCrateFacts};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-HEXARCH-SOURCE-22";

pub(crate) fn check(source: &G3RsHexarchSourceCrateFacts, results: &mut Vec<G3CheckResult>) {
    if source.layer != Some(G3RsHexarchLayer::Ports) {
        return;
    }

    if let Some(source_error_message) = &source.source_error_message {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            format!("ports crate `{}` source analysis failed", source.crate_name),
            source_error_message.clone(),
            Some(
                source
                    .source_error_rel_path
                    .clone()
                    .unwrap_or_else(|| source.rel_dir.clone()),
            ),
            None,
        ));
        return;
    }

    if source.public_free_fn_count > 0 {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
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
        ));
    }

    if source.public_inherent_method_count > 0 {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
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
        ));
    }

    if source.public_free_fn_count == 0 && source.public_inherent_method_count == 0 {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!(
                    "ports crate `{}` keeps public behavior in traits",
                    source.crate_name
                ),
                format!(
                    "Ports crate `{}` exposes no public free functions or public inherent methods on concrete types.",
                    source.crate_name
                ),
                Some(source.rel_dir.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_hexarch_22_ports_trait_dominance_tests/mod.rs"]
mod tests;
