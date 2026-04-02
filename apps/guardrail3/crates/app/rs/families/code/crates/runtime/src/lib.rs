mod api_shape;
mod cfg_and_paths;
mod discover;
mod facts;
mod hygiene;
mod inputs;
mod inventory;
mod lint_policy;
mod parse;

use self::facts::collect;
use self::inputs::{
    CodeInputFailureInput, ExceptionCommentInput, RustCodeFileInput, StructuralCapInput,
    UnsafeCodeLintInput,
};

#[cfg(test)]
use guardrail3_adapters_outbound_fs::RealFileSystem;
#[cfg(test)]
use guardrail3_app_core::project_walker::walk_project;
#[cfg(test)]
use guardrail3_app_rs_family_code_assertions as _;

#[cfg(test)]
const GOLDEN_REL: &str = "../../../../../../../tests/fixtures/full_golden";

pub fn check(
    surface: &guardrail3_app_rs_family_view::FamilyView,
    route: &guardrail3_app_rs_family_mapper::RsCodeRoute,
) -> Vec<guardrail3_domain_report::CheckResult> {
    mark_runtime_dependencies_used();
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        let input = CodeInputFailureInput::new(failure);
        lint_policy::rs_code_30_input_failures::check(&input, &mut results);
    }

    for lint in &facts.unsafe_code_lints {
        let input = UnsafeCodeLintInput::new(lint);
        lint_policy::rs_code_12_unsafe_code_lint::check(&input, &mut results);
    }

    for exception_comment in &facts.exception_comments {
        let input = ExceptionCommentInput::new(exception_comment);
        lint_policy::rs_code_07_exception_comment_inventory::check(&input, &mut results);
    }

    for structural_cap in &facts.structural_caps {
        let input = StructuralCapInput::new(structural_cap);
        inventory::rs_code_35_root_structural_cap::check(&input, &mut results);
    }

    for file in &facts.files {
        if route
            .scoped_files()
            .is_some_and(|files| !files.contains(&file.rel_path))
        {
            continue;
        }
        let Some(abs) = tree.abs_path(&file.rel_path) else { continue };
        let content = match guardrail3_shared_fs::read_file_err(&abs) {
            Ok(content) => content,
            Err(read_error) => {
                let message = format!("Failed to read Rust source file: {read_error}");
                let failure = CodeInputFailureInput {
                    rel_path: &file.rel_path,
                    message: &message,
                };
                lint_policy::rs_code_30_input_failures::check(&failure, &mut results);
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
                lint_policy::rs_code_30_input_failures::check(&failure, &mut results);
                continue;
            }
        };

        let input = RustCodeFileInput::new(file, &content, &ast);
        lint_policy::rs_code_01_crate_level_allow::check(&input, &mut results);
        lint_policy::rs_code_02_unused_crate_dependencies_allow::check(&input, &mut results);
        lint_policy::rs_code_03_item_level_allow_without_reason::check(&input, &mut results);
        lint_policy::rs_code_04_item_level_allow_with_reason::check(&input, &mut results);
        hygiene::rs_code_05_garde_skip_without_comment::check(&input, &mut results);
        hygiene::rs_code_06_garde_skip_with_comment::check(&input, &mut results);
        lint_policy::rs_code_08_cfg_attr_allow_inventory::check(&input, &mut results);
        hygiene::rs_code_09_file_length::check(&input, &mut results);
        hygiene::rs_code_10_use_count_error::check(&input, &mut results);
        hygiene::rs_code_11_use_count_warn::check(&input, &mut results);
        hygiene::rs_code_13_todo_macros::check(&input, &mut results);
        hygiene::rs_code_15_direct_fs_usage::check(&input, &mut results);
        hygiene::rs_code_16_panic_macro::check(&input, &mut results);
        cfg_and_paths::rs_code_17_impl_allow_blast_radius::check(&input, &mut results);
        cfg_and_paths::rs_code_18_always_true_cfg_attr_bypass::check(&input, &mut results);
        inventory::rs_code_19_large_type_inventory::check(&input, &mut results);
        cfg_and_paths::rs_code_20_extern_allow::check(&input, &mut results);
        cfg_and_paths::rs_code_21_fs_glob_import::check(&input, &mut results);
        lint_policy::rs_code_22_deny_forbid_without_reason::check(&input, &mut results);
        cfg_and_paths::rs_code_23_include_bypass::check(&input, &mut results);
        // RS-CODE-24 removed: #[path] detection moved to RS-ARCH-09.
        api_shape::rs_code_31_public_struct_named_fields::check(&input, &mut results);
        api_shape::rs_code_25_public_result_error_type::check(&input, &mut results);
        api_shape::rs_code_33_public_weak_error_forms::check(&input, &mut results);
        api_shape::rs_code_34_generic_parameter_cap::check(&input, &mut results);
        api_shape::rs_code_26_lib_glob_reexport::check(&input, &mut results);
        api_shape::rs_code_27_facade_only_lib::check(&input, &mut results);
        inventory::rs_code_29_large_trait_inventory::check(&input, &mut results);
        inventory::rs_code_32_test_expect_message_quality::check(&input, &mut results);
        cfg_and_paths::rs_code_36_string_dispatch_cap::check(&input, &mut results);
    }

    results
}

fn mark_runtime_dependencies_used() {
    use {
        glob as _, guardrail3_domain_config as _, guardrail3_domain_modules as _,
        guardrail3_outbound_traits as _, quote as _, semver as _, serde_yaml as _,
    };
}

#[cfg(test)]
#[must_use]
pub(crate) fn check_test_root(
    root: &std::path::Path,
) -> Vec<guardrail3_domain_report::CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    let surface = guardrail3_app_rs_family_view::FamilyView::from_tree(&tree);
    check_test_tree(&surface)
}

#[cfg(test)]
#[must_use]
pub(crate) fn check_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<guardrail3_domain_report::CheckResult> {
    check(
        &guardrail3_app_rs_family_view::FamilyView::from_tree(tree),
        &family_route_for_tests(tree),
    )
}

#[cfg(test)]
fn family_route_for_tests(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsCodeRoute {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let config = parse_guardrail_config(tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Code,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(
        tree,
        &scope,
        config.as_ref(),
        &selected,
        None,
    )
    .map_rs_code()
}

#[cfg(test)]
fn parse_guardrail_config(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Option<guardrail3_domain_config::types::GuardrailConfig> {
    tree.file_content("guardrail3.toml").and_then(|content| {
        toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
    })
}

#[cfg(test)]
pub(crate) fn copy_test_fixture() -> test_support::TempDir {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL);
    test_support::copy_tree(&root)
}
