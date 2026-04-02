use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_23_adapter_pub_trait as assertions;

#[test]
fn pub_crate_trait_in_adapter_does_not_fire() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
        "pub(crate) trait HiddenRepo {\n}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
