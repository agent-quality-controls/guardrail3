use g3rs_test_types::G3RsTestFileKind;
use g3rs_test_types::G3RsTestFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3RsTestFileTreeChecksInput) -> Vec<G3CheckResult> {
    if !input.has_tests {
        return Vec::new();
    }

    let mut results = Vec::new();

    for failure in &input.input_failures {
        crate::input_failures::check(
            &input.root_rel_dir,
            &failure.rel_path,
            &failure.message,
            &mut results,
        );
    }

    crate::owned_sidecar_shape::collect(input, &mut results);
    crate::runtime_assertions_split::collect(input, &mut results);

    for file in &input.files {
        if !matches!(file.kind, G3RsTestFileKind::TestSupport) {
            continue;
        }
        crate::test_support_generic::check(
            &crate::support::TestSupportFileInput::new(
                file,
                &input.files,
                &input.local_runtime_packages,
                &input.local_assertions_packages,
            ),
            &mut results,
        );
    }

    results
}
