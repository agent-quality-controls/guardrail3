use g3_toolchain_content_checks_assertions::rs_toolchain_02_channel_and_components as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_toolchain_table_is_missing() {
    let results = run_check("profile = \"minimal\"\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "toolchain table missing",
            "Add a `[toolchain]` table with `channel` and `components`.",
            "rust-toolchain.toml",
            false,
        )],
    );
}
