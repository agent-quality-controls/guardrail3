use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::RsTestRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::ToolChecker;

use crate::facts::TestFileKind;

pub fn check(
    surface: &FamilyView,
    route: &RsTestRoute,
    tc: &dyn ToolChecker,
) -> Vec<CheckResult> {
    let tree = surface;
    let facts = crate::discover::collect(tree, route.roots(), tc);
    let mut results = Vec::new();
    let discovered_root_dirs = facts
        .roots
        .iter()
        .map(|root| root.rel_dir.as_str())
        .collect::<BTreeSet<_>>();

    for failure in facts
        .input_failures
        .iter()
        .filter(|failure| failure.rel_path.ends_with("Cargo.toml"))
        .filter(|failure| !discovered_root_dirs.contains(failure.root_rel_dir.as_str()))
    {
        crate::mutation::rs_test_10_input_failures::check(
            &crate::inputs::InputFailureTestInput::new(failure),
            &mut results,
        );
    }

    for root in &facts.roots {
        if !root_is_active_in_scope(root, &facts, route.scoped_files()) {
            continue;
        }
        let analysis = crate::analysis::analyze_root(tree, root, &facts, route.scoped_files());
        let mutation_active =
            root.mutants_exists || root.has_mutants_profile || root.mutation_hook_active;
        let mut had_root_input_failures = false;
        let root_input = crate::inputs::RootTestInput::new(
            root,
            analysis.has_tests,
            analysis.has_tokio_tests,
            facts.cargo_mutants_installed,
            root.mutation_hook_active,
            &root.mutation_hook_files,
        );

        for failure in crate::analysis::active_failures_for_root(&facts, root, &analysis, mutation_active)
        {
            had_root_input_failures = true;
            crate::mutation::rs_test_10_input_failures::check(
                &crate::inputs::InputFailureTestInput::new(failure),
                &mut results,
            );
        }

        if analysis.has_tests {
            crate::structure::rs_test_02_owned_sidecar_shape::collect(
                tree,
                root,
                &analysis.files,
                route.scoped_files(),
                &mut results,
            );

            crate::structure::rs_test_03_runtime_assertions_split::collect(
                tree,
                root,
                &analysis.files,
                route.scoped_files(),
                &facts.local_package_names,
                &mut results,
            );

            for file in &analysis.files {
                let file_input = crate::inputs::TestFileInput::new(&file.facts, &file.parsed);

                if matches!(file.facts.kind, TestFileKind::AssertionsModule) {
                    let empty = BTreeSet::new();
                    let proof_bearing_exported_functions = analysis
                        .proof_bearing_assertions_by_file
                        .get(&file.facts.rel_path)
                        .unwrap_or(&empty);
                    crate::structure::rs_test_16_assertions_modules_prove::check(
                        &crate::inputs::AssertionsModuleInput::new(
                            &file.facts,
                            &file.parsed,
                            proof_bearing_exported_functions,
                        ),
                        &mut results,
                    );
                }

                if crate::analysis::is_test_support_file(root, &file.facts.rel_path) {
                    crate::structure::rs_test_18_test_support_generic::check(
                        &crate::inputs::TestSupportFileInput::new(
                            &file.facts,
                            &file.parsed,
                            &analysis.local_runtime_packages,
                            &analysis.local_assertions_packages,
                        ),
                        &mut results,
                    );
                }

                if matches!(file.facts.kind, TestFileKind::Source) {
                    for module in &file.parsed.cfg_test_modules {
                        crate::structure::rs_test_01_inline_test_bodies::check(
                            &crate::inputs::CfgTestModuleInput::new(&file.facts, module),
                            &mut results,
                        );
                    }
                }

                crate::assertion_quality::rs_test_04_ignore_reason::check(&file_input, &mut results);

                for function in &file.parsed.test_functions {
                    let proof_bearing_assertion_functions = file
                        .facts
                        .assertions_package_name
                        .as_deref()
                        .and_then(|package| {
                            analysis.proof_bearing_assertions_by_package.get(package)
                        });
                    let function_input = crate::inputs::TestFunctionInput::new(
                        &file.facts,
                        &file.parsed,
                        function,
                        proof_bearing_assertion_functions,
                    );
                    crate::structure::rs_test_16_assertions_modules_prove::check_sidecar_semantic_proof(
                        &function_input,
                        &mut results,
                    );
                    crate::assertion_quality::rs_test_05_should_panic_expected::check(&function_input, &mut results);
                    crate::assertion_quality::rs_test_06_tautological_assertions::check(&function_input, &mut results);
                    crate::assertion_quality::rs_test_07_real_proof_site::check(&function_input, &mut results);
                    crate::assertion_quality::rs_test_08_weak_matches_assert::check(&function_input, &mut results);
                    crate::structure::rs_test_17_external_harnesses_use_assertions::check(
                        &function_input,
                        &mut results,
                    );
                }
            }

            crate::mutation::rs_test_09_nextest_timeouts::check(&root_input, &mut results);
        }

        crate::mutation::rs_test_10_input_failures::emit_inventory_if_clean(
            root,
            &mut results,
            had_root_input_failures,
        );

        if mutation_active {
            crate::mutation::rs_test_11_cargo_mutants_installed::check(&root_input, &mut results);
            crate::mutation::rs_test_12_mutants_toml_exists::check(&root_input, &mut results);
            crate::mutation::rs_test_13_mutants_profile_present::check(&root_input, &mut results);
            crate::mutation::rs_test_14_mutation_hook_present::check(&root_input, &mut results);
            crate::mutation::rs_test_15_mutants_config_sane::check(&root_input, &mut results);
        }
    }

    results
}

fn root_is_active_in_scope(
    root: &crate::facts::TestRootFacts,
    facts: &crate::facts::TestFacts,
    scoped_files: Option<&BTreeSet<String>>,
) -> bool {
    let Some(scoped_files) = scoped_files else {
        return true;
    };

    scoped_files.contains(&root.cargo_rel_path)
        || scoped_files.contains(&root.mutants_rel_path)
        || scoped_files.contains(&root.nextest_rel_path)
        || root
            .mutation_hook_files
            .iter()
            .any(|hook_rel_path| scoped_files.contains(hook_rel_path))
        || facts
            .files
            .iter()
            .filter(|file| file.root_rel_dir == root.rel_dir)
            .any(|file| scoped_files.contains(&file.rel_path))
}
