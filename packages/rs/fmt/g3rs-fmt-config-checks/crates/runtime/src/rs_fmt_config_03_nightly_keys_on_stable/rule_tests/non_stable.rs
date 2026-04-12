use g3rs_fmt_config_checks_assertions::rs_fmt_config_03_nightly_keys_on_stable as assertions;

use super::helpers::{parsed_toolchain, run_check};

#[test]
fn stays_quiet_when_toolchain_channel_is_not_stable() {
    let results = run_check(
        r#"
edition = "2024"
group_imports = "StdExternalCrate"
"#,
        parsed_toolchain(
            r#"
[toolchain]
channel = "nightly"
components = ["clippy", "rustfmt"]
"#,
        ),
    );

    assertions::assert_no_findings(&results);
}
