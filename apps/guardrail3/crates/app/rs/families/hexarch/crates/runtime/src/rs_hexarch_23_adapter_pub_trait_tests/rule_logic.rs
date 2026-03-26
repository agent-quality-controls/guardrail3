use super::super::run_source_case;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_23_adapter_pub_trait as assertions;

#[test]
fn adapter_public_trait_errors() {
    let results = run_source_case(
        "api-adapter-http",
        "apps/api/crates/adapters/http",
        1,
        0,
        None,
        None,
    );

    assertions::assert_error_file_single(&results, "", "apps/api/crates/adapters/http");
}
