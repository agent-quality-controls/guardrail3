use std::collections::BTreeSet;
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;
use crate::test_support::{copy_fixture, remove_dir, write_file};

#[test]
fn replacing_outbound_dirs_with_files_hits_every_owned_directional_container() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        &format!("{}/adapters", inner_hex()),
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        &format!("{}/ports", inner_hex()),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
        write_file(tmp.path(), &format!("{dir}/outbound"), "not a directory");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-03");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/adapters".to_owned(),
        "apps/backend/crates/adapters".to_owned(),
        "apps/worker/crates/adapters".to_owned(),
        format!("{}/adapters", inner_hex()),
        "apps/devctl/crates/ports".to_owned(),
        "apps/backend/crates/ports".to_owned(),
        "apps/worker/crates/ports".to_owned(),
        format!("{}/ports", inner_hex()),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        errors.len(),
        8,
        "expected one error per owned directional container with outbound replaced by a file: {errors:#?}"
    );
    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for outbound file replacements: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("outbound"));
    }
}

#[test]
fn replacing_inbound_dirs_with_files_on_outer_roots_does_not_double_fire_nested_hex() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        write_file(
            tmp.path(),
            &format!("apps/{app}/crates/adapters/inbound"),
            "not a directory",
        );
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
        write_file(
            tmp.path(),
            &format!("apps/{app}/crates/ports/inbound"),
            "not a directory",
        );
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-03");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/adapters".to_owned(),
        "apps/backend/crates/adapters".to_owned(),
        "apps/worker/crates/adapters".to_owned(),
        "apps/devctl/crates/ports".to_owned(),
        "apps/backend/crates/ports".to_owned(),
        "apps/worker/crates/ports".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        errors.len(),
        6,
        "expected one error per surviving outer owned directional container with inbound replaced by a file: {errors:#?}"
    );
    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for inbound file replacements: {errors:#?}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "nested hex should be unreachable after replacing backend inbound dirs with files: {errors:#?}"
    );
}
