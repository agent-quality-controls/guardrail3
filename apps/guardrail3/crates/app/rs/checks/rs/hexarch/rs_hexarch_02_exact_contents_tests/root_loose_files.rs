use std::collections::BTreeSet;

use super::super::super::test_support::{
    INNER_HEX, copy_fixture, errors_by_id, run_family, write_file,
};

#[test]
fn root_loose_files_hit_each_owned_hex_root_once() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");

    assert_eq!(
        errors.len(),
        4,
        "expected one loose-file hit per owned hex root: {errors:#?}"
    );

    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates".to_owned(),
        "apps/backend/crates".to_owned(),
        "apps/worker/crates".to_owned(),
        INNER_HEX.to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected root loose-file hit set: {errors:#?}"
    );

    for error in &errors {
        assert!(error.title.contains("loose files in"));
        assert!(error.message.contains("mod.rs"));
    }
}

#[test]
fn root_gitkeep_is_still_allowed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");

    assert!(
        errors.is_empty(),
        "top-level .gitkeep should remain allowed for rule 02: {errors:#?}"
    );
}
