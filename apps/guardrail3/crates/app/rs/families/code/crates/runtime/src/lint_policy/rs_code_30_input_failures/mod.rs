mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn run_tree(tree: &guardrail3_app_rs_family_view::FamilyView) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}
#[cfg(test)]
pub(crate) use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};
#[cfg(test)]
pub(crate) fn check_input_failure(rel_path: &str, message: &str) -> Vec<CheckResult> {
    let input = crate::inputs::CodeInputFailureInput { rel_path, message };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
#[cfg(test)]

mod tests;
