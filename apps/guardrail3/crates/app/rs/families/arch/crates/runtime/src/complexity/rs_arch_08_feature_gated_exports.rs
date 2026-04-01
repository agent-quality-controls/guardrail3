use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::{CrateNode, FacadeSurface};

const ID: &str = "RS-ARCH-08";

pub(crate) fn check(
    node: &CrateNode,
    surface: Option<&FacadeSurface>,
    results: &mut Vec<CheckResult>,
) {
    let Some(lib_rel) = &node.lib_rs_rel else {
        return;
    };
    let Some(surface) = surface else {
        return;
    };
    if surface.pub_export_count == 0 {
        return;
    }

    // Check: every pub export must have a feature gate.
    if surface.ungated_export_count > 0 {
        let ungated_items: Vec<String> = surface
            .pub_mods
            .iter()
            .chain(surface.pub_uses.iter())
            .filter(|item| item.feature_gate.is_none())
            .map(|item| format!("`{}`", item.name))
            .collect();
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "facade exports not feature-gated".to_owned(),
            format!(
                "lib.rs in `{}` has {} ungated pub exports: {}. \
                 Every pub mod/use in lib.rs must be behind #[cfg(feature = \"...\")]. \
                 Add a [features] section with named sub-features, an `all` meta-feature, \
                 and `default = [\"all\"]`.",
                node.rel_dir,
                surface.ungated_export_count,
                ungated_items.join(", ")
            ),
            Some(lib_rel.clone()),
            None,
            false,
        ));
    }

    // Check: `all` feature must not directly gate pub items.
    if surface.gated_on_all_count > 0 {
        let all_gated_items: Vec<String> = surface
            .pub_mods
            .iter()
            .chain(surface.pub_uses.iter())
            .filter(|item| item.gated_on_all)
            .map(|item| format!("`{}`", item.name))
            .collect();
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "`all` feature must not directly gate exports".to_owned(),
            format!(
                "lib.rs in `{}` has {} exports gated directly on `all`: {}. \
                 The `all` feature must only enable other named sub-features, \
                 not directly gate pub items. Split exports into meaningful \
                 named features and re-export through `all`.",
                node.rel_dir,
                surface.gated_on_all_count,
                all_gated_items.join(", ")
            ),
            Some(lib_rel.clone()),
            None,
            false,
        ));
    }

    // Check: Cargo.toml must have [features] with `all` and `default`.
    if !node.has_all_feature && surface.pub_export_count > 0 {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "missing `all` feature".to_owned(),
            format!(
                "Crate `{}` has pub exports but no `all` feature in [features]. \
                 Add `all = [\"feature1\", \"feature2\", ...]` and `default = [\"all\"]`.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
            false,
        ));
    }

    if node.has_all_feature && !node.has_default_feature {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "missing `default` feature".to_owned(),
            format!(
                "Crate `{}` has `all` feature but no `default = [\"all\"]`. \
                 Consumers should get everything by default unless they opt out.",
                node.rel_dir
            ),
            Some(node.cargo_rel_path.clone()),
            None,
            false,
        ));
    }

    if surface.ungated_export_count == 0
        && surface.gated_on_all_count == 0
        && node.has_all_feature
        && node.has_default_feature
    {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "facade exports properly feature-gated".to_owned(),
                format!(
                    "lib.rs in `{}` has all exports behind named features with `all` + `default`.",
                    node.rel_dir
                ),
                Some(lib_rel.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}
