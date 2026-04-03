use guardrail3_app_rs_family_libarch_assertions::rs_libarch_10_infra_not_public_surface as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_10_infra_not_public_surface::{
    ExpectedRuleResult, Severity,
};

use test_support::{remove_dir, temp_repo, write_layered_library};

mod golden;

const ROOT_CARGO: &str = "packages/shared/Cargo.toml";
const ROOT_LIB: &str = "packages/shared/src/lib.rs";

#[test]
fn stays_quiet_when_facade_does_not_export_infra() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());

    assertions::assert_rule_quiet(&super::run_family_check(tmp.path()));
}

#[test]
fn errors_when_facade_exports_infra() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    test_support::write_file(tmp.path(), ROOT_LIB, "pub use infra::InfraType;\n");

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_LIB),
            message_contains: Some("re-exports `infra` crate"),
            ..Default::default()
        }],
    );
}

#[test]
fn stays_quiet_without_infra_even_if_facade_is_unreadable() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    test_support::write_file(
        tmp.path(),
        ROOT_CARGO,
        "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\nmembers = [\"crates/api\", \"crates/core\"]\n\n[workspace.dependencies]\napi = { path = \"crates/api\" }\ncore = { path = \"crates/core\" }\n",
    );
    remove_dir(tmp.path(), "packages/shared/crates/infra");
    test_support::write_file(tmp.path(), ROOT_LIB, "pub use api::{\n");

    assertions::assert_rule_quiet(&super::run_family_check(tmp.path()));
}
