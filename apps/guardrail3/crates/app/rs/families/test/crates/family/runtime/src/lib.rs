mod discover;
mod facts;
mod inputs;
mod parse;
mod rs_test_01_inline_test_bodies;
mod rs_test_02_owned_sidecar_shape;
mod rs_test_03_runtime_assertions_split;
mod rs_test_04_ignore_reason;
mod rs_test_05_should_panic_expected;
mod rs_test_06_tautological_assertions;
mod rs_test_07_real_proof_site;
mod rs_test_08_weak_matches_assert;
mod rs_test_09_nextest_timeouts;
mod rs_test_10_input_failures;
mod rs_test_11_cargo_mutants_installed;
mod rs_test_12_mutants_toml_exists;
mod rs_test_13_mutants_profile_present;
mod rs_test_14_mutation_hook_present;
mod rs_test_15_mutants_config_sane;

use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::ToolChecker;

use self::facts::{DiscoveredTestFile, InputFailureFacts, TestFacts, TestFileKind, TestRootFacts, collect};
use self::inputs::{
    CfgTestModuleInput, InputFailureTestInput, RootTestInput, TestFileInput, TestFunctionInput,
};
use self::parse::{ParsedTestFile, parse_rust_file};

pub(crate) struct AnalyzedFile {
    pub(crate) facts: DiscoveredTestFile,
    pub(crate) parsed: ParsedTestFile,
}

#[derive(Default)]
struct RootAnalysis {
    files: Vec<AnalyzedFile>,
    has_tests: bool,
    has_tokio_tests: bool,
    input_failures: Vec<InputFailureFacts>,
}

pub fn check(
    tree: &ProjectTree,
    tc: &dyn ToolChecker,
    scoped_files: Option<&BTreeSet<String>>,
) -> Vec<CheckResult> {
    let facts = collect(tree, tc);
    let mut results = Vec::new();

    for failure in facts
        .input_failures
        .iter()
        .filter(|failure| failure.rel_path.ends_with("Cargo.toml"))
    {
        rs_test_10_input_failures::check(&InputFailureTestInput::new(failure), &mut results);
    }

    for root in &facts.roots {
        let analysis = analyze_root(tree, root, &facts, scoped_files);
        let mutation_active =
            root.mutants_exists || root.has_mutants_profile || !root.mutation_hook_files.is_empty();
        let root_input = RootTestInput::new(
            root,
            analysis.has_tests,
            analysis.has_tokio_tests,
            facts.cargo_mutants_installed,
            &root.mutation_hook_files,
        );

        for failure in active_failures_for_root(&facts, root, &analysis, mutation_active) {
            rs_test_10_input_failures::check(&InputFailureTestInput::new(failure), &mut results);
        }

        if analysis.has_tests {
            rs_test_02_owned_sidecar_shape::collect(
                tree,
                root,
                &analysis.files,
                scoped_files,
                &mut results,
            );

            rs_test_03_runtime_assertions_split::collect(
                root,
                &analysis.files,
                scoped_files,
                &facts.local_package_names,
                &mut results,
            );

            for file in &analysis.files {
                let file_input = TestFileInput::new(&file.facts, &file.parsed);

                if matches!(file.facts.kind, TestFileKind::Source) {
                    for module in &file.parsed.cfg_test_modules {
                        rs_test_01_inline_test_bodies::check(
                            &CfgTestModuleInput::new(&file.facts, module),
                            &mut results,
                        );
                    }
                }

                rs_test_04_ignore_reason::check(&file_input, &mut results);

                for function in &file.parsed.test_functions {
                    let function_input = TestFunctionInput::new(&file.facts, &file.parsed, function);
                    rs_test_05_should_panic_expected::check(&function_input, &mut results);
                    rs_test_06_tautological_assertions::check(&function_input, &mut results);
                    rs_test_07_real_proof_site::check(&function_input, &mut results);
                    rs_test_08_weak_matches_assert::check(&function_input, &mut results);
                }
            }

            rs_test_09_nextest_timeouts::check(&root_input, &mut results);
        }

        if mutation_active {
            rs_test_11_cargo_mutants_installed::check(&root_input, &mut results);
            rs_test_12_mutants_toml_exists::check(&root_input, &mut results);
            rs_test_13_mutants_profile_present::check(&root_input, &mut results);
            rs_test_14_mutation_hook_present::check(&root_input, &mut results);
            rs_test_15_mutants_config_sane::check(&root_input, &mut results);
        }
    }

    results
}

fn analyze_root(
    tree: &ProjectTree,
    root: &TestRootFacts,
    facts: &TestFacts,
    scoped_files: Option<&BTreeSet<String>>,
) -> RootAnalysis {
    let mut analysis = RootAnalysis::default();

    for file in facts.files.iter().filter(|file| file.root_rel_dir == root.rel_dir) {
        if matches!(
            file.kind,
            TestFileKind::InternalSidecarMod
                | TestFileKind::InternalSidecarSupport
                | TestFileKind::ExternalHarness
        ) {
            analysis.has_tests = true;
        }

        let content = match guardrail3_shared_fs::read_file_err(&tree.abs_path(&file.rel_path)) {
            Ok(content) => content,
            Err(read_error) => {
                analysis.input_failures.push(InputFailureFacts {
                    root_rel_dir: root.rel_dir.clone(),
                    rel_path: file.rel_path.clone(),
                    message: format!(
                        "Failed to read Rust source file for test-family analysis: {read_error}"
                    ),
                });
                continue;
            }
        };

        let ast = match parse_rust_file(&content) {
            Ok(ast) => ast,
            Err(parse_error) => {
                analysis.input_failures.push(InputFailureFacts {
                    root_rel_dir: root.rel_dir.clone(),
                    rel_path: file.rel_path.clone(),
                    message: format!(
                        "Failed to parse Rust source file for test-family analysis: {parse_error}"
                    ),
                });
                continue;
            }
        };

        let parsed = parse::analyze(&ast, &content);
        analysis.has_tests |= !parsed.test_functions.is_empty();
        analysis.has_tokio_tests |= parsed
            .test_functions
            .iter()
            .any(|function| function.uses_tokio_test_attr);
        if scoped_files.is_some_and(|paths| !paths.contains(&file.rel_path)) {
            continue;
        }
        analysis.files.push(AnalyzedFile {
            facts: file.clone(),
            parsed,
        });
    }

    analysis
}

fn active_failures_for_root<'a>(
    facts: &'a TestFacts,
    root: &'a TestRootFacts,
    analysis: &'a RootAnalysis,
    mutation_active: bool,
) -> Vec<&'a InputFailureFacts> {
    let async_active = analysis.has_tests && (root.tokio_dependency_present || analysis.has_tokio_tests);

    facts.input_failures
        .iter()
        .filter(|failure| failure.root_rel_dir == root.rel_dir)
        .chain(analysis.input_failures.iter())
        .filter(|failure| {
            if failure.rel_path.ends_with("nextest.toml") {
                return async_active;
            }
            if failure.rel_path.ends_with("mutants.toml") {
                return mutation_active;
            }
            true
        })
        .collect::<Vec<_>>()
}
