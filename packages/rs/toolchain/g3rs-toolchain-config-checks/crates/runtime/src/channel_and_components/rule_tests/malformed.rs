use g3rs_toolchain_config_checks_assertions::channel_and_components::rule as assertions;

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

#[test]
fn errors_when_channel_head_is_malformed_stable() {
    let results = run_check(
        r#"
[toolchain]
channel = "stable-foo"
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
fn errors_when_channel_head_is_malformed_nightly() {
    let results = run_check(
        r#"
[toolchain]
channel = "nightlyish"
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
fn errors_when_channel_only_appears_in_later_segment() {
    let results = run_check(
        r#"
[toolchain]
channel = "x86_64-unknown-linux-gnu-nightly"
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
fn accepts_stable_target_triple_suffix() {
    let results = run_check(
        r#"
[toolchain]
channel = "stable-x86_64-unknown-linux-gnu"
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_contains(
        &results,
        assertions::info(
            "toolchain channel is stable",
            "channel = \"stable\".",
            "rust-toolchain.toml",
            true,
        ),
    );
}
