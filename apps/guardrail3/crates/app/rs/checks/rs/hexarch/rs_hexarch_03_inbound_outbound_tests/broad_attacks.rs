use std::collections::BTreeSet;

use super::super::super::test_support::{
    INNER_HEX, copy_fixture, errors_by_id, remove_dir, run_family,
};

#[test]
fn missing_outbound_in_adapters_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        &format!("{INNER_HEX}/adapters"),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-03");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/adapters".to_owned(),
        "apps/backend/crates/adapters".to_owned(),
        "apps/worker/crates/adapters".to_owned(),
        format!("{INNER_HEX}/adapters"),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("outbound"));
        assert!(error.title.contains("adapters"));
    }
}

#[test]
fn removing_outer_backend_inbound_destroys_the_nested_hex_path_and_does_not_double_fire() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-03");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/adapters".to_owned(),
        "apps/backend/crates/adapters".to_owned(),
        "apps/worker/crates/adapters".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "inner hex should be unreachable after removing backend adapters/inbound: {errors:#?}"
    );
}
