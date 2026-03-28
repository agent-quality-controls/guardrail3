use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_15_boundary_config as assertions;

#[test]
fn missing_one_app_boundary_only_hits_that_app() {
    let tmp = copy_fixture();
    let guardrail_path = tmp.path().join("guardrail3.toml");
    let guardrail = std::fs::read_to_string(&guardrail_path).expect("read guardrail config");
    let updated = guardrail.replace(
        "\n[rust.apps.worker]\ntype = \"service\"\n\n[rust.apps.worker.checks]\nhexarch = true\ngarde = false\ntest = true\nrelease = false\n",
        "\n",
    );
    std::fs::write(&guardrail_path, updated).expect("remove worker boundary config");

    let results = super::run_family(tmp.path());
    assertions::assert_title_set(
        &results,
        "",
        1,
        &["app boundary `apps/worker` missing rust.apps config"],
    );
}
