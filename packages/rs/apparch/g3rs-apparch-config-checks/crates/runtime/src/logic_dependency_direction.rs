use g3rs_apparch_types::{G3RsApparchCrateDependencyChecksInput, G3RsApparchLayer};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/logic-dependency-direction";

pub(crate) fn check(
    input: &G3RsApparchCrateDependencyChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let krate = &input.krate;
    if krate.layer != Some(G3RsApparchLayer::Logic) {
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
                Some(G3RsApparchLayer::Logic)
                    | Some(G3RsApparchLayer::IoInbound)
                    | Some(G3RsApparchLayer::IoOutbound)
            ) && !crate::run::is_package_internal_assertions_to_runtime_edge(krate, target)
        })
        .collect::<Vec<_>>();

    if violating.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!(
                    "logic crate `{}` depends only on allowed layers",
                    crate::run::display_crate(krate)
                ),
                format!(
                    "Logic crate `{}` has no workspace-internal dependencies on forbidden apparch layers.",
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
                "logic crate `{}` depends on forbidden crate `{}`",
                crate::run::display_crate(krate),
                crate::run::display_crate(target)
            ),
            format!(
                "Logic crate `{}` must not depend on other `logic/` crates or io crates. Move the dependency on `{}` outward into `io/inbound/`, or extract a shared contract into `types/`.",
                crate::run::display_crate(krate),
                crate::run::display_crate(target)
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "logic_dependency_direction_tests/mod.rs"]
mod logic_dependency_direction_tests;
