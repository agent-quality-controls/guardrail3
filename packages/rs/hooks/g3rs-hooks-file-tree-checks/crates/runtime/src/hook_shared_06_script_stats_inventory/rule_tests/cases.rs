use g3rs_hooks_file_tree_checks_assertions::hook_shared_06_script_stats_inventory::rule as assertions;

use super::helpers::script;
use super::super::check;

#[test]
fn inventories_script_stats() {
    let mut results = Vec::new();
    check(&script(".githooks/pre-commit", 3, 48, Some(true)), &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Info,
            "pre-commit script stats",
            "3 lines, 48 bytes",
            Some(".githooks/pre-commit"),
            true,
        ),
    );
}
