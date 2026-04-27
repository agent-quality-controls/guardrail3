use g3rs_toolchain_config_checks_assertions::channel_and_components::rule as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_channel_is_missing() {
    let results = run_check(
        r#"
[toolchain]
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_contains(
        &results,
        assertions::warn(
            "toolchain channel missing",
            "Add `channel = \"stable\"` under `[toolchain]`.",
            "rust-toolchain.toml",
            false,
        ),
    );
}
