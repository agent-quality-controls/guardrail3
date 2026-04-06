use g3rs_deny_config_checks_assertions::rs_deny_config_21_unknown_keys as assertions;

use super::helpers::run_check;

#[test]
fn unknown_top_level_section_warns() {
    let results = run_check(
        r#"
[foo]
bar = 1
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown top-level deny key",
            "`deny.toml` uses unknown top-level key `foo`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn multiple_unknown_top_level_keys_warns_each() {
    let results = run_check(
        r#"
[foo]
bar = 1

[zzz]
aaa = 2
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unknown top-level deny key",
                "`deny.toml` uses unknown top-level key `foo`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unknown top-level deny key",
                "`deny.toml` uses unknown top-level key `zzz`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
