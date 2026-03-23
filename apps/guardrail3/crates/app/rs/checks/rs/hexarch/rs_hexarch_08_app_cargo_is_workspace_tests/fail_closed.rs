use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn parse_error_hits_every_mutated_app() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        write_file(tmp.path(), &format!("apps/{app}/Cargo.toml"), "[workspace");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-08");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/Cargo.toml",
        "apps/backend/Cargo.toml",
        "apps/worker/Cargo.toml",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("parse error"));
    }
}
