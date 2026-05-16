#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::same_line_has_comment;
use crate::parse::find_garde_skips_with_types;
use crate::parse::types::GardeSkipInfo;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/garde-skip-without-comment";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_garde_skips_with_types(input.source) {
        if info.is_exempt {
            continue;
        }
        if same_line_has_comment(input.content, info.line) {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "garde(skip) without comment".to_owned(),
            format!(
                "`#[garde(skip)]` on non-exempt {} requires documentation. Add a `// reason:` comment explaining why validation is skipped.",
                target_label(&info)
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

/// Implements `target label`.
fn target_label(info: &GardeSkipInfo) -> String {
    if info.is_type_level {
        format!("type `{}`", info.field_name)
    } else {
        format!("field `{}: {}`", info.field_name, info.field_type)
    }
}
