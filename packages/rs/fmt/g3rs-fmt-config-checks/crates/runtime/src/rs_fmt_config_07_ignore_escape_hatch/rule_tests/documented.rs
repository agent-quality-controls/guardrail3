use g3rs_fmt_config_checks_assertions::rs_fmt_config_07_ignore_escape_hatch as assertions;

use super::helpers::{escape_hatch, run_check};

#[test]
fn warns_for_documented_ignore_escape_hatch() {
    let results = run_check(
        r#"
edition = "2024"
ignore = ["generated/**"]
"#,
        vec![escape_hatch(
            "Generated code rewrites break formatter stability.",
        )],
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "rustfmt ignore count",
                "`rustfmt.toml` has 1 rustfmt ignore escape hatch.",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt ignore escape hatch",
                "`rustfmt.toml` excludes paths from formatting with documented reason `Generated code rewrites break formatter stability.`: [\"generated/**\"]",
                "rustfmt.toml",
                false,
            ),
        ],
    );
}
