use g3rs_fmt_config_checks_assertions::extra_settings::rule as assertions;

use super::helpers::run_check;

#[test]
fn inventories_empty_skip_macro_invocations() {
    let results = run_check(
        r#"
edition = "2024"
skip_macro_invocations = []
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "rustfmt extra setting: skip_macro_invocations",
            "`skip_macro_invocations` in `rustfmt.toml` is not part of the standard rustfmt baseline. Verify it is intentional.",
            "rustfmt.toml",
            true,
        )],
    );
}

#[test]
fn inventories_nonempty_skip_macro_invocations() {
    let results = run_check(
        r#"
edition = "2024"
skip_macro_invocations = ["sqlx::query!"]
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "rustfmt extra setting: skip_macro_invocations",
            "`skip_macro_invocations` in `rustfmt.toml` is not part of the standard rustfmt baseline. Verify it is intentional.",
            "rustfmt.toml",
            true,
        )],
    );
}
