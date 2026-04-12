use g3rs_hexarch_types::{G3RsHexarchLayer, G3RsHexarchSourceCrateFacts};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-HEXARCH-SOURCE-23";

pub(crate) fn check(source: &G3RsHexarchSourceCrateFacts, results: &mut Vec<G3CheckResult>) {
    if source.layer != Some(G3RsHexarchLayer::Adapters) {
        return;
    }

    if let Some(source_error_message) = &source.source_error_message {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!("adapter crate `{}` source analysis failed", source.crate_name),
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

    if source.pub_trait_count == 0 {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!(
                    "adapter crate `{}` defines no public traits",
                    source.crate_name
                ),
                format!(
                    "Adapter crate `{}` keeps its public surface free of adapter-owned public traits.",
                    source.crate_name
                ),
                Some(source.rel_dir.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        format!("adapter crate `{}` defines public traits", source.crate_name),
        format!(
            "Adapter crate `{}` defines {} public trait(s). Adapters should implement port traits, not define their own public trait surface.",
            source.crate_name, source.pub_trait_count
        ),
        Some(source.rel_dir.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rs_hexarch_23_adapter_pub_trait_tests/mod.rs"]
mod tests;
