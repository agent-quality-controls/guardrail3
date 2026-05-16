#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::line_text;
use crate::parse::visitors::find_forbidden_macros;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/panic-macro";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    for info in find_forbidden_macros(input.source, input.is_test) {
        if info.in_test_context {
            continue;
        }
        let line = info.line;
        let macro_name = info.macro_name;
        let base_name = macro_name.rsplit("::").next().unwrap_or(&macro_name);
        if base_name != "panic" {
            continue;
        }

        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "panic! macro".to_owned(),
            format!(
                "`panic!()` macro found: {}.",
                line_text(input.content, line)
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
        ));
    }
}
