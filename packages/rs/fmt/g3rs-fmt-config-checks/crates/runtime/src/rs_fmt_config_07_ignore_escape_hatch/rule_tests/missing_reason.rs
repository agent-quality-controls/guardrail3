use g3rs_fmt_config_checks_assertions::rs_fmt_config_07_ignore_escape_hatch as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_ignore_has_no_waiver_reason() {
    let results = run_check(
        r#"
edition = "2024"
ignore = ["generated/**"]
"#,
        Vec::new(),
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "rustfmt ignore missing reason",
                "`rustfmt.toml` uses `ignore = [\"generated/**\"]` without a matching waiver reason. Add a waiver entry in guardrail3-rs.toml with rule = \"RS-FMT-CONFIG-07\", file = \"rustfmt.toml\", selector = \"ignore\", and a reason explaining why these paths are excluded.",
                "rustfmt.toml",
                false,
            ),
            assertions::warn(
                "rustfmt ignore count",
                "`rustfmt.toml` has 1 rustfmt ignore waiver.",
                "rustfmt.toml",
                false,
            ),
        ],
    );
}
