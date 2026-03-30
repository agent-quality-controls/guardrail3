use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_23_adapter_pub_trait as assertions;

#[test]
fn inline_adapter_module_with_public_trait_errors() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
        "pub mod nested {\n    pub trait InlineBoundary {\n    }\n}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_single(
        &results,
        "",
        "apps/backend/crates/adapters/outbound/postgres",
    );
}
