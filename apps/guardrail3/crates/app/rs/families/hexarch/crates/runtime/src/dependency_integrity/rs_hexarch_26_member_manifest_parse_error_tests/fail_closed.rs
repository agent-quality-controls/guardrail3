use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_26_member_manifest_parse_error as assertions;

use super::copy_fixture;

#[test]
fn malformed_member_manifest_errors_in_family_run() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/domain/engine/Cargo.toml"),
        "[package\nname = \"broken-engine\"\n",
    )
    .expect("write malformed member cargo");

    let results = super::super::results_for_test_root(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/domain/engine/Cargo.toml"],
        Some("cannot verify dependency direction"),
    );
}
