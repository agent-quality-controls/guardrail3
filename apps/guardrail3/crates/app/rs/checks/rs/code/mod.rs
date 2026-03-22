mod discover;
mod facts;
mod inputs;
mod parse;
mod rs_code_09_file_length;
mod rs_code_10_use_count_error;
mod rs_code_11_use_count_warn;
mod rs_code_12_unsafe_code_lint;
mod rs_code_13_todo_macros;
mod rs_code_14_unwrap_expect;
mod rs_code_15_direct_fs_usage;
mod rs_code_16_panic_macro;
mod rs_code_19_large_type_inventory;

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;

use self::facts::collect;
use self::inputs::{RustCodeFileInput, UnsafeCodeLintInput};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
    let mut results = Vec::new();

    for lint in &facts.unsafe_code_lints {
        let input = UnsafeCodeLintInput::new(lint);
        rs_code_12_unsafe_code_lint::check(&input, &mut results);
    }

    for file in &facts.files {
        let Some(content) = crate::fs::read_file(&tree.abs_path(&file.rel_path)) else {
            continue;
        };
        let Ok(ast) = parse::parse_rust_file(&content) else {
            continue;
        };

        let input = RustCodeFileInput::new(file, &content, &ast);
        rs_code_09_file_length::check(&input, &mut results);
        rs_code_10_use_count_error::check(&input, &mut results);
        rs_code_11_use_count_warn::check(&input, &mut results);
        rs_code_13_todo_macros::check(&input, &mut results);
        rs_code_14_unwrap_expect::check(&input, &mut results);
        rs_code_15_direct_fs_usage::check(&input, &mut results);
        rs_code_16_panic_macro::check(&input, &mut results);
        rs_code_19_large_type_inventory::check(&input, &mut results);
    }

    results
}
