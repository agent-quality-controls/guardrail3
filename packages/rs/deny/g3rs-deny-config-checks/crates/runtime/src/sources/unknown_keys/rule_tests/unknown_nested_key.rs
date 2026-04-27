use g3rs_deny_config_checks_assertions::sources::unknown_keys::rule as assertions;

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

#[test]
fn unknown_graph_target_key_warns() {
    let results = run_check(
        r#"
[graph]
targets = [
  { triple = "aarch64-apple-darwin", bogus = true },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown graph.targets key",
            "`deny.toml` uses unknown `[[graph.targets]].bogus` at index 0.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_license_clarify_key_warns() {
    let results = run_check(
        r#"
[licenses]
clarify = [
  { crate = "ring", expression = "MIT", license-files = [{ path = "LICENSE", hash = 1 }], bogus = true },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown licenses.clarify key",
            "`deny.toml` uses unknown `[[licenses.clarify]].bogus` at index 0.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_bans_workspace_dependencies_key_warns() {
    let results = run_check(
        r#"
[bans.workspace-dependencies]
duplicates = "deny"
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown bans.workspace-dependencies key",
            "`deny.toml` uses unknown `[bans.workspace-dependencies].bogus`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn unknown_bans_build_key_warns() {
    let results = run_check(
        r#"
[bans.build]
executables = "deny"
bogus = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unknown bans.build key",
            "`deny.toml` uses unknown `[bans.build].bogus`.",
            "deny.toml",
            false,
        )],
    );
}
