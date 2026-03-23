use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_garde_skips_with_types;

const ID: &str = "RS-CODE-05";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_garde_skips_with_types(input.ast) {
        if info.is_primitive {
            continue;
        }
        let has_comment = input
            .content
            .lines()
            .nth(info.line.saturating_sub(1))
            .is_some_and(|line| line.contains("//"));
        if has_comment {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "garde(skip) without comment".to_owned(),
            message: format!(
                "`#[garde(skip)]` on non-primitive field `{}: {}` requires documentation.",
                info.field_name, info.field_type
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_05_garde_skip_without_comment_tests/mod.rs"]
mod tests;
