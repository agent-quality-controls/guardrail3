use g3_deny_content_checks_assertions::rs_deny_28_unknown_keys as assertions;

use super::helpers::run_check;

#[test]
fn deny_entry_without_name_or_crate_warns() {
    let results = run_check(
        r#"
[bans]
deny = [
    { version = "1.0.0", reason = "no name here" },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unsupported [bans].deny entry schema",
            "`deny.toml` uses unsupported schema for `[bans].deny` entry at index 0; expected string or table with `name` or `crate`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn deny_entry_with_empty_wrapper_warns() {
    let results = run_check(
        r#"
[bans]
deny = [
    { name = "openssl", wrappers = [""] },
]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unsupported [[bans.deny]].wrappers schema",
            "`deny.toml` uses unsupported schema for `[[bans.deny]].wrappers`; expected non-empty strings.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn feature_entry_without_name_warns() {
    let results = run_check(
        r#"
[bans]
[[bans.features]]
deny = ["full"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unsupported [bans.features] entry schema",
            "`deny.toml` uses unsupported schema for `[bans.features]` entry at index 0; expected table with `name` or `crate`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn license_exception_without_name_warns() {
    let results = run_check(
        r#"
[licenses]
[[licenses.exceptions]]
allow = ["MIT"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "unsupported [licenses].exceptions entry schema",
            "`deny.toml` uses unsupported schema for `[licenses].exceptions` entry at index 0; expected table with `name` or `crate`.",
            "deny.toml",
            false,
        )],
    );
}
