use super::super::test_support::{INNER_HEX, assert_no_error, copy_fixture, empty_dir, errors_by_id, run_family};

#[test]
fn golden_has_no_rule_05_errors() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-05");
}

#[test]
fn empty_app_containers_hit_outer_and_inner_hex() {
    let tmp = copy_fixture();
    for path in [
        "apps/devctl/crates/app",
        "apps/backend/crates/app",
        "apps/worker/crates/app",
        &format!("{INNER_HEX}/app"),
    ] {
        empty_dir(tmp.path(), path);
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(errors.len(), 4, "expected one empty-container error per app root: {errors:#?}");
    for error in &errors {
        assert!(error.title.contains("empty container"));
        assert!(error.message.contains("is empty"));
    }
}

#[test]
fn emptying_outer_backend_adapters_inbound_destroys_inner_hex_path() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        empty_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(errors.len(), 3, "expected outer-only empty-container errors: {errors:#?}");
    assert!(
        !errors
            .iter()
            .any(|error| error.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "inner hex should be unreachable after emptying backend adapters/inbound: {errors:#?}"
    );
}
