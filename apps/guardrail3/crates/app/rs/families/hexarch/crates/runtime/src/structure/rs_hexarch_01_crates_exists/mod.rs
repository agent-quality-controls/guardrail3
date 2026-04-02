mod rule;
pub use rule::{check};

#[cfg(test)]
pub(crate) fn check_with_top_level_entries_for_tests(
    top_level_crates_entry_count: usize,
) -> Vec<CheckResult> {
    let input = AppHexarchInput {
        app_name: "backend",
        app_rel_dir: "apps/backend",
        cargo_rel_path: "apps/backend/Cargo.toml",
        cargo_parse_error: None,
        is_workspace: true,
        top_level_crates_entry_count,
        src_dir_exists: false,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
#[cfg(test)]
pub(crate) fn discovered_app_rel_dirs_for_tests(
    root: &std::path::Path,
) -> std::collections::BTreeSet<String> {
    let tree = test_support::walk(root);
    let route = crate::family_route_for_tests(&tree);
    crate::facts::collect(&tree, &route)
        .apps
        .into_iter()
        .map(|app| app.app_rel_dir)
        .collect()
}
#[cfg(test)]
pub(crate) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod tests;
