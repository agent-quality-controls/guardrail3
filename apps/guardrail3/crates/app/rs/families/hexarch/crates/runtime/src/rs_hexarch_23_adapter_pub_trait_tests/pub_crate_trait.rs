use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_23_adapter_pub_trait as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn pub_crate_trait_in_adapter_does_not_fire() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
        "pub(crate) trait HiddenRepo {\n}\n",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-23").is_empty(),
        "{results:#?}"
    );
}
