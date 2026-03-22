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

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;

use self::facts::collect;
use self::inputs::{ExceptionCommentInput, RustCodeFileInput, UnsafeCodeLintInput};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
    let mut results = Vec::new();

    for lint in &facts.unsafe_code_lints {
        let input = UnsafeCodeLintInput::new(lint);
        rs_code_12_unsafe_code_lint::check(&input, &mut results);
    }

    for exception_comment in &facts.exception_comments {
        let input = ExceptionCommentInput::new(exception_comment);
        rs_code_07_exception_comment_inventory::check(&input, &mut results);
    }

    for file in &facts.files {
        let Some(content) = crate::fs::read_file(&tree.abs_path(&file.rel_path)) else {
            continue;
        };
        let Ok(ast) = parse::parse_rust_file(&content) else {
            continue;
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
