mod discover;
mod facts;
mod inputs;
mod parse;
mod rs_test_01_cargo_mutants_installed;
mod rs_test_02_mutants_toml_exists;
mod rs_test_03_mutants_profile_present;
mod rs_test_04_tests_exist;
mod rs_test_05_test_coverage_inventory;
mod rs_test_06_integration_tests_exist;
mod rs_test_07_ignore_without_reason;
mod rs_test_08_mutation_hook_present;
mod rs_test_09_no_inline_tests_in_src;
mod rs_test_10_test_function_naming;
mod rs_test_11_cfg_test_module_naming;
mod rs_test_12_nextest_timeouts_present;
mod rs_test_13_should_panic_expected;
mod rs_test_14_tautological_assertions;
mod rs_test_15_test_without_assertions;
mod rs_test_16_test_file_length;
mod rs_test_17_weak_matches_assert;
mod rs_test_18_mutants_config_content;
mod rs_test_19_input_failures;

#[cfg(test)]
mod test_support;

use std::collections::BTreeMap;

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;
use crate::ports::outbound::ToolChecker;

use self::facts::{TestCoverageFacts, TestFileFacts, collect};
use self::inputs::{
    HookTestInput, InputFailureTestInput, RootTestInput, TestCoverageInput, TestFileInput,
    TestFunctionInput, TestModuleInput, ToolTestInput,
};

pub fn check(tree: &ProjectTree, tc: &dyn ToolChecker) -> Vec<CheckResult> {
    let facts = collect(tree, tc);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        rs_test_19_input_failures::check(&InputFailureTestInput::new(failure), &mut results);
    }

    let tool_input = ToolTestInput::new(&facts.tool);
    rs_test_01_cargo_mutants_installed::check(&tool_input, &mut results);

    let hook_input = HookTestInput::new(&facts.hook);
    rs_test_08_mutation_hook_present::check(&hook_input, &mut results);

    let coverage_by_root = analyze_roots(tree, &facts.files, &mut results);

    for root in &facts.roots {
        let root_input = RootTestInput::new(root);
        rs_test_02_mutants_toml_exists::check(&root_input, &mut results);
        rs_test_03_mutants_profile_present::check(&root_input, &mut results);
        rs_test_12_nextest_timeouts_present::check(&root_input, &mut results);
        rs_test_18_mutants_config_content::check(&root_input, &mut results);

        if let Some(coverage) = coverage_by_root.get(&root.rel_dir) {
            let coverage_input = TestCoverageInput::new(coverage);
            rs_test_04_tests_exist::check(&coverage_input, &mut results);
            rs_test_05_test_coverage_inventory::check(&coverage_input, &mut results);
            rs_test_06_integration_tests_exist::check(&coverage_input, &mut results);
        }
    }

    results
}

fn analyze_roots(
    tree: &ProjectTree,
    files: &[TestFileFacts],
    results: &mut Vec<CheckResult>,
) -> BTreeMap<String, TestCoverageFacts> {
    let mut coverage_by_root = BTreeMap::<String, TestCoverageFacts>::new();

    for file in files {
        let content = match crate::fs::read_file_err(&tree.abs_path(&file.rel_path)) {
            Ok(content) => content,
            Err(read_error) => {
                let message =
                    format!("Failed to read Rust source file for test analysis: {read_error}");
                rs_test_19_input_failures::check(
                    &InputFailureTestInput::inline(&file.rel_path, &message),
                    results,
                );
                continue;
            }
        };
        let ast = match parse::parse_rust_file(&content) {
            Ok(ast) => ast,
            Err(parse_error) => {
                let message =
                    format!("Failed to parse Rust source file for test analysis: {parse_error}");
                rs_test_19_input_failures::check(
                    &InputFailureTestInput::inline(&file.rel_path, &message),
                    results,
                );
                continue;
            }
        };

        let parsed = parse::analyze(&ast, &content);

        let coverage = coverage_by_root
            .entry(file.root_rel_dir.clone())
            .or_insert_with(|| TestCoverageFacts::new(file.root_rel_dir.clone()));
        if file.is_integration_test_file {
            coverage.integration_test_exists = true;
        }
        if file.is_src_file && !file.is_test_sidecar_file {
            coverage.public_fn_count = coverage.public_fn_count.saturating_add(parsed.pub_fn_count);
        }
        coverage.test_fn_count = coverage
            .test_fn_count
            .saturating_add(parsed.test_functions.len());
        if !parsed.test_functions.is_empty() {
            coverage.has_any_tests = true;
        }

        let file_input = TestFileInput::new(file, &content, &parsed);
        rs_test_07_ignore_without_reason::check(&file_input, results);
        rs_test_09_no_inline_tests_in_src::check(&file_input, results);
        rs_test_16_test_file_length::check(&file_input, results);

        for module in &parsed.cfg_test_modules {
            rs_test_11_cfg_test_module_naming::check(&TestModuleInput::new(file, module), results);
        }

        for function in &parsed.test_functions {
            let input = TestFunctionInput::new(file, function);
            rs_test_10_test_function_naming::check(&input, results);
            rs_test_13_should_panic_expected::check(&input, results);
            rs_test_14_tautological_assertions::check(&input, results);
            rs_test_15_test_without_assertions::check(&input, results);
            rs_test_17_weak_matches_assert::check(&input, results);
        }
    }

    coverage_by_root
}
