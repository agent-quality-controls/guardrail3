use g3_toolchain_content_checks_assertions::rs_toolchain_02_channel_and_components as assertions;

use super::helpers::run_check;

#[test]
fn warns_for_each_missing_required_component() {
    let results = run_check(
        r#"
[toolchain]
channel = "stable"
components = ["clippy"]
"#,
    );

    assertions::assert_contains(
        &results,
        assertions::warn(
            "toolchain component `rustfmt` missing",
            "Add `rustfmt` to `[toolchain].components`.",
            "rust-toolchain.toml",
            false,
        ),
    );
}
