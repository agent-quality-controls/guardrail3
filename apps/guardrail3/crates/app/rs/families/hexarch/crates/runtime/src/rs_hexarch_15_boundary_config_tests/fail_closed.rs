use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_15_boundary_config as assertions;

#[test]
fn malformed_guardrail_config_warns_in_family_run() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path().join("guardrail3.toml"),
        "[rust.apps.backend\nprofile = \"service\"\n",
    )
    .expect("write malformed guardrail config");

    let results = super::run_family(tmp.path());
    assertions::assert_result_title_contains(
        &results,
        "",
        1,
        &["guardrail3.toml"],
        "parse or validation error",
    );
    assertions::assert_error_title_forbidden(&results, "", &["missing rust.apps config"]);
}
