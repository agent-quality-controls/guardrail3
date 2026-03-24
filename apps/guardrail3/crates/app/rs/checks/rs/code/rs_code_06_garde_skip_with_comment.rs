use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{GardeSkipInfo, find_garde_skips_with_types, same_line_reason};

const ID: &str = "RS-CODE-06";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_garde_skips_with_types(input.ast) {
        if info.is_primitive {
            continue;
        }
        let same_line = input.content.lines().nth(info.line.saturating_sub(1));
        let has_comment = same_line.is_some_and(|line| line.contains("//"));
        let has_reason = same_line_reason(input.content, info.line).is_some();
        if !has_comment || has_reason {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "garde(skip) comment missing reason".to_owned(),
            message: format!(
                "`#[garde(skip)]` on non-primitive {} needs `// reason:`.",
                target_label(&info)
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

fn target_label(info: &GardeSkipInfo) -> String {
    if info.is_type_level {
        format!("type `{}`", info.field_name)
    } else {
        format!("field `{}: {}`", info.field_name, info.field_type)
    }
}

#[cfg(test)]
#[path = "rs_code_06_garde_skip_with_comment_tests/mod.rs"]
mod tests;
