use std::collections::BTreeSet;

use g3rs_test_types::{G3RsTestFileKind, G3RsTestSourceChecksInput};
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsTestSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for input_failure in &input.input_failures {
        crate::input_failures::check(
            &input.root_rel_dir,
            &input_failure.rel_path,
            &input_failure.message,
            &mut results,
        );
    }

    for file in &input.files {
        let file_input = crate::support::TestFileInput::new(file);

        if matches!(file.kind, G3RsTestFileKind::AssertionsModule) {
            let empty = BTreeSet::new();
            crate::assertions_modules_prove::check(
                &crate::support::AssertionsModuleInput::new(
                    file,
                    if file.proof_bearing_exported_functions.is_empty() {
                        &empty
                    } else {
                        &file.proof_bearing_exported_functions
                    },
                ),
                &mut results,
            );
        }

        if matches!(file.kind, G3RsTestFileKind::Source) {
            for module in &file.parsed.cfg_test_modules {
                crate::inline_test_bodies::check(
                    &crate::support::CfgTestModuleInput::new(file, module),
                    &mut results,
                );
            }
        }

        crate::ignore_reason::check(&file_input, &mut results);

        for function in &file.parsed.test_functions {
            let function_input = crate::support::TestFunctionInput::new(
                file,
                function,
                if file.proof_bearing_assertion_functions.is_empty() {
                    None
                } else {
                    Some(&file.proof_bearing_assertion_functions)
                },
            );

            crate::assertions_modules_prove::check_sidecar_semantic_proof(
                &function_input,
                &mut results,
            );
            crate::should_panic_expected::check(&function_input, &mut results);
            crate::tautological_assertions::check(&function_input, &mut results);
            crate::real_proof_site::check(&function_input, &mut results);
            crate::weak_matches_assert::check(&function_input, &mut results);
            crate::external_harnesses_use_assertions::check(&function_input, &mut results);
        }
    }

    results
}
