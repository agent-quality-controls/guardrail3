use std::collections::BTreeSet;

use g3rs_test_types::{
    G3RsTestComponentFileTreeFacts, G3RsTestFileTreeChecksInput, G3RsTestFileTreeInputFailure,
    G3RsTestSourceFile,
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
        components,
        has_tests,
        local_package_names,
        local_runtime_packages,
        local_assertions_packages,
        input_failures,
    }
}
