use g3rs_toolchain_config_checks_assertions::channel_and_components::rule as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_channel_is_prerelease_rc() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0-rc.1"
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "toolchain channel is unsupported",
            "Channel value is not recognized. Use `channel = \"stable\"` or a pinned stable version.",
            "rust-toolchain.toml",
            false,
        ),
    );
}

#[test]
fn errors_when_channel_is_prerelease_dev() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0-dev"
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "toolchain channel is unsupported",
            "Channel value is not recognized. Use `channel = \"stable\"` or a pinned stable version.",
            "rust-toolchain.toml",
            false,
        ),
    );
}
