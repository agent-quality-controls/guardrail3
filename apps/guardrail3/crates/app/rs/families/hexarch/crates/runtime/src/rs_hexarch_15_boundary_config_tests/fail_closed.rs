use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_15_boundary_config as assertions;
use super::copy_fixture;

#[test]
fn malformed_guardrail_config_warns_in_family_run() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path().join("guardrail3.toml"),
        "[rust.apps.backend\nprofile = \"service\"\n",
    )
    .expect("write malformed guardrail config");

    let results = super::run_family(tmp.path());
    let warnings = assertions::error_results(&results, "");
    assert_eq!(
        warnings.len(),
        1,
        "expected one parse warning: {warnings:#?}"
    );
    assert!(warnings[0].title.contains("parse error"));
    assert_eq!(warnings[0].file.as_deref(), Some("guardrail3.toml"));
    assert!(
        warnings
            .iter()
            .all(|result| !result.title.contains("missing rust.apps config")),
        "parse failures should block boundary-missing warnings: {warnings:#?}"
    );
}
