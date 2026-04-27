use g3rs_toolchain_config_checks_assertions::channel_and_components::rule as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_channel_is_nightly() {
    let results = run_check(
        r#"
[toolchain]
channel = "nightly"
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "toolchain channel is nightly",
            "Channel is set to nightly. Use `channel = \"stable\"` or a pinned stable version.",
            "rust-toolchain.toml",
            false,
        ),
    );
}

#[test]
fn errors_when_channel_is_beta() {
    let results = run_check(
        r#"
[toolchain]
channel = "beta"
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "toolchain channel is beta",
            "Channel is set to beta. Use `channel = \"stable\"` or a pinned stable version.",
            "rust-toolchain.toml",
            false,
        ),
    );
}
