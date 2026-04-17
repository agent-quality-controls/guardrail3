use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestSourceChecksInput;
use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsTestSourceChecksInput) -> Vec<G3CheckResult> {
    let analysis = match crate::support::analyze_root(input) {
        Ok(analysis) => analysis,
        Err(parse_failure) => {
            let mut results = Vec::new();
            crate::rs_test_10_input_failures::check(
                &input.root_rel_dir,
                &parse_failure.rel_path,
                &parse_failure.reason,
                &mut results,
            );
            return results;
        }
    };

    let mut results = Vec::new();

    for file in &analysis.files {
        let file_input = crate::support::TestFileInput::new(&file.file, &file.parsed);

        if matches!(file.file.kind, G3RsTestFileKind::AssertionsModule) {
            let empty = BTreeSet::new();
            let proof_bearing_exported_functions = analysis
                .proof_bearing_assertions_by_file
                .get(&file.file.rel_path)
                .unwrap_or(&empty);
            crate::rs_test_16_assertions_modules_prove::check(
                &crate::support::AssertionsModuleInput::new(
                    &file.file,
                    &file.parsed,
                    proof_bearing_exported_functions,
                ),
                &mut results,
            );
        }

        if matches!(file.file.kind, G3RsTestFileKind::Source) {
            for module in &file.parsed.cfg_test_modules {
                crate::rs_test_01_inline_test_bodies::check(
                    &crate::support::CfgTestModuleInput::new(&file.file, module),
                    &mut results,
                );
            }
        }

        crate::rs_test_04_ignore_reason::check(&file_input, &mut results);

        for function in &file.parsed.test_functions {
            let proof_bearing_assertion_functions = file
                .file
                .assertions_package_name
                .as_deref()
                .and_then(|package| analysis.proof_bearing_assertions_by_package.get(package));
            let function_input = crate::support::TestFunctionInput::new(
                &file.file,
                &file.parsed,
                function,
                proof_bearing_assertion_functions,
            );

            crate::rs_test_16_assertions_modules_prove::check_sidecar_semantic_proof(
                &function_input,
                &mut results,
            );
            crate::rs_test_05_should_panic_expected::check(&function_input, &mut results);
            crate::rs_test_06_tautological_assertions::check(&function_input, &mut results);
            crate::rs_test_07_real_proof_site::check(&function_input, &mut results);
            crate::rs_test_08_weak_matches_assert::check(&function_input, &mut results);
            crate::rs_test_17_external_harnesses_use_assertions::check(
                &function_input,
                &mut results,
            );
        }
    }

    results
}
