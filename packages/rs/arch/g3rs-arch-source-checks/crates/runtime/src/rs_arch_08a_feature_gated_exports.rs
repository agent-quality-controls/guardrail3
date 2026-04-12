use g3rs_arch_types::{G3RsArchFacadeSurface, G3RsArchSourceCrate};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-08A";

pub(crate) fn check(
    node: &G3RsArchSourceCrate,
    surface: Option<&G3RsArchFacadeSurface>,
    results: &mut Vec<G3CheckResult>,
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

    if surface.ungated_export_count > 0 {
        let ungated_items = surface
            .pub_exports
            .iter()
            .filter(|item| item.feature_gate.is_none())
            .map(|item| format!("`{}`", item.name))
            .collect::<Vec<_>>();
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "facade exports not feature-gated".to_owned(),
            format!(
                "lib.rs in `{}` has {} ungated pub exports: {}. Every pub mod/use in lib.rs must be behind #[cfg(feature = \"...\")].",
                node.rel_dir,
                surface.ungated_export_count,
                ungated_items.join(", ")
            ),
            Some(lib_rel.clone()),
            None,
        ));
    }

    if surface.gated_on_all_count > 0 {
        let all_gated_items = surface
            .pub_exports
            .iter()
            .filter(|item| item.gated_on_all)
            .map(|item| format!("`{}`", item.name))
            .collect::<Vec<_>>();
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "`all` feature must not directly gate exports".to_owned(),
            format!(
                "lib.rs in `{}` has {} exports gated directly on `all`: {}. The `all` feature must only enable other named sub-features, not directly gate pub items.",
                node.rel_dir,
                surface.gated_on_all_count,
                all_gated_items.join(", ")
            ),
            Some(lib_rel.clone()),
            None,
        ));
    }

    if surface.ungated_export_count == 0 && surface.gated_on_all_count == 0 {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "facade exports properly feature-gated".to_owned(),
                format!(
                    "lib.rs in `{}` has all exports behind named features.",
                    node.rel_dir
                ),
                Some(lib_rel.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_arch_08a_feature_gated_exports_tests/mod.rs"]
mod tests;
