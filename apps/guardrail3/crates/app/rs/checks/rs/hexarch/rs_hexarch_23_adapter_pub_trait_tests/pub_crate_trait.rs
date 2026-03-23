use super::super::super::test_support::{assert_no_error, copy_fixture, run_family, write_file};

#[test]
fn pub_crate_trait_in_adapter_does_not_fire() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
        "pub(crate) trait HiddenRepo {\n}\n",
    );

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-23");
}
