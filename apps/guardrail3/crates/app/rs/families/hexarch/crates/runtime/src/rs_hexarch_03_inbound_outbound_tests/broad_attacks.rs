use std::collections::BTreeSet;
const FIXTURE: test_support::HexarchFixture = test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;
use test_support::{copy_fixture, remove_dir};

#[test]
fn missing_outbound_in_adapters_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        &format!("{}/adapters", inner_hex()),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
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
fn missing_inbound_in_adapters_hits_only_outer_containers_because_nested_hex_becomes_unreachable() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
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
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("inbound") && error.title.contains("adapters")),
        "{errors:#?}"
    );
    assert!(
        !errors.iter().any(|error| error
            .file
            .as_deref()
            .is_some_and(|file| file.starts_with(inner_hex()))),
        "{errors:#?}"
    );
}

#[test]
fn missing_inbound_in_ports_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        &format!("{}/ports", inner_hex()),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-03");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/ports".to_owned(),
        "apps/backend/crates/ports".to_owned(),
        "apps/worker/crates/ports".to_owned(),
        format!("{}/ports", inner_hex()),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("inbound") && error.title.contains("ports")),
        "{errors:#?}"
    );
}

#[test]
fn missing_outbound_in_ports_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        &format!("{}/ports", inner_hex()),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-03");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/ports".to_owned(),
        "apps/backend/crates/ports".to_owned(),
        "apps/worker/crates/ports".to_owned(),
        format!("{}/ports", inner_hex()),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("outbound") && error.title.contains("ports")),
        "{errors:#?}"
    );
}

#[test]
fn both_direction_dirs_missing_in_ports_emit_two_missing_results_per_owned_container() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        &format!("{}/ports", inner_hex()),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-03");
    assert_eq!(errors.len(), 8, "{errors:#?}");
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("/inbound/ directory"))
            .count(),
        4,
        "{errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("/outbound/ directory"))
            .count(),
        4,
        "{errors:#?}"
    );
}

#[test]
fn both_direction_dirs_missing_in_adapters_emit_two_missing_results_per_surviving_outer_container()
{
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-03");
    assert_eq!(errors.len(), 6, "{errors:#?}");
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("inbound"))
            .count(),
        3,
        "{errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("outbound"))
            .count(),
        3,
        "{errors:#?}"
    );
    assert!(
        !errors.iter().any(|error| error
            .file
            .as_deref()
            .is_some_and(|file| file.starts_with(inner_hex()))),
        "{errors:#?}"
    );
}

#[test]
fn removing_outer_backend_inbound_destroys_the_nested_hex_path_and_does_not_double_fire() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
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
