use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_facade_body_items;

const ID: &str = "RS-CODE-27";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") || !is_lib_rs(input.rel_path) {
        return;
    }

    for item in find_facade_body_items(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "lib.rs should stay facade-only".to_owned(),
            message: format!(
                "lib.rs contains {} `{}`. Keep lib.rs limited to facade declarations and type/const definitions.",
                item.kind, item.name
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(item.line),
            inventory: false,
        });
    }
}

fn is_lib_rs(rel_path: &str) -> bool {
    rel_path.rsplit('/').next().is_some_and(|name| name == "lib.rs")
}

#[cfg(test)]
#[path = "rs_code_27_facade_only_lib_tests.rs"]
mod tests;
