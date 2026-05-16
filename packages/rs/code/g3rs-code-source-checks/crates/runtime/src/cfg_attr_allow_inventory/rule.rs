#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::find_cfg_attr_lint_policies;
use crate::parse::types::CfgPredicateTruth;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/cfg-attr-allow-inventory";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_cfg_attr_lint_policies(input.source) {
        if info.truth != CfgPredicateTruth::Unknown {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            if info.kind.attr_name() == "allow" {
                "conditional cfg_attr allow".to_owned()
            } else {
                "conditional cfg_attr expect".to_owned()
            },
            format!(
                "Conditional cfg_attr {} for `{}`.",
                info.kind.attr_name(),
                info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}
