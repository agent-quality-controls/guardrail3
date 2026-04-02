mod rule;
pub use rule::check;
pub(crate) use rule::emit_inventory_if_clean;

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
pub(crate) fn run_family_with_tool(
    root: &std::path::Path,
    cargo_mutants_installed: bool,
) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    let checker = if cargo_mutants_installed {
        test_support::StubToolChecker::with_tools(["cargo-mutants"])
    } else {
        test_support::StubToolChecker::default()
    };
    crate::check_test_tree(&tree, &checker)
}
#[cfg(test)]

mod tests;
