use g3rs_deny_config_checks_assertions::rs_deny_config_21_unknown_keys as assertions;

use super::helpers::run_check;

#[test]
fn unknown_advisory_ignore_entry_key_warns() {
    let results = run_check(
        r#"
[advisories]
ignore = [
    { id = "RUSTSEC-2024-0001", reason = "valid reason text here", bogus = true },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown advisories.ignore key",
            "`deny.toml` uses unknown `[[advisories.ignore]].bogus` at index 0.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_bans_skip_entry_key_warns() {
    let results = run_check(
        r#"
[bans]
skip = [
    { name = "regex", version = "1.0.0", reason = "valid reason text here", bogus = true },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown bans.skip key",
            "`deny.toml` uses unknown `[[bans.skip]].bogus` at index 0.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_bans_features_entry_key_warns() {
    let results = run_check(
        r#"
[bans]
[[bans.features]]
name = "tokio"
deny = ["full"]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown bans.features key",
            "`deny.toml` uses unknown `[[bans.features]].bogus` at index 0.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_licenses_exceptions_entry_key_warns() {
    let results = run_check(
        r#"
[licenses]
[[licenses.exceptions]]
name = "unicode-ident"
allow = ["MIT"]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown licenses.exceptions key",
            "`deny.toml` uses unknown `[[licenses.exceptions]].bogus` at index 0.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_sources_allow_org_key_warns() {
    let results = run_check(
        r#"
[sources]
[sources.allow-org]
github = ["myorg"]
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown sources.allow-org key",
            "`deny.toml` uses unknown `[sources.allow-org].bogus`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_licenses_private_key_warns() {
    let results = run_check(
        r#"
[licenses]
[licenses.private]
ignore = true
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown licenses.private key",
            "`deny.toml` uses unknown `[licenses.private].bogus`.",
            "deny.toml",
            false,
        )],
    );
}
