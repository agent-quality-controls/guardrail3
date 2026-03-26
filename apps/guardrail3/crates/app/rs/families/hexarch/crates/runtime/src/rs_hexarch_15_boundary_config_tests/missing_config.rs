use super::super::check_boundary_config_for_test as check_boundary_config;

#[test]
fn non_app_boundaries_do_not_warn() {
    let results = check_boundary_config("packages/shared", false, false, None);

    assert!(
        results.is_empty(),
        "non-app boundaries should stay out of scope for RS-HEXARCH-15: {results:#?}"
    );
}
