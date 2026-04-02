mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}
#[cfg(test)]

mod tests;
