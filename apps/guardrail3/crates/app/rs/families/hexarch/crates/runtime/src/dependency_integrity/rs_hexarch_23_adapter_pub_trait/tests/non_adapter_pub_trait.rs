use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_23_adapter_pub_trait as assertions;

#[test]
fn pub_trait_in_non_adapter_crate_is_ignored() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/handlers/src/lib.rs",
        "pub trait BoundaryApi {\n}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
