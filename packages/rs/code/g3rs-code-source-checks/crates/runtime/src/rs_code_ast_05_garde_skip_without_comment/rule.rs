use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{GardeSkipInfo, find_garde_skips_with_types, same_line_has_comment};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-05";

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

fn target_label(info: &GardeSkipInfo) -> String {
    if info.is_type_level {
        format!("type `{}`", info.field_name)
    } else {
        format!("field `{}: {}`", info.field_name, info.field_type)
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
