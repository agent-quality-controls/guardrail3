mod rule;
pub use rule::{check, check_parseable};
#[cfg(test)]
use guardrail3_domain_report::CheckResult;
#[cfg(test)]
use crate::inputs::PolicyContextFailureInput;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    if let Some(parse_error) = facts.policy_context_parse_error.as_deref() {
        check(&PolicyContextFailureInput::new(parse_error), &mut results);
    } else if tree.file_exists("guardrail3.toml") {
        check_parseable(&mut results);
    }
    results
}

#[cfg(test)]
mod tests;
