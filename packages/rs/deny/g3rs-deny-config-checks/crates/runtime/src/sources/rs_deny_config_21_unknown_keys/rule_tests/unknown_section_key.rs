use g3rs_deny_config_checks_assertions::sources::rs_deny_config_21_unknown_keys::rule as assertions;

use super::helpers::run_check;

#[test]
fn unknown_bans_key_warns() {
    let results = run_check(
        r#"
[bans]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown bans key",
            "`deny.toml` uses unknown `[bans].bogus`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_advisories_key_warns() {
    let results = run_check(
        r#"
[advisories]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown advisories key",
            "`deny.toml` uses unknown `[advisories].bogus`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_sources_key_warns() {
    let results = run_check(
        r#"
[sources]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown sources key",
            "`deny.toml` uses unknown `[sources].bogus`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_licenses_key_warns() {
    let results = run_check(
        r#"
[licenses]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown licenses key",
            "`deny.toml` uses unknown `[licenses].bogus`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_graph_key_warns() {
    let results = run_check(
        r#"
[graph]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown graph key",
            "`deny.toml` uses unknown `[graph].bogus`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_output_key_warns() {
    let results = run_check(
        r#"
[output]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown output key",
            "`deny.toml` uses unknown `[output].bogus`.",
            "deny.toml",
            false,
        )],
    );
}
