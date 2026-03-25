use super::super::super::test_support::{assert_no_error, copy_fixture, run_family, write_file};

#[test]
fn pub_trait_in_non_adapter_crate_is_ignored() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/handlers/src/lib.rs",
        "pub trait BoundaryApi {\n}\n",
    );

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-23");
}
