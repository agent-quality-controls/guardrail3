use g3rs_apparch_types::G3RsApparchCrateDependencyChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/dev-dependency-direction";

pub(crate) fn check(
    input: &G3RsApparchCrateDependencyChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let krate = &input.krate;
    let Some(source_layer) = krate.layer else {
        return;
    };
    let violating = input
        .internal_dependencies
        .iter()
        .filter(|dependency| dependency.kind.is_dev())
        .filter(|dependency| {
            dependency.target.layer.is_some_and(|target_layer| {
                crate::run::forbidden_runtime_dependency(source_layer, target_layer)
                    && !crate::run::is_package_internal_runtime_to_assertions_dev_edge(
                        krate,
                        &dependency.target,
                    )
            })
        })
        .collect::<Vec<_>>();

    if violating.is_empty() {
        return;
    }

    for dependency in violating {
        let target = &dependency.target;
        let Some(target_layer) = target.layer else {
            continue;
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "dev-dependency direction violation".to_owned(),
            format!(
                "{} crate `{}` dev-depends on forbidden {} crate `{}` via `{}`. Consider restructuring the test dependency instead of reaching across apparch layers.",
                crate::run::layer_label(source_layer),
                crate::run::display_crate(krate),
                crate::run::layer_label(target_layer),
                crate::run::display_crate(target),
                dependency.kind.label()
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "dev_dependency_direction_tests/mod.rs"]
mod dev_dependency_direction_tests;
