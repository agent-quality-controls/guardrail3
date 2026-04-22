use std::collections::BTreeSet;

use g3rs_test_types::{
    G3RsTestAnalyzedSourceFile, G3RsTestComponentFileTreeFacts, G3RsTestFileKind,
    G3RsTestFileTreeChecksInput, G3RsTestFileTreeInputFailure, G3RsTestSourceFile,
};

pub(crate) fn build_file_tree_checks_input(
    root_rel_dir: String,
    cargo_rel_path: String,
    files: Vec<G3RsTestSourceFile>,
    components: Vec<G3RsTestComponentFileTreeFacts>,
    local_package_names: BTreeSet<String>,
    mut input_failures: Vec<G3RsTestFileTreeInputFailure>,
) -> G3RsTestFileTreeChecksInput {
    let (files, mut parse_failures) = crate::source_analysis::analyze_file_tree_files(files);
    let existing_file_paths = files
        .iter()
        .map(|file| file.rel_path.clone())
        .collect::<BTreeSet<_>>();
    let components = enrich_components(components, &files);
    input_failures.append(&mut parse_failures);
    input_failures.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    input_failures
        .dedup_by(|left, right| left.rel_path == right.rel_path && left.message == right.message);

    let local_runtime_packages = components
        .iter()
        .filter_map(|component| component.runtime_package_name.clone())
        .collect::<BTreeSet<_>>();
    let local_assertions_packages = components
        .iter()
        .filter_map(|component| component.assertions_package_name.clone())
        .collect::<BTreeSet<_>>();
    let has_tests = components.iter().any(|component| {
        !component.sidecars.is_empty() || !component.external_harnesses.is_empty()
    }) || files
        .iter()
        .any(crate::source_analysis::file_activates_test_rules);

    G3RsTestFileTreeChecksInput {
        root_rel_dir,
        cargo_rel_path,
        files,
        existing_file_paths,
        components,
        has_tests,
        local_package_names,
        local_runtime_packages,
        local_assertions_packages,
        input_failures,
    }
}

fn enrich_components(
    components: Vec<G3RsTestComponentFileTreeFacts>,
    files: &[G3RsTestAnalyzedSourceFile],
) -> Vec<G3RsTestComponentFileTreeFacts> {
    components
        .into_iter()
        .map(|mut component| {
            component.source_module_names = files
                .iter()
                .filter(|file| {
                    file.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                        && matches!(file.kind, G3RsTestFileKind::Source)
                })
                .filter_map(|file| file.owner_module_name.clone())
                .collect();
            component.sidecar_files = files
                .iter()
                .filter(|file| {
                    file.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                        && matches!(
                            file.kind,
                            G3RsTestFileKind::InternalSidecarMod
                                | G3RsTestFileKind::InternalSidecarSupport
                        )
                })
                .cloned()
                .collect();
            component.external_harness_files = files
                .iter()
                .filter(|file| {
                    file.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                        && matches!(file.kind, G3RsTestFileKind::ExternalHarness)
                })
                .cloned()
                .collect();
            component.assertions_module_files = files
                .iter()
                .filter(|file| {
                    file.component_rel_dir.as_deref() == Some(component.rel_dir.as_str())
                        && matches!(file.kind, G3RsTestFileKind::AssertionsModule)
                })
                .cloned()
                .collect();
            component
        })
        .collect()
}
