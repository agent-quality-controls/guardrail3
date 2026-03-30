use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_15_boundary_config as assertions;

#[test]
fn missing_all_app_boundaries_hits_each_app_boundary() {
    let tmp = copy_fixture();
    let guardrail_path = tmp.path().join("guardrail3.toml");
    let guardrail = std::fs::read_to_string(&guardrail_path)
        .expect("failed to read fixture guardrail3.toml for boundary-config test");
    let updated = guardrail
        .replace(
            "\n[rust.apps.backend]\ntype = \"service\"\n\n[rust.apps.backend.checks]\nhexarch = true\ngarde = false\ntest = true\nrelease = false\n",
            "\n",
        )
        .replace(
            "\n[rust.apps.worker]\ntype = \"service\"\n\n[rust.apps.worker.checks]\nhexarch = true\ngarde = false\ntest = true\nrelease = false\n",
            "\n",
        )
        .replace(
            "\n[rust.apps.devctl]\ntype = \"service\"\n\n[rust.apps.devctl.checks]\nhexarch = true\ngarde = false\ntest = true\nrelease = false\n",
            "\n",
        );
    std::fs::write(&guardrail_path, updated).expect("remove all app boundary configs");

    let results = super::run_family(tmp.path());
    assertions::assert_title_set(
        &results,
        "",
        3,
        &[
            "app boundary `apps/backend` missing rust.apps config",
            "app boundary `apps/devctl` missing rust.apps config",
            "app boundary `apps/worker` missing rust.apps config",
        ],
    );
}
