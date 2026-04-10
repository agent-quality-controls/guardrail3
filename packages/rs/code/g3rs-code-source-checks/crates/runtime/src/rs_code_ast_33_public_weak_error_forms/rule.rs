use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{PublicResultErrorKind, find_public_result_error_types};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-33";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_public_result_error_types(input.source) {
        let problem = match info.kind {
            PublicResultErrorKind::String => "Result<_, String>",
            PublicResultErrorKind::StrRef => "Result<_, &str>",
            PublicResultErrorKind::AnyhowError => "Result<_, anyhow::Error>",
            PublicResultErrorKind::BoxDynError => "Result<_, Box<dyn Error>>",
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "weak public error form".to_owned(),
            format!(
                "Public function `{}` returns `{problem}`. Use a typed public error instead.",
                info.fn_name
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
