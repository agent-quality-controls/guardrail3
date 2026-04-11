use g3rs_test_file_tree_checks_types::G3RsTestFileTreeChecksInput;
use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsTestFileTreeChecksInput) -> Vec<G3CheckResult> {
    let analysis = crate::support::analyze_root(input);
    if !analysis.has_tests {
        return Vec::new();
    }

    let mut results = Vec::new();

    for failure in &analysis.input_failures {
        crate::rs_test_10_input_failures::check(
            &input.root_rel_dir,
            &failure.rel_path,
            &failure.message,
            &mut results,
        );
    }

    crate::rs_test_02_owned_sidecar_shape::collect(input, &analysis.files, &mut results);
    crate::rs_test_03_runtime_assertions_split::collect(input, &analysis, &mut results);

    for file in &analysis.files {
        if !matches!(file.file.kind, G3RsTestFileKind::TestSupport) {
            continue;
        }
        crate::rs_test_18_test_support_generic::check(
            &crate::support::TestSupportFileInput::new(
                &file.file,
                &file.parsed,
                &analysis.local_runtime_packages,
                &analysis.local_assertions_packages,
            ),
            &mut results,
        );
    }

    results
}
