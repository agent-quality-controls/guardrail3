use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::AppHexarchInput;

const ID: &str = "RS-HEXARCH-01";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.top_level_crates_entry_count > 0 {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("Service `{}` missing crates/ directory", input.app_name),
        message: format!(
            "Service `{}` has no `crates/` directory. Create it with the hex arch template: `crates/{{adapters/{{inbound,outbound}}, app, domain, ports/{{inbound,outbound}}}}` and add optional `crates/macros/` only if needed.",
            input.app_name
        ),
        file: Some(input.app_rel_dir.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub(super) fn check_with_top_level_entries_for_tests(
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
pub(super) fn discovered_app_rel_dirs_for_tests(
    root: &std::path::Path,
) -> std::collections::BTreeSet<String> {
    let tree = test_support::walk(root);
    let route = super::family_route_for_tests(&tree);
    super::facts::collect(&tree, &route)
        .apps
        .into_iter()
        .map(|app| app.app_rel_dir)
        .collect()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_01_crates_exists_tests/mod.rs"]
mod rs_hexarch_01_crates_exists_tests;
