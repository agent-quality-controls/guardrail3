use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn unexpected_directional_dir_hits_only_the_mutated_owned_container() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/ports/sideways/.gitkeep", "");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-03");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl/crates/ports/sideways".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    assert!(
        errors[0]
            .title
            .contains("unexpected directory crates/ports/sideways/")
    );
}
