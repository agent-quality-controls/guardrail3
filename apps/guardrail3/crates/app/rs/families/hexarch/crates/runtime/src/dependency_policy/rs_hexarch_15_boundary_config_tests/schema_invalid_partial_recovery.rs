use guardrail3_app_rs_family_hexarch_assertions::dependency_policy::rs_hexarch_15_boundary_config as assertions;

use super::copy_fixture;

#[test]
fn schema_invalid_guardrail_config_still_flags_missing_app_boundaries_from_raw_keys() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path().join("guardrail3.toml"),
        r#"[rust.checks]
hexarch = true

[rust.apps.backend]
type = "service"

[rust.apps.backend.checks]
hexarch = true
tests = true
"#,
    )
    .expect("write schema-invalid guardrail config");

    let results = super::run_family(tmp.path());
    assertions::assert_title_set(
        &results,
        "",
        3,
        &[
            "guardrail3.toml parse or validation error blocks hexarch boundary checks",
            "app boundary `apps/devctl` missing rust.apps config",
            "app boundary `apps/worker` missing rust.apps config",
        ],
    );
}
