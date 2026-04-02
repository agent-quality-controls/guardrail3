use std::collections::BTreeSet;

use super::{copy_fixture, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_04_loose_files as assertions;

fn replace_child_dir_with_file(root: &std::path::Path, child_rel: &str) {
    remove_dir(root, child_rel);
    write_file(root, child_rel, "// replaced child dir");
}

#[test]
fn replacing_real_child_dirs_with_files_hits_only_the_still_nonempty_containers() {
    let tmp = copy_fixture();
    let replacements = [
        ("apps/backend/crates/app", "commands"),
        ("apps/backend/crates/domain", "engine"),
        ("apps/backend/crates/adapters/inbound", "mcp"),
        ("apps/backend/crates/adapters/outbound", "postgres"),
        ("apps/backend/crates/ports/outbound", "events"),
        ("apps/worker/crates/adapters/outbound", "db"),
    ];

    for (container, child) in &replacements {
        replace_child_dir_with_file(tmp.path(), &format!("{container}/{child}"));
    }

    let results = super::run_family(tmp.path());
    let actual_files = replacements
        .iter()
        .map(|(container, _)| (*container).to_owned())
        .collect::<BTreeSet<_>>();
    assertions::assert_error_summary(
        &results,
        "",
        replacements.len(),
        &actual_files,
        None,
        None,
        None,
        Some(&["that don't belong"]),
    );
}
