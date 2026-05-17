use g3rs_apparch_types::{G3RsApparchCrateDependencyChecksInput, G3RsApparchLayer};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/types-dependency-direction";

pub(crate) fn check(
    input: &G3RsApparchCrateDependencyChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let krate = &input.krate;
    if krate.layer != Some(G3RsApparchLayer::Types) {
        return;
    }

    let violating = input
        .internal_dependencies
        .iter()
        .filter(|dependency| !dependency.kind.is_dev())
        .map(|dependency| &dependency.target)
        .filter(|target| {
            matches!(
                target.layer,
                Some(
                    G3RsApparchLayer::Types
                        | G3RsApparchLayer::Logic
                        | G3RsApparchLayer::IoInbound
                        | G3RsApparchLayer::IoOutbound
                )
            )
        })
        .collect::<Vec<_>>();

    if violating.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!(
                    "types crate `{}` depends only on allowed layers",
                    crate::run::display_crate(krate)
                ),
                format!(
                    "Types crate `{}` has no workspace-internal dependencies on other apparch layers.",
                    crate::run::display_crate(krate)
                ),
                Some(krate.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    for target in violating {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!(
                "types crate `{}` depends on forbidden crate `{}`",
                crate::run::display_crate(krate),
                crate::run::display_crate(target)
            ),
            format!(
                "Types crate `{}` must not depend on other apparch crates, including other `types/` crates. Remove the dependency on `{}` or move the shared contract into one owning `types/` crate.",
                crate::run::display_crate(krate),
                crate::run::display_crate(target)
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
        ));
    }
}
