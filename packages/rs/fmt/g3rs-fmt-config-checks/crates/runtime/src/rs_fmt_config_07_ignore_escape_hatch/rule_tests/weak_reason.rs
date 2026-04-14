use g3rs_fmt_config_checks_assertions::rs_fmt_config_07_ignore_escape_hatch as assertions;

use super::helpers::{run_check, waiver};

#[test]
fn errors_when_ignore_reason_is_too_weak() {
    let results = run_check(
        r#"
edition = "2024"
ignore = ["generated/**"]
"#,
        vec![waiver("temp")],
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "rustfmt ignore reason too weak",
                "`rustfmt.toml` uses `ignore = [\"generated/**\"]` with a weak reason: reason must not be a placeholder. Provide a more specific reason explaining why these paths cannot be formatted.",
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
