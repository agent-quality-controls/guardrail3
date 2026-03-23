use std::collections::BTreeSet;

use super::super::super::test_support::{
    copy_fixture, errors_by_id, remove_dir, run_family, write_file,
};

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

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(
        errors.len(),
        replacements.len(),
        "expected one loose-file hit per replaced multi-child container: {errors:#?}"
    );

    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = replacements
        .iter()
        .map(|(container, _)| (*container).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "replacement hits should stay on the owning container path: {errors:#?}"
    );
    for error in &errors {
        assert!(
            error.message.contains("that don't belong"),
            "expected loose-file message, got: '{}'",
            error.message
        );
    }
}
