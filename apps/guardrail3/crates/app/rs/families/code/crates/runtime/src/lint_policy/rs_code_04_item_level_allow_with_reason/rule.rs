use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::inputs::RustCodeFileInput;
use crate::parse::{find_item_lint_policies, same_line_reason};

const ID: &str = "RS-CODE-04";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_item_lint_policies(input.ast) {
        let line = info.line;
        let Some(reason) = same_line_reason(input.content, line) else {
            continue;
        };
        if !reason_text_is_useful(&reason) {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            if info.kind.attr_name() == "allow" {
                "item-level allow with reason".to_owned()
            } else {
                "item-level expect with reason".to_owned()
            },
            format!(
                "#[{}({})] reason: {reason}",
                info.kind.attr_name(),
                info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
            false,
        ));
    }
}




// reason: test-only sidecar module wiring
