use guardrail3_app_rs_family_libarch_assertions::rs_libarch_02_layered_root_workspace_facade as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_02_layered_root_workspace_facade::{
    ExpectedRuleResult, Severity,
};

use test_support::{temp_repo, write_layered_library};

mod golden;

const ROOT_CARGO: &str = "packages/shared/Cargo.toml";

#[test]
fn inventory_when_layered_root_is_workspace_facade() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            file: Some(ROOT_CARGO),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn ignores_layered_root_after_workspace_table_is_removed() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    test_support::write_file(
        tmp.path(),
        ROOT_CARGO,
        "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\napi = { path = \"crates/api\" }\ncore = { path = \"crates/core\" }\ninfra = { path = \"crates/infra\" }\n",
    );

    assertions::assert_rule_quiet(&super::run_family_check(tmp.path()));
}
