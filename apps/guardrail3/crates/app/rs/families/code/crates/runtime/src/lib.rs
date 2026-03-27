mod discover;
mod facts;
mod inputs;
mod parse;
mod rs_code_01_crate_level_allow;
mod rs_code_02_unused_crate_dependencies_allow;
mod rs_code_03_item_level_allow_without_reason;
mod rs_code_04_item_level_allow_with_reason;
mod rs_code_05_garde_skip_without_comment;
mod rs_code_06_garde_skip_with_comment;
mod rs_code_07_exception_comment_inventory;
mod rs_code_08_cfg_attr_allow_inventory;
mod rs_code_09_file_length;
mod rs_code_10_use_count_error;
mod rs_code_11_use_count_warn;
mod rs_code_12_unsafe_code_lint;
mod rs_code_13_todo_macros;
mod rs_code_14_unwrap_expect;
mod rs_code_15_direct_fs_usage;
mod rs_code_16_panic_macro;
mod rs_code_17_impl_allow_blast_radius;
mod rs_code_18_always_true_cfg_attr_bypass;
mod rs_code_19_large_type_inventory;
mod rs_code_20_extern_allow;
mod rs_code_21_fs_glob_import;
mod rs_code_22_deny_forbid_without_reason;
mod rs_code_23_include_bypass;
mod rs_code_24_path_attr;
mod rs_code_25_public_result_error_type;
mod rs_code_26_lib_glob_reexport;
mod rs_code_27_facade_only_lib;
mod rs_code_28_inline_pub_mod_in_lib;
mod rs_code_29_large_trait_inventory;
mod rs_code_30_input_failures;

use {
    glob as _, guardrail3_domain_config as _, guardrail3_domain_modules as _,
    guardrail3_outbound_traits as _, quote as _, semver as _, serde_yaml as _,
};
use guardrail3_app_rs_placement::collect as collect_scope;
use guardrail3_app_rs_family_mapper::RsCodeRoute;
use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

use self::facts::collect;
use self::inputs::{
    CodeInputFailureInput, ExceptionCommentInput, RustCodeFileInput, UnsafeCodeLintInput,
};

#[cfg(test)]
use guardrail3_app_rs_family_code_assertions as _;

#[cfg(test)]
const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/r_arch_01/golden";

pub fn check(tree: &ProjectTree, route: &RsCodeRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        let input = CodeInputFailureInput::new(failure);
        rs_code_30_input_failures::check(&input, &mut results);
    }

    for lint in &facts.unsafe_code_lints {
        let input = UnsafeCodeLintInput::new(lint);
        rs_code_12_unsafe_code_lint::check(&input, &mut results);
    }

    for exception_comment in &facts.exception_comments {
        let input = ExceptionCommentInput::new(exception_comment);
        rs_code_07_exception_comment_inventory::check(&input, &mut results);
    }

    for file in &facts.files {
        if route
            .scoped_files
            .as_ref()
            .is_some_and(|files| !files.contains(&file.rel_path))
        {
            continue;
        }
        let content = match guardrail3_shared_fs::read_file_err(&tree.abs_path(&file.rel_path)) {
            Ok(content) => content,
            Err(read_error) => {
                let message = format!("Failed to read Rust source file: {read_error}");
                let failure = CodeInputFailureInput {
                    rel_path: &file.rel_path,
                    message: &message,
                };
                rs_code_30_input_failures::check(&failure, &mut results);
                continue;
            }
        };
        let ast = match parse::parse_rust_file(&content) {
            Ok(ast) => ast,
            Err(parse_error) => {
                let message = format!("Failed to parse Rust source file: {parse_error}");
                let failure = CodeInputFailureInput {
                    rel_path: &file.rel_path,
                    message: &message,
                };
                rs_code_30_input_failures::check(&failure, &mut results);
                continue;
            }
        };

        let input = RustCodeFileInput::new(file, &content, &ast);
        rs_code_01_crate_level_allow::check(&input, &mut results);
        rs_code_02_unused_crate_dependencies_allow::check(&input, &mut results);
        rs_code_03_item_level_allow_without_reason::check(&input, &mut results);
        rs_code_04_item_level_allow_with_reason::check(&input, &mut results);
        rs_code_05_garde_skip_without_comment::check(&input, &mut results);
        rs_code_06_garde_skip_with_comment::check(&input, &mut results);
        rs_code_08_cfg_attr_allow_inventory::check(&input, &mut results);
        rs_code_09_file_length::check(&input, &mut results);
        rs_code_10_use_count_error::check(&input, &mut results);
        rs_code_11_use_count_warn::check(&input, &mut results);
        rs_code_13_todo_macros::check(&input, &mut results);
        rs_code_14_unwrap_expect::check(&input, &mut results);
        rs_code_15_direct_fs_usage::check(&input, &mut results);
        rs_code_16_panic_macro::check(&input, &mut results);
        rs_code_17_impl_allow_blast_radius::check(&input, &mut results);
        rs_code_18_always_true_cfg_attr_bypass::check(&input, &mut results);
        rs_code_19_large_type_inventory::check(&input, &mut results);
        rs_code_20_extern_allow::check(&input, &mut results);
        rs_code_21_fs_glob_import::check(&input, &mut results);
        rs_code_22_deny_forbid_without_reason::check(&input, &mut results);
        rs_code_23_include_bypass::check(&input, &mut results);
        rs_code_24_path_attr::check(&input, &mut results);
        rs_code_25_public_result_error_type::check(&input, &mut results);
        rs_code_26_lib_glob_reexport::check(&input, &mut results);
        rs_code_27_facade_only_lib::check(&input, &mut results);
        rs_code_28_inline_pub_mod_in_lib::check(&input, &mut results);
        rs_code_29_large_trait_inventory::check(&input, &mut results);
    }

    results
}

#[must_use]
pub fn check_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    check_test_tree(&tree)
}

#[must_use]
pub fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    check(tree, &family_route_for_tests(tree))
}

fn family_route_for_tests(tree: &ProjectTree) -> RsCodeRoute {
    let scope = collect_scope(tree);
    let config = parse_guardrail_config(tree);
    let selected = guardrail3_validation_model::RustFamilySelection::new(
        std::collections::BTreeSet::from([guardrail3_validation_model::RustValidateFamily::Code]),
    );
    guardrail3_app_rs_family_mapper::FamilyMapper::new(
        tree,
        &scope,
        config.as_ref(),
        &selected,
        None,
    )
    .map_rs_code()
}

fn parse_guardrail_config(tree: &ProjectTree) -> Option<GuardrailConfig> {
    tree.file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok())
}

#[cfg(test)]
fn copy_test_fixture() -> test_support::TempDir {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL);
    test_support::copy_tree(&root)
}
