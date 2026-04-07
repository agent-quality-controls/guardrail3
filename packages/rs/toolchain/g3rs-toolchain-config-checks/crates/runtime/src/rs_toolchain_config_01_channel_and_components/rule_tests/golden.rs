use g3rs_toolchain_config_checks_assertions::rs_toolchain_config_01_channel_and_components as assertions;

use super::helpers::run_check;

#[test]
fn inventories_stable_channel_and_required_components() {
    let results = run_check(
        r#"
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::info(
                "toolchain channel is stable",
                "channel = \"stable\".",
                "rust-toolchain.toml",
                true,
            ),
            assertions::info(
                "toolchain component `clippy` present",
                "`clippy` is listed in `components`.",
                "rust-toolchain.toml",
                true,
            ),
            assertions::info(
                "toolchain component `rustfmt` present",
                "`rustfmt` is listed in `components`.",
                "rust-toolchain.toml",
                true,
            ),
        ],
    );
}
