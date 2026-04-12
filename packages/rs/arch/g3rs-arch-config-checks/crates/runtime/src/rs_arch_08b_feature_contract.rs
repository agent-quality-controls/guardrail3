use g3rs_arch_types::G3RsArchConfigCrate;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-CONFIG-08";

pub(crate) fn check(node: &G3RsArchConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !node.requires_feature_contract {
        return;
    }

    if !node.has_all_feature {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "missing `all` feature".to_owned(),
            format!(
                "Crate `{}` has facade exports but no `all` feature in [features]. Add `all = [\"feature1\", \"feature2\", ...]` and `default = [\"all\"]`.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
        ));
    }

    if node.has_all_feature && node.all_feature_deps.is_empty() {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "`all` feature is empty".to_owned(),
            format!(
                "Crate `{}` has an `all` feature but it enables no sub-features. `all` must list the named sub-features: `all = [\"types\", \"api\", ...]`.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
        ));
    }

    if node.has_all_feature && !node.has_default_feature {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "missing `default` feature".to_owned(),
            format!(
                "Crate `{}` has `all` feature but no `default` feature. Add `default = [\"all\"]` so consumers get everything by default.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
        ));
    }

    if node.has_default_feature && !node.default_feature_deps.iter().any(|dep| dep == "all") {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "`default` feature must include `all`".to_owned(),
            format!(
                "Crate `{}` has a `default` feature but it does not include `all`. Set `default = [\"all\"]` so consumers get everything unless they opt out.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
        ));
    }

    if node.has_all_feature
        && !node.all_feature_deps.is_empty()
        && node.has_default_feature
        && node.default_feature_deps.iter().any(|dep| dep == "all")
    {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "feature contract supports facade exports".to_owned(),
                format!(
                    "Crate `{}` has facade exports with a valid `all` + `default` feature contract.",
                    node.rel_dir
                ),
                Some(node.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_arch_08b_feature_contract_tests/mod.rs"]
mod tests;
