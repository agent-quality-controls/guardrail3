use g3rs_fmt_config_checks_assertions::nightly_keys_on_stable::rule as assertions;

use super::helpers::{parsed_toolchain, run_check};

#[test]
fn stays_quiet_when_no_nightly_only_keys_are_present() {
    let results = run_check(
        r#"
edition = "2024"
max_width = 100
"#,
        parsed_toolchain(
            r#"
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
"#,
        ),
    );

    assertions::assert_no_findings(&results);
}
