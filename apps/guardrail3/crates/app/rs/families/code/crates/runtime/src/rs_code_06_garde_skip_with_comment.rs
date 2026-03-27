use guardrail3_domain_report::{CheckResult, Severity};

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
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> tempfile::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content).expect("valid rust");
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_06_garde_skip_with_comment_tests/mod.rs"]
mod rs_code_06_garde_skip_with_comment_tests;
