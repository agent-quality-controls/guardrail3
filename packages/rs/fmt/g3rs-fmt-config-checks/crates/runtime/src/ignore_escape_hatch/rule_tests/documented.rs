use g3rs_fmt_config_checks_assertions::ignore_escape_hatch::rule as assertions;

use super::helpers::{run_check, waiver};

#[test]
fn warns_for_documented_ignore_waiver() {
    let results = run_check(
        r#"
edition = "2024"
ignore = ["generated/**"]
"#,
        vec![waiver("Generated code rewrites break formatter stability.")],
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "rustfmt ignore count",
                "`rustfmt.toml` has 1 rustfmt ignore waiver.",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt ignore waiver",
                "`rustfmt.toml` excludes paths from formatting with documented reason `Generated code rewrites break formatter stability.`: [\"generated/**\"]",
                "rustfmt.toml",
                false,
            ),
        ],
    );
}
