use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::AppHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-01";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.top_level_crates_entry_count > 0 {
        push_success(
            results,
            ID,
            format!("Service `{}` has crates/ directory", input.app_name),
            format!(
                "Service `{}` owns app-local crates under `{}/crates`.",
                input.app_name, input.app_rel_dir
            ),
            Some(input.app_rel_dir.to_owned()),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!("Service `{}` missing crates/ directory", input.app_name),
        format!(
            "Service `{}` has no `crates/` directory. Create it with the hexarch template: `crates/{{adapters/{{inbound,outbound}}, app, domain, ports/{{inbound,outbound}}}}` and add optional `crates/macros/` only if needed.",
            input.app_name
        ),
        Some(input.app_rel_dir.to_owned()),
        None,
        false,
    ));
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
    let route = crate::family_route_for_tests(&tree);
    crate::facts::collect(&tree, &route)
        .apps
        .into_iter()
        .map(|app| app.app_rel_dir)
        .collect()
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod rs_hexarch_01_crates_exists_tests;
