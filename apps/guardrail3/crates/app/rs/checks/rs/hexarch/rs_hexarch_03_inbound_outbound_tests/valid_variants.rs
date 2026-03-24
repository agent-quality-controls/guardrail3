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

#[test]
fn unexpected_dir_in_adapters_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        &format!("{}/adapters", super::super::super::test_support::INNER_HEX),
    ] {
        write_file(tmp.path(), &format!("{dir}/shared/.gitkeep"), "");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-03");
    assert_eq!(errors.len(), 4, "{errors:#?}");
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("unexpected") && error.title.contains("shared")),
        "{errors:#?}"
    );
}

#[test]
fn unexpected_dir_in_ports_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        &format!("{}/ports", super::super::super::test_support::INNER_HEX),
    ] {
        write_file(tmp.path(), &format!("{dir}/common/.gitkeep"), "");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-03");
    assert_eq!(errors.len(), 4, "{errors:#?}");
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("unexpected") && error.title.contains("common")),
        "{errors:#?}"
    );
}

#[test]
fn deep_unexpected_dir_tree_blames_only_the_top_level_unexpected_dir() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        &format!("{}/adapters", super::super::super::test_support::INNER_HEX),
        &format!("{}/ports", super::super::super::test_support::INNER_HEX),
    ] {
        write_file(
            tmp.path(),
            &format!("{dir}/utils/helpers/deep/lib.rs"),
            "// buried",
        );
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-03");
    assert_eq!(errors.len(), 8, "{errors:#?}");
    assert!(
        errors.iter().all(|error| error.title.contains("utils")),
        "{errors:#?}"
    );
    assert!(
        errors
            .iter()
            .all(|error| !error.title.contains("helpers") && !error.title.contains("deep")),
        "{errors:#?}"
    );
}
