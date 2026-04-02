use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::inputs::RustCodeFileInput;
use crate::parse::{
    GardeSkipInfo, find_garde_skips_with_types, same_line_has_comment, same_line_reason,
};

const ID: &str = "RS-CODE-06";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_garde_skips_with_types(input.ast) {
        if info.is_exempt {
            continue;
        }
        let has_comment = same_line_has_comment(input.content, info.line);
        if !has_comment {
            continue;
        }
        if let Some(reason) = same_line_reason(input.content, info.line) {
            if !reason_text_is_useful(&reason) {
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    "garde(skip) reason too weak".to_owned(),
                    format!(
                        "`#[garde(skip)]` on non-exempt {} reason must be specific and at least two words. Weak reason `{reason}` found.",
                        target_label(&info)
                    ),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                    false,
                ));
                continue;
            }
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "garde(skip) with reason".to_owned(),
                format!(
                    "`#[garde(skip)]` on non-exempt {} reason: {reason}",
                    target_label(&info)
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
                false,
            ));
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "garde(skip) comment missing reason".to_owned(),
            format!(
                "`#[garde(skip)]` on non-exempt {} needs `// reason:`.",
                target_label(&info)
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
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
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]

// reason: test-only sidecar module wiring
mod tests;
