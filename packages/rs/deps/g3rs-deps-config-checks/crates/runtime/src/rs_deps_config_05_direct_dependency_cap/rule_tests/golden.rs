use super::helpers::{
    build_dependency, dev_dependency, run_check, runtime_dependency, target_runtime_dependency,
};

#[test]
fn deduplicates_package_names_across_sections_aliases_and_targets() {
    let mut dependencies = (0..24)
        .map(|index| runtime_dependency(&format!("dep{index}")))
        .collect::<Vec<_>>();
    dependencies.push(runtime_dependency("serde"));
    dependencies.push(build_dependency("serde"));
    dependencies.push(dev_dependency("serde"));
    dependencies.push(target_runtime_dependency("serde", "cfg(unix)"));

    let results = run_check(&dependencies);
    assert!(
        results
            .iter()
            .all(|result| result.id() != "RS-DEPS-CONFIG-05")
    );
}

#[test]
fn counts_external_workspace_and_vendored_path_but_skips_internal_workspace_path() {
    let mut dependencies = (0..23)
        .map(|index| runtime_dependency(&format!("dep{index}")))
        .collect::<Vec<_>>();
    dependencies.push(runtime_dependency("reqwest"));
    dependencies.push(runtime_dependency("serde"));
    dependencies.push(target_runtime_dependency("bytes", "cfg(unix)"));

    let results = run_check(&dependencies);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-05"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Error)
            && result
                .message()
                .contains("Crate `api` has 26 unique direct dependencies")
    }));
}
