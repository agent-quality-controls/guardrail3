use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn fixture_backed_same_layer_cycle_reports_once() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/types/Cargo.toml",
        "[package]\nname = \"backend-domain-types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nengine_alias = { package = \"backend-domain-engine\", path = \"../engine\" }\n",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-19");

    assert_eq!(
        errors.len(),
        1,
        "expected exactly one same-layer cycle result for the backend fixture mutation: {errors:#?}"
    );
    assert_eq!(errors[0].title, "same-layer domain dependency cycle");
    assert!(
        errors[0]
            .message
            .contains("apps/backend/crates/domain/engine")
            && errors[0]
                .message
                .contains("apps/backend/crates/domain/types"),
        "expected the cycle message to name the real backend domain members: {errors:#?}"
    );
}
