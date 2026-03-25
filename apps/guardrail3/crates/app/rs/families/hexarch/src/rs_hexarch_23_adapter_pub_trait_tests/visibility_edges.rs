use super::super::super::test_support::{assert_no_error, copy_fixture, run_family, write_file};

#[test]
fn pub_super_trait_in_multi_file_adapter_crate_is_ignored() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
        "mod nested;\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/nested/mod.rs",
        "pub(super) trait InternalBoundary {\n}\n",
    );

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-23");
}
