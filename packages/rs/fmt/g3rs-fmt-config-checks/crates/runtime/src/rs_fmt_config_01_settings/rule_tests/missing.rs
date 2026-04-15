use g3rs_fmt_config_checks_assertions::rs_fmt_config_01_settings::rule as assertions;

use super::helpers::{parsed_rustfmt, run_check};

#[test]
fn warns_for_missing_and_wrong_settings() {
    let results = run_check(
        parsed_rustfmt(
            r#"
edition = "2021"
max_width = 120
"#,
        ),
        r#"
[workspace.package]
edition = "2024"
"#,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "rustfmt edition wrong",
                "edition = 2021 but expected 2024. Update edition in rustfmt.toml.",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt max_width wrong",
                "max_width = 120 but expected 100. Update max_width in rustfmt.toml.",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt reorder_imports missing",
                "reorder_imports must be set to true",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt reorder_modules missing",
                "reorder_modules must be set to true",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt style_edition missing",
                "style_edition must be set to 2024",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt tab_spaces missing",
                "tab_spaces must be set to 4",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt use_field_init_shorthand missing",
                "use_field_init_shorthand must be set to true",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt use_try_shorthand missing",
                "use_try_shorthand must be set to true",
                "rustfmt.toml",
                false,
            ),
        ],
    );
}
