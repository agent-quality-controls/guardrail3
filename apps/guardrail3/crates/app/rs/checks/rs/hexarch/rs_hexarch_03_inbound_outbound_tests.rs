use super::super::test_support::{INNER_HEX, assert_no_error, copy_fixture, errors_by_id, remove_dir, run_family, write_file};

#[test]
fn golden_has_no_rule_03_errors() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-03");
}

#[test]
fn missing_outbound_in_adapters_everywhere_hits_outer_and_inner_hex() {
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
    assert_eq!(errors.len(), 4, "expected one missing outbound per adapters dir: {errors:#?}");
    for error in &errors {
        assert!(error.title.contains("outbound"));
        assert!(error.title.contains("adapters"));
    }
}

#[test]
fn removing_outer_backend_inbound_destroys_inner_hex_path_and_does_not_double_fire() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-03");
    assert_eq!(errors.len(), 3, "expected outer-only errors: {errors:#?}");
    assert!(
        !errors
            .iter()
            .any(|error| error.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "inner hex should be unreachable after removing backend adapters/inbound: {errors:#?}"
    );
}

#[test]
fn unexpected_directional_dir_is_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/ports/sideways/.gitkeep", "");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-03");
    assert_eq!(errors.len(), 1, "expected one unexpected directional dir: {errors:#?}");
    assert!(errors[0].title.contains("unexpected directory crates/ports/sideways/"));
}
