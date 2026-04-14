use g3rs_arch_types::G3RsArchConfigCrate;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-CONFIG-08";

pub(crate) fn check(node: &G3RsArchConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !node.requires_feature_contract {
        return;
    }

    if !node.has_default_feature {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "missing `default` feature".to_owned(),
            format!(
                "Crate `{}` has facade exports but no `default` feature in [features]. Add `default = [\"feature1\", \"feature2\", ...]` so the facade has an explicit feature contract.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
        ));
        return;
    }

    if node.default_feature_deps.is_empty() {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "`default` feature is empty".to_owned(),
            format!(
                "Crate `{}` has facade exports but `default` enables no named features. Add at least one named feature such as `default = [\"api\"]`.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
        ));
        return;
    }

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            "feature contract supports facade exports".to_owned(),
            format!(
                "Crate `{}` has facade exports with a valid named-feature contract.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
        )
        .into_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_arch_08b_feature_contract_tests/mod.rs"]
mod tests;
