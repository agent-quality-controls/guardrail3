use g3rs_apparch_types::{
    G3RsApparchLayer, G3RsApparchPublicItemKind, G3RsApparchTypesPublicSurfaceChecksInput,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/types-public-surface";

pub(crate) fn check(
    input: &G3RsApparchTypesPublicSurfaceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let krate = &input.krate;
    if krate.layer != Some(G3RsApparchLayer::Types) {
        return;
    }

    for fact in input.public_behavior_items.iter().filter(|fact| {
        matches!(
            fact.kind,
            G3RsApparchPublicItemKind::FreeFunction | G3RsApparchPublicItemKind::InherentMethod
        )
    }) {
        let detail = match fact.kind {
            G3RsApparchPublicItemKind::FreeFunction => {
                format!("public free function `{}`", fact.item_name)
            }
            G3RsApparchPublicItemKind::InherentMethod => format!(
                "public inherent method `{}::{}`",
                fact.owner_name.as_deref().unwrap_or("type"),
                fact.item_name
            ),
            G3RsApparchPublicItemKind::Trait => continue,
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!(
                "types crate `{}` exposes behavioral API",
                crate::run::display_crate(krate)
            ),
            format!(
                "Types crate `{}` exposes {}. Keep public behavior out of `types/`; move workflow or implementation logic into `logic/` and keep `types/` focused on contracts and passive data.",
                crate::run::display_crate(krate),
                detail
            ),
            Some(fact.rel_path.clone()),
            None,
        ));
    }

    if input.public_behavior_items.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!(
                    "types crate `{}` keeps public behavior out of its surface",
                    crate::run::display_crate(krate)
                ),
                format!(
                    "Types crate `{}` exposes no public free functions or public inherent methods on concrete types.",
                    crate::run::display_crate(krate)
                ),
                Some(krate.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "types_public_surface_tests/mod.rs"]
mod types_public_surface_tests;
