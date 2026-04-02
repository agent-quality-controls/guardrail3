use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::RsTestRoute;
use guardrail3_app_rs_family_view::FamilyView;
#[cfg(feature = "api")]
pub use guardrail3_domain_report::{CheckResult, Severity};

mod analysis;
mod discover;
mod facts;
mod inputs;
mod parse;
mod assertion_quality;
mod mutation;
mod structure;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use self::facts::TestFileKind;

pub fn check(
    surface: &FamilyView,
    route: &RsTestRoute,
    tc: &dyn ToolChecker,
) -> Vec<CheckResult> {
    let tree = surface;
    let facts = discover::collect(tree, route.roots(), tc);
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
        mutation::rs_test_10_input_failures::check(
            &inputs::InputFailureTestInput::new(failure),
            &mut results,
        );
    }

    for root in &facts.roots {
        if !root_is_active_in_scope(root, &facts, route.scoped_files()) {
            continue;
        }
        let analysis = analysis::analyze_root(tree, root, &facts, route.scoped_files());
        let mutation_active =
            root.mutants_exists || root.has_mutants_profile || root.mutation_hook_active;
        let mut had_root_input_failures = false;
        let root_input = inputs::RootTestInput::new(
            root,
            analysis.has_tests,
            analysis.has_tokio_tests,
            facts.cargo_mutants_installed,
            root.mutation_hook_active,
            &root.mutation_hook_files,
        );

        for failure in analysis::active_failures_for_root(&facts, root, &analysis, mutation_active)
        {
            had_root_input_failures = true;
            mutation::rs_test_10_input_failures::check(
                &inputs::InputFailureTestInput::new(failure),
                &mut results,
            );
        }

        if analysis.has_tests {
            structure::rs_test_02_owned_sidecar_shape::collect(
                tree,
                root,
                &analysis.files,
                route.scoped_files(),
                &mut results,
            );

            structure::rs_test_03_runtime_assertions_split::collect(
                tree,
                root,
                &analysis.files,
                route.scoped_files(),
                &facts.local_package_names,
                &mut results,
            );

            for file in &analysis.files {
                let file_input = inputs::TestFileInput::new(&file.facts, &file.parsed);

                if matches!(file.facts.kind, TestFileKind::AssertionsModule) {
                    let empty = BTreeSet::new();
                    let proof_bearing_exported_functions = analysis
                        .proof_bearing_assertions_by_file
                        .get(&file.facts.rel_path)
                        .unwrap_or(&empty);
                    structure::rs_test_16_assertions_modules_prove::check(
                        &inputs::AssertionsModuleInput::new(
                            &file.facts,
                            &file.parsed,
                            proof_bearing_exported_functions,
                        ),
                        &mut results,
                    );
                }

                if analysis::is_test_support_file(root, &file.facts.rel_path) {
                    structure::rs_test_18_test_support_generic::check(
                        &inputs::TestSupportFileInput::new(
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
                        structure::rs_test_01_inline_test_bodies::check(
                            &inputs::CfgTestModuleInput::new(&file.facts, module),
                            &mut results,
                        );
                    }
                }

                assertion_quality::rs_test_04_ignore_reason::check(&file_input, &mut results);

                for function in &file.parsed.test_functions {
                    let proof_bearing_assertion_functions = file
                        .facts
                        .assertions_package_name
                        .as_deref()
                        .and_then(|package| {
                            analysis.proof_bearing_assertions_by_package.get(package)
                        });
                    let function_input = inputs::TestFunctionInput::new(
                        &file.facts,
                        &file.parsed,
                        function,
                        proof_bearing_assertion_functions,
                    );
                    structure::rs_test_16_assertions_modules_prove::check_sidecar_semantic_proof(
                        &function_input,
                        &mut results,
                    );
                    assertion_quality::rs_test_05_should_panic_expected::check(&function_input, &mut results);
                    assertion_quality::rs_test_06_tautological_assertions::check(&function_input, &mut results);
                    assertion_quality::rs_test_07_real_proof_site::check(&function_input, &mut results);
                    assertion_quality::rs_test_08_weak_matches_assert::check(&function_input, &mut results);
                    structure::rs_test_17_external_harnesses_use_assertions::check(
                        &function_input,
                        &mut results,
                    );
                }
            }

            mutation::rs_test_09_nextest_timeouts::check(&root_input, &mut results);
        }

        mutation::rs_test_10_input_failures::emit_inventory_if_clean(
            root,
            &mut results,
            had_root_input_failures,
        );

        if mutation_active {
            mutation::rs_test_11_cargo_mutants_installed::check(&root_input, &mut results);
            mutation::rs_test_12_mutants_toml_exists::check(&root_input, &mut results);
            mutation::rs_test_13_mutants_profile_present::check(&root_input, &mut results);
            mutation::rs_test_14_mutation_hook_present::check(&root_input, &mut results);
            mutation::rs_test_15_mutants_config_sane::check(&root_input, &mut results);
        }
    }

    results
}

fn root_is_active_in_scope(
    root: &facts::TestRootFacts,
    facts: &facts::TestFacts,
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

#[cfg(test)]
pub fn check_test_tree(tree: &ProjectTree, tc: &dyn ToolChecker) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Test]));
    let route = FamilyMapper::new(tree, &scope, None, &selection, None).map_rs_test();
    check(&FamilyView::from_tree(tree), &route, tc)
}
