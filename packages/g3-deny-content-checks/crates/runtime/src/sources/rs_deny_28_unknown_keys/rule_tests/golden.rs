use g3_deny_content_checks_assertions::rs_deny_28_unknown_keys as assertions;

use super::helpers::run_check;

#[test]
fn all_known_keys_produce_no_findings() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []

[advisories]
yanked = "warn"

[bans]
multiple-versions = "warn"

[licenses]
allow = ["MIT", "Apache-2.0"]
"#,
    );
    assertions::assert_no_findings(&results);
}

#[test]
fn empty_file_produces_no_findings() {
    let results = run_check("");
    assertions::assert_no_findings(&results);
}
